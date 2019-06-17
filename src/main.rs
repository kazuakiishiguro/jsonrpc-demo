extern crate jsonrpc_core as rpc;
extern crate jsonrpc_http_server as http;
extern crate jsonrpc_pubsub as pubsub;
extern crate jsonrpc_tcp_server as tcp;
#[macro_use]
extern crate jsonrpc_macros as macros;

use rpc::{
    futures::{self, Future},
    BoxFuture,
};
use std::{sync::Arc, thread};

build_rpc_trait! {
    pub trait Rpc {
        type Metadata;
        /// Adds two numbers and returns a result
        #[rpc(name = "sum")]
        fn sum(&self, u64, u64) -> BoxFuture<u64>;
    }
}

#[derive(Default)]
struct RpcImpl {}
impl Rpc for RpcImpl {
    type Metadata = Option<Arc<pubsub::Session>>;

    fn sum(&self, a: u64, b: u64) -> BoxFuture<u64> {
        let (tx, rx) = futures::oneshot();
        thread::spawn(move || tx.send(a + b));
        Box::new(rx.map_err(|_| unreachable!()))
    }
}

fn main() {
    let rpc = RpcImpl::default();
    let mut io = pubsub::PubSubHandler::default();
    io.extend_with(rpc.to_delegate());

    let address = "127.0.0.1:3030".parse().expect("Valid address given");
    let server1 = http::ServerBuilder::new(io)
        .threads(3)
        .start_http(&address)
        .expect("Server should start");
    server1.wait();
}