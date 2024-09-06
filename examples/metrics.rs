use anyhow::Result;
use rand::Rng;
use rs_concurrency::Metrics;
use std::thread;
use std::time::Duration;

const N: usize = 2;
const M: usize = 2;

fn main() -> Result<()> {
    let metrics = Metrics::new();

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

fn task_worker(idx: usize, metrics: Metrics) -> Result<()> {
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

fn request_worker(metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));

            let page = rng.gen_range(1..256);
            metrics.inc(format!("req.page.{}", page)).unwrap();
        }

        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}
