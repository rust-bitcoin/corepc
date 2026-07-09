//! This example demonstrates the `async` feature.

#[cfg(not(bitreq_wasm))]
fn main() -> Result<(), bitreq::Error> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("failed to build Tokio runtime");

    runtime.block_on(async {
        let response = bitreq::get("http://httpbin.org/get").send_async().await?;

        println!("Status: {}", response.status_code);
        println!("Body: {}", response.as_str()?);

        Ok(())
    })
}

#[cfg(bitreq_wasm)]
fn main() {}
