use axum::{Json, Router, body::Body, extract::Path, http::{StatusCode, header}, response::{Html, IntoResponse, Response}, routing::get};
use fib_core::fibonacci;
use serde::Serialize;
use tower_http::trace::TraceLayer;

static INDEX_HTML: &str = include_str!("index.html");
static FIB_CORE_JS: &[u8] = include_bytes!("../../fib-core/pkg/fib_core.js");
static FIB_CORE_WASM: &[u8] = include_bytes!("../../fib-core/pkg/fib_core_bg.wasm");

async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

async fn pkg_js() -> impl IntoResponse {
    Response::builder()
        .header(header::CONTENT_TYPE, "text/javascript; charset=utf-8")
        .body(Body::from(FIB_CORE_JS.to_vec()))
        .unwrap()
}

async fn pkg_wasm() -> impl IntoResponse {
    Response::builder()
        .header(header::CONTENT_TYPE, "application/wasm")
        .body(Body::from(FIB_CORE_WASM.to_vec()))
        .unwrap()
}

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
        .route("/", get(index))
        .route("/pkg/fib_core.js", get(pkg_js))
        .route("/pkg/fib_core_bg.wasm", get(pkg_wasm))
        .route("/fib/{n}", get(fib_number))
        .route("/fib/sequence/{n}", get(fib_sequence))
        .layer(TraceLayer::new_for_http())
}

fn main() {
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

    // --- / (index) ---

    #[tokio::test]
    async fn index_returns_html() {
        let resp = server().get("/").await;
        resp.assert_status_ok();
        let ct = resp.headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert!(ct.contains("text/html"), "expected text/html, got: {ct}");
        assert!(resp.text().contains("Fibonacci"));
    }

    #[tokio::test]
    async fn pkg_js_serves_javascript() {
        let resp = server().get("/pkg/fib_core.js").await;
        resp.assert_status_ok();
        let ct = resp.headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert!(ct.contains("text/javascript"), "expected text/javascript, got: {ct}");
        assert!(!resp.text().is_empty());
    }

    #[tokio::test]
    async fn pkg_wasm_serves_wasm() {
        let resp = server().get("/pkg/fib_core_bg.wasm").await;
        resp.assert_status_ok();
        let ct = resp.headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert_eq!(ct, "application/wasm", "expected application/wasm, got: {ct}");
        // All valid WASM binaries start with the magic bytes `\0asm`
        assert_eq!(&resp.as_bytes()[..4], b"\0asm");
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
