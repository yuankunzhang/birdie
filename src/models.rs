//! Data models.
use serde::Deserialize;

use crate::spot::account::Balance;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTransferDetails {
    pub rows: Vec<MarginTransferRow>,
    /// Example: `1`
    pub total: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTransferRow {
    /// Example: "0.10000000"
    pub amount: String,
    /// Example: "BNB"
    pub asset: String,
    /// Example: "CONFIRMED"
    pub status: String,
    /// Example: 1566898617000
    pub timestamp: i64,
    /// Example: 5240372201
    pub tx_id: i64,
    /// Example: "ROLL_IN"
    pub r#type: String,
    /// Example: "SPOT"
    pub trans_from: String,
    /// Example: "ISOLATED_MARGIN"
    pub trans_to: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedMarginAccountInfo {
    pub assets: Vec<IsolatedMarginAccountAsset>,
    /// Example: `"0.00000000"`
    pub total_asset_of_btc: String,
    /// Example: `"0.00000000"`
    pub total_liability_of_btc: String,
    /// Example: `"0.00000000"`
    pub total_net_asset_of_btc: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedMarginAccountAsset {
    pub base_asset: Asset,
    pub quote_asset: Asset,
    /// Example: "BTCUSDT"
    pub symbol: String,
    pub isolated_created: bool,
    pub enabled: bool,
    /// Example: "0.00000000"
    pub margin_level: String,
    /// Example: "EXCESSIVE"
    pub margin_level_status: String,
    /// Example: "0.00000000"
    pub margin_ratio: String,
    /// Example: "10000.00000000"
    pub index_price: String,
    /// Example: "1.00000000"
    pub liquidate_price: String,
    /// Example: "1.00000000"
    pub liquidate_rate: String,
    pub trade_enabled: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    /// Example: "BTC"
    pub asset: String,
    pub borrow_enabled: bool,
    /// Example: "0.00000000"
    pub borrowed: String,
    /// Example: "0.00000000"
    pub free: String,
    /// Example: "0.00000000"
    pub interest: String,
    /// Example: "0.00000000"
    pub locked: String,
    /// Example: "0.00000000"
    pub net_asset: String,
    /// Example: "0.00000000"
    pub net_asset_of_btc: String,
    pub repay_enabled: bool,
    /// Example: "0.00000000"
    pub total_asset: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookTicker {
    /// Example: `"BNBBTC"`
    pub symbol: String,
    /// Example: `"16.36240000"`
    pub bid_price: String,
    /// Example: `"256.78000000"`
    pub bid_qty: String,
    /// Example: `"16.36450000"`
    pub ask_price: String,
    /// Example: `"12.56000000"`
    pub ask_qty: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceTicker {
    /// Example: `"BNBBTC"`
    pub symbol: String,
    /// Example: `"0.17160000"`
    pub price: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepaymentInfo {
    /// Example: `"BUSD"`
    pub loan_coin: String,
    /// Example: `"100.5"`
    pub remaining_principal: String,
    /// Example: `"0"`
    pub remaining_interest: String,
    /// Example: `"BNB"`
    pub collateral_coin: String,
    /// Example: `"5.253"`
    pub remaining_collateral: String,
    /// Example: `"0.25"`
    pub current_ltv: String,
    /// Example: `"Repaying"`
    pub repay_status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepaymentInfo2 {
    /// Example: `"BUSD"`
    pub loan_coin: String,
    /// Example: `"BNB"`
    pub collateral_coin: String,
    /// Example: `"Repaying"`
    pub repay_status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    /// Example: `"BNBBTC"`
    pub symbol: String,
    /// Example: `"0.17160000"`
    pub price_change: String,
    /// Example: `"1.060"`
    pub price_change_percent: String,
    /// Example: `"16.35920000"`
    pub prev_close_price: String,
    /// Example: `"27.84000000"`
    pub last_price: String,
    /// Example: `"16.34488284"`
    pub bid_price: String,
    /// Example: `"16.34488284"`
    pub bid_qty: String,
    /// Example: `"16.35920000"`
    pub ask_price: String,
    /// Example: `"25.06000000"`
    pub ask_qty: String,
    /// Example: `"16.18760000"`
    pub open_price: String,
    /// Example: `"16.55000000"`
    pub high_price: String,
    /// Example: `"16.16940000"`
    pub low_price: String,
    /// Example: `"1678279.95000000"`
    pub volume: String,
    /// Example: `"27431289.14792300"`
    pub quote_volume: String,
    /// Example: `1592808788637`
    pub open_time: i64,
    /// Example: `1592895188637`
    pub close_time: i64,
    /// Example: `62683296`
    pub first_id: i64,
    /// Example: `62739253`
    pub last_id: i64,
    /// Example: `55958`
    pub count: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayTicker {
    /// Example: `"BTCUSDT"`
    pub symbol: String,
    /// Example: `"-83.13000000"`
    pub price_change: String,
    /// Example: `"-0.317"`
    pub price_change_percent: String,
    /// Example: `"26234.58803036"`
    pub weighted_avg_price: String,
    /// Example: `"26304.80000000"`
    pub open_price: String,
    /// Example: `"26397.46000000"`
    pub high_price: String,
    /// Example: `"26088.34000000"`
    pub low_price: String,
    /// Example: `"26221.67000000"`
    pub last_price: String,
    /// Example: `"18495.35066000"`
    pub volume: String,
    /// Example: `"485217905.04210480"`
    pub quote_volume: String,
    /// Example: `1695686400000`
    pub open_time: i64,
    /// Example: `1695772799999`
    pub close_time: i64,
    /// Example: `3220151555`
    pub first_id: i64,
    /// Example: `3220849281`
    pub last_id: i64,
    /// Example: `697727`
    pub count: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// Example: `345196462`
    pub tran_id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BnbBurnStatus {
    pub spot_bnb_burn: bool,
    /// Example: `False`
    pub interest_bnb_burn: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotSpot {
    /// Example: `200`
    pub code: i64,
    /// Example: `""`
    pub msg: String,
    pub snapshot_vos: Vec<SnapshotVos>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotVos {
    pub balances: Vec<Balance>,
    /// Example: "0.09905021"
    pub total_asset_of_btc: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotMargin {
    /// Example: `200`
    pub code: i64,
    /// Example: `""`
    pub msg: String,
    pub snapshot_vos: Vec<SnapshotMarginVos>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotMarginVos {
    pub data: SnapshotMarginVosData,
    /// Example: "margin"
    pub r#type: String,
    /// Example: 1576281599000
    pub update_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotMarginVosData {
    /// Example: "2748.02909813"
    pub margin_level: String,
    /// Example: "0.00274803"
    pub total_asset_of_btc: String,
    /// Example: "0.00000100"
    pub total_liability_of_btc: String,
    /// Example: "0.00274750"
    pub total_net_asset_of_btc: String,
    pub user_assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotFutures {
    /// Example: `200`
    pub code: i64,
    /// Example: `""`
    pub msg: String,
    pub snapshot_vos: Vec<SnapshotFuturesVos>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotFuturesVos {
    pub data: SnapshotFuturesVosData,
    /// Example: "futures"
    pub r#type: String,
    /// Example: 1576281599000
    pub update_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotFuturesVosData {
    pub assets: Vec<SnapshotFuturesVosDataAsset>,
    pub position: Vec<SnapshotFuturesVosDataPosition>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotFuturesVosDataAsset {
    /// Example: "USDT"
    pub asset: String,
    /// Example: "118.99782335"
    pub margin_balance: String,
    /// Example: "120.23811389"
    pub wallet_balance: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotFuturesVosDataPosition {
    /// Example: "7130.41000000"
    pub entry_price: String,
    /// Example: "7257.66239673"
    pub mark_price: String,
    /// Example: "0.01000000"
    pub position_amt: String,
    /// Example: "BTCUSDT"
    pub symbol: String,
    /// Example: "1.24029054"
    #[serde(rename = "unRealizedProfit")]
    pub unrealized_profit: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountUsdtFuturesDetails {
    pub future_account_resp: FutureAccountResp,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureAccountResp {
    /// Example: "abc@test.com"
    pub email: String,
    pub assets: Vec<FutureAccountAsset>,
    pub can_deposit: bool,
    pub can_trade: bool,
    pub can_withdraw: bool,
    /// Example: 2
    pub fee_tier: i64,
    /// Example: "0.88308000"
    pub max_withdraw_amount: String,
    /// Example: "0.00000000"
    pub total_initial_margin: String,
    /// Example: "0.00000000"
    pub total_maintenance_margin: String,
    /// Example: "0.88308000"
    pub total_margin_balance: String,
    /// Example: "0.00000000"
    pub total_open_order_initial_margin: String,
    /// Example: "0.00000000"
    pub total_position_initial_margin: String,
    /// Example: "0.00000000"
    pub total_unrealized_profit: String,
    /// Example: "0.88308000"
    pub total_wallet_balance: String,
    /// Example: 1576756674610
    pub update_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureAccountAsset {
    /// Example: "USDT"
    pub asset: String,
    /// Example: "0.00000000"
    pub initial_margin: String,
    /// Example: "0.00000000"
    pub maintenance_margin: String,
    /// Example: "0.88308000"
    pub margin_balance: String,
    /// Example: "0.88308000"
    pub max_withdraw_amount: String,
    /// Example: "0.00000000"
    pub open_order_initial_margin: String,
    /// Example: "0.00000000"
    pub position_initial_margin: String,
    /// Example: "0.00000000"
    pub unrealized_profit: String,
    /// Example: "0.88308000"
    pub wallet_balance: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountCoinFuturesDetails {
    /// Example: `"abc@test.com"`
    pub email: String,
    pub assets: Vec<FutureAccountAsset>,
    pub can_deposit: bool,
    pub can_trade: bool,
    pub can_withdraw: bool,
    /// Example: `2`
    pub fee_tier: i64,
    /// Example: `1598959682001`
    pub update_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountUsdtFuturesSummary {
    pub future_account_summary_resp: FutureAccountSummaryResp,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureAccountSummaryResp {
    /// Example: "9.83137400"
    pub total_initial_margin: String,
    /// Example: "0.41568700"
    pub total_maintenance_margin: String,
    /// Example: "23.03235621"
    pub total_margin_balance: String,
    /// Example: "9.00000000"
    pub total_open_order_initial_margin: String,
    /// Example: "0.83137400"
    pub total_position_initial_margin: String,
    /// Example: "0.03219710"
    pub total_unrealize_profit: String,
    /// Example: "22.15879444"
    pub total_wallet_balance: String,
    /// Example: "USD"
    pub asset: String,
    pub sub_account_list: Vec<FutureAccountSubAccount>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureAccountSubAccount {
    /// Example: "123@test.com"
    pub email: String,
    /// Example: "9.00000000"
    pub total_initial_margin: String,
    /// Example: "0.00000000"
    pub total_maintenance_margin: String,
    /// Example: "22.12659734"
    pub total_margin_balance: String,
    /// Example: "9.00000000"
    pub total_open_order_initial_margin: String,
    /// Example: "0.00000000"
    pub total_position_initial_margin: String,
    /// Example: "0.00000000"
    pub total_unrealize_profit: String,
    /// Example: "22.12659734"
    pub total_wallet_balance: String,
    /// Example: "USD"
    pub asset: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountCoinFuturesSummary {
    pub delivery_account_summary_resp: DeliveryAccountSummaryResp,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryAccountSummaryResp {
    /// Example: 25.03221121
    pub total_margin_balance_of_btc: String,
    /// Example: 0.12233410
    pub total_unrealized_profit_of_btc: String,
    /// Example: 22.15879444
    pub total_wallet_balance_of_btc: String,
    /// Example: BTC
    pub asset: String,
    pub sub_account_list: Vec<DeliveryAccountSubAccount>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryAccountSubAccount {
    /// Example: 123@test.com
    pub email: String,
    /// Example: 22.12659734
    pub total_margin_balance: String,
    /// Example: 0.00000000
    pub total_unrealized_profit: String,
    /// Example: 22.12659734
    pub total_wallet_balance: String,
    /// Example: BTC
    pub asset: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountUsdtFuturesPositionRisk {
    pub future_position_risk_vos: Vec<FuturePositionRiskVos>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturePositionRiskVos {
    /// Example: "9975.12000"
    pub entry_price: String,
    /// Example: "50"
    pub leverage: String,
    /// Example: "1000000"
    pub max_notional: String,
    /// Example: "7963.54"
    pub liquidation_price: String,
    /// Example: "9973.50770517"
    pub mark_price: String,
    /// Example: "0.010"
    pub position_amount: String,
    /// Example: "BTCUSDT"
    pub symbol: String,
    /// Example: "-0.01612295"
    pub unrealized_profit: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountCoinFuturesPositionRisk {
    pub delivery_position_risk_vos: Vec<DeliveryPositionRiskVos>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPositionRiskVos {
    /// Example: "9975.12000"
    pub entry_price: String,
    /// Example: "9973.50770517"
    pub mark_price: String,
    /// Example: "20"
    pub leverage: String,
    pub isolated: String,
    /// Example: "9973.50770517"
    pub isolate_wallet: String,
    /// Example: "0.00000000"
    pub isolate_margin: String,
    /// Example: "false"
    pub is_auto_add_margin: String,
    /// Example: "BOTH"
    pub position_side: String,
    /// Example: "1.230"
    pub position_amount: String,
    /// Example: "BTCUSD_201225"
    pub symbol: String,
    /// Example: "-0.01612295"
    pub unrealized_profit: String,
}
