use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use httpress::{Benchmark, HookAction, HttpMethod, RequestConfig, RequestContext, Result};


#[tokio::main]
async fn main() -> Result<()> {
    println!("Benchmark with retry on 503 status code using hooks\n");

    // Run benchmark with after_request hook for metrics collection
    let results = Benchmark::builder()
        // Request with 503 return code
        .request_fn(|_ctx: RequestContext| {
            RequestConfig {
                url: format!("http://localhost:3000/status/{}", "503"),
                method: HttpMethod::Get,
                headers: HashMap::new(),
                body: None,
            }
        })
        .concurrency(10)
        .requests(100)
        .show_progress(true)
        .after_request(move |ctx| match ctx.status {
            Some(503_u16) => {
                sleep(Duration::from_millis(500));
                HookAction::Retry
            },
            _ => HookAction::Continue,
        })
        .build()?
        .run()
        .await?;

    // Print built-in results
    results.print();

    Ok(())
}
