
use futures::stream::Stream;
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::Waker,
};

use crate::v1alpha1::bindings::*;
use crate::v1alpha1::event::*;
use crate::v1alpha1::mdspi::*;
use crate::v1alpha1::traderspi::*;


struct MdSpiInner {
    buf: std::collections::VecDeque<MdSpiEvent>,
    waker: Option<Waker>,
}

impl MdSpiInner {
    fn push(&mut self, msg: MdSpiEvent) {
        self.buf.push_back(msg);
        if let Some(waker) = self.waker.take() {
            waker.wake()
        }
    }
}

pub struct MdSpiStream {
    inner: Arc<Mutex<MdSpiInner>>,
}

impl Stream for MdSpiStream {
    type Item = MdSpiEvent;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut futures::task::Context<'_>,
    ) -> futures::task::Poll<Option<Self::Item>> {
        use futures::task::Poll;
        let mut inner = self.inner.lock().unwrap();
        if let Some(i) = inner.buf.pop_front() {
            Poll::Ready(Some(i))
        } else {
            inner.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}
impl MdSpi for MdSpiStream { 
   
    fn on_front_connected(&mut self) {
        self.inner.lock().unwrap().push(MdSpiEvent::OnFrontConnected(
            MdSpiOnFrontConnectedEvent { 
 
            }
        ))
    }
   
    fn on_front_disconnected(&mut self, reason: i32) {
        self.inner.lock().unwrap().push(MdSpiEvent::OnFrontDisconnected(
            MdSpiOnFrontDisconnectedEvent { 
                reason: reason 
            }
        ))
    }
   
    fn on_heart_beat_warning(&mut self, time_lapse: i32) {
        self.inner.lock().unwrap().push(MdSpiEvent::OnHeartBeatWarning(
            MdSpiOnHeartBeatWarningEvent { 
                time_lapse: time_lapse 
            }
        ))
    }
   
    fn on_rsp_user_login(&mut self, rsp_user_login: Option<&CThostFtdcRspUserLoginField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(MdSpiEvent::OnRspUserLogin(
            MdSpiOnRspUserLoginEvent { 
                rsp_user_login: rsp_user_login.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_user_logout(&mut self, user_logout: Option<&CThostFtdcUserLogoutField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(MdSpiEvent::OnRspUserLogout(
            MdSpiOnRspUserLogoutEvent { 
                user_logout: user_logout.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_multicast_instrument(&mut self, multicast_instrument: Option<&CThostFtdcMulticastInstrumentField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(MdSpiEvent::OnRspQryMulticastInstrument(
            MdSpiOnRspQryMulticastInstrumentEvent { 
                multicast_instrument: multicast_instrument.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_error(&mut self, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(MdSpiEvent::OnRspError(
            MdSpiOnRspErrorEvent { 
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_sub_market_data(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(MdSpiEvent::OnRspSubMarketData(
            MdSpiOnRspSubMarketDataEvent { 
                specific_instrument: specific_instrument.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_unsub_market_data(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(MdSpiEvent::OnRspUnSubMarketData(
            MdSpiOnRspUnSubMarketDataEvent { 
                specific_instrument: specific_instrument.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_sub_for_quote_rsp(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(MdSpiEvent::OnRspSubForQuoteRsp(
            MdSpiOnRspSubForQuoteRspEvent { 
                specific_instrument: specific_instrument.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_unsub_for_quote_rsp(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(MdSpiEvent::OnRspUnSubForQuoteRsp(
            MdSpiOnRspUnSubForQuoteRspEvent { 
                specific_instrument: specific_instrument.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rtn_depth_market_data(&mut self, depth_market_data: Option<&CThostFtdcDepthMarketDataField>) {
        self.inner.lock().unwrap().push(MdSpiEvent::OnRtnDepthMarketData(
            MdSpiOnRtnDepthMarketDataEvent { 
                depth_market_data: depth_market_data.cloned() 
            }
        ))
    }
   
    fn on_rtn_for_quote_rsp(&mut self, for_quote_rsp: Option<&CThostFtdcForQuoteRspField>) {
        self.inner.lock().unwrap().push(MdSpiEvent::OnRtnForQuoteRsp(
            MdSpiOnRtnForQuoteRspEvent { 
                for_quote_rsp: for_quote_rsp.cloned() 
            }
        ))
    }
}
    

struct TraderSpiInner {
    buf: std::collections::VecDeque<TraderSpiEvent>,
    waker: Option<Waker>,
}

impl TraderSpiInner {
    fn push(&mut self, msg: TraderSpiEvent) {
        self.buf.push_back(msg);
        if let Some(waker) = self.waker.take() {
            waker.wake()
        }
    }
}

pub struct TraderSpiStream {
    inner: Arc<Mutex<TraderSpiInner>>,
}

impl Stream for TraderSpiStream {
    type Item = TraderSpiEvent;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut futures::task::Context<'_>,
    ) -> futures::task::Poll<Option<Self::Item>> {
        use futures::task::Poll;
        let mut inner = self.inner.lock().unwrap();
        if let Some(i) = inner.buf.pop_front() {
            Poll::Ready(Some(i))
        } else {
            inner.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}
impl TraderSpi for TraderSpiStream { 
   
    fn on_front_connected(&mut self) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnFrontConnected(
            TraderSpiOnFrontConnectedEvent { 
 
            }
        ))
    }
   
    fn on_front_disconnected(&mut self, reason: i32) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnFrontDisconnected(
            TraderSpiOnFrontDisconnectedEvent { 
                reason: reason 
            }
        ))
    }
   
    fn on_heart_beat_warning(&mut self, time_lapse: i32) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnHeartBeatWarning(
            TraderSpiOnHeartBeatWarningEvent { 
                time_lapse: time_lapse 
            }
        ))
    }
   
    fn on_rsp_authenticate(&mut self, rsp_authenticate_field: Option<&CThostFtdcRspAuthenticateField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspAuthenticate(
            TraderSpiOnRspAuthenticateEvent { 
                rsp_authenticate_field: rsp_authenticate_field.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_user_login(&mut self, rsp_user_login: Option<&CThostFtdcRspUserLoginField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspUserLogin(
            TraderSpiOnRspUserLoginEvent { 
                rsp_user_login: rsp_user_login.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_user_logout(&mut self, user_logout: Option<&CThostFtdcUserLogoutField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspUserLogout(
            TraderSpiOnRspUserLogoutEvent { 
                user_logout: user_logout.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_user_password_update(&mut self, user_password_update: Option<&CThostFtdcUserPasswordUpdateField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspUserPasswordUpdate(
            TraderSpiOnRspUserPasswordUpdateEvent { 
                user_password_update: user_password_update.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_trading_account_password_update(&mut self, trading_account_password_update: Option<&CThostFtdcTradingAccountPasswordUpdateField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspTradingAccountPasswordUpdate(
            TraderSpiOnRspTradingAccountPasswordUpdateEvent { 
                trading_account_password_update: trading_account_password_update.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_user_auth_method(&mut self, rsp_user_auth_method: Option<&CThostFtdcRspUserAuthMethodField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspUserAuthMethod(
            TraderSpiOnRspUserAuthMethodEvent { 
                rsp_user_auth_method: rsp_user_auth_method.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_gen_user_captcha(&mut self, rsp_gen_user_captcha: Option<&CThostFtdcRspGenUserCaptchaField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspGenUserCaptcha(
            TraderSpiOnRspGenUserCaptchaEvent { 
                rsp_gen_user_captcha: rsp_gen_user_captcha.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_gen_user_text(&mut self, rsp_gen_user_text: Option<&CThostFtdcRspGenUserTextField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspGenUserText(
            TraderSpiOnRspGenUserTextEvent { 
                rsp_gen_user_text: rsp_gen_user_text.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_order_insert(&mut self, input_order: Option<&CThostFtdcInputOrderField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspOrderInsert(
            TraderSpiOnRspOrderInsertEvent { 
                input_order: input_order.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_parked_order_insert(&mut self, parked_order: Option<&CThostFtdcParkedOrderField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspParkedOrderInsert(
            TraderSpiOnRspParkedOrderInsertEvent { 
                parked_order: parked_order.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_parked_order_action(&mut self, parked_order_action: Option<&CThostFtdcParkedOrderActionField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspParkedOrderAction(
            TraderSpiOnRspParkedOrderActionEvent { 
                parked_order_action: parked_order_action.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_order_action(&mut self, input_order_action: Option<&CThostFtdcInputOrderActionField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspOrderAction(
            TraderSpiOnRspOrderActionEvent { 
                input_order_action: input_order_action.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_max_order_volume(&mut self, qry_max_order_volume: Option<&CThostFtdcQryMaxOrderVolumeField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryMaxOrderVolume(
            TraderSpiOnRspQryMaxOrderVolumeEvent { 
                qry_max_order_volume: qry_max_order_volume.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_settlement_info_confirm(&mut self, settlement_info_confirm: Option<&CThostFtdcSettlementInfoConfirmField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspSettlementInfoConfirm(
            TraderSpiOnRspSettlementInfoConfirmEvent { 
                settlement_info_confirm: settlement_info_confirm.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_remove_parked_order(&mut self, remove_parked_order: Option<&CThostFtdcRemoveParkedOrderField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspRemoveParkedOrder(
            TraderSpiOnRspRemoveParkedOrderEvent { 
                remove_parked_order: remove_parked_order.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_remove_parked_order_action(&mut self, remove_parked_order_action: Option<&CThostFtdcRemoveParkedOrderActionField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspRemoveParkedOrderAction(
            TraderSpiOnRspRemoveParkedOrderActionEvent { 
                remove_parked_order_action: remove_parked_order_action.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_exec_order_insert(&mut self, input_exec_order: Option<&CThostFtdcInputExecOrderField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspExecOrderInsert(
            TraderSpiOnRspExecOrderInsertEvent { 
                input_exec_order: input_exec_order.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_exec_order_action(&mut self, input_exec_order_action: Option<&CThostFtdcInputExecOrderActionField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspExecOrderAction(
            TraderSpiOnRspExecOrderActionEvent { 
                input_exec_order_action: input_exec_order_action.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_for_quote_insert(&mut self, input_for_quote: Option<&CThostFtdcInputForQuoteField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspForQuoteInsert(
            TraderSpiOnRspForQuoteInsertEvent { 
                input_for_quote: input_for_quote.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_quote_insert(&mut self, input_quote: Option<&CThostFtdcInputQuoteField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQuoteInsert(
            TraderSpiOnRspQuoteInsertEvent { 
                input_quote: input_quote.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_quote_action(&mut self, input_quote_action: Option<&CThostFtdcInputQuoteActionField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQuoteAction(
            TraderSpiOnRspQuoteActionEvent { 
                input_quote_action: input_quote_action.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_batch_order_action(&mut self, input_batch_order_action: Option<&CThostFtdcInputBatchOrderActionField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspBatchOrderAction(
            TraderSpiOnRspBatchOrderActionEvent { 
                input_batch_order_action: input_batch_order_action.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_option_self_close_insert(&mut self, input_option_self_close: Option<&CThostFtdcInputOptionSelfCloseField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspOptionSelfCloseInsert(
            TraderSpiOnRspOptionSelfCloseInsertEvent { 
                input_option_self_close: input_option_self_close.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_option_self_close_action(&mut self, input_option_self_close_action: Option<&CThostFtdcInputOptionSelfCloseActionField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspOptionSelfCloseAction(
            TraderSpiOnRspOptionSelfCloseActionEvent { 
                input_option_self_close_action: input_option_self_close_action.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_comb_action_insert(&mut self, input_comb_action: Option<&CThostFtdcInputCombActionField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspCombActionInsert(
            TraderSpiOnRspCombActionInsertEvent { 
                input_comb_action: input_comb_action.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_order(&mut self, order: Option<&CThostFtdcOrderField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryOrder(
            TraderSpiOnRspQryOrderEvent { 
                order: order.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_trade(&mut self, trade: Option<&CThostFtdcTradeField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryTrade(
            TraderSpiOnRspQryTradeEvent { 
                trade: trade.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_investor_position(&mut self, investor_position: Option<&CThostFtdcInvestorPositionField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryInvestorPosition(
            TraderSpiOnRspQryInvestorPositionEvent { 
                investor_position: investor_position.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_trading_account(&mut self, trading_account: Option<&CThostFtdcTradingAccountField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryTradingAccount(
            TraderSpiOnRspQryTradingAccountEvent { 
                trading_account: trading_account.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_investor(&mut self, investor: Option<&CThostFtdcInvestorField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryInvestor(
            TraderSpiOnRspQryInvestorEvent { 
                investor: investor.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_trading_code(&mut self, trading_code: Option<&CThostFtdcTradingCodeField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryTradingCode(
            TraderSpiOnRspQryTradingCodeEvent { 
                trading_code: trading_code.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_instrument_margin_rate(&mut self, instrument_margin_rate: Option<&CThostFtdcInstrumentMarginRateField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryInstrumentMarginRate(
            TraderSpiOnRspQryInstrumentMarginRateEvent { 
                instrument_margin_rate: instrument_margin_rate.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_instrument_commission_rate(&mut self, instrument_commission_rate: Option<&CThostFtdcInstrumentCommissionRateField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryInstrumentCommissionRate(
            TraderSpiOnRspQryInstrumentCommissionRateEvent { 
                instrument_commission_rate: instrument_commission_rate.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_exchange(&mut self, exchange: Option<&CThostFtdcExchangeField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryExchange(
            TraderSpiOnRspQryExchangeEvent { 
                exchange: exchange.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_product(&mut self, product: Option<&CThostFtdcProductField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryProduct(
            TraderSpiOnRspQryProductEvent { 
                product: product.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_instrument(&mut self, instrument: Option<&CThostFtdcInstrumentField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryInstrument(
            TraderSpiOnRspQryInstrumentEvent { 
                instrument: instrument.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_depth_market_data(&mut self, depth_market_data: Option<&CThostFtdcDepthMarketDataField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryDepthMarketData(
            TraderSpiOnRspQryDepthMarketDataEvent { 
                depth_market_data: depth_market_data.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_trader_offer(&mut self, trader_offer: Option<&CThostFtdcTraderOfferField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryTraderOffer(
            TraderSpiOnRspQryTraderOfferEvent { 
                trader_offer: trader_offer.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_settlement_info(&mut self, settlement_info: Option<&CThostFtdcSettlementInfoField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQrySettlementInfo(
            TraderSpiOnRspQrySettlementInfoEvent { 
                settlement_info: settlement_info.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_transfer_bank(&mut self, transfer_bank: Option<&CThostFtdcTransferBankField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryTransferBank(
            TraderSpiOnRspQryTransferBankEvent { 
                transfer_bank: transfer_bank.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_investor_position_detail(&mut self, investor_position_detail: Option<&CThostFtdcInvestorPositionDetailField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryInvestorPositionDetail(
            TraderSpiOnRspQryInvestorPositionDetailEvent { 
                investor_position_detail: investor_position_detail.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_notice(&mut self, notice: Option<&CThostFtdcNoticeField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryNotice(
            TraderSpiOnRspQryNoticeEvent { 
                notice: notice.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_settlement_info_confirm(&mut self, settlement_info_confirm: Option<&CThostFtdcSettlementInfoConfirmField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQrySettlementInfoConfirm(
            TraderSpiOnRspQrySettlementInfoConfirmEvent { 
                settlement_info_confirm: settlement_info_confirm.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_investor_position_combine_detail(&mut self, investor_position_combine_detail: Option<&CThostFtdcInvestorPositionCombineDetailField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryInvestorPositionCombineDetail(
            TraderSpiOnRspQryInvestorPositionCombineDetailEvent { 
                investor_position_combine_detail: investor_position_combine_detail.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_cfmmc_trading_account_key(&mut self, cfmmc_trading_account_key: Option<&CThostFtdcCFMMCTradingAccountKeyField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryCFMMCTradingAccountKey(
            TraderSpiOnRspQryCFMMCTradingAccountKeyEvent { 
                cfmmc_trading_account_key: cfmmc_trading_account_key.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_e_warrant_offset(&mut self, e_warrant_offset: Option<&CThostFtdcEWarrantOffsetField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryEWarrantOffset(
            TraderSpiOnRspQryEWarrantOffsetEvent { 
                e_warrant_offset: e_warrant_offset.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_investor_product_group_margin(&mut self, investor_product_group_margin: Option<&CThostFtdcInvestorProductGroupMarginField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryInvestorProductGroupMargin(
            TraderSpiOnRspQryInvestorProductGroupMarginEvent { 
                investor_product_group_margin: investor_product_group_margin.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_exchange_margin_rate(&mut self, exchange_margin_rate: Option<&CThostFtdcExchangeMarginRateField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryExchangeMarginRate(
            TraderSpiOnRspQryExchangeMarginRateEvent { 
                exchange_margin_rate: exchange_margin_rate.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_exchange_margin_rate_adjust(&mut self, exchange_margin_rate_adjust: Option<&CThostFtdcExchangeMarginRateAdjustField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryExchangeMarginRateAdjust(
            TraderSpiOnRspQryExchangeMarginRateAdjustEvent { 
                exchange_margin_rate_adjust: exchange_margin_rate_adjust.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_exchange_rate(&mut self, exchange_rate: Option<&CThostFtdcExchangeRateField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryExchangeRate(
            TraderSpiOnRspQryExchangeRateEvent { 
                exchange_rate: exchange_rate.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_sec_agent_acid_map(&mut self, sec_agent_acid_map: Option<&CThostFtdcSecAgentACIDMapField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQrySecAgentACIDMap(
            TraderSpiOnRspQrySecAgentACIDMapEvent { 
                sec_agent_acid_map: sec_agent_acid_map.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_product_exch_rate(&mut self, product_exch_rate: Option<&CThostFtdcProductExchRateField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryProductExchRate(
            TraderSpiOnRspQryProductExchRateEvent { 
                product_exch_rate: product_exch_rate.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_product_group(&mut self, product_group: Option<&CThostFtdcProductGroupField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryProductGroup(
            TraderSpiOnRspQryProductGroupEvent { 
                product_group: product_group.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_mm_instrument_commission_rate(&mut self, mm_instrument_commission_rate: Option<&CThostFtdcMMInstrumentCommissionRateField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryMMInstrumentCommissionRate(
            TraderSpiOnRspQryMMInstrumentCommissionRateEvent { 
                mm_instrument_commission_rate: mm_instrument_commission_rate.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_mm_option_instr_comm_rate(&mut self, mm_option_instr_comm_rate: Option<&CThostFtdcMMOptionInstrCommRateField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryMMOptionInstrCommRate(
            TraderSpiOnRspQryMMOptionInstrCommRateEvent { 
                mm_option_instr_comm_rate: mm_option_instr_comm_rate.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_instrument_order_comm_rate(&mut self, instrument_order_comm_rate: Option<&CThostFtdcInstrumentOrderCommRateField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryInstrumentOrderCommRate(
            TraderSpiOnRspQryInstrumentOrderCommRateEvent { 
                instrument_order_comm_rate: instrument_order_comm_rate.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_sec_agent_trading_account(&mut self, trading_account: Option<&CThostFtdcTradingAccountField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQrySecAgentTradingAccount(
            TraderSpiOnRspQrySecAgentTradingAccountEvent { 
                trading_account: trading_account.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_sec_agent_check_mode(&mut self, sec_agent_check_mode: Option<&CThostFtdcSecAgentCheckModeField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQrySecAgentCheckMode(
            TraderSpiOnRspQrySecAgentCheckModeEvent { 
                sec_agent_check_mode: sec_agent_check_mode.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_sec_agent_trade_info(&mut self, sec_agent_trade_info: Option<&CThostFtdcSecAgentTradeInfoField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQrySecAgentTradeInfo(
            TraderSpiOnRspQrySecAgentTradeInfoEvent { 
                sec_agent_trade_info: sec_agent_trade_info.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_option_instr_trade_cost(&mut self, option_instr_trade_cost: Option<&CThostFtdcOptionInstrTradeCostField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryOptionInstrTradeCost(
            TraderSpiOnRspQryOptionInstrTradeCostEvent { 
                option_instr_trade_cost: option_instr_trade_cost.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_option_instr_comm_rate(&mut self, option_instr_comm_rate: Option<&CThostFtdcOptionInstrCommRateField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryOptionInstrCommRate(
            TraderSpiOnRspQryOptionInstrCommRateEvent { 
                option_instr_comm_rate: option_instr_comm_rate.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_exec_order(&mut self, exec_order: Option<&CThostFtdcExecOrderField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryExecOrder(
            TraderSpiOnRspQryExecOrderEvent { 
                exec_order: exec_order.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_for_quote(&mut self, for_quote: Option<&CThostFtdcForQuoteField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryForQuote(
            TraderSpiOnRspQryForQuoteEvent { 
                for_quote: for_quote.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_quote(&mut self, quote: Option<&CThostFtdcQuoteField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryQuote(
            TraderSpiOnRspQryQuoteEvent { 
                quote: quote.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_option_self_close(&mut self, option_self_close: Option<&CThostFtdcOptionSelfCloseField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryOptionSelfClose(
            TraderSpiOnRspQryOptionSelfCloseEvent { 
                option_self_close: option_self_close.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_invest_unit(&mut self, invest_unit: Option<&CThostFtdcInvestUnitField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryInvestUnit(
            TraderSpiOnRspQryInvestUnitEvent { 
                invest_unit: invest_unit.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_comb_instrument_guard(&mut self, comb_instrument_guard: Option<&CThostFtdcCombInstrumentGuardField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryCombInstrumentGuard(
            TraderSpiOnRspQryCombInstrumentGuardEvent { 
                comb_instrument_guard: comb_instrument_guard.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_comb_action(&mut self, comb_action: Option<&CThostFtdcCombActionField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryCombAction(
            TraderSpiOnRspQryCombActionEvent { 
                comb_action: comb_action.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_transfer_serial(&mut self, transfer_serial: Option<&CThostFtdcTransferSerialField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryTransferSerial(
            TraderSpiOnRspQryTransferSerialEvent { 
                transfer_serial: transfer_serial.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_accountregister(&mut self, accountregister: Option<&CThostFtdcAccountregisterField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryAccountregister(
            TraderSpiOnRspQryAccountregisterEvent { 
                accountregister: accountregister.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_error(&mut self, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspError(
            TraderSpiOnRspErrorEvent { 
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rtn_order(&mut self, order: Option<&CThostFtdcOrderField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnOrder(
            TraderSpiOnRtnOrderEvent { 
                order: order.cloned() 
            }
        ))
    }
   
    fn on_rtn_trade(&mut self, trade: Option<&CThostFtdcTradeField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnTrade(
            TraderSpiOnRtnTradeEvent { 
                trade: trade.cloned() 
            }
        ))
    }
   
    fn on_err_rtn_order_insert(&mut self, input_order: Option<&CThostFtdcInputOrderField>, rsp_info: Option<&CThostFtdcRspInfoField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnErrRtnOrderInsert(
            TraderSpiOnErrRtnOrderInsertEvent { 
                input_order: input_order.cloned(),
                rsp_info: rsp_info.cloned() 
            }
        ))
    }
   
    fn on_err_rtn_order_action(&mut self, order_action: Option<&CThostFtdcOrderActionField>, rsp_info: Option<&CThostFtdcRspInfoField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnErrRtnOrderAction(
            TraderSpiOnErrRtnOrderActionEvent { 
                order_action: order_action.cloned(),
                rsp_info: rsp_info.cloned() 
            }
        ))
    }
   
    fn on_rtn_instrument_status(&mut self, instrument_status: Option<&CThostFtdcInstrumentStatusField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnInstrumentStatus(
            TraderSpiOnRtnInstrumentStatusEvent { 
                instrument_status: instrument_status.cloned() 
            }
        ))
    }
   
    fn on_rtn_bulletin(&mut self, bulletin: Option<&CThostFtdcBulletinField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnBulletin(
            TraderSpiOnRtnBulletinEvent { 
                bulletin: bulletin.cloned() 
            }
        ))
    }
   
    fn on_rtn_trading_notice(&mut self, trading_notice_info: Option<&CThostFtdcTradingNoticeInfoField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnTradingNotice(
            TraderSpiOnRtnTradingNoticeEvent { 
                trading_notice_info: trading_notice_info.cloned() 
            }
        ))
    }
   
    fn on_rtn_error_conditional_order(&mut self, error_conditional_order: Option<&CThostFtdcErrorConditionalOrderField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnErrorConditionalOrder(
            TraderSpiOnRtnErrorConditionalOrderEvent { 
                error_conditional_order: error_conditional_order.cloned() 
            }
        ))
    }
   
    fn on_rtn_exec_order(&mut self, exec_order: Option<&CThostFtdcExecOrderField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnExecOrder(
            TraderSpiOnRtnExecOrderEvent { 
                exec_order: exec_order.cloned() 
            }
        ))
    }
   
    fn on_err_rtn_exec_order_insert(&mut self, input_exec_order: Option<&CThostFtdcInputExecOrderField>, rsp_info: Option<&CThostFtdcRspInfoField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnErrRtnExecOrderInsert(
            TraderSpiOnErrRtnExecOrderInsertEvent { 
                input_exec_order: input_exec_order.cloned(),
                rsp_info: rsp_info.cloned() 
            }
        ))
    }
   
    fn on_err_rtn_exec_order_action(&mut self, exec_order_action: Option<&CThostFtdcExecOrderActionField>, rsp_info: Option<&CThostFtdcRspInfoField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnErrRtnExecOrderAction(
            TraderSpiOnErrRtnExecOrderActionEvent { 
                exec_order_action: exec_order_action.cloned(),
                rsp_info: rsp_info.cloned() 
            }
        ))
    }
   
    fn on_err_rtn_for_quote_insert(&mut self, input_for_quote: Option<&CThostFtdcInputForQuoteField>, rsp_info: Option<&CThostFtdcRspInfoField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnErrRtnForQuoteInsert(
            TraderSpiOnErrRtnForQuoteInsertEvent { 
                input_for_quote: input_for_quote.cloned(),
                rsp_info: rsp_info.cloned() 
            }
        ))
    }
   
    fn on_rtn_quote(&mut self, quote: Option<&CThostFtdcQuoteField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnQuote(
            TraderSpiOnRtnQuoteEvent { 
                quote: quote.cloned() 
            }
        ))
    }
   
    fn on_err_rtn_quote_insert(&mut self, input_quote: Option<&CThostFtdcInputQuoteField>, rsp_info: Option<&CThostFtdcRspInfoField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnErrRtnQuoteInsert(
            TraderSpiOnErrRtnQuoteInsertEvent { 
                input_quote: input_quote.cloned(),
                rsp_info: rsp_info.cloned() 
            }
        ))
    }
   
    fn on_err_rtn_quote_action(&mut self, quote_action: Option<&CThostFtdcQuoteActionField>, rsp_info: Option<&CThostFtdcRspInfoField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnErrRtnQuoteAction(
            TraderSpiOnErrRtnQuoteActionEvent { 
                quote_action: quote_action.cloned(),
                rsp_info: rsp_info.cloned() 
            }
        ))
    }
   
    fn on_rtn_for_quote_rsp(&mut self, for_quote_rsp: Option<&CThostFtdcForQuoteRspField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnForQuoteRsp(
            TraderSpiOnRtnForQuoteRspEvent { 
                for_quote_rsp: for_quote_rsp.cloned() 
            }
        ))
    }
   
    fn on_rtn_cfmmc_trading_account_token(&mut self, cfmmc_trading_account_token: Option<&CThostFtdcCFMMCTradingAccountTokenField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnCFMMCTradingAccountToken(
            TraderSpiOnRtnCFMMCTradingAccountTokenEvent { 
                cfmmc_trading_account_token: cfmmc_trading_account_token.cloned() 
            }
        ))
    }
   
    fn on_err_rtn_batch_order_action(&mut self, batch_order_action: Option<&CThostFtdcBatchOrderActionField>, rsp_info: Option<&CThostFtdcRspInfoField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnErrRtnBatchOrderAction(
            TraderSpiOnErrRtnBatchOrderActionEvent { 
                batch_order_action: batch_order_action.cloned(),
                rsp_info: rsp_info.cloned() 
            }
        ))
    }
   
    fn on_rtn_option_self_close(&mut self, option_self_close: Option<&CThostFtdcOptionSelfCloseField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnOptionSelfClose(
            TraderSpiOnRtnOptionSelfCloseEvent { 
                option_self_close: option_self_close.cloned() 
            }
        ))
    }
   
    fn on_err_rtn_option_self_close_insert(&mut self, input_option_self_close: Option<&CThostFtdcInputOptionSelfCloseField>, rsp_info: Option<&CThostFtdcRspInfoField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnErrRtnOptionSelfCloseInsert(
            TraderSpiOnErrRtnOptionSelfCloseInsertEvent { 
                input_option_self_close: input_option_self_close.cloned(),
                rsp_info: rsp_info.cloned() 
            }
        ))
    }
   
    fn on_err_rtn_option_self_close_action(&mut self, option_self_close_action: Option<&CThostFtdcOptionSelfCloseActionField>, rsp_info: Option<&CThostFtdcRspInfoField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnErrRtnOptionSelfCloseAction(
            TraderSpiOnErrRtnOptionSelfCloseActionEvent { 
                option_self_close_action: option_self_close_action.cloned(),
                rsp_info: rsp_info.cloned() 
            }
        ))
    }
   
    fn on_rtn_comb_action(&mut self, comb_action: Option<&CThostFtdcCombActionField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnCombAction(
            TraderSpiOnRtnCombActionEvent { 
                comb_action: comb_action.cloned() 
            }
        ))
    }
   
    fn on_err_rtn_comb_action_insert(&mut self, input_comb_action: Option<&CThostFtdcInputCombActionField>, rsp_info: Option<&CThostFtdcRspInfoField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnErrRtnCombActionInsert(
            TraderSpiOnErrRtnCombActionInsertEvent { 
                input_comb_action: input_comb_action.cloned(),
                rsp_info: rsp_info.cloned() 
            }
        ))
    }
   
    fn on_rsp_qry_contract_bank(&mut self, contract_bank: Option<&CThostFtdcContractBankField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryContractBank(
            TraderSpiOnRspQryContractBankEvent { 
                contract_bank: contract_bank.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_parked_order(&mut self, parked_order: Option<&CThostFtdcParkedOrderField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryParkedOrder(
            TraderSpiOnRspQryParkedOrderEvent { 
                parked_order: parked_order.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_parked_order_action(&mut self, parked_order_action: Option<&CThostFtdcParkedOrderActionField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryParkedOrderAction(
            TraderSpiOnRspQryParkedOrderActionEvent { 
                parked_order_action: parked_order_action.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_trading_notice(&mut self, trading_notice: Option<&CThostFtdcTradingNoticeField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryTradingNotice(
            TraderSpiOnRspQryTradingNoticeEvent { 
                trading_notice: trading_notice.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_broker_trading_params(&mut self, broker_trading_params: Option<&CThostFtdcBrokerTradingParamsField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryBrokerTradingParams(
            TraderSpiOnRspQryBrokerTradingParamsEvent { 
                broker_trading_params: broker_trading_params.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_broker_trading_algos(&mut self, broker_trading_algos: Option<&CThostFtdcBrokerTradingAlgosField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryBrokerTradingAlgos(
            TraderSpiOnRspQryBrokerTradingAlgosEvent { 
                broker_trading_algos: broker_trading_algos.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_query_cfmmc_trading_account_token(&mut self, query_cfmmc_trading_account_token: Option<&CThostFtdcQueryCFMMCTradingAccountTokenField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQueryCFMMCTradingAccountToken(
            TraderSpiOnRspQueryCFMMCTradingAccountTokenEvent { 
                query_cfmmc_trading_account_token: query_cfmmc_trading_account_token.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rtn_from_bank_to_future_by_bank(&mut self, rsp_transfer: Option<&CThostFtdcRspTransferField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnFromBankToFutureByBank(
            TraderSpiOnRtnFromBankToFutureByBankEvent { 
                rsp_transfer: rsp_transfer.cloned() 
            }
        ))
    }
   
    fn on_rtn_from_future_to_bank_by_bank(&mut self, rsp_transfer: Option<&CThostFtdcRspTransferField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnFromFutureToBankByBank(
            TraderSpiOnRtnFromFutureToBankByBankEvent { 
                rsp_transfer: rsp_transfer.cloned() 
            }
        ))
    }
   
    fn on_rtn_repeal_from_bank_to_future_by_bank(&mut self, rsp_repeal: Option<&CThostFtdcRspRepealField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnRepealFromBankToFutureByBank(
            TraderSpiOnRtnRepealFromBankToFutureByBankEvent { 
                rsp_repeal: rsp_repeal.cloned() 
            }
        ))
    }
   
    fn on_rtn_repeal_from_future_to_bank_by_bank(&mut self, rsp_repeal: Option<&CThostFtdcRspRepealField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnRepealFromFutureToBankByBank(
            TraderSpiOnRtnRepealFromFutureToBankByBankEvent { 
                rsp_repeal: rsp_repeal.cloned() 
            }
        ))
    }
   
    fn on_rtn_from_bank_to_future_by_future(&mut self, rsp_transfer: Option<&CThostFtdcRspTransferField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnFromBankToFutureByFuture(
            TraderSpiOnRtnFromBankToFutureByFutureEvent { 
                rsp_transfer: rsp_transfer.cloned() 
            }
        ))
    }
   
    fn on_rtn_from_future_to_bank_by_future(&mut self, rsp_transfer: Option<&CThostFtdcRspTransferField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnFromFutureToBankByFuture(
            TraderSpiOnRtnFromFutureToBankByFutureEvent { 
                rsp_transfer: rsp_transfer.cloned() 
            }
        ))
    }
   
    fn on_rtn_repeal_from_bank_to_future_by_future_manual(&mut self, rsp_repeal: Option<&CThostFtdcRspRepealField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnRepealFromBankToFutureByFutureManual(
            TraderSpiOnRtnRepealFromBankToFutureByFutureManualEvent { 
                rsp_repeal: rsp_repeal.cloned() 
            }
        ))
    }
   
    fn on_rtn_repeal_from_future_to_bank_by_future_manual(&mut self, rsp_repeal: Option<&CThostFtdcRspRepealField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnRepealFromFutureToBankByFutureManual(
            TraderSpiOnRtnRepealFromFutureToBankByFutureManualEvent { 
                rsp_repeal: rsp_repeal.cloned() 
            }
        ))
    }
   
    fn on_rtn_query_bank_balance_by_future(&mut self, notify_query_account: Option<&CThostFtdcNotifyQueryAccountField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnQueryBankBalanceByFuture(
            TraderSpiOnRtnQueryBankBalanceByFutureEvent { 
                notify_query_account: notify_query_account.cloned() 
            }
        ))
    }
   
    fn on_err_rtn_bank_to_future_by_future(&mut self, req_transfer: Option<&CThostFtdcReqTransferField>, rsp_info: Option<&CThostFtdcRspInfoField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnErrRtnBankToFutureByFuture(
            TraderSpiOnErrRtnBankToFutureByFutureEvent { 
                req_transfer: req_transfer.cloned(),
                rsp_info: rsp_info.cloned() 
            }
        ))
    }
   
    fn on_err_rtn_future_to_bank_by_future(&mut self, req_transfer: Option<&CThostFtdcReqTransferField>, rsp_info: Option<&CThostFtdcRspInfoField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnErrRtnFutureToBankByFuture(
            TraderSpiOnErrRtnFutureToBankByFutureEvent { 
                req_transfer: req_transfer.cloned(),
                rsp_info: rsp_info.cloned() 
            }
        ))
    }
   
    fn on_err_rtn_repeal_bank_to_future_by_future_manual(&mut self, req_repeal: Option<&CThostFtdcReqRepealField>, rsp_info: Option<&CThostFtdcRspInfoField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnErrRtnRepealBankToFutureByFutureManual(
            TraderSpiOnErrRtnRepealBankToFutureByFutureManualEvent { 
                req_repeal: req_repeal.cloned(),
                rsp_info: rsp_info.cloned() 
            }
        ))
    }
   
    fn on_err_rtn_repeal_future_to_bank_by_future_manual(&mut self, req_repeal: Option<&CThostFtdcReqRepealField>, rsp_info: Option<&CThostFtdcRspInfoField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnErrRtnRepealFutureToBankByFutureManual(
            TraderSpiOnErrRtnRepealFutureToBankByFutureManualEvent { 
                req_repeal: req_repeal.cloned(),
                rsp_info: rsp_info.cloned() 
            }
        ))
    }
   
    fn on_err_rtn_query_bank_balance_by_future(&mut self, req_query_account: Option<&CThostFtdcReqQueryAccountField>, rsp_info: Option<&CThostFtdcRspInfoField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnErrRtnQueryBankBalanceByFuture(
            TraderSpiOnErrRtnQueryBankBalanceByFutureEvent { 
                req_query_account: req_query_account.cloned(),
                rsp_info: rsp_info.cloned() 
            }
        ))
    }
   
    fn on_rtn_repeal_from_bank_to_future_by_future(&mut self, rsp_repeal: Option<&CThostFtdcRspRepealField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnRepealFromBankToFutureByFuture(
            TraderSpiOnRtnRepealFromBankToFutureByFutureEvent { 
                rsp_repeal: rsp_repeal.cloned() 
            }
        ))
    }
   
    fn on_rtn_repeal_from_future_to_bank_by_future(&mut self, rsp_repeal: Option<&CThostFtdcRspRepealField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnRepealFromFutureToBankByFuture(
            TraderSpiOnRtnRepealFromFutureToBankByFutureEvent { 
                rsp_repeal: rsp_repeal.cloned() 
            }
        ))
    }
   
    fn on_rsp_from_bank_to_future_by_future(&mut self, req_transfer: Option<&CThostFtdcReqTransferField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspFromBankToFutureByFuture(
            TraderSpiOnRspFromBankToFutureByFutureEvent { 
                req_transfer: req_transfer.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_from_future_to_bank_by_future(&mut self, req_transfer: Option<&CThostFtdcReqTransferField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspFromFutureToBankByFuture(
            TraderSpiOnRspFromFutureToBankByFutureEvent { 
                req_transfer: req_transfer.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_query_bank_account_money_by_future(&mut self, req_query_account: Option<&CThostFtdcReqQueryAccountField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQueryBankAccountMoneyByFuture(
            TraderSpiOnRspQueryBankAccountMoneyByFutureEvent { 
                req_query_account: req_query_account.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rtn_open_account_by_bank(&mut self, open_account: Option<&CThostFtdcOpenAccountField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnOpenAccountByBank(
            TraderSpiOnRtnOpenAccountByBankEvent { 
                open_account: open_account.cloned() 
            }
        ))
    }
   
    fn on_rtn_cancel_account_by_bank(&mut self, cancel_account: Option<&CThostFtdcCancelAccountField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnCancelAccountByBank(
            TraderSpiOnRtnCancelAccountByBankEvent { 
                cancel_account: cancel_account.cloned() 
            }
        ))
    }
   
    fn on_rtn_change_account_by_bank(&mut self, change_account: Option<&CThostFtdcChangeAccountField>) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRtnChangeAccountByBank(
            TraderSpiOnRtnChangeAccountByBankEvent { 
                change_account: change_account.cloned() 
            }
        ))
    }
   
    fn on_rsp_qry_classified_instrument(&mut self, instrument: Option<&CThostFtdcInstrumentField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryClassifiedInstrument(
            TraderSpiOnRspQryClassifiedInstrumentEvent { 
                instrument: instrument.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_comb_promotion_param(&mut self, comb_promotion_param: Option<&CThostFtdcCombPromotionParamField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryCombPromotionParam(
            TraderSpiOnRspQryCombPromotionParamEvent { 
                comb_promotion_param: comb_promotion_param.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_risk_settle_invst_position(&mut self, risk_settle_invst_position: Option<&CThostFtdcRiskSettleInvstPositionField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryRiskSettleInvstPosition(
            TraderSpiOnRspQryRiskSettleInvstPositionEvent { 
                risk_settle_invst_position: risk_settle_invst_position.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_risk_settle_product_status(&mut self, risk_settle_product_status: Option<&CThostFtdcRiskSettleProductStatusField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryRiskSettleProductStatus(
            TraderSpiOnRspQryRiskSettleProductStatusEvent { 
                risk_settle_product_status: risk_settle_product_status.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_spbm_future_parameter(&mut self, spbm_future_parameter: Option<&CThostFtdcSPBMFutureParameterField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQrySPBMFutureParameter(
            TraderSpiOnRspQrySPBMFutureParameterEvent { 
                spbm_future_parameter: spbm_future_parameter.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_spbm_option_parameter(&mut self, spbm_option_parameter: Option<&CThostFtdcSPBMOptionParameterField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQrySPBMOptionParameter(
            TraderSpiOnRspQrySPBMOptionParameterEvent { 
                spbm_option_parameter: spbm_option_parameter.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_spbm_intra_parameter(&mut self, spbm_intra_parameter: Option<&CThostFtdcSPBMIntraParameterField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQrySPBMIntraParameter(
            TraderSpiOnRspQrySPBMIntraParameterEvent { 
                spbm_intra_parameter: spbm_intra_parameter.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_spbm_inter_parameter(&mut self, spbm_inter_parameter: Option<&CThostFtdcSPBMInterParameterField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQrySPBMInterParameter(
            TraderSpiOnRspQrySPBMInterParameterEvent { 
                spbm_inter_parameter: spbm_inter_parameter.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_spbm_portf_definition(&mut self, spbm_portf_definition: Option<&CThostFtdcSPBMPortfDefinitionField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQrySPBMPortfDefinition(
            TraderSpiOnRspQrySPBMPortfDefinitionEvent { 
                spbm_portf_definition: spbm_portf_definition.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_spbm_investor_portf_def(&mut self, spbm_investor_portf_def: Option<&CThostFtdcSPBMInvestorPortfDefField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQrySPBMInvestorPortfDef(
            TraderSpiOnRspQrySPBMInvestorPortfDefEvent { 
                spbm_investor_portf_def: spbm_investor_portf_def.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_investor_portf_margin_ratio(&mut self, investor_portf_margin_ratio: Option<&CThostFtdcInvestorPortfMarginRatioField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryInvestorPortfMarginRatio(
            TraderSpiOnRspQryInvestorPortfMarginRatioEvent { 
                investor_portf_margin_ratio: investor_portf_margin_ratio.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_investor_prod_spbm_detail(&mut self, investor_prod_spbm_detail: Option<&CThostFtdcInvestorProdSPBMDetailField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryInvestorProdSPBMDetail(
            TraderSpiOnRspQryInvestorProdSPBMDetailEvent { 
                investor_prod_spbm_detail: investor_prod_spbm_detail.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_investor_commodity_spmm_margin(&mut self, investor_commodity_spmm_margin: Option<&CThostFtdcInvestorCommoditySPMMMarginField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryInvestorCommoditySPMMMargin(
            TraderSpiOnRspQryInvestorCommoditySPMMMarginEvent { 
                investor_commodity_spmm_margin: investor_commodity_spmm_margin.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_investor_commodity_group_spmm_margin(&mut self, investor_commodity_group_spmm_margin: Option<&CThostFtdcInvestorCommodityGroupSPMMMarginField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryInvestorCommodityGroupSPMMMargin(
            TraderSpiOnRspQryInvestorCommodityGroupSPMMMarginEvent { 
                investor_commodity_group_spmm_margin: investor_commodity_group_spmm_margin.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_spmm_inst_param(&mut self, spmm_inst_param: Option<&CThostFtdcSPMMInstParamField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQrySPMMInstParam(
            TraderSpiOnRspQrySPMMInstParamEvent { 
                spmm_inst_param: spmm_inst_param.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_spmm_product_param(&mut self, spmm_product_param: Option<&CThostFtdcSPMMProductParamField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQrySPMMProductParam(
            TraderSpiOnRspQrySPMMProductParamEvent { 
                spmm_product_param: spmm_product_param.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_spbm_add_on_inter_parameter(&mut self, spbm_add_on_inter_parameter: Option<&CThostFtdcSPBMAddOnInterParameterField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQrySPBMAddOnInterParameter(
            TraderSpiOnRspQrySPBMAddOnInterParameterEvent { 
                spbm_add_on_inter_parameter: spbm_add_on_inter_parameter.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_rcams_comb_product_info(&mut self, rcams_comb_product_info: Option<&CThostFtdcRCAMSCombProductInfoField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryRCAMSCombProductInfo(
            TraderSpiOnRspQryRCAMSCombProductInfoEvent { 
                rcams_comb_product_info: rcams_comb_product_info.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_rcams_instr_parameter(&mut self, rcams_instr_parameter: Option<&CThostFtdcRCAMSInstrParameterField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryRCAMSInstrParameter(
            TraderSpiOnRspQryRCAMSInstrParameterEvent { 
                rcams_instr_parameter: rcams_instr_parameter.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_rcams_intra_parameter(&mut self, rcams_intra_parameter: Option<&CThostFtdcRCAMSIntraParameterField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryRCAMSIntraParameter(
            TraderSpiOnRspQryRCAMSIntraParameterEvent { 
                rcams_intra_parameter: rcams_intra_parameter.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_rcams_inter_parameter(&mut self, rcams_inter_parameter: Option<&CThostFtdcRCAMSInterParameterField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryRCAMSInterParameter(
            TraderSpiOnRspQryRCAMSInterParameterEvent { 
                rcams_inter_parameter: rcams_inter_parameter.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_rcams_short_opt_adjust_param(&mut self, rcams_short_opt_adjust_param: Option<&CThostFtdcRCAMSShortOptAdjustParamField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryRCAMSShortOptAdjustParam(
            TraderSpiOnRspQryRCAMSShortOptAdjustParamEvent { 
                rcams_short_opt_adjust_param: rcams_short_opt_adjust_param.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_rcams_investor_comb_position(&mut self, rcams_investor_comb_position: Option<&CThostFtdcRCAMSInvestorCombPositionField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryRCAMSInvestorCombPosition(
            TraderSpiOnRspQryRCAMSInvestorCombPositionEvent { 
                rcams_investor_comb_position: rcams_investor_comb_position.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_investor_prod_rcams_margin(&mut self, investor_prod_rcams_margin: Option<&CThostFtdcInvestorProdRCAMSMarginField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryInvestorProdRCAMSMargin(
            TraderSpiOnRspQryInvestorProdRCAMSMarginEvent { 
                investor_prod_rcams_margin: investor_prod_rcams_margin.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_rule_instr_parameter(&mut self, rule_instr_parameter: Option<&CThostFtdcRULEInstrParameterField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryRULEInstrParameter(
            TraderSpiOnRspQryRULEInstrParameterEvent { 
                rule_instr_parameter: rule_instr_parameter.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_rule_intra_parameter(&mut self, rule_intra_parameter: Option<&CThostFtdcRULEIntraParameterField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryRULEIntraParameter(
            TraderSpiOnRspQryRULEIntraParameterEvent { 
                rule_intra_parameter: rule_intra_parameter.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_rule_inter_parameter(&mut self, rule_inter_parameter: Option<&CThostFtdcRULEInterParameterField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryRULEInterParameter(
            TraderSpiOnRspQryRULEInterParameterEvent { 
                rule_inter_parameter: rule_inter_parameter.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
   
    fn on_rsp_qry_investor_prod_rule_margin(&mut self, investor_prod_rule_margin: Option<&CThostFtdcInvestorProdRULEMarginField>, rsp_info: Option<&CThostFtdcRspInfoField>, request_id: i32, is_last: bool) {
        self.inner.lock().unwrap().push(TraderSpiEvent::OnRspQryInvestorProdRULEMargin(
            TraderSpiOnRspQryInvestorProdRULEMarginEvent { 
                investor_prod_rule_margin: investor_prod_rule_margin.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id: request_id,
                is_last: is_last 
            }
        ))
    }
}
    
