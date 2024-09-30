use super::{App, Snapshot};
use std::sync::Arc;

/// Request sent by data-plane reactors to notify the data-plane of a shard failure.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// # JWT token which identifies the shard and authorizes the request.
    /// The token subject is the shard ID.
    pub token: String,
    /// # Error encountered by the shard.
    pub error: String,
}

#[derive(Debug, Default, serde::Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// # Number of milliseconds to wait before retrying the request.
    /// Zero if the request was successful.
    pub retry_millis: u64,
}

#[axum::debug_handler]
pub async fn notify_shard_failure(
    axum::extract::State(app): axum::extract::State<Arc<App>>,
    axum::Json(request): axum::Json<Request>,
) -> axum::response::Response {
    super::wrap(async move { do_notify_shard_failure(&app, &request).await }).await
}

#[tracing::instrument(skip(app), err(level = tracing::Level::WARN))]
async fn do_notify_shard_failure(
    app: &App,
    Request { token, error }: &Request,
) -> anyhow::Result<Response> {
    let (_header, claims) = super::parse_untrusted_data_plane_claims(token)?;

    match Snapshot::evaluate(
        &app.snapshot,
        claims.iat,
        |Snapshot {
             data_planes, tasks, ..
         }: &Snapshot| {
            _ = super::verify_data_plane_claims(
                data_planes,
                tasks,
                &claims.sub,
                &claims.iss,
                token,
            )?;
            Ok(())
        },
    ) {
        Ok(()) => {
            // TODO(johnny): This is a placeholder for enqueuing an automation to perform
            // shard restart.
            tracing::info!(%error, %claims.sub, %claims.iss, "notified of shard failure");

            Ok(Response {
                ..Default::default()
            })
        }
        Err(Ok(retry_millis)) => Ok(Response {
            retry_millis,
            ..Default::default()
        }),
        Err(Err(err)) => Err(err),
    }
}
