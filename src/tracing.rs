use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_tracing() {
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "realworld=debug,tower_http=debug,axum=trace".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init()
}