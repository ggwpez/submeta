# SubMeta

Minimal CLI to extract information from encoded V14 metadata.  

## Usage

For example show all Polkadot pallets with their storage items, you need [jq] installed:

```rust
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "state_getMetadata"}' https://rpc.polkadot.io:443 |\
jq '.result' -r |\
cargo run -- pallets --with-storage
```

<!-- LINKS -->

[jq]: https://stedolan.github.io/jq/
