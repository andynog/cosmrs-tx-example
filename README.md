# cosmrs-tx-example

Fetch transactions from Cosmos using gRPC

## Configuration

To run, set an environment variable with a Cosmos chain gRPC address e.g. http://localhost:9090

`export NODE_GRPC=http://localhost:9090`

## Running

`$ cargo run`

This should output something like (this is from a Cosmos Hub node)

```
Getting messages for transactions in height 8150664
8150664,"18CF16312CFAD57C320FB8A61477BD78B29062CABD8328AB747A2D28C5EC0F0A","MsgUpdateClient|MsgRecvPacket"
8150664,"F119370AFF4136E04E147BE20ED2942D8F99C32EF0EDD5022355F11D21C85A9F","MsgSend"
```