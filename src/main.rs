use metrics::{counter, histogram};
use metrics_util::Handle;
use std::time;
use tokio::task;
use tokio::time::interval;

async fn writer() {
    let mut total_loops = 0;
    loop {
        let before = time::Instant::now();
        counter!("total_loops", total_loops);
        total_loops += 1;
        let after = time::Instant::now();
        histogram!("total_loops_latency", after - before);
    }
}

async fn poller() {
    let mut interval = interval(time::Duration::from_secs(1));
    let controller = metrics_histo::get_controller();

    loop {
        interval.tick().await;
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

#[tokio::main]
async fn main() {
    metrics_histo::metrics();
    metrics_histo::tracing();

    task::spawn(writer());
    poller().await;
}
