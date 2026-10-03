#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("niwoe-portal starting");
    if let Err(e) = niwoe_portal::run().await {
        tracing::error!("portal failed: {e}");
        std::process::exit(1);
    }
}
