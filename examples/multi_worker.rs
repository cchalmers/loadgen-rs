use crossbeam::channel;
use loadgen::*;
use std::time::Duration;

fn main() {
    let (sender, receiver) = channel::unbounded::<Query<Vec<usize>>>();

    // setup the worker threads
    for i in 0..6 {
        let receiver = receiver.clone();
        std::thread::spawn(move || {
            for q in receiver {
                eprintln!("worker {}: {:?}", i, &q[..]);
                std::thread::sleep(std::time::Duration::from_millis(200));
                q.complete(&[]);
            }
        });
    }

    // multi-stream settings
    let mut settings = TestSettings::default();
    settings.scenario = mlperf::TestScenario::MultiStream;
    settings.multi_stream_max_async_queries = 6;
    settings.multi_stream_samples_per_query = 4;

    // log settings
    let mut log_settings = LogSettings::default();
    log_settings.log_output.prefix.assign("multi_worker_");

    // library that generates a boring input
    let mut library = Samples::new(100, |i| vec![i; 8]);

    // test that sends queries to the workers and prints out latencies
    let query = move |q: Query<Vec<usize>>| sender.send(q).unwrap();
    let report = |latencies: &[i64]| {
        for d in latencies {
            eprintln!("{:.3?} ms", Duration::from_nanos(*d as u64).as_millis());
        }
    };
    let mut sut = Test::new(&library, query, report);

    // start_test(&mut sut, &mut library, &settings, &log_settings);
}
