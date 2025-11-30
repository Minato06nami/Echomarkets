#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::{EchoMarkets, Market};
use async_graphql::{EmptySubscription, Object, Schema};
use linera_sdk::{
    abis::ServiceAbi,
    linera_base_types::{WithServiceAbi, ChainId},
    views::{View, RootView},
    Service, ServiceRuntime,
};
use std::sync::Arc;
use echo_markets::EchoMarketsAbi;

pub struct EchoMarketsService {
    state: EchoMarkets,
    runtime: Arc<ServiceRuntime<Self>>,
}

linera_sdk::service!(EchoMarketsService);

impl WithServiceAbi for EchoMarketsService {
    type Abi = EchoMarketsAbi;
}

impl Service for EchoMarketsService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = EchoMarkets::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        EchoMarketsService {
            state,
            runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, request: async_graphql::Request) -> async_graphql::Response {
        let schema = Schema::build(
            QueryRoot { state: &self.state },
            MutationRoot,
            EmptySubscription,
        )
        .finish();
        schema.execute(request).await
    }
}

struct QueryRoot<'a> {
    state: &'a EchoMarkets,
}

#[Object]
impl<'a> QueryRoot<'a> {
    async fn markets(&self) -> Vec<Market> {
        let mut markets = Vec::new();
        let count = *self.state.market_counter.get();
        for i in 0..count {
            if let Some(m) = self.state.markets.get(&i).await.expect("Failed to get market") {
                markets.push(m);
            }
        }
        markets
    }

    async fn is_trader(&self) -> bool {
        *self.state.is_trader.get()
    }
    
    async fn trader_name(&self) -> String {
        self.state.trader_name.get().clone()
    }
    
    async fn following(&self) -> Vec<ChainId> {
        let mut list = Vec::new();
        let count = self.state.following.count();
        for i in 0..count {
            if let Some(c) = self.state.following.get(i).await.expect("Err") {
                list.push(c);
            }
        }
        list
    }
    
    async fn subscribers_count(&self) -> u64 {
        self.state.subscribers.count() as u64
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    // Mutations are handled via Operations, but we can expose helpers if needed.
    // For Linera, usually we use `do_operation` via the client, but we can document them here.
    async fn ping(&self) -> String {
        "pong".to_string()
    }
}
