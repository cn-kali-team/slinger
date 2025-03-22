#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  #[cfg(feature = "tls")]
  {
    use slinger::tls::PeerCertificate;
    use slinger::ClientBuilder;
    let client = ClientBuilder::new().build().unwrap();
    let resp = client.get("https://httpbin.org/get").send().await?;
    let certificate = resp.extensions().get::<PeerCertificate>().unwrap();
    println!("{:?}", certificate);
  }
  Ok(())
}
