//! This example demonstrates the client builder with custom DER certificate.
// to run cargo run --example custom_cert --features async

#[cfg(feature = "async")]
fn main() -> Result<(), bitreq::Error> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .expect("failed to build Tokio runtime");

    runtime.block_on(request_with_client())
}

async fn request_with_client() -> Result<(), bitreq::Error> {
    let url = "http://example.com";
    let cert_der = include_bytes!("../tests/test_cert.der");
    let client = bitreq::Client::builder().with_root_certificate(cert_der.as_slice()).build();

    let response = client.send_async(bitreq::get(url)).await.unwrap();

    println!("Status: {}", response.status_code);
    println!("Body: {}", response.as_str()?);

    Ok(())
}
