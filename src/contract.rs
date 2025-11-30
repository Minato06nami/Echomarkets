#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::EchoMarkets;
use linera_sdk::{
    abis::ContractAbi,
    linera_base_types::{ChainId, WithContractAbi, Amount},
    views::{RootView, View},
    Contract, ContractRuntime,
};
use echo_markets::{EchoMarketsAbi, Message, Operation};

pub struct EchoMarketsContract {
    state: EchoMarkets,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(EchoMarketsContract);

impl WithContractAbi for EchoMarketsContract {
    type Abi = EchoMarketsAbi;
}

impl Contract for EchoMarketsContract {
    type Message = Message;
    type Parameters = ();
    type InstantiationArgument = ();
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = EchoMarkets::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        EchoMarketsContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        self.state.market_counter.set(0);
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            Operation::CreateMarket { question, end_time } => {
                let id = *self.state.market_counter.get();
                self.state.market_counter.set(id + 1);
                
                let market = state::Market {
                    id,
                    question,
                    end_time,
                    resolved: false,
                    outcome: None,
                    pool_yes: Amount::ZERO,
                    pool_no: Amount::ZERO,
                };
                
                self.state.markets.insert(&id, market).expect("Failed to insert market");
            }
            Operation::RegisterAsTrader { name } => {
                self.state.is_trader.set(true);
                self.state.trader_name.set(name);
            }
            Operation::Subscribe { trader_chain_id } => {
                self.state.following.push(trader_chain_id);
                let msg = Message::SubscriptionRequest {
                    subscriber_chain_id: self.runtime.chain_id(),
                };
                self.runtime.prepare_message(msg)
                    .with_authentication()
                    .send_to(trader_chain_id);
            }
            Operation::PlaceBet { market_id, prediction, amount } => {
                // 1. Update Local Market
                let mut market = self.state.markets.get(&market_id).await.expect("Error").expect("Market not found");
                if prediction {
                    market.pool_yes = market.pool_yes.saturating_add(amount);
                } else {
                    market.pool_no = market.pool_no.saturating_add(amount);
                }
                self.state.markets.insert(&market_id, market).expect("Failed to update");

                // 2. IF Trader, Broadcast Signal
                if *self.state.is_trader.get() {
                    let msg = Message::TradeSignal {
                        market_id,
                        prediction,
                        amount_percentage: 10, 
                    };
                    let count = self.state.subscribers.count();
                    for i in 0..count {
                        if let Some(sub_chain) = self.state.subscribers.get(i).await.expect("Failed to get sub") {
                             self.runtime.prepare_message(msg.clone())
                                .send_to(sub_chain);
                        }
                    }
                }
            }
        }
    }

    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            Message::SubscriptionRequest { subscriber_chain_id } => {
                self.state.subscribers.push(subscriber_chain_id);
            }
            Message::TradeSignal { market_id, prediction, amount_percentage: _ } => {
                // AUTO-EXECUTE COPY TRADE
                let amount = Amount::from_tokens(10); 
                let mut market = self.state.markets.get(&market_id).await.expect("Error").expect("Market not found");
                if prediction {
                    market.pool_yes = market.pool_yes.saturating_add(amount);
                } else {
                    market.pool_no = market.pool_no.saturating_add(amount);
                }
                self.state.markets.insert(&market_id, market).expect("Failed to update");
            }
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}