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

//        /// SUT calls this to notify loadgen of completed samples.
//        ///
//        ///  - The samples may be from any combination of queries or partial queries as issued by
//        ///    SystemUnderTest::IssueQuery.
//        ///
//        ///  - The SUT is responsible for allocating and owning the response data which must remain
//        ///    valid for the duration of this call. The loadgen will copy the response data if
//        ///    needed (e.g. for accuracy mode). Note: This setup requires the allocation to be
//        ///    timed, which will benefit SUTs that efficiently recycle response memory. All calls to
//        ///    QuerySampleComplete are thread-safe and wait-free bounded.
//        ///
//        ///  - Any number of threads can call QuerySampleComplete simultaneously. Regardless of
//        ///    where any other thread stalls, the current thread will finish QuerySampleComplete in
//        ///    a bounded number of cycles.
//        ///
//        fn query_samples_complete(responses: &mut QuerySampleResponse, response_count: usize);

//        /// Starts the test against SUT with the specified settings.
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

use loadgen::*;
use std::sync::mpsc;

fn main() {
    let (sender, receiver) = mpsc::channel::<Vec<QuerySample>>();
    std::thread::spawn(move || {
        for samples in receiver {
            for sample in samples {
                eprintln!("got sample {:?}", sample);

                let response = [QuerySampleResponse {
                    id: sample.id,
                    data: 0,
                    size: 0,
                }];
                unsafe { query_samples_complete(&response) }
            }
        }
    });

    let load_sample = |ixs: &[QuerySampleIndex]| eprintln!("load {:?}", ixs);
    let unload_sample = |ixs: &[QuerySampleIndex]| eprintln!("unload {:?}", ixs);
    let qsl_callbacks = QSLCallbacks::new(load_sample, unload_sample);

    let report_latency = |latencies: &[i64]| eprintln!("latency: {:?}", latencies);
    let run_querys = move |queries: &[QuerySample]| sender.send(queries.to_vec()).unwrap();
    let sut_callbacks = SystemUnderTestCallbacks::new(run_querys, report_latency);
    let sut = SystemUnderTest::new("sys-under-test", sut_callbacks);

    let qsl = QuerySampleLibrary::new("qsl-name", 300, qsl_callbacks);

    let settings = test_settings();

    start_test(&sut, &qsl, &settings);
}
