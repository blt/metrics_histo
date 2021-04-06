use metrics::{counter, histogram};
use metrics_util::Handle;
use std::{thread, time};

fn writer() {
    let mut total_loops = 0;
    loop {
        let before = time::Instant::now();
        counter!("total_loops", total_loops);
        total_loops += 1;
        let after = time::Instant::now();
        histogram!("total_loops_latency", after - before);
    }
}

fn poller() {
    let controller = metrics_histo::get_controller();
    let delay = time::Duration::from_secs(1);

    loop {
        thread::sleep(delay);
        for kv in controller.registry.map.iter() {
            let _key = kv.key();
            let handle = kv.value();

            match handle {
                Handle::Counter(_) => {
                    drop(handle.read_counter());
                }
                Handle::Gauge(_) => {
                    drop(handle.read_gauge());
                }
                Handle::Histogram(_) => {
                    handle.read_histogram_with_clear(|_| {});
                }
            }
        }
    }
}

fn main() {
    metrics_histo::init();

    let mut handles = Vec::new();
    handles.push(thread::spawn(writer));
    handles.push(thread::spawn(poller));
    for handle in handles.drain(..) {
        handle.join().unwrap();
    }
}
