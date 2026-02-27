use crate::fib::fibonacci;
use axum::{Json, Router, extract::Path, http::StatusCode, routing::get};
use serde::Serialize;
use tower_http::trace::TraceLayer;

#[derive(Serialize)]
struct FibNumberResponse {
    n: u64,
    value: u64,
}

#[derive(Serialize)]
struct FibSequenceResponse {
    n: u64,
    values: Vec<u64>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn validate_n(n: u64) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    if n == 0 || n > 92 {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ErrorResponse {
                error: "n must be between 1 and 92".into(),
            }),
        ));
    }
    Ok(())
}

async fn fib_number(
    Path(n): Path<u64>,
) -> Result<Json<FibNumberResponse>, (StatusCode, Json<ErrorResponse>)> {
    validate_n(n)?;
    let mut fib = fibonacci();
    let value = (0..n).map(|_| fib()).last().unwrap_or(0);
    Ok(Json(FibNumberResponse { n, value }))
}

async fn fib_sequence(
    Path(n): Path<u64>,
) -> Result<Json<FibSequenceResponse>, (StatusCode, Json<ErrorResponse>)> {
    validate_n(n)?;
    let mut fib = fibonacci();
    let values: Vec<u64> = (0..n).map(|_| fib()).collect();
    Ok(Json(FibSequenceResponse { n, values }))
}

pub(crate) fn router() -> Router {
    Router::new()
        .route("/fib/{n}", get(fib_number))
        .route("/fib/sequence/{n}", get(fib_sequence))
        .layer(TraceLayer::new_for_http())
}

pub fn run() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        println!("Listening on http://0.0.0.0:3000");
        axum::serve(listener, router()).await.unwrap();
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    fn server() -> TestServer {
        TestServer::new(router()).unwrap()
    }

    // --- /fib/{n} ---

    #[tokio::test]
    async fn number_returns_correct_value() {
        let resp = server().get("/fib/10").await;
        resp.assert_status_ok();
        resp.assert_json_contains(&serde_json::json!({ "n": 10, "value": 55 }));
    }

    #[tokio::test]
    async fn number_min_boundary() {
        let resp = server().get("/fib/1").await;
        resp.assert_status_ok();
        resp.assert_json_contains(&serde_json::json!({ "n": 1, "value": 1 }));
    }

    #[tokio::test]
    async fn number_max_boundary() {
        let resp = server().get("/fib/92").await;
        resp.assert_status_ok();
        resp.assert_json_contains(&serde_json::json!({ "n": 92 }));
    }

    #[tokio::test]
    async fn number_rejects_zero() {
        let resp = server().get("/fib/0").await;
        resp.assert_status_unprocessable_entity();
        resp.assert_json_contains(&serde_json::json!({ "error": "n must be between 1 and 92" }));
    }

    #[tokio::test]
    async fn number_rejects_above_max() {
        let resp = server().get("/fib/93").await;
        resp.assert_status_unprocessable_entity();
        resp.assert_json_contains(&serde_json::json!({ "error": "n must be between 1 and 92" }));
    }

    // --- /fib/sequence/{n} ---

    #[tokio::test]
    async fn sequence_returns_correct_values() {
        let resp = server().get("/fib/sequence/5").await;
        resp.assert_status_ok();
        resp.assert_json_contains(&serde_json::json!({ "n": 5, "values": [1, 1, 2, 3, 5] }));
    }

    #[tokio::test]
    async fn sequence_min_boundary() {
        let resp = server().get("/fib/sequence/1").await;
        resp.assert_status_ok();
        resp.assert_json_contains(&serde_json::json!({ "n": 1, "values": [1] }));
    }

    #[tokio::test]
    async fn sequence_max_boundary() {
        let resp = server().get("/fib/sequence/92").await;
        resp.assert_status_ok();
        let body: serde_json::Value = resp.json();
        assert_eq!(body["n"], 92);
        assert_eq!(body["values"].as_array().unwrap().len(), 92);
    }

    #[tokio::test]
    async fn sequence_rejects_zero() {
        let resp = server().get("/fib/sequence/0").await;
        resp.assert_status_unprocessable_entity();
        resp.assert_json_contains(&serde_json::json!({ "error": "n must be between 1 and 92" }));
    }

    #[tokio::test]
    async fn sequence_rejects_above_max() {
        let resp = server().get("/fib/sequence/93").await;
        resp.assert_status_unprocessable_entity();
        resp.assert_json_contains(&serde_json::json!({ "error": "n must be between 1 and 92" }));
    }
}
