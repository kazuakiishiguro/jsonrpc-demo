# jsonrpc demo 
this is a smiple(est) example of jsonrpc network demo

## setup 

```bash
git clone https://github.com/kazuakiishiguro/jsonrpc-demo.git
cd jsonrpc-demo 
cargo build --release
```

## test
```bash
./target/release/jsonrpc-demo 
node test.js // {"jsonrpc":"2.0","result":12,"id":1}
```