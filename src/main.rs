use std::env;
use std::sync::Arc;

use cosmos_sdk_proto::cosmos::tx::v1beta1::{GetTxsEventRequest, GetTxsEventResponse};
use cosmos_sdk_proto::cosmos::tx::v1beta1::service_client::ServiceClient;
use cosmrs::Tx;
use prost_types::Any;
use tokio::runtime::Runtime;

fn main() {
    // Change this to a desired
    let height: i64 = 5258498;
    println!("Getting messages for transactions in height {:?}", height);

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
        match tx_resp.clone().tx {
            None => {
                println!("No transaction");
            }
            Some(tx) => {
                let tx_result = Tx::from_bytes(tx.value.as_slice());
                match tx_result {
                    Ok(t) => {
                        let mut m_types = Vec::<String>::new();
                        for msg in t.body.messages {
                            let m = Any::from(msg.clone());
                            let m_type = m.type_url.split('.').last();
                            match m_type {
                                None => {
                                }
                                Some(t) => {
                                    m_types.push(t.to_string());
                                }
                            }
                        }
                        println!("{:?},{:?},{:?}", tx_resp.height, tx_resp.txhash, m_types.join("|"));
                    }
                    Err(e) => {
                        println!("Error converting tx: Block {:?} - Height {:?} - Error: {:?}", tx_resp.height, tx_resp.txhash, e);
                    }
                }
            }
        }
    }
}