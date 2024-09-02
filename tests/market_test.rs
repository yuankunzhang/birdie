use birdie::{
    errors::{BinanceError, BinanceErrorCode},
    rest::{
        market::{AggregateTradesListParams, CheckServerTimeParams, ExchangeInfoParams, OldTradeLookupParams, OrderBookParams, RecentTradesListParams, TestConnectivityParams},
        Endpoint, RestError,
    },
};

mod common;

#[tokio::test]
async fn test_connectivity() {
    let birdie = common::setup();
    let params = TestConnectivityParams::new();
    let resp = birdie
        .rest()
        .market()
        .test_connectivity()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn check_server_time() {
    let birdie = common::setup();
    let params = CheckServerTimeParams::new();
    let resp = birdie
        .rest()
        .market()
        .check_server_time()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn exchange_info() {
    let birdie = common::setup();

    // no params
    let params = ExchangeInfoParams::new();
    let resp = birdie.rest().market().exchange_info().request(params).await;
    assert!(resp.is_ok());

    // symbol param
    let params = ExchangeInfoParams::new().symbol("BTCUSDT");
    let resp = birdie.rest().market().exchange_info().request(params).await;
    assert!(resp.is_ok());

    // invalid symbol param
    let params = ExchangeInfoParams::new().symbol("NONEXIST");
    let resp = birdie.rest().market().exchange_info().request(params).await;
    assert!(resp.is_err());
    let RestError::Binance(s, Some(BinanceError { code, msg })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400 Bad Request");
    assert_eq!(msg, "Invalid symbol.".to_owned());
    assert!(matches!(code, BinanceErrorCode::BadSymbol));

    // symbols param
    let params = ExchangeInfoParams::new().symbols(&["BTCUSDT", "ETHUSDT"]);
    let resp = birdie.rest().market().exchange_info().request(params).await;
    assert!(resp.is_ok());
    let resp = resp.unwrap();
    assert_eq!(resp.symbols.len(), 2);

    // invalid symbols param
    let params = ExchangeInfoParams::new().symbols(&["BTCUSDT", "NONEXIST"]);
    let resp = birdie.rest().market().exchange_info().request(params).await;
    assert!(resp.is_err());
    let RestError::Binance(s, Some(BinanceError { code, .. })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400 Bad Request");
    assert!(matches!(code, BinanceErrorCode::BadSymbol));

    // both symbol and symbols param
    let params = ExchangeInfoParams::new().symbol("BTCUSDT").symbols(&["ETHUSDT"]);
    let resp = birdie.rest().market().exchange_info().request(params).await;
    assert!(resp.is_err());
    let RestError::Binance(s, Some(BinanceError { code, .. })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400 Bad Request");
    assert!(matches!(code, BinanceErrorCode::OptionalParamsBadCombo));
}

#[tokio::test]
async fn order_book() {
    let birdie = common::setup();
    let params = OrderBookParams::new("BTCUSDT").limit(10);
    let resp = birdie
        .rest()
        .market()
        .order_book()
        .request(params)
        .await;
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
