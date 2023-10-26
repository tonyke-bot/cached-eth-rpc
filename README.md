# ETH RPC Cache Layer
A simple http server to cache specific eth rpc requests in memory. Useful for massive repeatedly requests to ETH rpc endpoints. 
Multiple endpoints/chains can be configured to be cached.

### Usage (with docker)
```shell
docker run \
  -d \
  -p 8124:8124 \
  -v ./data/:/data/ \
  -e DATA_PERSISTENCE=1 \
  -e ENDPOINTS="eth-chain=https://rpc.ankr.com/eth,bsc-chain=https://rpc.ankr.com/bsc" \
  --name cached-eth-rpc \
  ghcr.io/tonyke-bot/cached-eth-rpc
```

### Usage
With
```shell
cargo run --release -- \
  --port 8124 \
  --endpoint "eth-chain=https://rpc.ankr.com/eth" \
  --endpoint "bsc-chain=https://rpc.ankr.com/bsc"
```
Following redirection will be made:
* http://localhost:8124/eth-chain -> https://rpc.ankr.com/eth
* http://localhost:8124/bsc-chain -> https://rpc.ankr.com/bsc

### Supported methods
Mainly supported requests with determined block number. Other methods will be directly send to the configured ETH rpc endpoint.

- `eth_call`
- `eth_chainId`
- `eth_getBalance`
- `eth_getBlockByNumber`
- `eth_getCode`
- `eth_getStorageAt`
- `eth_getTransactionByHash`
- `eth_getTransactionCount`
- `eth_getTransactionReceipt`
- `eth_getTransactionByBlockNumberAndIndex`
- `eth_getTransactionByBlockHashAndIndex`
