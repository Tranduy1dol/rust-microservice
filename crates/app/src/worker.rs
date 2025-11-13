use app_core::events::AppEvent;
use tokio::sync::mpsc;

pub async fn event_worker(mut rx: mpsc::Receiver<AppEvent>) {
    tracing::info!("Event worker started.");
    while let Some(event) = rx.recv().await {
        let event_span = tracing::info_span!("Processing Event", event_type = event.name());
        let _guard = event_span.enter();

        if let AppEvent::UserRegistered { user_id, email } = event {
            tracing::info!(
                "Sending welcome email to (user_id: {}, email: {})",
                user_id,
                email
            );
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            tracing::info!("Email sent to {}", email);
        }
    }
}
