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

#[derive(Default)]
struct FibService;

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
            .add_service(FibonacciServiceServer::new(service))
            .add_service(reflection)
            .serve(addr)
            .await
            .unwrap();
    });
}
