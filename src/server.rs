use tonic::{transport::Server, Request, Response, Status};
use solana_client::rpc_client::RpcClient;

use metaboss::metaboss_server::{Metaboss, MetabossServer};
use metaboss::{DecodeResponse, DecodeRequest};
use metaboss_service::decode::{decode_metadata_grpc};

const URL: &str = "https://api.devnet.solana.com";

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
        let client = RpcClient::new(URL.to_string());
        let account = Some(String::from(request.into_inner().mint));
        let uri = decode_metadata_grpc(&client, account.as_ref())
            .unwrap_or(String::from("Unable to get uri"));

        let response = metaboss::DecodeResponse {
            uri: format!("{}", uri.to_string().trim_matches(char::from(0))).into(),
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