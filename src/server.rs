use tonic::{transport::Server, Request, Response, Status};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;

use metaboss::metaboss_server::{Metaboss, MetabossServer};
use metaboss::{DecodeResponse, DecodeRequest, BurnRequest, BurnResponse, MintRequest, MintResponse};
use metaboss_service::burn::burn_one_grpc;
use metaboss_service::decode::{decode_metadata_grpc};
use metaboss_service::mint::mint_one_grpc;

const URL: &str = "https://api.devnet.solana.com";
const WALLET: [u8; 64] = [180,8,195,203,95,124,6,242,200,136,99,68,133,80,118,231,25,161,0,196,253,100,222,109,13,224,40,185,114,220,68,157,199,58,41,43,81,122,148,14,123,43,154,187,107,242,7,61,178,45,66,25,76,15,234,145,165,248,25,180,30,239,69,1];

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
        match decode_metadata_grpc(&client, account.as_ref()) {
            Ok(uri) => {
                let response = metaboss::DecodeResponse {
                    uri: format!("{}", uri.to_string().trim_matches(char::from(0))).into(),
                };
                Ok(Response::new(response))
            },
            Err(e) => {
                Err(Status::invalid_argument(e.to_string()))
            }
        }
    }

    async fn burn(
        &self,
        request: Request<BurnRequest>,
    ) -> Result<Response<BurnResponse>, Status> {
        println!("Received request from: {:?}", request);
        let client = RpcClient::new(URL.to_string());
        let account = String::from(request.into_inner().mint);
        let keypair = Keypair::from_bytes(&WALLET).unwrap();
        match  burn_one_grpc(&client, keypair, account) {
            Ok(signature) => {
                let response = metaboss::BurnResponse {
                    signature: format!("{}", signature).into(),
                };
                Ok(Response::new(response))
            },
            Err(e) => {
                Err(Status::invalid_argument(e.to_string()))
            }
        }
    }

    async fn mint(
        &self,
        request: Request<MintRequest>,
    ) -> Result<Response<MintResponse>, Status> {
        println!("Received request from: {:?}", request);
        let client = RpcClient::new(URL.to_string());
        let keypair = Keypair::from_bytes(&WALLET).unwrap();
        let request_message = request.into_inner();
        let receiver = String::from(request_message.receiver);
        let nft_data = String::from(request_message.meta);

        println!("receiver: {}", &receiver);
        println!("META: {}", &nft_data);

        match  mint_one_grpc(&client, keypair, receiver, nft_data, false, false, false) {
            Ok(mint) => {
                let response = metaboss::MintResponse {
                    mint: format!("{}", mint).into(),
                };
                Ok(Response::new(response))
            },
            Err(e) => {
                Err(Status::invalid_argument(e.to_string()))
            }
        }
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