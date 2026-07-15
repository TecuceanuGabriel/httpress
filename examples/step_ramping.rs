use httpress::{Benchmark, RateContext};
use std::time::Duration;

#[tokio::main]
async fn main() -> httpress::Result<()> {
    println!("Step Ramping Example");
    println!("============================\n");
    println!("This benchmark demonstrates dynamic rate control by ramping");
    println!("from 100 req/s to 1000 req/s by 100 each step with 5 sec waiting.\n");

    let results = Benchmark::builder()
        .url("http://localhost:3000")
        .concurrency(50)
        .duration(Duration::from_secs(10))
        .show_progress(true)
        .rate_fn(|ctx: RateContext| {
            // Step by step ramp from 100 to 1000 req/s over 10 seconds by 100
            let elapsed_secs = ctx.elapsed.as_secs_f64();
            let step = (elapsed_secs.floor() as u64).min(9);

            ((step + 1) * 100) as f64
        })
        .build()?
        .run()
        .await?;

    results.print();

    Ok(())
}
