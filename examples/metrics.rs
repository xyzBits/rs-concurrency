use anyhow::Result;
use rs_concurrency::Metrics;

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

    println!("{:?}", metrics.snapshot());

    Ok(())
}
