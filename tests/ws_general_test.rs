use birdie::{
    errors::{BinanceError, BinanceErrorCode},
    spot::general::{CheckServerTimeParams, ExchangeInfoParams, TestConnectivityParams},
    web_socket_api::{ConnectionStatus, WebSocket, WebSocketApiError},
};

mod common;

#[tokio::test]
async fn ws_test_connectivity() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    let params = TestConnectivityParams::new();
    let resp = client.general().test_connectivity().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn ws_check_server_time() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    let params = CheckServerTimeParams::new();
    let resp = client.general().check_server_time().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn ws_exchange_info() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    // no param
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
    let WebSocketApiError::Binance(s, Some(BinanceError { code, .. })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400");
    assert!(matches!(code, BinanceErrorCode::BadSymbol));

    // both symbol and symbols param
    let params = ExchangeInfoParams::new()
        .symbol("BTCUSDT")
        .symbols(&["ETHUSDT"]);
    let resp = client.general().exchange_info().request(params).await;
    assert!(resp.is_err());
    let WebSocketApiError::Binance(s, Some(BinanceError { code, .. })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400");
    assert!(matches!(code, BinanceErrorCode::OptionalParamsBadCombo));
}
