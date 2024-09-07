use birdie::{
    enums::{OrderSide, OrderType, TimeInForce},
    errors::{BinanceError, BinanceErrorCode},
    rest_api::{Endpoint, RestApiError},
    spot::trade::{
        AllOrdersParams, CancelAllOpenOrdersParams, CancelOrderParams, NewOrderParams,
        QueryOrderParams, TestNewOrderParams,
    },
};

mod common;

#[tokio::test]
async fn rest_test_new_order() {
    let birdie = common::setup();

    // invalid price
    let new_order_params = NewOrderParams::new("BTCUSDT", OrderSide::Buy, OrderType::Limit)
        .time_in_force(TimeInForce::Gtc)
        .quantity(1.0)
        .price(0.1);
    let params = TestNewOrderParams::new(new_order_params);
    let resp = birdie
        .rest_api()
        .trade()
        .test_new_order()
        .request(params)
        .await;
    assert!(resp.is_err());
    let RestApiError::Binance(s, Some(BinanceError { code, msg })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400 Bad Request");
    assert_eq!(msg, "Filter failure: PERCENT_PRICE_BY_SIDE");
    assert!(matches!(code, BinanceErrorCode::FilterFailure));

    // invalid symbol
    let new_order_params = NewOrderParams::new("NONEXIST", OrderSide::Buy, OrderType::Limit)
        .time_in_force(TimeInForce::Gtc)
        .quantity(1.0)
        .price(0.1);
    let params = TestNewOrderParams::new(new_order_params);
    let resp = birdie
        .rest_api()
        .trade()
        .test_new_order()
        .request(params)
        .await;
    assert!(resp.is_err());
    let RestApiError::Binance(s, Some(BinanceError { code, msg })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400 Bad Request");
    assert_eq!(msg, "Invalid symbol.");
    assert!(matches!(code, BinanceErrorCode::BadSymbol));
}

#[tokio::test]
async fn rest_query_order() {
    let birdie = common::setup();

    let params = QueryOrderParams::new("BTCUSDT");
    let resp = birdie
        .rest_api()
        .trade()
        .query_order()
        .request(params)
        .await;
    assert!(resp.is_err());
    let RestApiError::Binance(s, Some(BinanceError { code, .. })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400 Bad Request");
    assert!(matches!(
        code,
        BinanceErrorCode::MandatoryParamEmptyOrMalformed
    ));

    let params = QueryOrderParams::new("BTCUSDT").order_id(0);
    let resp = birdie
        .rest_api()
        .trade()
        .query_order()
        .request(params)
        .await;
    assert!(resp.is_err());
    let RestApiError::Binance(s, Some(BinanceError { code, .. })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400 Bad Request");
    assert!(matches!(code, BinanceErrorCode::OrderArchived));
}

#[tokio::test]
async fn rest_cancel_order() {
    let birdie = common::setup();

    let params = CancelOrderParams::new("BTCUSDT").order_id(1);
    let resp = birdie
        .rest_api()
        .trade()
        .cancel_order()
        .request(params)
        .await;
    assert!(resp.is_err());
    let RestApiError::Binance(s, Some(BinanceError { code, .. })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400 Bad Request");
    assert!(matches!(code, BinanceErrorCode::CancelRejected));
}

#[tokio::test]
async fn rest_cancel_all_open_orders() {
    let birdie = common::setup();

    let params = CancelAllOpenOrdersParams::new("BTCUSDT");
    let resp = birdie
        .rest_api()
        .trade()
        .cancel_all_open_orders()
        .request(params)
        .await;
    assert!(resp.is_err());
    let RestApiError::Binance(s, Some(BinanceError { code, .. })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400 Bad Request");
    assert!(matches!(code, BinanceErrorCode::CancelRejected));
}

#[tokio::test]
async fn rest_all_orders() {
    let birdie = common::setup();

    let params = AllOrdersParams::new("BTCUSDT");
    let resp = birdie.rest_api().trade().all_orders().request(params).await;
    assert!(resp.is_ok());
}
