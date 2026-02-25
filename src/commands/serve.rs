use crate::fib::fibonacci;
use axum::{Json, Router, extract::Path, http::StatusCode, routing::get};
use serde::Serialize;

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

pub fn run() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let app = Router::new()
            .route("/fib/{n}", get(fib_number))
            .route("/fib/sequence/{n}", get(fib_sequence));

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        println!("Listening on http://0.0.0.0:3000");
        axum::serve(listener, app).await.unwrap();
    });
}
