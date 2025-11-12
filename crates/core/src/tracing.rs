use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_standard_tracing(crate_name: &str, package_name: &str, level: &str) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{crate_name}={level},{package_name}={level}").into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
