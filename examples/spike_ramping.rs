use httpress::{Benchmark, RateContext};
use std::time::Duration;

#[tokio::main]
async fn main() -> httpress::Result<()> {
    println!("Spike Ramping Example");
    println!("============================\n");
    println!("This benchmark demonstrates spike rate by ramping");
    println!("base line on 100 rps and spikes to 1000 rps for 1 sec.\n");

    let results = Benchmark::builder()
        .url("http://localhost:3000")
        .concurrency(50)
        .duration(Duration::from_secs(10))
        .show_progress(true)
        .rate_fn(|ctx: RateContext| {
            let baseline_rate = 100.0;
            let spike_rate = 1000.0;
            let period_secs = 5.0;
            let spike_duration = 1.0;

            let elapsed = ctx.elapsed.as_secs_f64();
            let phase = elapsed % period_secs;

            if phase < spike_duration {
                spike_rate
            } else {
                baseline_rate
            }
        })
        .build()?
        .run()
        .await?;

    results.print();

    Ok(())
}
