//! Error codes for Binance (version 2024-08-16).
//!
//! See [docs](https://developers.binance.com/docs/binance-spot-api-docs/errors)
//! for more information.

use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize)]
pub struct BinanceError {
    pub code: BinanceErrorCode,
    pub msg: String,
}

#[derive(Debug, Deserialize_repr)]
#[serde(untagged)]
#[repr(i32)]
pub enum BinanceErrorCode {
    InvalidRequest = 2,
    /// An unknown error occurred while processing the request.
    Unknown = -1000,
    /// Internal error; unable to process your request. Please try again.
    Disconnected = -1001,
    /// You are not authorized to execute this request.
    Unauthorized = -1002,
    /// - Too many requests queued.
    /// - Too much request weight used; current limit is %s request weight per %s.
    ///   Please use WebSocket Streams for live updates to avoid polling the API.
    /// - Way too much request weight used; IP banned until %s. Please use
    ///   WebSocket Streams for live updates to avoid bans.
    TooManyRequests = -1003,
    /// An unexpected response was received from the message bus. Execution
    /// status unknown.
    UnexpectedResp = -1006,
    /// Timeout waiting for response from backend server. Send status unknown;
    /// execution status unknown.
    Timeout = -1007,
    /// Server is currently overloaded with other requests. Please try again in
    /// a few minutes.
    ServerBusy = -1008,
    /// ERROR_MSG_RECEIVED
    ErrorMsgReceived = -1010,
    /// Filter failure
    FilterFailure = -1013,
    /// Unsupported order combination.
    UnknownOrderComposition = -1014,
    /// - Too many new orders.
    /// - Too many new orders; current limit is %s orders per %s.
    TooManyOrders = -1015,
    /// This service is no longer available.
    ServiceShuttingDown = -1016,
    /// This operation is not supported.
    UnsupportedOperation = -1020,
    /// - Timestamp for this request is outside of the recvWindow.
    /// - Timestamp for this request was 1000ms ahead of the server's time.
    InvalidTimestamp = -1021,
    /// Signature for this request is not valid.
    InvalidSignature = -1022,
    /// - Illegal characters found in a parameter.
    /// - Illegal characters found in parameter '%s'; legal range is '%s'.
    IllegalChars = -1100,
    /// - Too many parameters sent for this endpoint.
    /// - Too many parameters; expected '%s' and received '%s'.
    /// - Duplicate values for a parameter detected.
    TooManyParameters = -1101,
    /// - A mandatory parameter was not sent, was empty/null, or malformed.
    /// - Mandatory parameter '%s' was not sent, was empty/null, or malformed.
    /// - Param '%s' or '%s' must be sent, but both were empty/null!
    MandatoryParamEmptyOrMalformed = -1102,
    /// An unknown parameter was sent.
    UnknownParam = -1103,
    /// - Not all sent parameters were read.
    /// - Not all sent parameters were read; read '%s' parameter(s) but was sent '%s'.
    UnreadParameters = -1104,
    /// - A parameter was empty.
    /// - Parameter '%s' was empty.
    ParamEmpty = -1105,
    /// - A parameter was sent when not required.
    /// - Parameter '%s' sent when not required.
    ParamNotRequired = -1106,
    /// Parameter '%s' overflowed.
    ParamOverflow = -1108,
    /// Parameter '%s' has too much precision.
    BadPrecision = -1111,
    /// No orders on book for symbol.
    NoDepth = -1112,
    /// TimeInForce parameter sent when not required.
    TifNotRequired = -1114,
    /// Invalid timeInForce.
    InvalidTif = -1115,
    /// Invalid orderType.
    InvalidOrderType = -1116,
    /// Invalid side.
    InvalidSide = -1117,
    /// New client order ID was empty.
    EmptyNewClOrdId = -1118,
    /// Original client order ID was empty.
    EmptyOrigClOrdId = -1119,
    /// Invalid interval.
    BadInterval = -1120,
    /// Invalid symbol.
    BadSymbol = -1121,
    /// This listenKey does not exist.
    InvalidListenKey = -1125,
    /// - Lookup interval is too big.
    /// - More than %s hours between startTime and endTime.
    MoreThanXxHours = -1127,
    /// Combination of optional parameters invalid.
    OptionalParamsBadCombo = -1128,
    /// - Invalid data sent for a parameter.
    /// - Data sent for parameter '%s' is not valid.
    InvalidParameter = -1130,
    /// `strategyType` was less than 1000000.
    BadStrategyType = -1134,
    /// - Invalid JSON Request
    /// - JSON sent for parameter '%s' is not valid
    InvalidJson = -1135,
    /// Invalid ticker type.
    InvalidTickerType = -1139,
    /// `cancelRestrictions` has to be either `ONLY_NEW` or
    /// `ONLY_PARTIALLY_FILLED`.
    InvalidCancelRestrictions = -1145,
    /// Symbol is present multiple times in the list.
    DuplicateSymbol = -1151,
    /// Invalid `X-MBX-SBE` header; expected `<SCHEMA_ID>:<VERSION>`.
    InvalidSbeHeader = -1152,
    /// Unsupported SBE schema ID or version specified in the `X-MBX-SBE` header.
    UnsupportedSchemaId = -1153,
    /// SBE is not enabled.
    SbeDisabled = -1155,
    /// - Order type not supported in OCO.
    /// - If the order type provided in the aboveType and/or belowType is not
    ///   supported.
    OcoOrderTypeRejected = -1158,
    /// - Parameter '%s' is not supported if `aboveTimeInForce`/`belowTimeInForce`
    ///   is not GTC.
    /// - If the order type for the above or below leg is `STOP_LOSS_LIMIT`, and
    ///   `icebergQty` is provided for that leg, the `timeInForce` has to be `GTC`
    ///   else it will throw an error.
    OcoIcebergqtyTimeinforce = -1160,
    /// Requested operation is not allowed in DropCopy sessions.
    NotAllowedInDropCopySessions = -1183,
    /// DropCopy sessions are not supported on this server. Please reconnect to a
    /// drop copy server.
    DropCopySessionNotAllowed = -1184,
    /// Only DropCopy sessions are supported on this server. Either reconnect to
    /// order entry server or send `DropCopyFlag (9406)` field.
    DropCopySessionRequired = -1185,
    /// NEW_ORDER_REJECTED
    NewOrderRejected = -2010,
    /// CANCEL_REJECTED
    CancelRejected = -2011,
    /// Order does not exist.
    NoSuchOrder = -2013,
    /// API-key format invalid.
    BadApiKeyFmt = -2014,
    /// Invalid API-key, IP, or permissions for action.
    RejectedMbxKey = -2015,
    /// No trading window could be found for the symbol. Try ticker/24hrs
    /// instead.
    NoTradingWindow = -2016,
    /// This code is sent when either the cancellation of the order failed or the new order placement failed but not both.
    OrderCancelReplacePartiallyFailed = -2021,
    /// This code is sent when both the cancellation of the order failed and the new order placement failed.
    OrderCancelReplaceFailed = -2022,
    /// Order was canceled or expired with no executed qty over 90 days ago and
    /// has been archived.
    OrderArchived = -2026,
}
