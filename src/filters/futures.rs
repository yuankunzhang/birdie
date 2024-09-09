use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SymbolFilter {
    PriceFilter(PriceFilter),
    PercentPrice(PercentPrice),
    LotSize(LotSize),
    MinNotional(MinNotional),
    MarketLotSize(MarketLotSize),
    MaxNumOrders(MaxNumOrders),
    MaxNumAlgoOrders(MaxNumAlgoOrders),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceFilter {
    filter_type: String,
    min_price: String,
    max_price: String,
    tick_size: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PercentPrice {
    filter_type: String,
    multiplier_up: String,
    multiplier_down: String,
    avg_price_mins: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LotSize {
    filter_type: String,
    min_notional: String,
    apply_to_market: bool,
    avg_price_mins: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinNotional {
    filter_type: String,
    min_notional: String,
    apply_to_market: bool,
    avg_price_mins: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketLotSize {
    filter_type: String,
    min_qty: String,
    max_qty: String,
    step_size: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaxNumOrders {
    filter_type: String,
    max_num_orders: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaxNumAlgoOrders {
    filter_type: String,
    max_num_algo_orders: i64,
}
