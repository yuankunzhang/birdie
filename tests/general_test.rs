use birdie::{
    errors::{BinanceError, BinanceErrorCode},
    rest::{
        general::{CheckServerTimeParams, ExchangeInfoParams, TestConnectivityParams},
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
        .general()
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
        .general()
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
    let resp = birdie
        .rest()
        .general()
        .exchange_info()
        .request(params)
        .await;
    assert!(resp.is_ok());

    // symbol param
    let params = ExchangeInfoParams::new().symbol("BTCUSDT");
    let resp = birdie
        .rest()
        .general()
        .exchange_info()
        .request(params)
        .await;
    assert!(resp.is_ok());

    // invalid symbol param
    let params = ExchangeInfoParams::new().symbol("NONEXIST");
    let resp = birdie
        .rest()
        .general()
        .exchange_info()
        .request(params)
        .await;
    assert!(resp.is_err());
    let RestError::Binance(s, Some(BinanceError { code, msg })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400 Bad Request");
    assert_eq!(msg, "Invalid symbol.".to_owned());
    assert!(matches!(code, BinanceErrorCode::BadSymbol));

    // symbols param
    let params = ExchangeInfoParams::new().symbols(&["BTCUSDT", "ETHUSDT"]);
    let resp = birdie
        .rest()
        .general()
        .exchange_info()
        .request(params)
        .await;
    assert!(resp.is_ok());
    let resp = resp.unwrap();
    assert_eq!(resp.symbols.len(), 2);

    // invalid symbols param
    let params = ExchangeInfoParams::new().symbols(&["BTCUSDT", "NONEXIST"]);
    let resp = birdie
        .rest()
        .general()
        .exchange_info()
        .request(params)
        .await;
    assert!(resp.is_err());
    let RestError::Binance(s, Some(BinanceError { code, .. })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400 Bad Request");
    assert!(matches!(code, BinanceErrorCode::BadSymbol));

    // both symbol and symbols param
    let params = ExchangeInfoParams::new()
        .symbol("BTCUSDT")
        .symbols(&["ETHUSDT"]);
    let resp = birdie
        .rest()
        .general()
        .exchange_info()
        .request(params)
        .await;
    assert!(resp.is_err());
    let RestError::Binance(s, Some(BinanceError { code, .. })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400 Bad Request");
    assert!(matches!(code, BinanceErrorCode::OptionalParamsBadCombo));
}
