//! This is a simple example to demonstrate the usage of this library.

#![cfg(feature = "std")]

#[cfg(not(bitreq_wasm))]
fn main() -> Result<(), bitreq::Error> {
    let response = bitreq::get("http://example.com").send()?;
    let html = response.as_str()?;
    println!("{}", html);
    Ok(())
}

#[cfg(bitreq_wasm)]
fn main() {}
