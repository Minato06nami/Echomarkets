use linera_sdk::{
    abis::{ContractAbi, ServiceAbi},
    linera_base_types::{Amount, ChainId, Timestamp},
};
use serde::{Deserialize, Serialize};

pub struct EchoMarketsAbi;

impl EchoMarketsAbi {
    pub type Operation = Operation;
    pub type Response = ();
}

impl ContractAbi for EchoMarketsAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for EchoMarketsAbi {
    type Query = async_graphql::Request;
    type QueryResponse = async_graphql::Response;
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    // Market Operations
    CreateMarket {
        question: String,
        end_time: Timestamp,
    },
    PlaceBet {
        market_id: u64,
        prediction: bool, // true = Yes/Up, false = No/Down
        amount: Amount,
    },
    // Social / Copy-Trading Operations
    RegisterAsTrader {
        name: String,
    },
    Subscribe {
        trader_chain_id: ChainId,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)] // Added Clone here
pub enum Message {
    // Sent from Subscriber to Leader to subscribe
    SubscriptionRequest {
        subscriber_chain_id: ChainId,
    },
    // Sent from Leader to Subscriber to trigger a copy-trade
    TradeSignal {
        market_id: u64,
        prediction: bool,
        amount_percentage: u8, 
    },
}