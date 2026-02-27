use crate::fib::fibonacci;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, transport::Server};
use tonic_reflection::server::Builder as ReflectionBuilder;

pub mod proto {
    tonic::include_proto!("fib");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("fib_descriptor");
}

use proto::fibonacci_service_server::{FibonacciService, FibonacciServiceServer};
use proto::{NumberRequest, NumberResponse, SequenceRequest, SequenceResponse};
use tower_http::trace::TraceLayer;

#[derive(Default)]
pub(crate) struct FibService;

#[tonic::async_trait]
impl FibonacciService for FibService {
    async fn number(
        &self,
        request: Request<NumberRequest>,
    ) -> Result<Response<NumberResponse>, Status> {
        let n = request.into_inner().n;
        if n == 0 || n > 92 {
            return Err(Status::invalid_argument("n must be between 1 and 92"));
        }

        let mut fib = fibonacci();
        let value = (0..n).map(|_| fib()).last().unwrap_or(0);

        Ok(Response::new(NumberResponse { n, value }))
    }

    type SequenceStream = ReceiverStream<Result<SequenceResponse, Status>>;

    async fn sequence(
        &self,
        request: Request<SequenceRequest>,
    ) -> Result<Response<Self::SequenceStream>, Status> {
        let n = request.into_inner().n;
        if n == 0 || n > 92 {
            return Err(Status::invalid_argument("n must be between 1 and 92"));
        }

        let (tx, rx) = mpsc::channel(32);

        tokio::spawn(async move {
            let mut fib = fibonacci();
            for _ in 0..n {
                let value = fib();
                if tx.send(Ok(SequenceResponse { value })).await.is_err() {
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

pub fn run() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let addr = "0.0.0.0:50051".parse().unwrap();
        let service = FibService::default();

        let reflection = ReflectionBuilder::configure()
            .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
            .build_v1()
            .unwrap();

        println!("gRPC server listening on {}", addr);

        Server::builder()
            .layer(TraceLayer::new_for_grpc())
            .add_service(FibonacciServiceServer::new(service))
            .add_service(reflection)
            .serve(addr)
            .await
            .unwrap();
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use proto::{NumberRequest, SequenceRequest};
    use tokio_stream::StreamExt;
    use tonic::Code;

    fn service() -> FibService {
        FibService::default()
    }

    // --- Number RPC ---

    #[tokio::test]
    async fn number_returns_correct_value() {
        let resp = service()
            .number(Request::new(NumberRequest { n: 10 }))
            .await
            .unwrap();
        let msg = resp.into_inner();
        assert_eq!(msg.n, 10);
        assert_eq!(msg.value, 55);
    }

    #[tokio::test]
    async fn number_min_boundary() {
        let resp = service()
            .number(Request::new(NumberRequest { n: 1 }))
            .await
            .unwrap();
        assert_eq!(resp.into_inner().value, 1);
    }

    #[tokio::test]
    async fn number_max_boundary() {
        let resp = service()
            .number(Request::new(NumberRequest { n: 92 }))
            .await
            .unwrap();
        assert_eq!(resp.into_inner().n, 92);
    }

    #[tokio::test]
    async fn number_rejects_zero() {
        let err = service()
            .number(Request::new(NumberRequest { n: 0 }))
            .await
            .unwrap_err();
        assert_eq!(err.code(), Code::InvalidArgument);
    }

    #[tokio::test]
    async fn number_rejects_above_max() {
        let err = service()
            .number(Request::new(NumberRequest { n: 93 }))
            .await
            .unwrap_err();
        assert_eq!(err.code(), Code::InvalidArgument);
    }

    // --- Sequence RPC ---

    #[tokio::test]
    async fn sequence_returns_correct_values() {
        let resp = service()
            .sequence(Request::new(SequenceRequest { n: 5 }))
            .await
            .unwrap();
        let values: Vec<u64> = resp
            .into_inner()
            .map(|r| r.unwrap().value)
            .collect()
            .await;
        assert_eq!(values, vec![1, 1, 2, 3, 5]);
    }

    #[tokio::test]
    async fn sequence_min_boundary() {
        let resp = service()
            .sequence(Request::new(SequenceRequest { n: 1 }))
            .await
            .unwrap();
        let values: Vec<u64> = resp
            .into_inner()
            .map(|r| r.unwrap().value)
            .collect()
            .await;
        assert_eq!(values, vec![1]);
    }

    #[tokio::test]
    async fn sequence_max_boundary() {
        let resp = service()
            .sequence(Request::new(SequenceRequest { n: 92 }))
            .await
            .unwrap();
        let values: Vec<u64> = resp
            .into_inner()
            .map(|r| r.unwrap().value)
            .collect()
            .await;
        assert_eq!(values.len(), 92);
    }

    #[tokio::test]
    async fn sequence_rejects_zero() {
        let err = service()
            .sequence(Request::new(SequenceRequest { n: 0 }))
            .await
            .unwrap_err();
        assert_eq!(err.code(), Code::InvalidArgument);
    }

    #[tokio::test]
    async fn sequence_rejects_above_max() {
        let err = service()
            .sequence(Request::new(SequenceRequest { n: 93 }))
            .await
            .unwrap_err();
        assert_eq!(err.code(), Code::InvalidArgument);
    }
}
