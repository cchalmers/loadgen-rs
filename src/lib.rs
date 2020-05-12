pub mod ffi;

use std::fmt;
use std::os::raw::c_char;

use ffi::root::mlperf;
use mlperf::QuerySampleResponse;
pub use mlperf::{QuerySampleIndex, TestSettings};

/// A trait for the system under test. Implementors of this can give this to `start_test` to run a
/// loadgen test. Note that this will need to talk to a `QuerySampleLibrary` to receive the actual
/// data corresponding to a query.
pub trait SystemUnderTest: Sync {
    /// Name of the system.
    fn name(&self) -> &str;

    /// Receieve a vector of queries. Queries can be completed with the result data. Note that a
    /// dropped query will be marked as complete with no data.
    ///
    /// The samples may be from any combination of queries or partial queries.
    fn issue_query(&mut self, queries: QuerySamples);

    /// Receive the latencies of the results. (Default implementation to do nothing)
    #[allow(unused_variables)]
    fn report_latency_results(&mut self, latencies: &[i64]) {}
}

pub struct QuerySamples<'a> {
    iter: std::slice::Iter<'a, mlperf::QuerySample>,
}

impl<'a> Iterator for QuerySamples<'a> {
    type Item = QuerySample;

    fn next(&mut self) -> Option<QuerySample> {
        self.iter.next().map(|q| QuerySample(q.clone()))
    }
}

impl<'a> Drop for QuerySamples<'a> {
    fn drop(&mut self) {
        self.for_each(|q| drop(q))
    }
}

/// A query from loadgen. When finished you can mark a query as complete with the data of the
/// result. This will emit a warning if the query was dropped.
#[derive(Debug)]
pub struct QuerySample(mlperf::QuerySample);

/// Unsafe because providing an incorrect id causes a segfault. The query api is safe because it
/// uses the id it was created with.
unsafe fn query_samples_complete(responses: &[QuerySampleResponse]) {
    mlperf::c::QuerySamplesComplete(
        responses.as_ptr() as *mut QuerySampleResponse,
        responses.len(),
    )
}

impl QuerySample {
    /// Get the index of this query. This index corresponds to an index in the query sample
    /// library.
    pub fn index(&self) -> QuerySampleIndex {
        self.0.index
    }

    /// Complete this query with the given data. In test mode this data is used to check accuracy,
    /// otherwise it is dropped.
    pub fn complete(self, data: &[u8]) {
        let response = [QuerySampleResponse {
            id: self.0.id,
            data: data.as_ptr() as usize,
            size: data.len(),
        }];
        unsafe { query_samples_complete(&response) }
        std::mem::forget(self);
    }
}

impl<'a> fmt::Debug for QuerySamples<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple("QuerySamples")
            .field(&self.iter.as_slice())
            .finish()
    }
}

impl Drop for QuerySample {
    fn drop(&mut self) {
        eprintln!(
            "WARNING query on index {} with id {} was not completed",
            self.0.index, self.0.id
        );
        let response = [QuerySampleResponse {
            id: self.0.id,
            data: 0,
            size: 0,
        }];
        unsafe { query_samples_complete(&response) }
    }
}

/// A trait for the system under test. Implementors of this can give this to `start_test` to run a
/// loadgen test.
pub trait QuerySampleLibrary: Sync {
    /// Name of the library.
    fn name(&self) -> &str;

    /// The total number of samples that can be loaded.
    fn total_samples(&self) -> usize;

    /// The number of samples to load in performance mode.
    fn performance_samples(&self) -> usize;

    /// Preload samples into ram. This function is not timed. (Default implementation to do
    /// nothing).
    #[allow(unused_variables)]
    fn load_samples(&mut self, samples: &[QuerySampleIndex]) {}

    /// Unload samples from ram. This function is not timed. (Default implementation to do
    /// nothing).
    #[allow(unused_variables)]
    fn unload_samples(&mut self, samples: &[QuerySampleIndex]) {}
}

/// Starts the test against SystemUnderTest with the specified settings.
pub fn start_test<QSL, SUT, S>(sut: &mut SUT, qsl: &mut QSL, test_settings: S)
where
    QSL: QuerySampleLibrary,
    SUT: SystemUnderTest,
    S: Into<TestSettings>,
{
    unsafe extern "C" fn load_samples_callback<QSL>(
        ctx: usize,
        samples: *const QuerySampleIndex,
        sample_len: usize,
    ) where
        QSL: QuerySampleLibrary,
    {
        let qsl = (ctx as *mut QSL).as_mut().unwrap();
        let slice = std::slice::from_raw_parts(samples, sample_len);
        qsl.load_samples(slice)
    }
    unsafe extern "C" fn unload_samples_callback<QSL>(
        ctx: usize,
        samples: *const QuerySampleIndex,
        sample_len: usize,
    ) where
        QSL: QuerySampleLibrary,
    {
        let qsl = (ctx as *mut QSL).as_mut().unwrap();
        let slice = std::slice::from_raw_parts(samples, sample_len);
        qsl.unload_samples(slice)
    }

    unsafe extern "C" fn run_query_callback<SUT>(
        ctx: usize,
        sample_ptr: *const mlperf::QuerySample,
        num_samples: usize,
    ) where
        SUT: SystemUnderTest,
    {
        let sut = (ctx as *mut SUT).as_mut().unwrap();
        let slice = std::slice::from_raw_parts(sample_ptr, num_samples);
        sut.issue_query(QuerySamples { iter: slice.iter() })
    }
    unsafe extern "C" fn run_flush_callback() {}
    unsafe extern "C" fn run_report_callback<SUT>(
        ctx: usize,
        latencies: *const i64,
        num_latencies: usize,
    ) where
        SUT: SystemUnderTest,
    {
        let sut = (ctx as *mut SUT).as_mut().unwrap();
        let slice = std::slice::from_raw_parts(latencies, num_latencies);
        sut.report_latency_results(slice)
    }

    let sut_ptr = sut as *mut SUT as usize;
    let name_bytes = sut.name().as_bytes();
    let name_chars = name_bytes.as_ptr() as *const c_char;

    let raw_sut = unsafe {
        mlperf::c::ConstructSUT(
            sut_ptr,
            name_chars,
            name_bytes.len(),
            Some(run_query_callback::<SUT>),
            Some(run_flush_callback),
            Some(run_report_callback::<SUT>),
        )
    };

    let qsl_ptr = qsl as *mut QSL as usize;
    let name_bytes = qsl.name().as_bytes();
    let name_chars = name_bytes.as_ptr() as *const c_char;

    let raw_qsl = unsafe {
        mlperf::c::ConstructQSL(
            qsl_ptr,
            name_chars,
            name_bytes.len(),
            qsl.total_samples(),
            qsl.performance_samples(),
            Some(load_samples_callback::<QSL>),
            Some(unload_samples_callback::<QSL>),
        )
    };

    let settings: TestSettings = test_settings.into();

    unsafe {
        mlperf::c::StartTest(raw_sut, raw_qsl, &settings);
        mlperf::c::DestroyQSL(raw_qsl);
        mlperf::c::DestroySUT(raw_sut);
    }
}

// pub struct QuerySampleLibraryCallbacks {
//     load_samples: Box<dyn FnMut(&[QuerySampleIndex]) + Send>,
//     unload_samples: Box<dyn FnMut(&[QuerySampleIndex]) + Send>,
// }

// impl QuerySampleLibraryCallbacks {
//     pub fn new<L, U>(load: L, unload: U) -> QuerySampleLibraryCallbacks
//     where
//         L: FnMut(&[QuerySampleIndex]) + Send + 'static,
//         U: FnMut(&[QuerySampleIndex]) + Send + 'static,
//     {
//         let load_samples = Box::new(load);
//         let unload_samples = Box::new(unload);
//         QuerySampleLibraryCallbacks {
//             load_samples,
//             unload_samples,
//         }
//     }
// }

// pub struct QuerySampleLibrary {
//     raw: *mut c_void,
//     _callbacks: Box<QuerySampleLibraryCallbacks>,
// }

// impl QuerySampleLibrary {
//     pub fn new(name: &str, num_samples: usize, callbacks: QuerySampleLibraryCallbacks) -> QuerySampleLibrary {
//         let name_bytes = name.as_bytes();
//         let name_chars = name_bytes.as_ptr() as *const c_char;
//         unsafe extern "C" fn load_samples_callback(
//             ctx: usize,
//             samples: *const mlperf::QuerySampleIndex,
//             sample_len: usize,
//         ) {
//             let callbacks_ptr = ctx as *mut QuerySampleLibraryCallbacks;
//             let slice = std::slice::from_raw_parts(samples, sample_len);
//             ((*callbacks_ptr).load_samples)(slice)
//         }
//         unsafe extern "C" fn unload_samples_callback(
//             ctx: usize,
//             samples: *const mlperf::QuerySampleIndex,
//             sample_len: usize,
//         ) {
//             let callbacks_ptr = ctx as *mut QuerySampleLibraryCallbacks;
//             let slice = std::slice::from_raw_parts(samples, sample_len);
//             ((*callbacks_ptr).unload_samples)(slice)
//         }
//         let _callbacks = Box::new(callbacks);
//         let ctx = &*_callbacks as *const QuerySampleLibraryCallbacks as usize;

//         let raw = unsafe {
//             mlperf::c::ConstructQuerySampleLibrary(
//                 ctx,
//                 name_chars,
//                 name_bytes.len(),
//                 num_samples,
//                 num_samples,
//                 Some(load_samples_callback),
//                 Some(unload_samples_callback),
//             )
//         };
//         QuerySampleLibrary { raw, _callbacks }
//     }
// }

// pub struct SystemUnderTestCallbacks {
//     issue_query: Box<dyn FnMut(&[QuerySample]) + Send>,
//     report_latency_results: Box<dyn FnMut(&[i64]) + Send>,
// }

// impl SystemUnderTestCallbacks {
//     pub fn new<Q, R>(issue: Q, report_latency: R) -> SystemUnderTestCallbacks
//     where
//         Q: FnMut(&[QuerySample]) + Send + 'static,
//         R: FnMut(&[i64]) + Send + 'static,
//     {
//         let issue_query = Box::new(issue);
//         let report_latency_results = Box::new(report_latency);
//         SystemUnderTestCallbacks {
//             issue_query,
//             report_latency_results,
//         }
//     }
// }

// pub struct SystemUnderTest {
//     raw: *mut c_void,
//     _callbacks: Box<SystemUnderTestCallbacks>,
// }

// impl SystemUnderTest {
//     pub fn new(name: &str, callbacks: SystemUnderTestCallbacks) -> SystemUnderTest {
//         let name_bytes = name.as_bytes();
//         let name_chars = name_bytes.as_ptr() as *const c_char;

//         // static functions pointers we can hand to the C api. The context is the Box of callbacks
//         // which we call
//         unsafe extern "C" fn run_query_callback(
//             ctx: usize,
//             sample_ptr: *const mlperf::QuerySample,
//             num_samples: usize,
//         ) {
//             let callbacks_ptr = ctx as *mut SystemUnderTestCallbacks;
//             let slice = std::slice::from_raw_parts(sample_ptr, num_samples);
//             ((*callbacks_ptr).issue_query)(slice)
//         }
//         unsafe extern "C" fn run_flush_callback() {}
//         unsafe extern "C" fn run_report_callback(
//             ctx: usize,
//             latencies: *const i64,
//             num_latencies: usize,
//         ) {
//             let callbacks_ptr = ctx as *mut SystemUnderTestCallbacks;
//             let slice = std::slice::from_raw_parts(latencies, num_latencies);
//             ((*callbacks_ptr).report_latency_results)(slice)
//         }
//         let _callbacks = Box::new(callbacks);
//         let ctx = &*_callbacks as *const SystemUnderTestCallbacks as usize;
//         let raw = unsafe {
//             mlperf::c::ConstructSystemUnderTest(
//                 ctx,
//                 name_chars,
//                 name_bytes.len(),
//                 Some(run_query_callback),
//                 Some(run_flush_callback),
//                 Some(run_report_callback),
//             )
//         };
//         SystemUnderTest { raw, _callbacks }
//     }
// }

pub enum Mode {
    Performance {
        issue_unique: bool,
        issue_same: bool,
        issue_same_index: u64,
        sample_count_override: u64,
    },
    Accuracy {
        log_rng_seed: u64,
        log_probability: f64,
    },
}

pub enum Scenario {
    SingleStream {
        expected_latency_ns: u64,
        target_latency_percentile: f64,
    },
    MultiStream {
        target_qps: f64,
        target_latency_ns: u64,
        target_latency_percentile: f64,
        samples_per_query: i32,
        max_async_queries: i32,
    },
    Server {
        target_qps: f64,
        target_latency_ns: u64,
        target_latency_percentile: f64,
        coalesce_queries: bool,
        find_peak_qps_decimals_of_precision: i32,
        find_peak_qps_boundary_step_size: f64,
        max_async_queries: u64,
    },
    Offline {
        expected_qps: f64,
    },
}

pub struct Settings {
    pub scenario: Scenario,
    pub mode: Mode,
    pub min_duration_ms: u64,
    pub max_duration_ms: u64,
    pub min_query_count: u64,
    pub max_query_count: u64,
    pub qsl_rng_seed: u64,
    pub sample_index_rng_seed: u64,
    pub schedule_rng_seed: u64,
    pub print_timestamps: bool,
}

impl Into<TestSettings> for Settings {
    fn into(self) -> TestSettings {
        let mut settings = TestSettings {
            scenario: 0,
            mode: 0,
            single_stream_expected_latency_ns: 0,
            single_stream_target_latency_percentile: 0.0,
            multi_stream_target_qps: 0.0,
            multi_stream_target_latency_ns: 0,
            multi_stream_target_latency_percentile: 0.0,
            multi_stream_samples_per_query: 0,
            multi_stream_max_async_queries: 0,
            server_target_qps: 0.0,
            server_target_latency_ns: 0,
            server_target_latency_percentile: 0.0,
            server_coalesce_queries: false,
            server_find_peak_qps_decimals_of_precision: 0,
            server_find_peak_qps_boundary_step_size: 0.0,
            server_max_async_queries: 0,
            offline_expected_qps: 0.0,
            min_duration_ms: 0,
            max_duration_ms: 0,
            min_query_count: 0,
            max_query_count: 0,
            qsl_rng_seed: 0,
            sample_index_rng_seed: 0,
            schedule_rng_seed: 0,
            accuracy_log_rng_seed: 0,
            accuracy_log_probability: 0.0,
            print_timestamps: false,
            performance_issue_unique: false,
            performance_issue_same: false,
            performance_issue_same_index: 0,
            performance_sample_count_override: 0,
        };
        match self.scenario {
            Scenario::SingleStream {
                expected_latency_ns,
                target_latency_percentile,
            } => {
                settings.scenario = mlperf::TestScenario_SingleStream;
                settings.single_stream_expected_latency_ns = expected_latency_ns;
                settings.single_stream_target_latency_percentile = target_latency_percentile;
            }
            Scenario::MultiStream {
                target_qps,
                target_latency_ns,
                target_latency_percentile,
                samples_per_query,
                max_async_queries,
            } => {
                settings.scenario = mlperf::TestScenario_MultiStream;
                settings.multi_stream_target_qps = target_qps;
                settings.multi_stream_target_latency_ns = target_latency_ns;
                settings.multi_stream_target_latency_percentile = target_latency_percentile;
                settings.multi_stream_samples_per_query = samples_per_query;
                settings.multi_stream_max_async_queries = max_async_queries;
            }
            Scenario::Server {
                target_qps,
                target_latency_ns,
                target_latency_percentile,
                coalesce_queries,
                find_peak_qps_decimals_of_precision,
                find_peak_qps_boundary_step_size,
                max_async_queries,
            } => {
                settings.scenario = mlperf::TestScenario_Server;
                settings.server_target_qps = target_qps;
                settings.server_target_latency_ns = target_latency_ns;
                settings.server_target_latency_percentile = target_latency_percentile;
                settings.server_coalesce_queries = coalesce_queries;
                settings.server_find_peak_qps_decimals_of_precision =
                    find_peak_qps_decimals_of_precision;
                settings.server_find_peak_qps_boundary_step_size = find_peak_qps_boundary_step_size;
                settings.server_max_async_queries = max_async_queries;
            }
            Scenario::Offline { expected_qps } => {
                settings.scenario = mlperf::TestScenario_Offline;
                settings.offline_expected_qps = expected_qps;
            }
        }

        match self.mode {
            Mode::Performance {
                issue_unique,
                issue_same,
                issue_same_index,
                sample_count_override,
            } => {
                settings.mode = mlperf::TestMode_PerformanceOnly;
                settings.performance_issue_unique = issue_unique;
                settings.performance_issue_same = issue_same;
                settings.performance_issue_same_index = issue_same_index;
                settings.performance_sample_count_override = sample_count_override;
            }
            Mode::Accuracy {
                log_rng_seed,
                log_probability,
            } => {
                settings.mode = mlperf::TestMode_AccuracyOnly;
                settings.accuracy_log_rng_seed = log_rng_seed;
                settings.accuracy_log_probability = log_probability;
            }
        }
        settings
    }
}

impl Default for TestSettings {
    fn default() -> TestSettings {
        TestSettings {
            scenario: mlperf::TestScenario_SingleStream,
            mode: mlperf::TestMode_PerformanceOnly,
            single_stream_expected_latency_ns: 1_000_000,
            single_stream_target_latency_percentile: 0.9,

            multi_stream_target_qps: 10.0,
            multi_stream_target_latency_ns: 100_000_000,
            multi_stream_target_latency_percentile: 0.9,
            multi_stream_samples_per_query: 4,
            multi_stream_max_async_queries: 1,

            server_target_qps: 1.0,
            server_target_latency_ns: 100_000_000,
            server_target_latency_percentile: 0.99,
            server_coalesce_queries: false,
            server_find_peak_qps_decimals_of_precision: 1,
            server_find_peak_qps_boundary_step_size: 1.0,
            server_max_async_queries: 1,

            offline_expected_qps: 1.0,

            min_duration_ms: 0,
            max_duration_ms: 10_000,
            min_query_count: 100,
            max_query_count: 1_000_000,
            qsl_rng_seed: 0,
            sample_index_rng_seed: 0,
            schedule_rng_seed: 0,
            accuracy_log_rng_seed: 0,
            accuracy_log_probability: 0.0,
            print_timestamps: false,

            performance_issue_unique: false,
            performance_issue_same: false,
            performance_issue_same_index: 0,
            performance_sample_count_override: 0,
        }
    }
}
