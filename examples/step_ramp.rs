use httpress::{Benchmark, RateContext};
use std::time::Duration;

#[tokio::main]
async fn main() -> httpress::Result<()> {
    println!("Step Ramp Rate Example");
    println!("======================\n");
    println!("This benchmark holds a fixed rate for a few seconds, then jumps");
    println!("to the next level: 100 -> 200 -> 300 req/s.\n");

    // Each rate level (req/s) is held for `secs_per_step` seconds before
    // stepping up to the next one. The final level is held until the benchmark ends.
    let rate_steps = [100.0, 200.0, 300.0];
    let secs_per_step = 5.0;
    let total_duration = Duration::from_secs_f64(secs_per_step * rate_steps.len() as f64);

    let results = Benchmark::builder()
        .url("http://localhost:3000")
        .concurrency(50)
        .duration(total_duration)
        .show_progress(true)
        .rate_fn(move |ctx: RateContext| {
            // Select the current step from how much time has elapsed, clamping
            // to the last step so we never index past the end.
            let step = (ctx.elapsed.as_secs_f64() / secs_per_step) as usize;
            rate_steps[step.min(rate_steps.len() - 1)]
        })
        .build()?
        .run()
        .await?;

    results.print();

    Ok(())
}
