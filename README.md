# cosmrs-tx-example

Fetch transactions from Cosmos using gRPC

## Configuration

To run, set an environment variable with a Cosmos chain gRPC address e.g. http://localhost:9090

`export NODE_GRPC=http://localhost:9090`

## Running

`$ cargo run`

This should output something like (this is from a Cosmos Hub node)

```
Getting messages for transactions in height 8143166
Msg(Any { type_url: "/cosmos.bank.v1beta1.MsgSend", value: [10, 45, 99, 111, 115, 109, 111, 115, 49, 55, 109, 117, 118, 100, 103, 107, 101, 112, 52, 110, 100, 112, 116, 110, 121, 103, 51, 56, 101, 117, 102, 120, 115, 115, 115, 113, 56, 106, 114, 51, 119, 110, 107, 121, 115, 121, 56, 18, 45, 99, 111, 115, 109, 111, 115, 49, 121, 117, 99, 121, 99, 114, 116, 51, 110, 56, 113, 115, 107, 117, 106, 114, 107, 54, 121, 108, 120, 104, 101, 115, 104, 102, 56, 109, 115, 115, 112, 102, 106, 53, 100, 112, 51, 51, 26, 15, 10, 5, 117, 97, 116, 111, 109, 18, 6, 57, 57, 55, 48, 54, 56] })
Msg(Any { type_url: "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward", value: [10, 45, 99, 111, 115, 109, 111, 115, 49, 106, 103, 103, 108, 55, 108, 116, 48, 51, 97, 112, 54, 100, 107, 103, 113, 53, 101, 48, 117, 51, 102, 118, 53, 122, 52, 109, 121, 103, 104, 50, 117, 119, 116, 109, 108, 103, 57, 18, 52, 99, 111, 115, 109, 111, 115, 118, 97, 108, 111, 112, 101, 114, 49, 116, 102, 108, 107, 51, 48, 109, 113, 53, 118, 103, 113, 106, 100, 108, 121, 57, 50, 107, 107, 104, 104, 113, 51, 114, 97, 101, 118, 50, 104, 110, 122, 54, 101, 101, 116, 101, 51] })
```