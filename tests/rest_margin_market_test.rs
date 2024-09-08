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
    let birdie = common::setup();
    let params = CrossMarginCollateralRatioParams::new();
    let resp = birdie
        .rest_api()
        .margin()
        .market()
        .cross_margin_collateral_ratio()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_get_all_cross_margin_pairs() {
    let birdie = common::setup();
    let params = GetAllCrossMarginPairsParams::new();
    let resp = birdie
        .rest_api()
        .margin()
        .market()
        .get_all_cross_margin_pairs()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_get_all_isolated_margin_symbol() {
    let birdie = common::setup();
    let params = GetAllIsolatedMarginSymbolParams::new();
    let resp = birdie
        .rest_api()
        .margin()
        .market()
        .get_all_isolated_margin_symbol()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_get_all_margin_assets() {
    let birdie = common::setup();
    let params = GetAllMarginAssetsParams::new();
    let resp = birdie
        .rest_api()
        .margin()
        .market()
        .get_all_margin_assets()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_get_delist_schedule() {
    let birdie = common::setup();
    let params = GetDelistScheduleParams::new();
    let resp = birdie
        .rest_api()
        .margin()
        .market()
        .get_delist_schedule()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_query_isolated_margin_tier_data() {
    let birdie = common::setup();
    let params = QueryIsolatedMarginTierDataParams::new("BTCUSDT");
    let resp = birdie
        .rest_api()
        .margin()
        .market()
        .query_isolated_margin_tier_data()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_query_liability_coin_leverage_bracket() {
    let birdie = common::setup();
    let params = QueryLiabilityCoinLeverageBracketParams::new();
    let resp = birdie
        .rest_api()
        .margin()
        .market()
        .query_liability_coin_leverage_bracket()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_query_margin_price_index() {
    let birdie = common::setup();
    let params = QueryMarginPriceIndexParams::new("BTCUSDT");
    let resp = birdie
        .rest_api()
        .margin()
        .market()
        .query_margin_price_index()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_query_margin_available_inventory() {
    let birdie = common::setup();
    let params = QueryMarginAvailableInventoryParams::new("MARGIN");
    let resp = birdie
        .rest_api()
        .margin()
        .market()
        .query_margin_available_inventory()
        .request(params)
        .await;
    assert!(resp.is_ok());
}
