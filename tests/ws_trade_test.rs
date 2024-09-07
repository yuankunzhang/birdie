use birdie::{
    enums::{OrderSide, OrderType, TimeInForce},
    errors::{BinanceError, BinanceErrorCode},
    spot::trade::{NewOrderParams, TestNewOrderParams},
    web_socket_api::{ConnectionStatus, WebSocket, WebSocketApiError},
};

mod common;

#[tokio::test]
async fn ws_test_new_order() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    // invalid price
    let new_order_params = NewOrderParams::new("BTCUSDT", OrderSide::Buy, OrderType::Limit)
        .time_in_force(TimeInForce::Gtc)
        .quantity(1.0)
        .price(0.1);
    let params = TestNewOrderParams::new(new_order_params);
    let resp = client.trade().test_new_order().request(params).await;
    assert!(resp.is_err());
    let WebSocketApiError::Binance(s, Some(BinanceError { code, msg })) = resp.err().unwrap()
    else {
        panic!()
    };
    assert_eq!(s, "400");
    assert_eq!(msg, "Filter failure: PERCENT_PRICE_BY_SIDE");
    assert!(matches!(code, BinanceErrorCode::FilterFailure));

    // invalid symbol
    let new_order_params = NewOrderParams::new("NONEXIST", OrderSide::Buy, OrderType::Limit)
        .time_in_force(TimeInForce::Gtc)
        .quantity(1.0)
        .price(0.1);
    let params = TestNewOrderParams::new(new_order_params);
    let resp = client.trade().test_new_order().request(params).await;
    assert!(resp.is_err());
    let WebSocketApiError::Binance(s, Some(BinanceError { code, msg })) = resp.err().unwrap()
    else {
        panic!()
    };
    assert_eq!(s, "400");
    assert_eq!(msg, "Invalid symbol.");
    assert!(matches!(code, BinanceErrorCode::BadSymbol));
}
