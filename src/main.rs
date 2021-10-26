use cosmos_sdk_proto::cosmos::tx::v1beta1::service_client::ServiceClient;
use cosmos_sdk_proto::cosmos::tx::v1beta1::{GetTxsEventRequest, GetTxsEventResponse};
use cosmrs::Tx;
//use prost_types::Any;
use std::env;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn main() {
    // Change this to a desired
    let height: i64 = 8143166;
    println!("Getting Cosmos Transactions for height {:?}", height);

    let grpc_addr: String;
    match env::var("NODE_GRPC") {
        Ok(val) => grpc_addr = val,
        Err(_e) => {
            println!("Please set NODE_GRPC environment variable");
            std::process::exit(1);
        }
    }
    let rt = Arc::new(Runtime::new().unwrap());
    let mut client = rt
        .block_on(ServiceClient::connect(grpc_addr))
        .map_err(|e| println!("{}", e))
        .unwrap();

    // Build events query
    let event_query = format!("tx.height={}", height);

    // Build a new request
    // TODO: Implement pagination
    let request = tonic::Request::new(GetTxsEventRequest {
        events: vec![event_query],
        pagination: None,
        order_by: 0,
    });

    // Send request and wait for response
    let response: GetTxsEventResponse = rt
        .block_on(client.get_txs_event(request))
        .unwrap()
        .into_inner();

    for tx_resp in response.tx_responses.iter() {
        let tx = Tx::from_bytes(tx_resp.clone().tx.unwrap().value.as_slice()).unwrap();
        for msg in tx.body.messages {
            //TODO: Need to get m.type_url
            //let m = Any::from(msg);

            println!("{:?}", msg);
        }
    }
}