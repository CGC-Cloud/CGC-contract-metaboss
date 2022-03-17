use tonic::{transport::Server, Request, Response, Status};

use metaboss::metaboss_server::{Metaboss, MetabossServer};
use metaboss::{DecodeResponse, DecodeRequest};

// Import the generated proto-rust file into a module
pub mod metaboss {
    tonic::include_proto!("metaboss");
}

#[derive(Debug, Default)]
pub struct MyMetaboss {}

#[tonic::async_trait]
impl Metaboss for MyMetaboss {
    async fn decode(
        &self,
        request: Request<DecodeRequest>,
    ) -> Result<Response<DecodeResponse>, Status> {
        println!("Received request from: {:?}", request);

        let response = metaboss::DecodeResponse {
            uri: format!("Hello {}!", request.into_inner().mint).into(),
        };

        Ok(Response::new(response))
    }
}

// Use the tokio runtime to run our server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let metaboss = MyMetaboss::default();

    println!("Starting gRPC Server on port 50051...");
    Server::builder()
        .add_service(MetabossServer::new(metaboss))
        .serve(addr)
        .await?;

    Ok(())
}