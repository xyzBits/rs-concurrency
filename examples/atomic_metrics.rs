use anyhow::Result;
use rand::Rng;
use rs_concurrency::AtomicMetrics;
use std::thread;
use std::time::Duration;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = AtomicMetrics::new(&[
        "call.thread.worker.0",
        "call.thread.worker.1",
        "req.page.1",
        "req.page.2",
        "req.page.3",
        "req.page.4",
    ]);

    for idx in 0..N {
        task_worker(idx, metrics.clone())?; // Arc::clone(&metrics.data)
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(5));
        println!("{}", metrics);
    }
}

fn task_worker(idx: usize, metrics: AtomicMetrics) -> Result<()> {
    // 闭包可以有返回值
    thread::spawn(move || {
        loop {
            // do something
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..5_000)));
            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }

        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}

fn request_worker(metrics: AtomicMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));

            let page = rng.gen_range(1..5);
            metrics.inc(format!("req.page.{}", page)).unwrap();
        }

        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}
