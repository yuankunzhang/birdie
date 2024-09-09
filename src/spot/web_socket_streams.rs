use serde::Deserialize;

use crate::web_socket_stream::Payload;

#[derive(Clone, Debug)]
pub enum SpotStreamPayload {
    AggregatedTrade(AggregatedTrade),
    Trade(Trade),
    Kline(Kline),
    MiniTicker(MiniTicker),
    Ticker(Ticker),
    RollingWindowTicker(RollingWindowTicker),
    BookTicker(BookTicker),
    AvgPrice(AvgPrice),
    PartialBookDepth(PartialBookDepth),
    Depth(Depth),
}

impl Payload for SpotStreamPayload {}

impl<'de> Deserialize<'de> for SpotStreamPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            stream: String,
            data: serde_json::Value,
        }

        let helper = Helper::deserialize(deserializer)?;

        if helper.stream.ends_with("@aggTrade") {
            Ok(SpotStreamPayload::AggregatedTrade(
                AggregatedTrade::deserialize(helper.data).map_err(serde::de::Error::custom)?,
            ))
        } else if helper.stream.ends_with("@trade") {
            Ok(SpotStreamPayload::Trade(
                Trade::deserialize(helper.data).map_err(serde::de::Error::custom)?,
            ))
        } else if helper.stream.contains("@kline_") {
            Ok(SpotStreamPayload::Kline(
                Kline::deserialize(helper.data).map_err(serde::de::Error::custom)?,
            ))
        } else if helper.stream.ends_with("@miniTicker") {
            Ok(SpotStreamPayload::MiniTicker(
                MiniTicker::deserialize(helper.data).map_err(serde::de::Error::custom)?,
            ))
        } else if helper.stream.ends_with("@ticker") {
            Ok(SpotStreamPayload::Ticker(
                Ticker::deserialize(helper.data).map_err(serde::de::Error::custom)?,
            ))
        } else if helper.stream.contains("@ticker_") {
            Ok(SpotStreamPayload::RollingWindowTicker(
                RollingWindowTicker::deserialize(helper.data).map_err(serde::de::Error::custom)?,
            ))
        } else if helper.stream.ends_with("@bookTicker") {
            Ok(SpotStreamPayload::BookTicker(
                BookTicker::deserialize(helper.data).map_err(serde::de::Error::custom)?,
            ))
        } else if helper.stream.ends_with("@avgPrice") {
            Ok(SpotStreamPayload::AvgPrice(
                AvgPrice::deserialize(helper.data).map_err(serde::de::Error::custom)?,
            ))
        } else if helper.stream.ends_with("@depth") {
            Ok(SpotStreamPayload::Depth(
                Depth::deserialize(helper.data).map_err(serde::de::Error::custom)?,
            ))
        } else if !helper.stream.ends_with("@depth") && helper.stream.contains("@depth") {
            Ok(SpotStreamPayload::PartialBookDepth(
                PartialBookDepth::deserialize(helper.data).map_err(serde::de::Error::custom)?,
            ))
        } else {
            Err(serde::de::Error::custom("Unknown stream type"))
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct AggregatedTrade {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub aggregated_trade_id: i64,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub quantity: String,
    #[serde(rename = "f")]
    pub first_trade_id: i64,
    #[serde(rename = "l")]
    pub last_trade_id: i64,
    #[serde(rename = "T")]
    pub trade_time: i64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(rename = "M")]
    pub ignore: bool,
}

impl Payload for AggregatedTrade {}

#[derive(Clone, Debug, Deserialize)]
pub struct Trade {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "t")]
    pub trade_id: i64,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub quantity: String,
    #[serde(rename = "T")]
    pub trade_time: i64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(rename = "M")]
    pub ignore: bool,
}

impl Payload for Trade {}

#[derive(Clone, Debug, Deserialize)]
pub struct Kline {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "k")]
    pub kline: KlineData,
}

impl Payload for Kline {}

#[derive(Clone, Debug, Deserialize)]
pub struct KlineData {
    #[serde(rename = "t")]
    pub start_time: i64,
    #[serde(rename = "T")]
    pub close_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: String,
    #[serde(rename = "f")]
    pub first_trade_id: i64,
    #[serde(rename = "L")]
    pub last_trade_id: i64,
    #[serde(rename = "o")]
    pub open_price: String,
    #[serde(rename = "c")]
    pub close_price: String,
    #[serde(rename = "h")]
    pub high_price: String,
    #[serde(rename = "l")]
    pub low_price: String,
    #[serde(rename = "v")]
    pub base_asset_volume: String,
    #[serde(rename = "n")]
    pub number_of_trades: i64,
    #[serde(rename = "x")]
    pub is_closed: bool,
    #[serde(rename = "q")]
    pub quote_asset_volume: String,
    #[serde(rename = "V")]
    pub taker_buy_base_asset_volume: String,
    #[serde(rename = "Q")]
    pub taker_buy_quote_asset_volume: String,
    #[serde(rename = "B")]
    pub ignore: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MiniTicker {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub close_price: String,
    #[serde(rename = "o")]
    pub open_price: String,
    #[serde(rename = "h")]
    pub high_price: String,
    #[serde(rename = "l")]
    pub low_price: String,
    #[serde(rename = "v")]
    pub base_asset_volume: String,
    #[serde(rename = "q")]
    pub quote_asset_volume: String,
}

impl Payload for MiniTicker {}

#[derive(Clone, Debug, Deserialize)]
pub struct Ticker {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub price_change: String,
    #[serde(rename = "P")]
    pub price_change_percent: String,
    #[serde(rename = "w")]
    pub weighted_avg_price: String,
    #[serde(rename = "x")]
    pub first_trade_price: String,
    #[serde(rename = "c")]
    pub last_price: String,
    #[serde(rename = "Q")]
    pub last_quantity: String,
    #[serde(rename = "b")]
    pub best_bid_price: String,
    #[serde(rename = "B")]
    pub best_bid_quantity: String,
    #[serde(rename = "a")]
    pub best_ask_price: String,
    #[serde(rename = "A")]
    pub best_ask_quantity: String,
    #[serde(rename = "o")]
    pub open_price: String,
    #[serde(rename = "h")]
    pub high_price: String,
    #[serde(rename = "l")]
    pub low_price: String,
    #[serde(rename = "v")]
    pub base_asset_volume: String,
    #[serde(rename = "q")]
    pub quote_asset_volume: String,
    #[serde(rename = "O")]
    pub open_time: i64,
    #[serde(rename = "C")]
    pub close_time: i64,
    #[serde(rename = "F")]
    pub first_trade_id: i64,
    #[serde(rename = "L")]
    pub last_trade_id: i64,
    #[serde(rename = "n")]
    pub number_of_trades: i64,
}

impl Payload for Ticker {}

#[derive(Clone, Debug, Deserialize)]
pub struct RollingWindowTicker {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub price_change: String,
    #[serde(rename = "P")]
    pub price_change_percent: String,
    #[serde(rename = "o")]
    pub open_price: String,
    #[serde(rename = "h")]
    pub high_price: String,
    #[serde(rename = "l")]
    pub low_price: String,
    #[serde(rename = "c")]
    pub close_price: String,
    #[serde(rename = "w")]
    pub weighted_avg_price: String,
    #[serde(rename = "v")]
    pub base_asset_volume: String,
    #[serde(rename = "q")]
    pub quote_asset_volume: String,
    #[serde(rename = "O")]
    pub open_time: i64,
    #[serde(rename = "C")]
    pub close_time: i64,
    #[serde(rename = "F")]
    pub first_trade_id: i64,
    #[serde(rename = "L")]
    pub last_trade_id: i64,
    #[serde(rename = "n")]
    pub number_of_trades: i64,
}

impl Payload for RollingWindowTicker {}

#[derive(Clone, Debug, Deserialize)]
pub struct BookTicker {
    #[serde(rename = "u")]
    pub order_book_update_id: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "b")]
    pub best_bid_price: String,
    #[serde(rename = "B")]
    pub best_bid_quantity: String,
    #[serde(rename = "a")]
    pub best_ask_price: String,
    #[serde(rename = "A")]
    pub best_ask_quantity: String,
}

impl Payload for BookTicker {}

#[derive(Clone, Debug, Deserialize)]
pub struct AvgPrice {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub average_price_interval: String,
    #[serde(rename = "w")]
    pub average_price: String,
    #[serde(rename = "T")]
    pub last_trade_time: i64,
}

impl Payload for AvgPrice {}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialBookDepth {
    pub last_update_id: i64,
    pub bids: Vec<(String, String)>,
    pub asks: Vec<(String, String)>,
}

impl Payload for PartialBookDepth {}

#[derive(Clone, Debug, Deserialize)]
pub struct Depth {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "U")]
    pub first_update_id: i64,
    #[serde(rename = "u")]
    pub final_update_id: i64,
    #[serde(rename = "b")]
    pub bids: Vec<(String, String)>,
    #[serde(rename = "a")]
    pub asks: Vec<(String, String)>,
}

impl Payload for Depth {}
