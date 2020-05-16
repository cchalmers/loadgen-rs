mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::fmt;
use std::os::raw::c_char;

pub use ffi::root::mlperf;
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
        self.iter.next().map(|q| QuerySample {
            id: q.id,
            index: q.index,
        })
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
pub struct QuerySample {
    id: usize,
    index: usize,
}

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
        self.index
    }

    /// Complete this query with the given data. In test mode this data is used to check accuracy,
    /// otherwise it is dropped.
    pub fn complete(self, data: &[u8]) {
        let response = [QuerySampleResponse {
            id: self.id,
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
            self.index, self.id
        );
        let response = [QuerySampleResponse {
            id: self.id,
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
pub fn start_test<QSL, SUT>(sut: &mut SUT, qsl: &mut QSL, test_settings: &TestSettings)
where
    QSL: QuerySampleLibrary,
    SUT: SystemUnderTest,
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

    unsafe {
        mlperf::c::StartTest(raw_sut, raw_qsl, test_settings);
        mlperf::c::DestroyQSL(raw_qsl);
        mlperf::c::DestroySUT(raw_sut);
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

mod test {
    use super::*;
    struct TestQSL;

    impl QuerySampleLibrary for TestQSL {
        fn name(&self) -> &str {
            "my_qsl"
        }
        fn total_samples(&self) -> usize {
            400
        }
        fn performance_samples(&self) -> usize {
            100
        }
        fn load_samples(&mut self, samples: &[QuerySampleIndex]) {
            eprintln!("load_samples {:?}", samples);
        }
        fn unload_samples(&mut self, samples: &[QuerySampleIndex]) {
            eprintln!("unload_samples {:?}", samples);
        }
    }

    struct TestSUT;

    impl SystemUnderTest for TestSUT {
        fn name(&self) -> &str {
            "my_sut"
        }
        fn issue_query(&mut self, queries: QuerySamples) {
            eprintln!("issue_query({:?})", queries);
            queries.into_iter().for_each(|q| q.complete(&[]))
        }
        fn report_latency_results(&mut self, latencies: &[i64]) {
            eprintln!("report_latency({:?})", latencies)
        }
    }

    #[test]
    // run with --nocapture to see output
    fn test_test() {
        let settings = TestSettings::default();
        start_test(&mut TestSUT, &mut TestQSL, &settings)
    }
}

use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

/// A `QuerySampleLibrary` opinionated wrapper that takes a closure to create a vector of bytes
/// that represents the same.
pub struct Samples<Create, T> {
    total_samples: usize,
    performance_samples: usize,
    create: Create,
    samples: Arc<RwLock<BTreeMap<usize, Arc<T>>>>,
}

impl<Create, T> Samples<Create, T> {
    pub fn new(num_samples: usize, create: Create) -> Samples<Create, T> {
        Samples {
            total_samples: num_samples,
            performance_samples: num_samples,
            create,
            samples: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    fn load(&mut self, ix: usize)
    where
        Create: FnMut(usize) -> T,
    {
        let v = (self.create)(ix);
        let mut map = self.samples.write().unwrap();
        map.insert(ix, Arc::new(v));
    }
}

impl<Create: FnMut(usize) -> T + Sync, T: Send + Sync> QuerySampleLibrary for Samples<Create, T> {
    fn name(&self) -> &str {
        "samples"
    }
    fn total_samples(&self) -> usize {
        self.total_samples
    }
    fn performance_samples(&self) -> usize {
        self.performance_samples
    }
    fn load_samples(&mut self, samples: &[QuerySampleIndex]) {
        samples.iter().for_each(|ix| self.load(*ix))
    }
    fn unload_samples(&mut self, samples: &[QuerySampleIndex]) {
        let mut map = self.samples.write().unwrap();
        samples.iter().for_each(|ix| {
            map.remove(ix);
        });
    }
}

pub struct Query<T> {
    sample: Arc<T>,
    query: QuerySample,
}

impl<T> Query<T> {
    pub fn sample(&self) -> &Arc<T> {
        &self.sample
    }

    pub fn complete(self, result: &[u8]) {
        self.query.complete(result)
    }
}

impl<T> std::borrow::Borrow<T> for Query<T> {
    fn borrow(&self) -> &T {
        &self.sample
    }
}

impl<T> std::ops::Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.sample
    }
}

pub struct Test<T, Run, Report> {
    run: Run,
    report: Report,
    library: Arc<RwLock<BTreeMap<usize, Arc<T>>>>,
}

impl<T, Run, Report> Test<T, Run, Report> {
    pub fn new<Create>(library: &Samples<Create, T>, run: Run, report: Report) -> Self {
        Test {
            run,
            report,
            library: library.samples.clone(),
        }
    }
}

impl<T: Send + Sync, Run: FnMut(Query<T>) + Sync, Report: FnMut(&[i64]) + Sync> SystemUnderTest
    for Test<T, Run, Report>
{
    fn name(&self) -> &str {
        "my_sut"
    }
    fn issue_query(&mut self, queries: QuerySamples) {
        queries.into_iter().for_each(|q| {
            let read_lock = self.library.read().unwrap();
            let sample = read_lock.get(&q.index()).expect("NO SAMPLE").clone();
            let q = Query { sample, query: q };
            (self.run)(q)
        })
    }
    fn report_latency_results(&mut self, latencies: &[i64]) {
        (self.report)(latencies);
    }
}
