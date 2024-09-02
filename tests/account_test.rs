use birdie::{
    rest::{
        account::AccountInformationParams, Endpoint
    },
};

mod common;

#[tokio::test]
async fn account_information() {
    let birdie = common::setup();
    let params = AccountInformationParams::new().omit_zero_balances(true);
    let resp = birdie
        .rest()
        .account()
        .account_information()
        .request(params)
        .await;
    assert!(resp.is_ok());
}
