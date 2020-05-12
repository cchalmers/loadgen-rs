//// #[cxx::bridge(namespace = org::example)]
//#[cxx::bridge(namespace = mlperf)]
//mod ffi {
//    struct SharedThing {
//        z: i32,
//        y: Box<ThingR>,
//        x: UniquePtr<ThingC>,
//    }

//    extern "C" {
//        include!("cbits/ccc.h");

//        type LogSettings;

//        // type QuerySample;
//        type QuerySampleResponse;
//        type QuerySampleLibrary;
//        type SystemUnderTest;

//        type TestSettings;

//        // type ResponseId;
//        // type QuerySampleIndex;
//        // type QuerySampleLatency;

//        // fn QuerySamplesComplete(responses: *mut QuerySampleResponse, response_count: usize);

//        /// SystemUnderTest calls this to notify loadgen of completed samples.
//        ///
//        ///  - The samples may be from any combination of queries or partial queries as issued by
//        ///    SystemUnderTest::IssueQuery.
//        ///
//        ///  - The SystemUnderTest is responsible for allocating and owning the response data which must remain
//        ///    valid for the duration of this call. The loadgen will copy the response data if
//        ///    needed (e.g. for accuracy mode). Note: This setup requires the allocation to be
//        ///    timed, which will benefit SystemUnderTests that efficiently recycle response memory. All calls to
//        ///    QuerySampleComplete are thread-safe and wait-free bounded.
//        ///
//        ///  - Any number of threads can call QuerySampleComplete simultaneously. Regardless of
//        ///    where any other thread stalls, the current thread will finish QuerySampleComplete in
//        ///    a bounded number of cycles.
//        ///
//        fn query_samples_complete(responses: &mut QuerySampleResponse, response_count: usize);

//        /// Starts the test against SystemUnderTest with the specified settings.
//        fn start_test(
//            sut: &mut SystemUnderTest,
//            qsl: &mut QuerySampleLibrary,
//            requested_settings: &TestSettings,
//            log_settings: &LogSettings,
//        );

//        type ThingC;
//        fn make_demo(appname: &str) -> UniquePtr<ThingC>;
//        fn get_name(thing: &ThingC) -> &CxxString;
//        fn do_thing(state: SharedThing);
//    }

//    extern "Rust" {
//        type ThingR;
//        fn print_r(r: &ThingR);
//    }
//}

//pub struct ThingR(usize);

//fn print_r(r: &ThingR) {
//    println!("called back with r={}", r.0);
//}

//fn main() {
//    let x = ffi::make_demo("demo of cxx::bridge");
//    println!("this is a {}", ffi::get_name(x.as_ref().unwrap()));

//    ffi::do_thing(ffi::SharedThing {
//        z: 222,
//        y: Box::new(ThingR(333)),
//        x,
//    });
//}

struct MyQuerySampleLibrary;

// pub trait SystemUnderTest: Sync {
// }

impl QuerySampleLibrary for MyQuerySampleLibrary {
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

struct MySystemUnderTest;

impl SystemUnderTest for MySystemUnderTest {
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

use loadgen::*;
use std::sync::mpsc;

use std::sync::{Arc, Mutex};
use std::time::Instant;

fn main() {
    let settings = TestSettings::default();
    start_test(&mut MySystemUnderTest, &mut MyQuerySampleLibrary, settings)
    // let timer = Arc::new(Mutex::new(Instant::now()));
    // let stimer = timer.clone();
    // let (sender, receiver) = mpsc::channel::<Vec<QuerySample>>();
    // std::thread::spawn(move || {
    //     for samples in receiver {
    //         for sample in samples {
    //             let diff = stimer.lock().unwrap().elapsed();
    //             eprintln!("{:?} got sample {:?}", diff, sample);

    //             let response = [QuerySampleResponse {
    //                 id: sample.id,
    //                 data: 0,
    //                 size: 0,
    //             }];
    //             unsafe { query_samples_complete(&response) }
    //         }
    //     }
    // });

    // let load_sample = move |ixs: &[QuerySampleIndex]| {
    //     {
    //         let mut timer = timer.lock().unwrap();
    //         *timer = Instant::now();
    //     }
    //     eprintln!("load {:?}", ixs);
    // };
    // let unload_sample =
    //     |ixs: &[QuerySampleIndex]| eprintln!("{:?}: unload {:?}", Instant::now(), ixs);
    // let qsl_callbacks = QuerySampleLibraryCallbacks::new(load_sample, unload_sample);

    // let report_latency =
    //     |latencies: &[i64]| eprintln!("{:?}: latency: {:?}", Instant::now(), latencies);
    // let run_querys = move |queries: &[QuerySample]| sender.send(queries.to_vec()).unwrap();
    // let sut_callbacks = SystemUnderTestCallbacks::new(run_querys, report_latency);
    // let sut = SystemUnderTest::new("sys-under-test", sut_callbacks);

    // let qsl = QuerySampleLibrary::new("qsl-name", 300, qsl_callbacks);

    // let settings = test_settings();

    // start_test(&sut, &qsl, &settings);
}
