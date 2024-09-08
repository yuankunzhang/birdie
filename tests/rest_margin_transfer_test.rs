use birdie::{
    margin::transfer::{GetCrossMarginTransferHistoryParams, QueryMaxTransferOutAmountParams},
    rest_api::Endpoint,
};

mod common;

#[tokio::test]
async fn rest_margin_get_cross_margin_transfer_history() {
    let client = common::setup_rest_api_client();
    let params = GetCrossMarginTransferHistoryParams::new();
    let resp = client
        .margin()
        .transfer()
        .get_cross_margin_transfer_history()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_margin_query_max_transfer_out_amount() {
    let client = common::setup_rest_api_client();
    let params = QueryMaxTransferOutAmountParams::new("BTC");
    let resp = client
        .margin()
        .transfer()
        .query_max_transfer_out_amount()
        .request(params)
        .await;
    assert!(resp.is_ok());
}
