use tracing_subscriber::{
    fmt::{self, time::ChronoLocal},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

fn main() {
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_timer(ChronoLocal::new("%m-%d %H:%M:%S".into()))
                .with_thread_names(true),
        )
        .init();

    tracing::trace!("This is a trace message");
    tracing::debug!("This is a debug message");
    tracing::info!("This is a info message");
    tracing::warn!("This is a warn message");
    tracing::error!("This is a error message");
}
