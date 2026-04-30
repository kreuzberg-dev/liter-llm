/// Install the ring crypto provider before any test in this process runs.
///
/// rustls 0.23+ requires an explicit provider. `ctor` runs this before
/// `main`, so every test binary gets the provider installed before any
/// `reqwest::Client` is constructed.
#[cfg(feature = "native-http")]
#[ctor::ctor]
fn init_crypto() {
    liter_llm::ensure_crypto_provider();
}
