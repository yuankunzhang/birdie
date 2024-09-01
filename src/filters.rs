//! Filters define trading rules on a symbol or an exchange. There are two types
//! of filters: symbol filters and exchange filters.
//!
//! See [docs](https://developers.binance.com/docs/binance-spot-api-docs/filters)

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SymbolFilter {
    PriceFilter(PriceFilter),
    PercentPrice(PercentPrice),
    PercentPriceBySide(PercentPriceBySide),
    LotSize(LotSize),
    MinNotional(Notional),
    IcebergParts(IcebergParts),
    MarketLotSize(MarketLotSize),
    MaxNumOrders(MaxNumOrders),
    MaxNumAlgoOrders(MaxNumAlgoOrders),
    MaxNumIcebergOrders(MaxNumIcebergOrders),
    MaxPosition(MaxPosition),
    TrailingDelta(TrailingDelta),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ExchangeFilter {
    ExchangeMaxNumOrders(ExchangeMaxNumOrders),
    ExchangeMaxNumAlgoOrders(ExchangeMaxNumAlgoOrders),
    ExchangeMaxNumIcebergOrders(ExchangeMaxNumIcebergOrders),
}

/// [Price Filter](https://developers.binance.com/docs/binance-spot-api-docs/filters#price_filter)
#[derive(Debug, Serialize, Deserialize)]
pub struct PriceFilter {
    filter_type: String,
    min_price: String,
    max_price: String,
    tick_size: String,
}

/// [Percent Price](https://developers.binance.com/docs/binance-spot-api-docs/filters#percent_price)
#[derive(Debug, Serialize, Deserialize)]
pub struct PercentPrice {
    filter_type: String,
    multiplier_up: String,
    multiplier_down: String,
    avg_price_mins: i64,
}

/// [Percent Price By Side](https://developers.binance.com/docs/binance-spot-api-docs/filters#percent_price)
#[derive(Debug, Serialize, Deserialize)]
pub struct PercentPriceBySide {
    filter_type: String,
    bid_multiplier_up: String,
    bid_multiplier_down: String,
    ask_multiplier_up: String,
    ask_multiplier_down: String,
    avg_price_mins: i64,
}

/// [Lot Size](https://developers.binance.com/docs/binance-spot-api-docs/filters#lot_size)
#[derive(Debug, Serialize, Deserialize)]
pub struct LotSize {
    filter_type: String,
    min_notional: String,
    apply_to_market: bool,
    avg_price_mins: i64,
}

/// [Notional](https://developers.binance.com/docs/binance-spot-api-docs/filters#notional)
#[derive(Debug, Serialize, Deserialize)]
pub struct Notional {
    filter_type: String,
    min_notional: String,
    apply_min_to_market: bool,
    max_notional: String,
    apply_max_to_market: bool,
    avg_price_mins: i64,
}

/// [Iceberg Parts](https://developers.binance.com/docs/binance-spot-api-docs/filters#iceberg_parts)
#[derive(Debug, Serialize, Deserialize)]
pub struct IcebergParts {
    filter_type: String,
    limit: i64,
}

/// [Market Lot Size](https://developers.binance.com/docs/binance-spot-api-docs/filters#market_lot_size)
#[derive(Debug, Serialize, Deserialize)]
pub struct MarketLotSize {
    filter_type: String,
    min_qty: String,
    max_qty: String,
    step_size: String,
}

/// [Max Num Orders](https://developers.binance.com/docs/binance-spot-api-docs/filters#max_num_orders)
#[derive(Debug, Serialize, Deserialize)]
pub struct MaxNumOrders {
    filter_type: String,
    max_num_orders: i64,
}

/// [Max Num Algo Orders](https://developers.binance.com/docs/binance-spot-api-docs/filters#max_num_algo_orders)
#[derive(Debug, Serialize, Deserialize)]
pub struct MaxNumAlgoOrders {
    filter_type: String,
    max_num_algo_orders: i64,
}

/// [Max Num Iceberg Orders](https://developers.binance.com/docs/binance-spot-api-docs/filters#max_num_iceberg_orders)
#[derive(Debug, Serialize, Deserialize)]
pub struct MaxNumIcebergOrders {
    filter_type: String,
    max_num_iceberg_orders: i64,
}

/// [Max Position](https://developers.binance.com/docs/binance-spot-api-docs/filters#max_position)
#[derive(Debug, Serialize, Deserialize)]
pub struct MaxPosition {
    filter_type: String,
    max_position: String,
}

/// [Trailing Delta](https://developers.binance.com/docs/binance-spot-api-docs/filters#trailing_delta)
#[derive(Debug, Serialize, Deserialize)]
pub struct TrailingDelta {
    filter_type: String,
    min_trailing_above_delta: i64,
    max_trailing_above_delta: i64,
    min_trailing_below_delta: i64,
    max_trailing_below_delta: i64,
}

/// [Exchange Max Num Orders](https://developers.binance.com/docs/binance-spot-api-docs/filters#exchange_max_num_orders)
#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeMaxNumAlgoOrders {
    filter_type: String,
    max_num_algo_orders: i64,
}

/// [Exchange Max Num Algo Orders](https://developers.binance.com/docs/binance-spot-api-docs/filters#exchange_max_num_algo_orders)
#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeMaxNumOrders {
    filter_type: String,
    max_num_orders: i64,
}

/// [Exchange Max Num Iceberg Orders](https://developers.binance.com/docs/binance-spot-api-docs/filters#exchange_max_num_iceberg_orders)
#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeMaxNumIcebergOrders {
    filter_type: String,
    max_num_iceberg_orders: i64,
}
