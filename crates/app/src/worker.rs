use app_core::events::AppEvent;
use tokio::sync::mpsc;

/// Continuously consumes `AppEvent` messages from the given receiver and handles them.
///
/// The worker processes incoming events until the channel closes. For `AppEvent::UserRegistered`
/// it logs a welcome-email action, creates a tracing span with the event type, and simulates
/// sending the email (with a short delay).
///
/// # Examples
///
/// ```
/// use tokio::sync::mpsc;
/// use app_core::events::AppEvent;
///
/// #[tokio::test]
/// async fn example_event_worker() {
///     let (tx, rx) = mpsc::channel(8);
///     // spawn the worker
///     tokio::spawn(async move {
///         crate::worker::event_worker(rx).await;
///     });
///
///     // send a UserRegistered event
///     let _ = tx.send(AppEvent::UserRegistered { user_id: 42, email: "user@example.com".into() }).await;
///     // drop the sender to let the worker exit once the queue is drained
///     drop(tx);
///     // allow some time for the worker to process the event
///     tokio::time::sleep(std::time::Duration::from_secs(3)).await;
/// }
/// ```
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