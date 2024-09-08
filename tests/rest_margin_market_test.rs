use birdie::{
    margin::market::{
        CrossMarginCollateralRatioParams, GetAllCrossMarginPairsParams,
        GetAllIsolatedMarginSymbolParams, GetAllMarginAssetsParams, GetDelistScheduleParams,
        QueryIsolatedMarginTierDataParams, QueryLiabilityCoinLeverageBracketParams,
        QueryMarginAvailableInventoryParams, QueryMarginPriceIndexParams,
    },
    rest_api::Endpoint,
};

mod common;

#[tokio::test]
async fn rest_cross_margin_collateral_ratio() {
    let client = common::setup_rest_api_client();
    let params = CrossMarginCollateralRatioParams::new();
    let resp = client
        .margin()
        .market()
        .cross_margin_collateral_ratio()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_get_all_cross_margin_pairs() {
    let client = common::setup_rest_api_client();
    let params = GetAllCrossMarginPairsParams::new();
    let resp = client
        .margin()
        .market()
        .get_all_cross_margin_pairs()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_get_all_isolated_margin_symbol() {
    let client = common::setup_rest_api_client();
    let params = GetAllIsolatedMarginSymbolParams::new();
    let resp = client
        .margin()
        .market()
        .get_all_isolated_margin_symbol()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_get_all_margin_assets() {
    let client = common::setup_rest_api_client();
    let params = GetAllMarginAssetsParams::new();
    let resp = client
        .margin()
        .market()
        .get_all_margin_assets()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_get_delist_schedule() {
    let client = common::setup_rest_api_client();
    let params = GetDelistScheduleParams::new();
    let resp = client
        .margin()
        .market()
        .get_delist_schedule()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_query_isolated_margin_tier_data() {
    let client = common::setup_rest_api_client();
    let params = QueryIsolatedMarginTierDataParams::new("BTCUSDT");
    let resp = client
        .margin()
        .market()
        .query_isolated_margin_tier_data()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_query_liability_coin_leverage_bracket() {
    let client = common::setup_rest_api_client();
    let params = QueryLiabilityCoinLeverageBracketParams::new();
    let resp = client
        .margin()
        .market()
        .query_liability_coin_leverage_bracket()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_query_margin_price_index() {
    let client = common::setup_rest_api_client();
    let params = QueryMarginPriceIndexParams::new("BTCUSDT");
    let resp = client
        .margin()
        .market()
        .query_margin_price_index()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_query_margin_available_inventory() {
    let client = common::setup_rest_api_client();
    let params = QueryMarginAvailableInventoryParams::new("MARGIN");
    let resp = client
        .margin()
        .market()
        .query_margin_available_inventory()
        .request(params)
        .await;
    assert!(resp.is_ok());
}
