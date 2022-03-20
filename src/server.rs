use std::env;
use tonic::{transport::Server, Request, Response, Status};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;

use metaboss::metaboss_server::{Metaboss, MetabossServer};
use metaboss::{DecodeResponse, DecodeRequest, BurnRequest, BurnResponse, MintRequest, MintResponse};
use metaboss_service::burn::burn_one_grpc;
use metaboss_service::decode::{decode_metadata_grpc};
use metaboss_service::mint::mint_one_grpc;

const URL: &str = "https://api.devnet.solana.com";

pub mod metaboss {
    tonic::include_proto!("metaboss");
}

#[derive(Debug, Default)]
pub struct MyMetaboss {
    pub wallet: String,
}

impl MyMetaboss {
    pub fn new() -> MyMetaboss {
        let wallet = env::var("WALLET").expect("please set env variable WALLET");
        MyMetaboss{
            wallet
        }
    }
}


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
        let request_message = request.into_inner();
        let mint = String::from(request_message.mint);
        let account = String::from(request_message.account);
        let keypair= Keypair::from_base58_string(&self.wallet);
        match  burn_one_grpc(&client, keypair, mint, account) {
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
        let keypair= Keypair::from_base58_string(&self.wallet);
        let request_message = request.into_inner();

        let receiver = String::from(request_message.receiver);
        let name = String::from(request_message.name);
        let symbol = String::from(request_message.symbol);
        let uri = String::from(request_message.uri);
        let seller_fee_basis_points = request_message.seller_fee_basis_points as u16;
        let creator = String::from(request_message.creator);

        match  mint_one_grpc(
            &client,
            keypair,
            receiver,
            name,
            symbol,
            uri,
            seller_fee_basis_points,
            creator,
            false,
            true,
            false) {
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
    let metaboss = MyMetaboss::new();

    println!("Starting gRPC Server on port 50051...");
    Server::builder()
        .add_service(MetabossServer::new(metaboss))
        .serve(addr)
        .await?;

    Ok(())
}