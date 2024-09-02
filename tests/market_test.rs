use birdie::{
    models::KlineInterval,
    rest::{
        market::{
            AggregateTradesListParams, KlineDataParams, OldTradeLookupParams, OrderBookParams,
            RecentTradesListParams, UiKlinesParams,
        },
        Endpoint,
    },
};

mod common;

#[tokio::test]
async fn order_book() {
    let birdie = common::setup();
    let params = OrderBookParams::new("BTCUSDT").limit(10);
    let resp = birdie.rest().market().order_book().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn recent_trades_list() {
    let birdie = common::setup();
    let params = RecentTradesListParams::new("BTCUSDT").limit(10);
    let resp = birdie
        .rest()
        .market()
        .recent_trades_list()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn old_trade_lookup() {
    let birdie = common::setup();
    let params = OldTradeLookupParams::new("BTCUSDT").limit(10);
    let resp = birdie
        .rest()
        .market()
        .old_trade_lookup()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn aggregate_trades_list() {
    let birdie = common::setup();
    let params = AggregateTradesListParams::new("BTCUSDT").limit(10);
    let resp = birdie
        .rest()
        .market()
        .aggregate_trades_list()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn kline_data() {
    let birdie = common::setup();
    let params = KlineDataParams::new("BTCUSDT", KlineInterval::OneHour).limit(10);
    let resp = birdie.rest().market().kline_data().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn ui_kline() {
    let birdie = common::setup();
    let params = UiKlinesParams::new("BTCUSDT", KlineInterval::OneHour).limit(10);
    let resp = birdie.rest().market().ui_klines().request(params).await;
    assert!(resp.is_ok());
}
