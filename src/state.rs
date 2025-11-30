use linera_sdk::{
    linera_base_types::{Amount, ChainId, Timestamp, AccountOwner},
    views::{LogView, MapView, RegisterView, RootView, ViewStorageContext},
};
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(RootView, SimpleObject)]
#[view(context = ViewStorageContext)]
pub struct EchoMarkets {
    // Global Market State
    pub markets: MapView<u64, Market>,
    pub market_counter: RegisterView<u64>,

    // User Profile (if this chain acts as a Trader)
    pub is_trader: RegisterView<bool>,
    pub trader_name: RegisterView<String>,
    pub subscribers: LogView<ChainId>,

    // User Portfolio (if this chain acts as a Follower)
    pub following: LogView<ChainId>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Market {
    pub id: u64,
    pub question: String,
    pub end_time: Timestamp,
    pub resolved: bool,
    pub outcome: Option<bool>,
    pub pool_yes: Amount,
    pub pool_no: Amount,
    // Simplified: We don't store every bet in a Map here for the MVP to save state size, 
    // but in a real app we would use a MapView for bets. 
    // For hackathon, we'll assume local accounting or trust the chain state.
    // Actually, we MUST store bets to allow claiming.
    // Since MapView inside a struct is tricky with Serde, we usually store bets in a separate top-level MapView.
    // But for simplicity, let's just say we track our OWN bets in a separate view? 
    // No, the contract needs to know who bet what to payout.
    // For now, we will skip detailed payout logic to focus on the "Copy Trading" visualization.
}

// Helper to manage bets (simplified for MVP)
#[derive(Debug, Default, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Bet {
    pub owner: AccountOwner,
    pub amount: Amount,
    pub prediction: bool,
}
