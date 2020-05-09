pub mod ffi;

use std::ffi::c_void;
use std::os::raw::c_char;

use ffi::root::mlperf;



pub struct QSLCallbacks {
    load_samples: Box<dyn FnMut(&[QuerySampleIndex]) + Send>,
    unload_samples: Box<dyn FnMut(&[QuerySampleIndex]) + Send>,
}

impl QSLCallbacks {
    pub fn new<L, U>(load: L, unload: U) -> QSLCallbacks
        where L: FnMut(&[QuerySampleIndex]) + Send + 'static,
              U: FnMut(&[QuerySampleIndex]) + Send + 'static,
    {
        let load_samples = Box::new(load);
        let unload_samples = Box::new(unload);
        QSLCallbacks {
            load_samples,
            unload_samples,
        }
    }
}

pub struct QuerySampleLibrary {
    raw: *mut c_void,
    _callbacks: Box<QSLCallbacks>,
}

impl QuerySampleLibrary {
    pub fn new(name: &str, num_samples: usize, callbacks: QSLCallbacks) -> QuerySampleLibrary {
        let name_bytes = name.as_bytes();
        let name_chars = name_bytes.as_ptr() as *const c_char;
        unsafe extern "C" fn load_samples_callback(
            ctx: usize,
            samples: *const mlperf::QuerySampleIndex,
            sample_len: usize,
        ) {
            let callbacks_ptr = ctx as *mut QSLCallbacks;
            let slice = std::slice::from_raw_parts(samples, sample_len);
            ((*callbacks_ptr).load_samples)(slice)
        }
        unsafe extern "C" fn unload_samples_callback(
            ctx: usize,
            samples: *const mlperf::QuerySampleIndex,
            sample_len: usize,
        ) {
            let callbacks_ptr = ctx as *mut QSLCallbacks;
            let slice = std::slice::from_raw_parts(samples, sample_len);
            ((*callbacks_ptr).unload_samples)(slice)
        }
        let _callbacks = Box::new(callbacks);
        let ctx = &*_callbacks as *const QSLCallbacks as usize;

        let raw = unsafe { mlperf::c::ConstructQSL(
            ctx,
            name_chars,
            name_bytes.len(),
            num_samples,
            num_samples,
            Some(load_samples_callback),
            Some(unload_samples_callback),
        )};
        QuerySampleLibrary { raw, _callbacks }
    }
}

pub use mlperf::QuerySampleIndex;
pub use mlperf::QuerySample;
pub use mlperf::QuerySampleResponse;
pub use mlperf::TestSettings;

pub struct SystemUnderTestCallbacks {
    issue_query: Box<dyn FnMut(&[QuerySample]) + Send>,
    report_latency_results: Box<dyn FnMut(&[i64]) + Send>,
}

impl SystemUnderTestCallbacks {
    // pub fn new(issue: impl FnMut(&[QuerySample]) + Send + 'static,
    //            report_latency: impl FnMut(&[i64]) + Send + 'static)
    //     -> SystemUnderTestCallbacks
    // {
    //     let issue_query = Box::new(issue);
    //     let report_latency_results = Box::new(report_latency);
    //     SystemUnderTestCallbacks {
    //         issue_query,
    //         report_latency_results,
    //     }
    // }
    pub fn new<Q, R>(issue: Q, report_latency: R) -> SystemUnderTestCallbacks
        where Q: FnMut(&[QuerySample]) + Send + 'static,
              R: FnMut(&[i64]) + Send + 'static,
    {
        let issue_query = Box::new(issue);
        let report_latency_results = Box::new(report_latency);
        SystemUnderTestCallbacks {
            issue_query,
            report_latency_results,
        }
    }
}

pub struct SystemUnderTest {
    raw: *mut c_void,
    _callbacks: Box<SystemUnderTestCallbacks>,
}

impl SystemUnderTest {
    pub fn new(name: &str, callbacks: SystemUnderTestCallbacks) -> SystemUnderTest {
        let name_bytes = name.as_bytes();
        let name_chars = name_bytes.as_ptr() as *const c_char;

        // static functions pointers we can hand to the C api. The context is the Box of callbacks
        // which we call
        unsafe extern fn run_query_callback(ctx: usize, sample_ptr: *const QuerySample, num_samples: usize) {
            let callbacks_ptr = ctx as *mut SystemUnderTestCallbacks;
            let slice = std::slice::from_raw_parts(sample_ptr, num_samples);
            ((*callbacks_ptr).issue_query)(slice)
        }
        unsafe extern fn run_flush_callback() {
        }
        unsafe extern fn run_report_callback(ctx: usize, latencies: *const i64, num_latencies: usize) {
            let callbacks_ptr = ctx as *mut SystemUnderTestCallbacks;
            let slice = std::slice::from_raw_parts(latencies, num_latencies);
            ((*callbacks_ptr).report_latency_results)(slice)
        }
        let _callbacks = Box::new(callbacks);
        let ctx = &*_callbacks as *const SystemUnderTestCallbacks as usize;
        let raw = unsafe {
            mlperf::c::ConstructSUT(
            ctx,
            name_chars,
            name_bytes.len(),
            Some(run_query_callback),
            Some(run_flush_callback),
            Some(run_report_callback),
            )};
        SystemUnderTest {
            raw,
            _callbacks,
        }
    }
}

pub fn test_settings() -> TestSettings {
    TestSettings {
        scenario: 0,
        mode: 0,
        single_stream_expected_latency_ns: 1_000,
        single_stream_target_latency_percentile: 0.8,
        multi_stream_target_qps: 0.8,
        multi_stream_target_latency_ns: 1_000,
        multi_stream_target_latency_percentile: 0.8,
        multi_stream_samples_per_query: 12,
        multi_stream_max_async_queries: 20,
        server_target_qps: 123.4,
        server_target_latency_ns: 1_000,
        server_target_latency_percentile: 0.8,
        server_coalesce_queries: true,
        server_find_peak_qps_decimals_of_precision: 3,
        server_find_peak_qps_boundary_step_size: 5.5,
        server_max_async_queries: 4,
        offline_expected_qps: 400.7,
        min_duration_ms: 1,
        max_duration_ms: 100,
        min_query_count: 12,
        max_query_count: 1_000_000,
        qsl_rng_seed: 7,
        sample_index_rng_seed: 6,
        schedule_rng_seed: 5,
        accuracy_log_rng_seed: 4,
        accuracy_log_probability: 0.5,
        print_timestamps: true,
        performance_issue_unique: true,
        performance_issue_same: true,
        performance_issue_same_index: 12,
        performance_sample_count_override: 7,
    }
}

/// Starts the test against SUT with the specified settings.
pub fn start_test(sut: &SystemUnderTest, qsl: &QuerySampleLibrary, test_settings: &TestSettings) {
    unsafe {
        mlperf::c::StartTest(sut.raw, qsl.raw, &*test_settings)
    }
}

/// Query Samples are unsafe. 
pub unsafe fn query_samples_complete(responses: &[QuerySampleResponse]) {
    mlperf::c::QuerySamplesComplete(responses.as_ptr() as *mut QuerySampleResponse, responses.len())
}
