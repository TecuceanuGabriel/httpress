use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use httpress::{Benchmark, HookAction, HttpMethod, RequestConfig, RequestContext, Result};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Benchmark with retry on 503 status code using hooks\n");

    // Run benchmark with after_request hook with 503 retrying
    let results = Benchmark::builder()
        // Return 503 status every 20 request
        .request_fn(|ctx: RequestContext| {
            let method = if ctx.request_number.is_multiple_of(20) {
                "503"
            } else {
                "200"
            };

            RequestConfig {
                url: format!("http://localhost:3000/status/{}", method),
                method: HttpMethod::Get,
                headers: HashMap::new(),
                body: None,
            }
        })
        .concurrency(10)
        .requests(1000)
        .show_progress(true)
        .after_request(move |ctx| match ctx.status {
            Some(503_u16) => {
                // Sleeping for 500ms after 503 status
                sleep(Duration::from_millis(500));
                HookAction::Retry
            }
            _ => HookAction::Continue,
        })
        .build()?
        .run()
        .await?;

    // Print built-in results
    results.print();

    Ok(())
}
