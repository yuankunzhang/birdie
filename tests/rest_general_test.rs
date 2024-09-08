use birdie::{
    errors::{BinanceError, BinanceErrorCode},
    rest_api::{Endpoint, RestApiError},
    spot::general::{CheckServerTimeParams, ExchangeInfoParams, TestConnectivityParams},
};

mod common;

#[tokio::test]
async fn rest_test_connectivity() {
    let client = common::setup_rest_api_client();
    let params = TestConnectivityParams::new();
    let resp = client.general().test_connectivity().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_check_server_time() {
    let client = common::setup_rest_api_client();
    let params = CheckServerTimeParams::new();
    let resp = client.general().check_server_time().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_exchange_info() {
    let client = common::setup_rest_api_client();

    // no params
    let params = ExchangeInfoParams::new();
    let resp = client.general().exchange_info().request(params).await;
    assert!(resp.is_ok());

    // symbol param
    let params = ExchangeInfoParams::new().symbol("BTCUSDT");
    let resp = client.general().exchange_info().request(params).await;
    assert!(resp.is_ok());

    // invalid symbol param
    let params = ExchangeInfoParams::new().symbol("NONEXIST");
    let resp = client.general().exchange_info().request(params).await;
    assert!(resp.is_err());
    let RestApiError::Binance(s, Some(BinanceError { code, msg })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400 Bad Request");
    assert_eq!(msg, "Invalid symbol.".to_owned());
    assert!(matches!(code, BinanceErrorCode::BadSymbol));

    // symbols param
    let params = ExchangeInfoParams::new().symbols(&["BTCUSDT", "ETHUSDT"]);
    let resp = client.general().exchange_info().request(params).await;
    assert!(resp.is_ok());
    let resp = resp.unwrap();
    assert_eq!(resp.symbols.len(), 2);

    // invalid symbols param
    let params = ExchangeInfoParams::new().symbols(&["BTCUSDT", "NONEXIST"]);
    let resp = client.general().exchange_info().request(params).await;
    assert!(resp.is_err());
    let RestApiError::Binance(s, Some(BinanceError { code, .. })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400 Bad Request");
    assert!(matches!(code, BinanceErrorCode::BadSymbol));

    // both symbol and symbols param
    let params = ExchangeInfoParams::new()
        .symbol("BTCUSDT")
        .symbols(&["ETHUSDT"]);
    let resp = client.general().exchange_info().request(params).await;
    assert!(resp.is_err());
    let RestApiError::Binance(s, Some(BinanceError { code, .. })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400 Bad Request");
    assert!(matches!(code, BinanceErrorCode::OptionalParamsBadCombo));
}
