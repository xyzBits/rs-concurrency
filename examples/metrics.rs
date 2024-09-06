use anyhow::Result;
use rand::Rng;
use rs_concurrency::Metrics;
use std::thread;
use std::time::Duration;

const N: usize = 2;
const M: usize = 2;

fn main() -> Result<()> {
    let mut metrics = Metrics::new();

    metrics.inc("req.page.1");
    metrics.inc("call.thread.worker.1");

    for i in 0..100 {
        metrics.inc("req.page.1");
        metrics.inc("req.page.2");

        if i % 2 == 0 {
            metrics.inc("req.page.3");
        }
    }

    for _ignore in 0..50 {
        metrics.dec("req.page.2");
    }

    for idx in 0..N {
        task_worker(idx, metrics);
    }

    for _ in 0..M {
        request_worker(metrics);
    }

    println!("{:?}", metrics.snapshot());

    Ok(())
}

fn task_worker(idx: usize, mut metrics: Metrics) {
    thread::spawn(move || loop {
        // do something
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(100..5_000)));
        metrics.inc(format!("call.thread.worker.{}", idx));
    });
}

fn request_worker(mut metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();

        thread::sleep(Duration::from_millis(rng.gen_range(50..800)));

        let page = rng.gen_range(1..256);
        metrics.inc(format!("req.page.{}", page));
    });
}
