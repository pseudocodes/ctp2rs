
use crate::v1alpha1::bindings::*;

#[derive(Clone, Debug)]
pub enum MdSpiEvent {
    OnFrontConnected(MdSpiOnFrontConnectedEvent),
    OnFrontDisconnected(MdSpiOnFrontDisconnectedEvent),
    OnHeartBeatWarning(MdSpiOnHeartBeatWarningEvent),
    OnRspUserLogin(MdSpiOnRspUserLoginEvent),
    OnRspUserLogout(MdSpiOnRspUserLogoutEvent),
    OnRspQryMulticastInstrument(MdSpiOnRspQryMulticastInstrumentEvent),
    OnRspError(MdSpiOnRspErrorEvent),
    OnRspSubMarketData(MdSpiOnRspSubMarketDataEvent),
    OnRspUnSubMarketData(MdSpiOnRspUnSubMarketDataEvent),
    OnRspSubForQuoteRsp(MdSpiOnRspSubForQuoteRspEvent),
    OnRspUnSubForQuoteRsp(MdSpiOnRspUnSubForQuoteRspEvent),
    OnRtnDepthMarketData(MdSpiOnRtnDepthMarketDataEvent),
    OnRtnForQuoteRsp(MdSpiOnRtnForQuoteRspEvent),
}
    

#[derive(Clone, Debug)]
pub struct MdSpiOnFrontConnectedEvent {

}

#[derive(Clone, Debug)]
pub struct MdSpiOnFrontDisconnectedEvent {
    pub reason: i32,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnHeartBeatWarningEvent {
    pub time_lapse: i32,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnRspUserLoginEvent {
    pub rsp_user_login: Option<CThostFtdcRspUserLoginField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnRspUserLogoutEvent {
    pub user_logout: Option<CThostFtdcUserLogoutField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnRspQryMulticastInstrumentEvent {
    pub multicast_instrument: Option<CThostFtdcMulticastInstrumentField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnRspErrorEvent {
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnRspSubMarketDataEvent {
    pub specific_instrument: Option<CThostFtdcSpecificInstrumentField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnRspUnSubMarketDataEvent {
    pub specific_instrument: Option<CThostFtdcSpecificInstrumentField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnRspSubForQuoteRspEvent {
    pub specific_instrument: Option<CThostFtdcSpecificInstrumentField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnRspUnSubForQuoteRspEvent {
    pub specific_instrument: Option<CThostFtdcSpecificInstrumentField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnRtnDepthMarketDataEvent {
    pub depth_market_data: Option<CThostFtdcDepthMarketDataField>,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnRtnForQuoteRspEvent {
    pub for_quote_rsp: Option<CThostFtdcForQuoteRspField>,
}

#[derive(Clone, Debug)]
pub enum TraderSpiEvent {
    OnFrontConnected(TraderSpiOnFrontConnectedEvent),
    OnFrontDisconnected(TraderSpiOnFrontDisconnectedEvent),
    OnHeartBeatWarning(TraderSpiOnHeartBeatWarningEvent),
    OnRspAuthenticate(TraderSpiOnRspAuthenticateEvent),
    OnRspUserLogin(TraderSpiOnRspUserLoginEvent),
    OnRspUserLogout(TraderSpiOnRspUserLogoutEvent),
    OnRspUserPasswordUpdate(TraderSpiOnRspUserPasswordUpdateEvent),
    OnRspTradingAccountPasswordUpdate(TraderSpiOnRspTradingAccountPasswordUpdateEvent),
    OnRspUserAuthMethod(TraderSpiOnRspUserAuthMethodEvent),
    OnRspGenUserCaptcha(TraderSpiOnRspGenUserCaptchaEvent),
    OnRspGenUserText(TraderSpiOnRspGenUserTextEvent),
    OnRspOrderInsert(TraderSpiOnRspOrderInsertEvent),
    OnRspParkedOrderInsert(TraderSpiOnRspParkedOrderInsertEvent),
    OnRspParkedOrderAction(TraderSpiOnRspParkedOrderActionEvent),
    OnRspOrderAction(TraderSpiOnRspOrderActionEvent),
    OnRspQryMaxOrderVolume(TraderSpiOnRspQryMaxOrderVolumeEvent),
    OnRspSettlementInfoConfirm(TraderSpiOnRspSettlementInfoConfirmEvent),
    OnRspRemoveParkedOrder(TraderSpiOnRspRemoveParkedOrderEvent),
    OnRspRemoveParkedOrderAction(TraderSpiOnRspRemoveParkedOrderActionEvent),
    OnRspExecOrderInsert(TraderSpiOnRspExecOrderInsertEvent),
    OnRspExecOrderAction(TraderSpiOnRspExecOrderActionEvent),
    OnRspForQuoteInsert(TraderSpiOnRspForQuoteInsertEvent),
    OnRspQuoteInsert(TraderSpiOnRspQuoteInsertEvent),
    OnRspQuoteAction(TraderSpiOnRspQuoteActionEvent),
    OnRspBatchOrderAction(TraderSpiOnRspBatchOrderActionEvent),
    OnRspOptionSelfCloseInsert(TraderSpiOnRspOptionSelfCloseInsertEvent),
    OnRspOptionSelfCloseAction(TraderSpiOnRspOptionSelfCloseActionEvent),
    OnRspCombActionInsert(TraderSpiOnRspCombActionInsertEvent),
    OnRspQryOrder(TraderSpiOnRspQryOrderEvent),
    OnRspQryTrade(TraderSpiOnRspQryTradeEvent),
    OnRspQryInvestorPosition(TraderSpiOnRspQryInvestorPositionEvent),
    OnRspQryTradingAccount(TraderSpiOnRspQryTradingAccountEvent),
    OnRspQryInvestor(TraderSpiOnRspQryInvestorEvent),
    OnRspQryTradingCode(TraderSpiOnRspQryTradingCodeEvent),
    OnRspQryInstrumentMarginRate(TraderSpiOnRspQryInstrumentMarginRateEvent),
    OnRspQryInstrumentCommissionRate(TraderSpiOnRspQryInstrumentCommissionRateEvent),
    OnRspQryExchange(TraderSpiOnRspQryExchangeEvent),
    OnRspQryProduct(TraderSpiOnRspQryProductEvent),
    OnRspQryInstrument(TraderSpiOnRspQryInstrumentEvent),
    OnRspQryDepthMarketData(TraderSpiOnRspQryDepthMarketDataEvent),
    OnRspQryTraderOffer(TraderSpiOnRspQryTraderOfferEvent),
    OnRspQrySettlementInfo(TraderSpiOnRspQrySettlementInfoEvent),
    OnRspQryTransferBank(TraderSpiOnRspQryTransferBankEvent),
    OnRspQryInvestorPositionDetail(TraderSpiOnRspQryInvestorPositionDetailEvent),
    OnRspQryNotice(TraderSpiOnRspQryNoticeEvent),
    OnRspQrySettlementInfoConfirm(TraderSpiOnRspQrySettlementInfoConfirmEvent),
    OnRspQryInvestorPositionCombineDetail(TraderSpiOnRspQryInvestorPositionCombineDetailEvent),
    OnRspQryCFMMCTradingAccountKey(TraderSpiOnRspQryCFMMCTradingAccountKeyEvent),
    OnRspQryEWarrantOffset(TraderSpiOnRspQryEWarrantOffsetEvent),
    OnRspQryInvestorProductGroupMargin(TraderSpiOnRspQryInvestorProductGroupMarginEvent),
    OnRspQryExchangeMarginRate(TraderSpiOnRspQryExchangeMarginRateEvent),
    OnRspQryExchangeMarginRateAdjust(TraderSpiOnRspQryExchangeMarginRateAdjustEvent),
    OnRspQryExchangeRate(TraderSpiOnRspQryExchangeRateEvent),
    OnRspQrySecAgentACIDMap(TraderSpiOnRspQrySecAgentACIDMapEvent),
    OnRspQryProductExchRate(TraderSpiOnRspQryProductExchRateEvent),
    OnRspQryProductGroup(TraderSpiOnRspQryProductGroupEvent),
    OnRspQryMMInstrumentCommissionRate(TraderSpiOnRspQryMMInstrumentCommissionRateEvent),
    OnRspQryMMOptionInstrCommRate(TraderSpiOnRspQryMMOptionInstrCommRateEvent),
    OnRspQryInstrumentOrderCommRate(TraderSpiOnRspQryInstrumentOrderCommRateEvent),
    OnRspQrySecAgentTradingAccount(TraderSpiOnRspQrySecAgentTradingAccountEvent),
    OnRspQrySecAgentCheckMode(TraderSpiOnRspQrySecAgentCheckModeEvent),
    OnRspQrySecAgentTradeInfo(TraderSpiOnRspQrySecAgentTradeInfoEvent),
    OnRspQryOptionInstrTradeCost(TraderSpiOnRspQryOptionInstrTradeCostEvent),
    OnRspQryOptionInstrCommRate(TraderSpiOnRspQryOptionInstrCommRateEvent),
    OnRspQryExecOrder(TraderSpiOnRspQryExecOrderEvent),
    OnRspQryForQuote(TraderSpiOnRspQryForQuoteEvent),
    OnRspQryQuote(TraderSpiOnRspQryQuoteEvent),
    OnRspQryOptionSelfClose(TraderSpiOnRspQryOptionSelfCloseEvent),
    OnRspQryInvestUnit(TraderSpiOnRspQryInvestUnitEvent),
    OnRspQryCombInstrumentGuard(TraderSpiOnRspQryCombInstrumentGuardEvent),
    OnRspQryCombAction(TraderSpiOnRspQryCombActionEvent),
    OnRspQryTransferSerial(TraderSpiOnRspQryTransferSerialEvent),
    OnRspQryAccountregister(TraderSpiOnRspQryAccountregisterEvent),
    OnRspError(TraderSpiOnRspErrorEvent),
    OnRtnOrder(TraderSpiOnRtnOrderEvent),
    OnRtnTrade(TraderSpiOnRtnTradeEvent),
    OnErrRtnOrderInsert(TraderSpiOnErrRtnOrderInsertEvent),
    OnErrRtnOrderAction(TraderSpiOnErrRtnOrderActionEvent),
    OnRtnInstrumentStatus(TraderSpiOnRtnInstrumentStatusEvent),
    OnRtnBulletin(TraderSpiOnRtnBulletinEvent),
    OnRtnTradingNotice(TraderSpiOnRtnTradingNoticeEvent),
    OnRtnErrorConditionalOrder(TraderSpiOnRtnErrorConditionalOrderEvent),
    OnRtnExecOrder(TraderSpiOnRtnExecOrderEvent),
    OnErrRtnExecOrderInsert(TraderSpiOnErrRtnExecOrderInsertEvent),
    OnErrRtnExecOrderAction(TraderSpiOnErrRtnExecOrderActionEvent),
    OnErrRtnForQuoteInsert(TraderSpiOnErrRtnForQuoteInsertEvent),
    OnRtnQuote(TraderSpiOnRtnQuoteEvent),
    OnErrRtnQuoteInsert(TraderSpiOnErrRtnQuoteInsertEvent),
    OnErrRtnQuoteAction(TraderSpiOnErrRtnQuoteActionEvent),
    OnRtnForQuoteRsp(TraderSpiOnRtnForQuoteRspEvent),
    OnRtnCFMMCTradingAccountToken(TraderSpiOnRtnCFMMCTradingAccountTokenEvent),
    OnErrRtnBatchOrderAction(TraderSpiOnErrRtnBatchOrderActionEvent),
    OnRtnOptionSelfClose(TraderSpiOnRtnOptionSelfCloseEvent),
    OnErrRtnOptionSelfCloseInsert(TraderSpiOnErrRtnOptionSelfCloseInsertEvent),
    OnErrRtnOptionSelfCloseAction(TraderSpiOnErrRtnOptionSelfCloseActionEvent),
    OnRtnCombAction(TraderSpiOnRtnCombActionEvent),
    OnErrRtnCombActionInsert(TraderSpiOnErrRtnCombActionInsertEvent),
    OnRspQryContractBank(TraderSpiOnRspQryContractBankEvent),
    OnRspQryParkedOrder(TraderSpiOnRspQryParkedOrderEvent),
    OnRspQryParkedOrderAction(TraderSpiOnRspQryParkedOrderActionEvent),
    OnRspQryTradingNotice(TraderSpiOnRspQryTradingNoticeEvent),
    OnRspQryBrokerTradingParams(TraderSpiOnRspQryBrokerTradingParamsEvent),
    OnRspQryBrokerTradingAlgos(TraderSpiOnRspQryBrokerTradingAlgosEvent),
    OnRspQueryCFMMCTradingAccountToken(TraderSpiOnRspQueryCFMMCTradingAccountTokenEvent),
    OnRtnFromBankToFutureByBank(TraderSpiOnRtnFromBankToFutureByBankEvent),
    OnRtnFromFutureToBankByBank(TraderSpiOnRtnFromFutureToBankByBankEvent),
    OnRtnRepealFromBankToFutureByBank(TraderSpiOnRtnRepealFromBankToFutureByBankEvent),
    OnRtnRepealFromFutureToBankByBank(TraderSpiOnRtnRepealFromFutureToBankByBankEvent),
    OnRtnFromBankToFutureByFuture(TraderSpiOnRtnFromBankToFutureByFutureEvent),
    OnRtnFromFutureToBankByFuture(TraderSpiOnRtnFromFutureToBankByFutureEvent),
    OnRtnRepealFromBankToFutureByFutureManual(TraderSpiOnRtnRepealFromBankToFutureByFutureManualEvent),
    OnRtnRepealFromFutureToBankByFutureManual(TraderSpiOnRtnRepealFromFutureToBankByFutureManualEvent),
    OnRtnQueryBankBalanceByFuture(TraderSpiOnRtnQueryBankBalanceByFutureEvent),
    OnErrRtnBankToFutureByFuture(TraderSpiOnErrRtnBankToFutureByFutureEvent),
    OnErrRtnFutureToBankByFuture(TraderSpiOnErrRtnFutureToBankByFutureEvent),
    OnErrRtnRepealBankToFutureByFutureManual(TraderSpiOnErrRtnRepealBankToFutureByFutureManualEvent),
    OnErrRtnRepealFutureToBankByFutureManual(TraderSpiOnErrRtnRepealFutureToBankByFutureManualEvent),
    OnErrRtnQueryBankBalanceByFuture(TraderSpiOnErrRtnQueryBankBalanceByFutureEvent),
    OnRtnRepealFromBankToFutureByFuture(TraderSpiOnRtnRepealFromBankToFutureByFutureEvent),
    OnRtnRepealFromFutureToBankByFuture(TraderSpiOnRtnRepealFromFutureToBankByFutureEvent),
    OnRspFromBankToFutureByFuture(TraderSpiOnRspFromBankToFutureByFutureEvent),
    OnRspFromFutureToBankByFuture(TraderSpiOnRspFromFutureToBankByFutureEvent),
    OnRspQueryBankAccountMoneyByFuture(TraderSpiOnRspQueryBankAccountMoneyByFutureEvent),
    OnRtnOpenAccountByBank(TraderSpiOnRtnOpenAccountByBankEvent),
    OnRtnCancelAccountByBank(TraderSpiOnRtnCancelAccountByBankEvent),
    OnRtnChangeAccountByBank(TraderSpiOnRtnChangeAccountByBankEvent),
    OnRspQryClassifiedInstrument(TraderSpiOnRspQryClassifiedInstrumentEvent),
    OnRspQryCombPromotionParam(TraderSpiOnRspQryCombPromotionParamEvent),
    OnRspQryRiskSettleInvstPosition(TraderSpiOnRspQryRiskSettleInvstPositionEvent),
    OnRspQryRiskSettleProductStatus(TraderSpiOnRspQryRiskSettleProductStatusEvent),
    OnRspQrySPBMFutureParameter(TraderSpiOnRspQrySPBMFutureParameterEvent),
    OnRspQrySPBMOptionParameter(TraderSpiOnRspQrySPBMOptionParameterEvent),
    OnRspQrySPBMIntraParameter(TraderSpiOnRspQrySPBMIntraParameterEvent),
    OnRspQrySPBMInterParameter(TraderSpiOnRspQrySPBMInterParameterEvent),
    OnRspQrySPBMPortfDefinition(TraderSpiOnRspQrySPBMPortfDefinitionEvent),
    OnRspQrySPBMInvestorPortfDef(TraderSpiOnRspQrySPBMInvestorPortfDefEvent),
    OnRspQryInvestorPortfMarginRatio(TraderSpiOnRspQryInvestorPortfMarginRatioEvent),
    OnRspQryInvestorProdSPBMDetail(TraderSpiOnRspQryInvestorProdSPBMDetailEvent),
    OnRspQryInvestorCommoditySPMMMargin(TraderSpiOnRspQryInvestorCommoditySPMMMarginEvent),
    OnRspQryInvestorCommodityGroupSPMMMargin(TraderSpiOnRspQryInvestorCommodityGroupSPMMMarginEvent),
    OnRspQrySPMMInstParam(TraderSpiOnRspQrySPMMInstParamEvent),
    OnRspQrySPMMProductParam(TraderSpiOnRspQrySPMMProductParamEvent),
    OnRspQrySPBMAddOnInterParameter(TraderSpiOnRspQrySPBMAddOnInterParameterEvent),
    OnRspQryRCAMSCombProductInfo(TraderSpiOnRspQryRCAMSCombProductInfoEvent),
    OnRspQryRCAMSInstrParameter(TraderSpiOnRspQryRCAMSInstrParameterEvent),
    OnRspQryRCAMSIntraParameter(TraderSpiOnRspQryRCAMSIntraParameterEvent),
    OnRspQryRCAMSInterParameter(TraderSpiOnRspQryRCAMSInterParameterEvent),
    OnRspQryRCAMSShortOptAdjustParam(TraderSpiOnRspQryRCAMSShortOptAdjustParamEvent),
    OnRspQryRCAMSInvestorCombPosition(TraderSpiOnRspQryRCAMSInvestorCombPositionEvent),
    OnRspQryInvestorProdRCAMSMargin(TraderSpiOnRspQryInvestorProdRCAMSMarginEvent),
    OnRspQryRULEInstrParameter(TraderSpiOnRspQryRULEInstrParameterEvent),
    OnRspQryRULEIntraParameter(TraderSpiOnRspQryRULEIntraParameterEvent),
    OnRspQryRULEInterParameter(TraderSpiOnRspQryRULEInterParameterEvent),
    OnRspQryInvestorProdRULEMargin(TraderSpiOnRspQryInvestorProdRULEMarginEvent),
}
    

#[derive(Clone, Debug)]
pub struct TraderSpiOnFrontConnectedEvent {

}

#[derive(Clone, Debug)]
pub struct TraderSpiOnFrontDisconnectedEvent {
    pub reason: i32,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnHeartBeatWarningEvent {
    pub time_lapse: i32,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspAuthenticateEvent {
    pub rsp_authenticate_field: Option<CThostFtdcRspAuthenticateField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspUserLoginEvent {
    pub rsp_user_login: Option<CThostFtdcRspUserLoginField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspUserLogoutEvent {
    pub user_logout: Option<CThostFtdcUserLogoutField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspUserPasswordUpdateEvent {
    pub user_password_update: Option<CThostFtdcUserPasswordUpdateField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspTradingAccountPasswordUpdateEvent {
    pub trading_account_password_update: Option<CThostFtdcTradingAccountPasswordUpdateField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspUserAuthMethodEvent {
    pub rsp_user_auth_method: Option<CThostFtdcRspUserAuthMethodField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspGenUserCaptchaEvent {
    pub rsp_gen_user_captcha: Option<CThostFtdcRspGenUserCaptchaField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspGenUserTextEvent {
    pub rsp_gen_user_text: Option<CThostFtdcRspGenUserTextField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspOrderInsertEvent {
    pub input_order: Option<CThostFtdcInputOrderField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspParkedOrderInsertEvent {
    pub parked_order: Option<CThostFtdcParkedOrderField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspParkedOrderActionEvent {
    pub parked_order_action: Option<CThostFtdcParkedOrderActionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspOrderActionEvent {
    pub input_order_action: Option<CThostFtdcInputOrderActionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryMaxOrderVolumeEvent {
    pub qry_max_order_volume: Option<CThostFtdcQryMaxOrderVolumeField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspSettlementInfoConfirmEvent {
    pub settlement_info_confirm: Option<CThostFtdcSettlementInfoConfirmField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspRemoveParkedOrderEvent {
    pub remove_parked_order: Option<CThostFtdcRemoveParkedOrderField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspRemoveParkedOrderActionEvent {
    pub remove_parked_order_action: Option<CThostFtdcRemoveParkedOrderActionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspExecOrderInsertEvent {
    pub input_exec_order: Option<CThostFtdcInputExecOrderField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspExecOrderActionEvent {
    pub input_exec_order_action: Option<CThostFtdcInputExecOrderActionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspForQuoteInsertEvent {
    pub input_for_quote: Option<CThostFtdcInputForQuoteField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQuoteInsertEvent {
    pub input_quote: Option<CThostFtdcInputQuoteField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQuoteActionEvent {
    pub input_quote_action: Option<CThostFtdcInputQuoteActionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspBatchOrderActionEvent {
    pub input_batch_order_action: Option<CThostFtdcInputBatchOrderActionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspOptionSelfCloseInsertEvent {
    pub input_option_self_close: Option<CThostFtdcInputOptionSelfCloseField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspOptionSelfCloseActionEvent {
    pub input_option_self_close_action: Option<CThostFtdcInputOptionSelfCloseActionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspCombActionInsertEvent {
    pub input_comb_action: Option<CThostFtdcInputCombActionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryOrderEvent {
    pub order: Option<CThostFtdcOrderField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryTradeEvent {
    pub trade: Option<CThostFtdcTradeField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInvestorPositionEvent {
    pub investor_position: Option<CThostFtdcInvestorPositionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryTradingAccountEvent {
    pub trading_account: Option<CThostFtdcTradingAccountField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInvestorEvent {
    pub investor: Option<CThostFtdcInvestorField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryTradingCodeEvent {
    pub trading_code: Option<CThostFtdcTradingCodeField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInstrumentMarginRateEvent {
    pub instrument_margin_rate: Option<CThostFtdcInstrumentMarginRateField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInstrumentCommissionRateEvent {
    pub instrument_commission_rate: Option<CThostFtdcInstrumentCommissionRateField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryExchangeEvent {
    pub exchange: Option<CThostFtdcExchangeField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryProductEvent {
    pub product: Option<CThostFtdcProductField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInstrumentEvent {
    pub instrument: Option<CThostFtdcInstrumentField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryDepthMarketDataEvent {
    pub depth_market_data: Option<CThostFtdcDepthMarketDataField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryTraderOfferEvent {
    pub trader_offer: Option<CThostFtdcTraderOfferField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQrySettlementInfoEvent {
    pub settlement_info: Option<CThostFtdcSettlementInfoField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryTransferBankEvent {
    pub transfer_bank: Option<CThostFtdcTransferBankField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInvestorPositionDetailEvent {
    pub investor_position_detail: Option<CThostFtdcInvestorPositionDetailField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryNoticeEvent {
    pub notice: Option<CThostFtdcNoticeField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQrySettlementInfoConfirmEvent {
    pub settlement_info_confirm: Option<CThostFtdcSettlementInfoConfirmField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInvestorPositionCombineDetailEvent {
    pub investor_position_combine_detail: Option<CThostFtdcInvestorPositionCombineDetailField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryCFMMCTradingAccountKeyEvent {
    pub cfmmc_trading_account_key: Option<CThostFtdcCFMMCTradingAccountKeyField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryEWarrantOffsetEvent {
    pub e_warrant_offset: Option<CThostFtdcEWarrantOffsetField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInvestorProductGroupMarginEvent {
    pub investor_product_group_margin: Option<CThostFtdcInvestorProductGroupMarginField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryExchangeMarginRateEvent {
    pub exchange_margin_rate: Option<CThostFtdcExchangeMarginRateField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryExchangeMarginRateAdjustEvent {
    pub exchange_margin_rate_adjust: Option<CThostFtdcExchangeMarginRateAdjustField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryExchangeRateEvent {
    pub exchange_rate: Option<CThostFtdcExchangeRateField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQrySecAgentACIDMapEvent {
    pub sec_agent_acid_map: Option<CThostFtdcSecAgentACIDMapField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryProductExchRateEvent {
    pub product_exch_rate: Option<CThostFtdcProductExchRateField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryProductGroupEvent {
    pub product_group: Option<CThostFtdcProductGroupField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryMMInstrumentCommissionRateEvent {
    pub mm_instrument_commission_rate: Option<CThostFtdcMMInstrumentCommissionRateField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryMMOptionInstrCommRateEvent {
    pub mm_option_instr_comm_rate: Option<CThostFtdcMMOptionInstrCommRateField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInstrumentOrderCommRateEvent {
    pub instrument_order_comm_rate: Option<CThostFtdcInstrumentOrderCommRateField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQrySecAgentTradingAccountEvent {
    pub trading_account: Option<CThostFtdcTradingAccountField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQrySecAgentCheckModeEvent {
    pub sec_agent_check_mode: Option<CThostFtdcSecAgentCheckModeField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQrySecAgentTradeInfoEvent {
    pub sec_agent_trade_info: Option<CThostFtdcSecAgentTradeInfoField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryOptionInstrTradeCostEvent {
    pub option_instr_trade_cost: Option<CThostFtdcOptionInstrTradeCostField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryOptionInstrCommRateEvent {
    pub option_instr_comm_rate: Option<CThostFtdcOptionInstrCommRateField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryExecOrderEvent {
    pub exec_order: Option<CThostFtdcExecOrderField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryForQuoteEvent {
    pub for_quote: Option<CThostFtdcForQuoteField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryQuoteEvent {
    pub quote: Option<CThostFtdcQuoteField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryOptionSelfCloseEvent {
    pub option_self_close: Option<CThostFtdcOptionSelfCloseField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInvestUnitEvent {
    pub invest_unit: Option<CThostFtdcInvestUnitField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryCombInstrumentGuardEvent {
    pub comb_instrument_guard: Option<CThostFtdcCombInstrumentGuardField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryCombActionEvent {
    pub comb_action: Option<CThostFtdcCombActionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryTransferSerialEvent {
    pub transfer_serial: Option<CThostFtdcTransferSerialField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryAccountregisterEvent {
    pub accountregister: Option<CThostFtdcAccountregisterField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspErrorEvent {
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnOrderEvent {
    pub order: Option<CThostFtdcOrderField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnTradeEvent {
    pub trade: Option<CThostFtdcTradeField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnErrRtnOrderInsertEvent {
    pub input_order: Option<CThostFtdcInputOrderField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnErrRtnOrderActionEvent {
    pub order_action: Option<CThostFtdcOrderActionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnInstrumentStatusEvent {
    pub instrument_status: Option<CThostFtdcInstrumentStatusField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnBulletinEvent {
    pub bulletin: Option<CThostFtdcBulletinField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnTradingNoticeEvent {
    pub trading_notice_info: Option<CThostFtdcTradingNoticeInfoField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnErrorConditionalOrderEvent {
    pub error_conditional_order: Option<CThostFtdcErrorConditionalOrderField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnExecOrderEvent {
    pub exec_order: Option<CThostFtdcExecOrderField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnErrRtnExecOrderInsertEvent {
    pub input_exec_order: Option<CThostFtdcInputExecOrderField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnErrRtnExecOrderActionEvent {
    pub exec_order_action: Option<CThostFtdcExecOrderActionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnErrRtnForQuoteInsertEvent {
    pub input_for_quote: Option<CThostFtdcInputForQuoteField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnQuoteEvent {
    pub quote: Option<CThostFtdcQuoteField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnErrRtnQuoteInsertEvent {
    pub input_quote: Option<CThostFtdcInputQuoteField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnErrRtnQuoteActionEvent {
    pub quote_action: Option<CThostFtdcQuoteActionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnForQuoteRspEvent {
    pub for_quote_rsp: Option<CThostFtdcForQuoteRspField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnCFMMCTradingAccountTokenEvent {
    pub cfmmc_trading_account_token: Option<CThostFtdcCFMMCTradingAccountTokenField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnErrRtnBatchOrderActionEvent {
    pub batch_order_action: Option<CThostFtdcBatchOrderActionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnOptionSelfCloseEvent {
    pub option_self_close: Option<CThostFtdcOptionSelfCloseField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnErrRtnOptionSelfCloseInsertEvent {
    pub input_option_self_close: Option<CThostFtdcInputOptionSelfCloseField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnErrRtnOptionSelfCloseActionEvent {
    pub option_self_close_action: Option<CThostFtdcOptionSelfCloseActionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnCombActionEvent {
    pub comb_action: Option<CThostFtdcCombActionField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnErrRtnCombActionInsertEvent {
    pub input_comb_action: Option<CThostFtdcInputCombActionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryContractBankEvent {
    pub contract_bank: Option<CThostFtdcContractBankField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryParkedOrderEvent {
    pub parked_order: Option<CThostFtdcParkedOrderField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryParkedOrderActionEvent {
    pub parked_order_action: Option<CThostFtdcParkedOrderActionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryTradingNoticeEvent {
    pub trading_notice: Option<CThostFtdcTradingNoticeField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryBrokerTradingParamsEvent {
    pub broker_trading_params: Option<CThostFtdcBrokerTradingParamsField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryBrokerTradingAlgosEvent {
    pub broker_trading_algos: Option<CThostFtdcBrokerTradingAlgosField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQueryCFMMCTradingAccountTokenEvent {
    pub query_cfmmc_trading_account_token: Option<CThostFtdcQueryCFMMCTradingAccountTokenField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnFromBankToFutureByBankEvent {
    pub rsp_transfer: Option<CThostFtdcRspTransferField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnFromFutureToBankByBankEvent {
    pub rsp_transfer: Option<CThostFtdcRspTransferField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnRepealFromBankToFutureByBankEvent {
    pub rsp_repeal: Option<CThostFtdcRspRepealField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnRepealFromFutureToBankByBankEvent {
    pub rsp_repeal: Option<CThostFtdcRspRepealField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnFromBankToFutureByFutureEvent {
    pub rsp_transfer: Option<CThostFtdcRspTransferField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnFromFutureToBankByFutureEvent {
    pub rsp_transfer: Option<CThostFtdcRspTransferField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnRepealFromBankToFutureByFutureManualEvent {
    pub rsp_repeal: Option<CThostFtdcRspRepealField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnRepealFromFutureToBankByFutureManualEvent {
    pub rsp_repeal: Option<CThostFtdcRspRepealField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnQueryBankBalanceByFutureEvent {
    pub notify_query_account: Option<CThostFtdcNotifyQueryAccountField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnErrRtnBankToFutureByFutureEvent {
    pub req_transfer: Option<CThostFtdcReqTransferField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnErrRtnFutureToBankByFutureEvent {
    pub req_transfer: Option<CThostFtdcReqTransferField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnErrRtnRepealBankToFutureByFutureManualEvent {
    pub req_repeal: Option<CThostFtdcReqRepealField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnErrRtnRepealFutureToBankByFutureManualEvent {
    pub req_repeal: Option<CThostFtdcReqRepealField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnErrRtnQueryBankBalanceByFutureEvent {
    pub req_query_account: Option<CThostFtdcReqQueryAccountField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnRepealFromBankToFutureByFutureEvent {
    pub rsp_repeal: Option<CThostFtdcRspRepealField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnRepealFromFutureToBankByFutureEvent {
    pub rsp_repeal: Option<CThostFtdcRspRepealField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspFromBankToFutureByFutureEvent {
    pub req_transfer: Option<CThostFtdcReqTransferField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspFromFutureToBankByFutureEvent {
    pub req_transfer: Option<CThostFtdcReqTransferField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQueryBankAccountMoneyByFutureEvent {
    pub req_query_account: Option<CThostFtdcReqQueryAccountField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnOpenAccountByBankEvent {
    pub open_account: Option<CThostFtdcOpenAccountField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnCancelAccountByBankEvent {
    pub cancel_account: Option<CThostFtdcCancelAccountField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRtnChangeAccountByBankEvent {
    pub change_account: Option<CThostFtdcChangeAccountField>,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryClassifiedInstrumentEvent {
    pub instrument: Option<CThostFtdcInstrumentField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryCombPromotionParamEvent {
    pub comb_promotion_param: Option<CThostFtdcCombPromotionParamField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryRiskSettleInvstPositionEvent {
    pub risk_settle_invst_position: Option<CThostFtdcRiskSettleInvstPositionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryRiskSettleProductStatusEvent {
    pub risk_settle_product_status: Option<CThostFtdcRiskSettleProductStatusField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQrySPBMFutureParameterEvent {
    pub spbm_future_parameter: Option<CThostFtdcSPBMFutureParameterField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQrySPBMOptionParameterEvent {
    pub spbm_option_parameter: Option<CThostFtdcSPBMOptionParameterField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQrySPBMIntraParameterEvent {
    pub spbm_intra_parameter: Option<CThostFtdcSPBMIntraParameterField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQrySPBMInterParameterEvent {
    pub spbm_inter_parameter: Option<CThostFtdcSPBMInterParameterField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQrySPBMPortfDefinitionEvent {
    pub spbm_portf_definition: Option<CThostFtdcSPBMPortfDefinitionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQrySPBMInvestorPortfDefEvent {
    pub spbm_investor_portf_def: Option<CThostFtdcSPBMInvestorPortfDefField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInvestorPortfMarginRatioEvent {
    pub investor_portf_margin_ratio: Option<CThostFtdcInvestorPortfMarginRatioField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInvestorProdSPBMDetailEvent {
    pub investor_prod_spbm_detail: Option<CThostFtdcInvestorProdSPBMDetailField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInvestorCommoditySPMMMarginEvent {
    pub investor_commodity_spmm_margin: Option<CThostFtdcInvestorCommoditySPMMMarginField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInvestorCommodityGroupSPMMMarginEvent {
    pub investor_commodity_group_spmm_margin: Option<CThostFtdcInvestorCommodityGroupSPMMMarginField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQrySPMMInstParamEvent {
    pub spmm_inst_param: Option<CThostFtdcSPMMInstParamField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQrySPMMProductParamEvent {
    pub spmm_product_param: Option<CThostFtdcSPMMProductParamField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQrySPBMAddOnInterParameterEvent {
    pub spbm_add_on_inter_parameter: Option<CThostFtdcSPBMAddOnInterParameterField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryRCAMSCombProductInfoEvent {
    pub rcams_comb_product_info: Option<CThostFtdcRCAMSCombProductInfoField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryRCAMSInstrParameterEvent {
    pub rcams_instr_parameter: Option<CThostFtdcRCAMSInstrParameterField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryRCAMSIntraParameterEvent {
    pub rcams_intra_parameter: Option<CThostFtdcRCAMSIntraParameterField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryRCAMSInterParameterEvent {
    pub rcams_inter_parameter: Option<CThostFtdcRCAMSInterParameterField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryRCAMSShortOptAdjustParamEvent {
    pub rcams_short_opt_adjust_param: Option<CThostFtdcRCAMSShortOptAdjustParamField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryRCAMSInvestorCombPositionEvent {
    pub rcams_investor_comb_position: Option<CThostFtdcRCAMSInvestorCombPositionField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInvestorProdRCAMSMarginEvent {
    pub investor_prod_rcams_margin: Option<CThostFtdcInvestorProdRCAMSMarginField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryRULEInstrParameterEvent {
    pub rule_instr_parameter: Option<CThostFtdcRULEInstrParameterField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryRULEIntraParameterEvent {
    pub rule_intra_parameter: Option<CThostFtdcRULEIntraParameterField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryRULEInterParameterEvent {
    pub rule_inter_parameter: Option<CThostFtdcRULEInterParameterField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct TraderSpiOnRspQryInvestorProdRULEMarginEvent {
    pub investor_prod_rule_margin: Option<CThostFtdcInvestorProdRULEMarginField>,
    pub rsp_info: Option<CThostFtdcRspInfoField>,
    pub request_id: i32,
    pub is_last: bool,
}
