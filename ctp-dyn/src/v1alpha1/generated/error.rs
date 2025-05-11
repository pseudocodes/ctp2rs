// 自动生成的代码 - 请勿手动修改
// 由 gen_error.rs 从 error.xml 生成

use std::fmt;
use std::error::Error as StdError;

/// CTP错误代码和消息，从error.xml转换而来
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CtpError {
    /// CTP:正确 (0)
    None,

    // 一般错误 (1-100)
    /// CTP:不在已同步状态 (1)
    InvalidDataSyncStatus,
    /// CTP:会话信息不一致 (2)
    InconsistentInformation,
    /// CTP:不合法的登录 (3)
    InvalidLogin,
    /// CTP:用户不活跃 (4)
    UserNotActive,
    /// CTP:重复的登录 (5)
    DuplicateLogin,
    /// CTP:还没有登录 (6)
    NotLoginYet,
    /// CTP:还没有初始化 (7)
    NotInited,
    /// CTP:前置不活跃 (8)
    FrontNotActive,
    /// CTP:无此权限 (9)
    NoPrivilege,
    /// CTP:修改别人的口令 (10)
    ChangeOtherPassword,
    /// CTP:找不到该用户 (11)
    UserNotFound,
    /// CTP:找不到该经纪公司 (12)
    BrokerNotFound,
    /// CTP:找不到投资者 (13)
    InvestorNotFound,
    /// CTP:原口令不匹配 (14)
    OldPasswordMismatch,
    /// CTP:报单字段有误 (15)
    BadField,
    /// CTP:找不到合约 (16)
    InstrumentNotFound,
    /// CTP:合约不能交易 (17)
    InstrumentNotTrading,
    /// CTP:经纪公司不是交易所的会员 (18)
    NotExchangeParticipant,
    /// CTP:投资者不活跃 (19)
    InvestorNotActive,
    /// CTP:投资者未在交易所开户 (20)
    NotExchangeClient,
    /// CTP:该交易席位未连接到交易所 (21)
    NoValidTraderAvailable,
    /// CTP:报单错误：不允许重复报单 (22)
    DuplicateOrderRef,
    /// CTP:错误的报单操作字段 (23)
    BadOrderActionField,
    /// CTP:撤单已报送，不允许重复撤单 (24)
    DuplicateOrderActionRef,
    /// CTP:撤单找不到相应报单 (25)
    OrderNotFound,
    /// CTP:报单已全成交或已撤销，不能再撤 (26)
    InsuitableOrderStatus,
    /// CTP:不支持的功能 (27)
    UnsupportedFunction,
    /// CTP:没有报单交易权限 (28)
    NoTradingRight,
    /// CTP:只能平仓 (29)
    CloseOnly,
    /// CTP:平仓量超过持仓量 (30)
    OverClosePosition,
    /// CTP:资金不足 (31)
    InsufficientMoney,
    /// CTP:主键重复 (32)
    DuplicatePk,
    /// CTP:找不到主键 (33)
    CannotFindPk,
    /// CTP:设置经纪公司不活跃状态失败 (34)
    CanNotInactiveBroker,
    /// CTP:经纪公司正在同步 (35)
    BrokerSynchronizing,
    /// CTP:经纪公司已同步 (36)
    BrokerSynchronized,
    /// CTP:现货交易不能卖空 (37)
    ShortSell,
    /// CTP:不合法的结算引用 (38)
    InvalidSettlementRef,
    /// CTP:交易所网络连接失败 (39)
    CffexNetworkError,
    /// CTP:交易所未处理请求超过许可数 (40)
    CffexOverRequest,
    /// CTP:交易所每秒发送请求数超过许可数 (41)
    CffexOverRequestPerSecond,
    /// CTP:结算结果未确认 (42)
    SettlementInfoNotConfirmed,
    /// CTP:没有对应的入金记录 (43)
    DepositNotFound,
    /// CTP:交易所已经进入连续交易状态 (44)
    ExchangTrading,
    /// CTP:找不到预埋（撤单）单 (45)
    ParkedorderNotFound,
    /// CTP:预埋（撤单）单已经发送 (46)
    ParkedorderHassended,
    /// CTP:预埋（撤单）单已经删除 (47)
    ParkedorderHasdelete,
    /// CTP:无效的投资者或者密码 (48)
    InvalidInvestoridorpassword,
    /// CTP:不合法的登录IP地址 (49)
    InvalidLoginIpaddress,
    /// CTP:平今仓位不足 (50)
    OverClosetodayPosition,
    /// CTP:平昨仓位不足 (51)
    OverCloseyesterdayPosition,
    /// CTP:经纪公司没有足够可用的条件单数量 (52)
    BrokerNotEnoughCondorder,
    /// CTP:投资者没有足够可用的条件单数量 (53)
    InvestorNotEnoughCondorder,
    /// CTP:经纪公司不支持条件单 (54)
    BrokerNotSupportCondorder,
    /// CTP:重发未知单经纪公司/投资者不匹配 (55)
    ResendOrderBrokerinvestorNotmatch,
    /// CTP:同步动态令牌失败 (56)
    SycOtpFailed,
    /// CTP:动态令牌校验错误 (57)
    OtpMismatch,
    /// CTP:找不到动态令牌配置信息 (58)
    OtpparamNotFound,
    /// CTP:不支持的动态令牌类型 (59)
    UnsupportedOtptype,
    /// CTP:用户在线会话超出上限 (60)
    SingleusersessionExceedLimit,
    /// CTP:该交易所不支持套利/做市商类型报单 (61)
    ExchangeUnsupportedArbitrage,
    /// CTP:没有条件单交易权限 (62)
    NoConditionalOrderRight,
    /// CTP:客户端认证失败 (63)
    AuthFailed,
    /// CTP:客户端未认证 (64)
    NotAuthent,
    /// CTP:该合约不支持互换类型报单 (65)
    SwaporderUnsupported,
    /// CTP:该期权合约只支持投机类型报单 (66)
    OptionsOnlySupportSpec,
    /// CTP:执行宣告错误，不允许重复执行 (67)
    DuplicateExecorderRef,
    /// CTP:重发未知执行宣告经纪公司/投资者不匹配 (68)
    ResendExecorderBrokerinvestorNotmatch,
    /// CTP:只有期权合约可执行 (69)
    ExecorderNotoptions,
    /// CTP:该期权合约不支持执行 (70)
    OptionsNotSupportExec,
    /// CTP:执行宣告字段有误 (71)
    BadExecorderActionField,
    /// CTP:执行宣告撤单已报送，不允许重复撤单 (72)
    DuplicateExecorderActionRef,
    /// CTP:执行宣告撤单找不到相应执行宣告 (73)
    ExecorderNotFound,
    /// CTP:执行仓位不足 (74)
    OverExecutePosition,
    /// CTP:连续登录失败次数超限，登录被禁止 (75)
    LoginForbidden,
    /// CTP:非法银期代理关系 (76)
    InvalidTransferAgent,
    /// CTP:无此功能 (77)
    NoFoundFunction,
    /// CTP:发送报单失败 (78)
    SendExchangeorderFailed,
    /// CTP:发送报单操作失败 (79)
    SendExchangeorderactionFailed,
    /// CTP:交易所不支持的价格类型 (80)
    PricetypeNotsupportByexchange,
    /// CTP:错误的执行类型 (81)
    BadExecuteType,
    /// CTP:无效的组合合约 (82)
    BadOptionInstr,
    /// CTP:该合约不支持询价 (83)
    InstrNotsupportForquote,
    /// CTP:重发未知报价经纪公司/投资者不匹配 (84)
    ResendQuoteBrokerinvestorNotmatch,
    /// CTP:该合约不支持报价 (85)
    InstrNotsupportQuote,
    /// CTP:报价撤单找不到相应报价 (86)
    QuoteNotFound,
    /// CTP:该期权合约不支持放弃执行 (87)
    OptionsNotSupportAbandon,
    /// CTP:该组合期权合约只支持IOC (88)
    ComboptionsSupportIocOnly,
    /// CTP:打开文件失败 (89)
    OpenFileFailed,
    /// CTP：查询未就绪，请稍后重试 (90)
    NeedRetry,
    /// CTP：交易所返回的错误 (91)
    ExchangeRtnerror,
    /// CTP:报价衍生单要等待交易所返回才能撤单 (92)
    QuoteDerivedorderActionerror,
    /// CTP:找不到组合合约映射 (93)
    InstrumentmapNotFound,
    /// CTP:不允许撤销OTC衍生报单 (94)
    CancellationOfOtcDerivedOrderNotAllowed,
    /// CTP：不支持的价格 (95)
    BadPriceValue,
    /// CTP:找不到SPBM期货合约参数 (96)
    SpbmfutparamNotFound,
    /// CTP:找不到SPBM期权合约参数 (97)
    SpbmoptparamNotFound,
    /// CTP:找不到SPBM品种内对锁仓折扣参数 (98)
    SpbmintraparamNotFound,
    /// CTP:找不到RULE合约参数 (99)
    RuleinstrparamNotFound,
    /// CTP:找不到RULE品种内对锁仓折扣参数 (100)
    RuleintraparamNotFound,
    // 灾备系统错误 (101-999)
    /// CTP:用户在本系统没有报单权限 (101)
    NoTradingRightInSepcDr,
    /// CTP:系统缺少灾备标示号 (102)
    NoDrNo,
    /// CTP:该交易所不支持批量撤单 (103)
    BatchactionNosupport,
    /// CTP:投资者限仓 (106)
    PosiLimit,
    /// CTP:当前时间禁止询价 (113)
    OutOfTimeinterval,
    /// CTP:当前价差禁止询价 (114)
    OutOfPriceinterval,
    /// CTP:下单频率限制 (116)
    OrderFreqLimit,
    /// CTP：您当前密码为弱密码，请修改成强密码后重新登录 (131)
    WeakPassword,
    /// CTP:当前时间禁止行权 (139)
    ExecForbiddenTime,
    /// CTP:首次登录必须修改密码，请修改密码后重新登录 (140)
    FirstLogin,
    /// CTP:您当前密码已过期，请修改后登录 (141)
    PwdOutOfDate,
    /// CTP:修改密码失败。新密码不允许与旧密码相同 (142)
    PwdMustDiff,
    /// CTP:您登录失败次数过多，IP被禁止登入CTP (143)
    IpForbidden,
    /// CTP:您当前IP在黑名单中，被禁止登录和认证 (144)
    IpBlack,
    /// CTP:终端在本系统没有认证权限 (145)
    NoAuthRightInSepcDr,
    /// CTP:缺少InvestorID字段，请填入InvestorID (146)
    InvestorIdIsMissing,
    /// CTP:缺少ExchangeID字段，请填入ExchangeID (147)
    ExchangeIdIsMissing,
    /// CTP:无效的ExchangeID字段，请填入正确的ExchangeID (148)
    ExchangeIdIsInvalid,
    /// CTP:缺少AccountID字段，请填入AccountID (149)
    AccountIdIsMissing,
    /// CTP:交易所代码错误 (150)
    ExchangeIdIsWrong,
    /// CTP:删除拆分组合指令：没有找到该记录 (151)
    DelCombActionNoRec,
    /// CTP:删除拆分组合指令：原指令需要等待30s 才能删除 (152)
    DelCombActionTooFast,
    /// CTP:拆分组合钱不足 (153)
    CombActionShortMoney,
    /// CTP:查询核心忙 请稍后重试 (154)
    QkBusy,
    /// CTP:未连接监控中心 (155)
    CfmmcNoConnection,
    /// CTP:平期权多头后资金为负（收益小于平仓手续费），只可由风控人员强平 (156)
    CloseOptionNoMoney,
    /// CTP:该交易所不支持撤销未知单 (157)
    CancelUnknownOrderUnsupported,
    /// CTP:超过信息量限制 (158)
    OverInfoCntLimit,
    /// CTP:超过个人预埋单最大量限制 (159)
    OverInvstParkedOrderLimit,
    /// CTP:超过经纪公司预埋单最大量限制 (160)
    OverBrokerParkedOrderLimit,
    /// CTP:预埋单:不支持的触发类型 (161)
    ParkedOrderWrongType,
    // 转账系统错误 (1000-1999)
    /// CTP:银期转账：发送机构代码错误 (1000)
    SendInstitutionCodeError,
    /// CTP:银期转账：取平台流水号错误 (1001)
    NoGetPlatformSn,
    /// CTP:银期转账：不合法的转账银行 (1002)
    IllegalTransferBank,
    /// CTP:银期转账：已经开户 (1003)
    AlreadyOpenAccount,
    /// CTP:银期转账：未开户 (1004)
    NotOpenAccount,
    /// CTP:银期转账：处理中 (1005)
    Processing,
    /// CTP:银期转账：交易超时 (1006)
    Overtime,
    /// CTP:银期转账：找不到记录 (1007)
    RecordNotFound,
    /// CTP:银期转账：找不到被冲正的原始交易 (1008)
    NoFoundReversalOriginalTransaction,
    /// CTP:银期转账：连接主机失败 (1009)
    ConnectHostFailed,
    /// CTP:银期转账：发送失败 (1010)
    SendFailed,
    /// CTP:银期转账：迟到应答 (1011)
    LateResponse,
    /// CTP:银期转账：冲正交易银行代码错误 (1012)
    ReversalBankidNotMatch,
    /// CTP:银期转账：冲正交易银行账户错误 (1013)
    ReversalBankaccountNotMatch,
    /// CTP:银期转账：冲正交易经纪公司代码错误 (1014)
    ReversalBrokeridNotMatch,
    /// CTP:银期转账：冲正交易资金账户错误 (1015)
    ReversalAccountidNotMatch,
    /// CTP:银期转账：冲正交易交易金额错误 (1016)
    ReversalAmountNotMatch,
    /// CTP:银期转账：数据库操作错误 (1017)
    DbOperationFailed,
    /// CTP:银期转账：发送到交易系统失败 (1018)
    SendAspFailure,
    /// CTP:银期转账：没有签到 (1019)
    NotSignin,
    /// CTP:银期转账：已经签到 (1020)
    AlreadySignin,
    /// CTP:银期转账：金额或次数超限 (1021)
    AmountOrTimesOver,
    /// CTP:银期转账：这一时间段不能转账 (1022)
    NotInTransferTime,
    /// 银行主机错 (1023)
    BankServerError,
    /// CTP:银期转账：银行已经冲正 (1024)
    BankSerialIsRepealed,
    /// CTP:银期转账：银行流水不存在 (1025)
    BankSerialNotExist,
    /// CTP:银期转账：机构没有签约 (1026)
    NotOrganMap,
    /// CTP:银期转账：存在转账，不能销户 (1027)
    ExistTransfer,
    /// CTP:银期转账：银行不支持冲正 (1028)
    BankForbidReversal,
    /// CTP:银期转账：重复的银行流水 (1029)
    DupBankSerial,
    /// CTP:银期转账：转账系统忙，稍后再试 (1030)
    FbtSystemBusy,
    /// CTP:银期转账：MAC密钥正在同步 (1031)
    MackeySyncing,
    /// CTP:银期转账：资金账户已经登记 (1032)
    AccountidAlreadyRegister,
    /// CTP:银期转账：银行账户已经登记 (1033)
    BankaccountAlreadyRegister,
    /// CTP:银期转账：重复的银行流水,重发成功 (1034)
    DupBankSerialRedoOk,
    /// CTP:银期转账：该币种代码不支持 (1035)
    CurrencyidNotSupported,
    /// CTP:银期转账：MAC值验证失败 (1036)
    InvalidMac,
    /// CTP:银期转账：不支持银行端发起的二级代理商转账和查询 (1037)
    NotSupportSecagentByBank,
    /// CTP:银期转账：PIN密钥正在同步 (1038)
    PinkeySyncing,
    /// CTP:银期转账：建行发起的二级代理商查询 (1039)
    SecagentQueryByCcb,
    /// CTP:银期转账：银行账户不能为空 (1040)
    BankaccountNotEmpty,
    /// CTP:银期转账：资金账户存在，预约开户失败 (1041)
    InvalidReserveOpenAccount,
    /// CTP:银期转账：开户请求的银行卡号和预留的账号不同 (1042)
    OpenAccountNotDefaultAccount,
    /// 银行系统内部错误 (1043)
    BankSystemInternalError,
    /// 银期转账：银期报盘机器时间偏移太大 (1044)
    OfferLocaltimeOffsetIsTooLarge,
    /// 银期转账：该期货流水号已经处理过 (1045)
    FutureserialHasBeenProcessed,
    // 附加转账错误 (2000-2999)
    /// CTP:该报盘未连接到银行 (2000)
    NoValidBankofferAvailable,
    /// CTP:资金密码错误 (2001)
    PasswordMismatch,
    /// CTP:银行流水号重复 (2004)
    DuplationBankSerial,
    /// CTP:报盘流水号重复 (2005)
    DuplationOfferSerial,
    /// CTP:被冲正流水不存在(冲正交易) (2006)
    SerialNotExsit,
    /// CTP:原流水已冲正(冲正交易) (2007)
    SerialIsRepealed,
    /// CTP:与原流水信息不符(冲正交易) (2008)
    SerialMismatch,
    /// CTP:证件号码或类型错误 (2009)
    IdentifiedcardnoMismatch,
    /// CTP:资金账户不存在 (2011)
    AccountNotFund,
    /// CTP:资金账户已经销户 (2012)
    AccountNotActive,
    /// CTP:该交易不能执行手工冲正 (2013)
    NotAllowRepealBymanual,
    /// CTP:转帐金额错误 (2014)
    AmountOutoftheway,
    /// CTP:找不到汇率 (2015)
    ExchangerateNotFound,
    /// CTP:找不到预约开户请求 (2016)
    ReserveOpenAccountNotFund,
    /// CTP:重复的预约开户请求 (2017)
    DuplicateReserveOpenAccountNotFund,
    /// 建行银期：密码与认证(业务错误) (2050)
    PwPassword,
    /// 建行银期：数量与限额(业务错误) (2051)
    AlAmountLimitation,
    /// 建行银期：权限控制(业务错误) (2052)
    AcAuthorityControl,
    /// 建行银期：信息滥缺(业务错误)或者数据内容相关(技术错误) (2053)
    DcDataContext,
    /// 建行银期：内容非法(业务错误) (2054)
    CeContentError,
    /// 建行银期：重复交易(业务错误) (2055)
    DoDuplicateOperation,
    /// 建行银期： 时间与期限(业务错误) (2056)
    TmTime,
    /// 建行银期：风险控制(业务错误) (2057)
    RcRiskControl,
    /// 建行银期：业务逻辑(业务错误) (2058)
    BlBusinessLogic,
    /// 建行银期： 不确定交易结果(技术错误) (2059)
    NaNa,
    /// 建行银期： 硬件错误(技术错误) (2060)
    HwHardware,
    /// 建行银期： 读写相关(技术错误) (2062)
    IoIo,
    /// 建行银期： 数据库相关(技术错误) (2063)
    DbDatabase,
    /// 建行银期：网络通讯(技术错误) (2064)
    NcNetworkCommunication,
    /// 建行银期：安全服务(技术错误) (2065)
    SsSecurityService,
    /// 建行银期： 组件模块(技术错误) (2066)
    CmComponents,
    /// 建行银期：流量控制(技术错误) (2067)
    FcFlowControl,
    /// 建行银期：技术逻辑(技术错误) (2069)
    TlTechnicalLogic,
    /// 建行银期：纯技术性错误(技术错误) (2070)
    AtAbsoluteTechnique,
    // 外汇系统错误 (3000-3999)
    /// CTP:银期换汇：取平台流水号错误 (3001)
    FbeNoGetPlatformSn,
    /// CTP:银期换汇：不合法的转账银行 (3002)
    FbeIllegalTransferBank,
    /// CTP:银期换汇：处理中 (3005)
    FbeProcessing,
    /// CTP:银期换汇：交易超时 (3006)
    FbeOvertime,
    /// CTP:银期换汇：找不到记录 (3007)
    FbeRecordNotFound,
    /// CTP:银期换汇：连接主机失败 (3009)
    FbeConnectHostFailed,
    /// CTP:银期换汇：发送失败 (3010)
    FbeSendFailed,
    /// CTP:银期换汇：迟到应答 (3011)
    FbeLateResponse,
    /// CTP:银期换汇：数据库操作错误 (3017)
    FbeDbOperationFailed,
    /// CTP:银期换汇：没有签到 (3019)
    FbeNotSignin,
    /// CTP:银期换汇：已经签到 (3020)
    FbeAlreadySignin,
    /// CTP:银期换汇：金额或次数超限 (3021)
    FbeAmountOrTimesOver,
    /// CTP:银期换汇：这一时间段不能换汇 (3022)
    FbeNotInTransferTime,
    /// CTP:银期换汇：银行主机错 (3023)
    FbeBankServerError,
    /// CTP:银期换汇：机构没有签约 (3026)
    FbeNotOrganMap,
    /// CTP:银期换汇：换汇系统忙，稍后再试 (3030)
    FbeSystemBusy,
    /// CTP:银期换汇：该币种代码不支持 (3035)
    FbeCurrencyidNotSupported,
    /// CTP:银期换汇：银行帐号不正确 (3036)
    FbeWrongBankAccount,
    /// CTP:银期换汇：银行帐户余额不足 (3037)
    FbeBankAccountNoFunds,
    /// CTP:银期换汇：凭证号重复 (3038)
    FbeDupCertNo,
    /// CTP: 不支持该API版本 (3039)
    ApiUnsupportedVersion,
    /// CTP: 无效的API KEY (3040)
    ApiInvalidKey,
    /// CTP:期权对冲,履约对冲:非期权合约 (3041)
    OptionSelfCloseNotOption,
    /// CTP:期权对冲,履约对冲:请求引用不合法 (3042)
    OptionSelfCloseDuplicateRef,
    /// CTP:期权对冲,履约对冲:非法字段  (3043)
    OptionSelfCloseBadField,
    /// CTP:期权对冲,履约对冲:撤销未找到记录 (3044)
    OptionSelfCloseRecNotFound,
    /// CTP:期权对冲,履约对冲:对冲状态不对，不能撤销 (3045)
    OptionSelfCloseStatusErr,
    /// CTP:期权对冲,履约对冲:不能重复设置，只能先撤销再设置 (3046)
    OptionSelfCloseDoubleSetErr,
    /// CTP:报价不支持改投机套保类型 (3047)
    QuoteWrongHedgeType,
    // 其他错误 (4040+)
    /// CTP:API Front shake hand err (4040)
    ApiFrontShakeHandErr,
    /// CTP:DUPLICATE_SUBMIT (4041)
    DuplicateSubmit,
    /// CTP:IP授权验证失败 (4042)
    AuthipCheckErr,
    /// CTP:用户与客户端授权验证失败 (4043)
    AuthuserCheckErr,
    /// CTP:报价指定的顶单编号不合法（中金所） (4050)
    QuoteWrongRepalaceSysid,
    /// CTP:您认证失败次数过多，IP进入认证禁止列表 (4060)
    AuthIpForbidden,
    /// CTP:未满足质押配比要求 (4061)
    MortgageNotBalance,
    /// CTP:SSL Connect Error. (4100)
    ScapiSslConnectErr,
    /// CTP:Wrong User ID or Name. (4101)
    ScapiWrongUseridorname,
    /// CTP:Cert Verify Failed. (4102)
    ScapiCertVerifyFailed,
    /// CTP:Cert Process Timeout. (4103)
    ScapiCertProcessTimeout,
    /// CTP:Login Error. (4104)
    ScapiLoginError,
    /// CTP:找不到RCAMS产品组合信息 (5000)
    RcamsCombproductinfoNotFound,
    /// CTP:找不到RCAMS空头期权风险调整参数 (5001)
    RcamsShortoptadjustparamNotFound,
    /// CTP:等待银期报盘处理结果 (999999)
    WaitingOfferRsp,

    // 未知错误
    Unknown(i32),
}

impl CtpError {
    /// 从错误码转换为CtpError枚举
    pub fn from_code(code: i32) -> Self {
        match code {
            0 => CtpError::None,
            1 => CtpError::InvalidDataSyncStatus,
            2 => CtpError::InconsistentInformation,
            3 => CtpError::InvalidLogin,
            4 => CtpError::UserNotActive,
            5 => CtpError::DuplicateLogin,
            6 => CtpError::NotLoginYet,
            7 => CtpError::NotInited,
            8 => CtpError::FrontNotActive,
            9 => CtpError::NoPrivilege,
            10 => CtpError::ChangeOtherPassword,
            11 => CtpError::UserNotFound,
            12 => CtpError::BrokerNotFound,
            13 => CtpError::InvestorNotFound,
            14 => CtpError::OldPasswordMismatch,
            15 => CtpError::BadField,
            16 => CtpError::InstrumentNotFound,
            17 => CtpError::InstrumentNotTrading,
            18 => CtpError::NotExchangeParticipant,
            19 => CtpError::InvestorNotActive,
            20 => CtpError::NotExchangeClient,
            21 => CtpError::NoValidTraderAvailable,
            22 => CtpError::DuplicateOrderRef,
            23 => CtpError::BadOrderActionField,
            24 => CtpError::DuplicateOrderActionRef,
            25 => CtpError::OrderNotFound,
            26 => CtpError::InsuitableOrderStatus,
            27 => CtpError::UnsupportedFunction,
            28 => CtpError::NoTradingRight,
            29 => CtpError::CloseOnly,
            30 => CtpError::OverClosePosition,
            31 => CtpError::InsufficientMoney,
            32 => CtpError::DuplicatePk,
            33 => CtpError::CannotFindPk,
            34 => CtpError::CanNotInactiveBroker,
            35 => CtpError::BrokerSynchronizing,
            36 => CtpError::BrokerSynchronized,
            37 => CtpError::ShortSell,
            38 => CtpError::InvalidSettlementRef,
            39 => CtpError::CffexNetworkError,
            40 => CtpError::CffexOverRequest,
            41 => CtpError::CffexOverRequestPerSecond,
            42 => CtpError::SettlementInfoNotConfirmed,
            43 => CtpError::DepositNotFound,
            44 => CtpError::ExchangTrading,
            45 => CtpError::ParkedorderNotFound,
            46 => CtpError::ParkedorderHassended,
            47 => CtpError::ParkedorderHasdelete,
            48 => CtpError::InvalidInvestoridorpassword,
            49 => CtpError::InvalidLoginIpaddress,
            50 => CtpError::OverClosetodayPosition,
            51 => CtpError::OverCloseyesterdayPosition,
            52 => CtpError::BrokerNotEnoughCondorder,
            53 => CtpError::InvestorNotEnoughCondorder,
            54 => CtpError::BrokerNotSupportCondorder,
            55 => CtpError::ResendOrderBrokerinvestorNotmatch,
            56 => CtpError::SycOtpFailed,
            57 => CtpError::OtpMismatch,
            58 => CtpError::OtpparamNotFound,
            59 => CtpError::UnsupportedOtptype,
            60 => CtpError::SingleusersessionExceedLimit,
            61 => CtpError::ExchangeUnsupportedArbitrage,
            62 => CtpError::NoConditionalOrderRight,
            63 => CtpError::AuthFailed,
            64 => CtpError::NotAuthent,
            65 => CtpError::SwaporderUnsupported,
            66 => CtpError::OptionsOnlySupportSpec,
            67 => CtpError::DuplicateExecorderRef,
            68 => CtpError::ResendExecorderBrokerinvestorNotmatch,
            69 => CtpError::ExecorderNotoptions,
            70 => CtpError::OptionsNotSupportExec,
            71 => CtpError::BadExecorderActionField,
            72 => CtpError::DuplicateExecorderActionRef,
            73 => CtpError::ExecorderNotFound,
            74 => CtpError::OverExecutePosition,
            75 => CtpError::LoginForbidden,
            76 => CtpError::InvalidTransferAgent,
            77 => CtpError::NoFoundFunction,
            78 => CtpError::SendExchangeorderFailed,
            79 => CtpError::SendExchangeorderactionFailed,
            80 => CtpError::PricetypeNotsupportByexchange,
            81 => CtpError::BadExecuteType,
            82 => CtpError::BadOptionInstr,
            83 => CtpError::InstrNotsupportForquote,
            84 => CtpError::ResendQuoteBrokerinvestorNotmatch,
            85 => CtpError::InstrNotsupportQuote,
            86 => CtpError::QuoteNotFound,
            87 => CtpError::OptionsNotSupportAbandon,
            88 => CtpError::ComboptionsSupportIocOnly,
            89 => CtpError::OpenFileFailed,
            90 => CtpError::NeedRetry,
            91 => CtpError::ExchangeRtnerror,
            92 => CtpError::QuoteDerivedorderActionerror,
            93 => CtpError::InstrumentmapNotFound,
            94 => CtpError::CancellationOfOtcDerivedOrderNotAllowed,
            95 => CtpError::BadPriceValue,
            96 => CtpError::SpbmfutparamNotFound,
            97 => CtpError::SpbmoptparamNotFound,
            98 => CtpError::SpbmintraparamNotFound,
            99 => CtpError::RuleinstrparamNotFound,
            100 => CtpError::RuleintraparamNotFound,
            101 => CtpError::NoTradingRightInSepcDr,
            102 => CtpError::NoDrNo,
            103 => CtpError::BatchactionNosupport,
            106 => CtpError::PosiLimit,
            113 => CtpError::OutOfTimeinterval,
            114 => CtpError::OutOfPriceinterval,
            116 => CtpError::OrderFreqLimit,
            131 => CtpError::WeakPassword,
            139 => CtpError::ExecForbiddenTime,
            140 => CtpError::FirstLogin,
            141 => CtpError::PwdOutOfDate,
            142 => CtpError::PwdMustDiff,
            143 => CtpError::IpForbidden,
            144 => CtpError::IpBlack,
            145 => CtpError::NoAuthRightInSepcDr,
            146 => CtpError::InvestorIdIsMissing,
            147 => CtpError::ExchangeIdIsMissing,
            148 => CtpError::ExchangeIdIsInvalid,
            149 => CtpError::AccountIdIsMissing,
            150 => CtpError::ExchangeIdIsWrong,
            151 => CtpError::DelCombActionNoRec,
            152 => CtpError::DelCombActionTooFast,
            153 => CtpError::CombActionShortMoney,
            154 => CtpError::QkBusy,
            155 => CtpError::CfmmcNoConnection,
            156 => CtpError::CloseOptionNoMoney,
            157 => CtpError::CancelUnknownOrderUnsupported,
            158 => CtpError::OverInfoCntLimit,
            159 => CtpError::OverInvstParkedOrderLimit,
            160 => CtpError::OverBrokerParkedOrderLimit,
            161 => CtpError::ParkedOrderWrongType,
            1000 => CtpError::SendInstitutionCodeError,
            1001 => CtpError::NoGetPlatformSn,
            1002 => CtpError::IllegalTransferBank,
            1003 => CtpError::AlreadyOpenAccount,
            1004 => CtpError::NotOpenAccount,
            1005 => CtpError::Processing,
            1006 => CtpError::Overtime,
            1007 => CtpError::RecordNotFound,
            1008 => CtpError::NoFoundReversalOriginalTransaction,
            1009 => CtpError::ConnectHostFailed,
            1010 => CtpError::SendFailed,
            1011 => CtpError::LateResponse,
            1012 => CtpError::ReversalBankidNotMatch,
            1013 => CtpError::ReversalBankaccountNotMatch,
            1014 => CtpError::ReversalBrokeridNotMatch,
            1015 => CtpError::ReversalAccountidNotMatch,
            1016 => CtpError::ReversalAmountNotMatch,
            1017 => CtpError::DbOperationFailed,
            1018 => CtpError::SendAspFailure,
            1019 => CtpError::NotSignin,
            1020 => CtpError::AlreadySignin,
            1021 => CtpError::AmountOrTimesOver,
            1022 => CtpError::NotInTransferTime,
            1023 => CtpError::BankServerError,
            1024 => CtpError::BankSerialIsRepealed,
            1025 => CtpError::BankSerialNotExist,
            1026 => CtpError::NotOrganMap,
            1027 => CtpError::ExistTransfer,
            1028 => CtpError::BankForbidReversal,
            1029 => CtpError::DupBankSerial,
            1030 => CtpError::FbtSystemBusy,
            1031 => CtpError::MackeySyncing,
            1032 => CtpError::AccountidAlreadyRegister,
            1033 => CtpError::BankaccountAlreadyRegister,
            1034 => CtpError::DupBankSerialRedoOk,
            1035 => CtpError::CurrencyidNotSupported,
            1036 => CtpError::InvalidMac,
            1037 => CtpError::NotSupportSecagentByBank,
            1038 => CtpError::PinkeySyncing,
            1039 => CtpError::SecagentQueryByCcb,
            1040 => CtpError::BankaccountNotEmpty,
            1041 => CtpError::InvalidReserveOpenAccount,
            1042 => CtpError::OpenAccountNotDefaultAccount,
            1043 => CtpError::BankSystemInternalError,
            1044 => CtpError::OfferLocaltimeOffsetIsTooLarge,
            1045 => CtpError::FutureserialHasBeenProcessed,
            2000 => CtpError::NoValidBankofferAvailable,
            2001 => CtpError::PasswordMismatch,
            2004 => CtpError::DuplationBankSerial,
            2005 => CtpError::DuplationOfferSerial,
            2006 => CtpError::SerialNotExsit,
            2007 => CtpError::SerialIsRepealed,
            2008 => CtpError::SerialMismatch,
            2009 => CtpError::IdentifiedcardnoMismatch,
            2011 => CtpError::AccountNotFund,
            2012 => CtpError::AccountNotActive,
            2013 => CtpError::NotAllowRepealBymanual,
            2014 => CtpError::AmountOutoftheway,
            2015 => CtpError::ExchangerateNotFound,
            2016 => CtpError::ReserveOpenAccountNotFund,
            2017 => CtpError::DuplicateReserveOpenAccountNotFund,
            2050 => CtpError::PwPassword,
            2051 => CtpError::AlAmountLimitation,
            2052 => CtpError::AcAuthorityControl,
            2053 => CtpError::DcDataContext,
            2054 => CtpError::CeContentError,
            2055 => CtpError::DoDuplicateOperation,
            2056 => CtpError::TmTime,
            2057 => CtpError::RcRiskControl,
            2058 => CtpError::BlBusinessLogic,
            2059 => CtpError::NaNa,
            2060 => CtpError::HwHardware,
            2062 => CtpError::IoIo,
            2063 => CtpError::DbDatabase,
            2064 => CtpError::NcNetworkCommunication,
            2065 => CtpError::SsSecurityService,
            2066 => CtpError::CmComponents,
            2067 => CtpError::FcFlowControl,
            2069 => CtpError::TlTechnicalLogic,
            2070 => CtpError::AtAbsoluteTechnique,
            3001 => CtpError::FbeNoGetPlatformSn,
            3002 => CtpError::FbeIllegalTransferBank,
            3005 => CtpError::FbeProcessing,
            3006 => CtpError::FbeOvertime,
            3007 => CtpError::FbeRecordNotFound,
            3009 => CtpError::FbeConnectHostFailed,
            3010 => CtpError::FbeSendFailed,
            3011 => CtpError::FbeLateResponse,
            3017 => CtpError::FbeDbOperationFailed,
            3019 => CtpError::FbeNotSignin,
            3020 => CtpError::FbeAlreadySignin,
            3021 => CtpError::FbeAmountOrTimesOver,
            3022 => CtpError::FbeNotInTransferTime,
            3023 => CtpError::FbeBankServerError,
            3026 => CtpError::FbeNotOrganMap,
            3030 => CtpError::FbeSystemBusy,
            3035 => CtpError::FbeCurrencyidNotSupported,
            3036 => CtpError::FbeWrongBankAccount,
            3037 => CtpError::FbeBankAccountNoFunds,
            3038 => CtpError::FbeDupCertNo,
            3039 => CtpError::ApiUnsupportedVersion,
            3040 => CtpError::ApiInvalidKey,
            3041 => CtpError::OptionSelfCloseNotOption,
            3042 => CtpError::OptionSelfCloseDuplicateRef,
            3043 => CtpError::OptionSelfCloseBadField,
            3044 => CtpError::OptionSelfCloseRecNotFound,
            3045 => CtpError::OptionSelfCloseStatusErr,
            3046 => CtpError::OptionSelfCloseDoubleSetErr,
            3047 => CtpError::QuoteWrongHedgeType,
            4040 => CtpError::ApiFrontShakeHandErr,
            4041 => CtpError::DuplicateSubmit,
            4042 => CtpError::AuthipCheckErr,
            4043 => CtpError::AuthuserCheckErr,
            4050 => CtpError::QuoteWrongRepalaceSysid,
            4060 => CtpError::AuthIpForbidden,
            4061 => CtpError::MortgageNotBalance,
            4100 => CtpError::ScapiSslConnectErr,
            4101 => CtpError::ScapiWrongUseridorname,
            4102 => CtpError::ScapiCertVerifyFailed,
            4103 => CtpError::ScapiCertProcessTimeout,
            4104 => CtpError::ScapiLoginError,
            5000 => CtpError::RcamsCombproductinfoNotFound,
            5001 => CtpError::RcamsShortoptadjustparamNotFound,
            999999 => CtpError::WaitingOfferRsp,
            unknown_code => CtpError::Unknown(unknown_code),
        }
    }

    /// 获取错误码
    pub fn code(&self) -> i32 {
        match self {
            CtpError::None => 0,
            CtpError::InvalidDataSyncStatus => 1,
            CtpError::InconsistentInformation => 2,
            CtpError::InvalidLogin => 3,
            CtpError::UserNotActive => 4,
            CtpError::DuplicateLogin => 5,
            CtpError::NotLoginYet => 6,
            CtpError::NotInited => 7,
            CtpError::FrontNotActive => 8,
            CtpError::NoPrivilege => 9,
            CtpError::ChangeOtherPassword => 10,
            CtpError::UserNotFound => 11,
            CtpError::BrokerNotFound => 12,
            CtpError::InvestorNotFound => 13,
            CtpError::OldPasswordMismatch => 14,
            CtpError::BadField => 15,
            CtpError::InstrumentNotFound => 16,
            CtpError::InstrumentNotTrading => 17,
            CtpError::NotExchangeParticipant => 18,
            CtpError::InvestorNotActive => 19,
            CtpError::NotExchangeClient => 20,
            CtpError::NoValidTraderAvailable => 21,
            CtpError::DuplicateOrderRef => 22,
            CtpError::BadOrderActionField => 23,
            CtpError::DuplicateOrderActionRef => 24,
            CtpError::OrderNotFound => 25,
            CtpError::InsuitableOrderStatus => 26,
            CtpError::UnsupportedFunction => 27,
            CtpError::NoTradingRight => 28,
            CtpError::CloseOnly => 29,
            CtpError::OverClosePosition => 30,
            CtpError::InsufficientMoney => 31,
            CtpError::DuplicatePk => 32,
            CtpError::CannotFindPk => 33,
            CtpError::CanNotInactiveBroker => 34,
            CtpError::BrokerSynchronizing => 35,
            CtpError::BrokerSynchronized => 36,
            CtpError::ShortSell => 37,
            CtpError::InvalidSettlementRef => 38,
            CtpError::CffexNetworkError => 39,
            CtpError::CffexOverRequest => 40,
            CtpError::CffexOverRequestPerSecond => 41,
            CtpError::SettlementInfoNotConfirmed => 42,
            CtpError::DepositNotFound => 43,
            CtpError::ExchangTrading => 44,
            CtpError::ParkedorderNotFound => 45,
            CtpError::ParkedorderHassended => 46,
            CtpError::ParkedorderHasdelete => 47,
            CtpError::InvalidInvestoridorpassword => 48,
            CtpError::InvalidLoginIpaddress => 49,
            CtpError::OverClosetodayPosition => 50,
            CtpError::OverCloseyesterdayPosition => 51,
            CtpError::BrokerNotEnoughCondorder => 52,
            CtpError::InvestorNotEnoughCondorder => 53,
            CtpError::BrokerNotSupportCondorder => 54,
            CtpError::ResendOrderBrokerinvestorNotmatch => 55,
            CtpError::SycOtpFailed => 56,
            CtpError::OtpMismatch => 57,
            CtpError::OtpparamNotFound => 58,
            CtpError::UnsupportedOtptype => 59,
            CtpError::SingleusersessionExceedLimit => 60,
            CtpError::ExchangeUnsupportedArbitrage => 61,
            CtpError::NoConditionalOrderRight => 62,
            CtpError::AuthFailed => 63,
            CtpError::NotAuthent => 64,
            CtpError::SwaporderUnsupported => 65,
            CtpError::OptionsOnlySupportSpec => 66,
            CtpError::DuplicateExecorderRef => 67,
            CtpError::ResendExecorderBrokerinvestorNotmatch => 68,
            CtpError::ExecorderNotoptions => 69,
            CtpError::OptionsNotSupportExec => 70,
            CtpError::BadExecorderActionField => 71,
            CtpError::DuplicateExecorderActionRef => 72,
            CtpError::ExecorderNotFound => 73,
            CtpError::OverExecutePosition => 74,
            CtpError::LoginForbidden => 75,
            CtpError::InvalidTransferAgent => 76,
            CtpError::NoFoundFunction => 77,
            CtpError::SendExchangeorderFailed => 78,
            CtpError::SendExchangeorderactionFailed => 79,
            CtpError::PricetypeNotsupportByexchange => 80,
            CtpError::BadExecuteType => 81,
            CtpError::BadOptionInstr => 82,
            CtpError::InstrNotsupportForquote => 83,
            CtpError::ResendQuoteBrokerinvestorNotmatch => 84,
            CtpError::InstrNotsupportQuote => 85,
            CtpError::QuoteNotFound => 86,
            CtpError::OptionsNotSupportAbandon => 87,
            CtpError::ComboptionsSupportIocOnly => 88,
            CtpError::OpenFileFailed => 89,
            CtpError::NeedRetry => 90,
            CtpError::ExchangeRtnerror => 91,
            CtpError::QuoteDerivedorderActionerror => 92,
            CtpError::InstrumentmapNotFound => 93,
            CtpError::CancellationOfOtcDerivedOrderNotAllowed => 94,
            CtpError::BadPriceValue => 95,
            CtpError::SpbmfutparamNotFound => 96,
            CtpError::SpbmoptparamNotFound => 97,
            CtpError::SpbmintraparamNotFound => 98,
            CtpError::RuleinstrparamNotFound => 99,
            CtpError::RuleintraparamNotFound => 100,
            CtpError::NoTradingRightInSepcDr => 101,
            CtpError::NoDrNo => 102,
            CtpError::BatchactionNosupport => 103,
            CtpError::PosiLimit => 106,
            CtpError::OutOfTimeinterval => 113,
            CtpError::OutOfPriceinterval => 114,
            CtpError::OrderFreqLimit => 116,
            CtpError::WeakPassword => 131,
            CtpError::ExecForbiddenTime => 139,
            CtpError::FirstLogin => 140,
            CtpError::PwdOutOfDate => 141,
            CtpError::PwdMustDiff => 142,
            CtpError::IpForbidden => 143,
            CtpError::IpBlack => 144,
            CtpError::NoAuthRightInSepcDr => 145,
            CtpError::InvestorIdIsMissing => 146,
            CtpError::ExchangeIdIsMissing => 147,
            CtpError::ExchangeIdIsInvalid => 148,
            CtpError::AccountIdIsMissing => 149,
            CtpError::ExchangeIdIsWrong => 150,
            CtpError::DelCombActionNoRec => 151,
            CtpError::DelCombActionTooFast => 152,
            CtpError::CombActionShortMoney => 153,
            CtpError::QkBusy => 154,
            CtpError::CfmmcNoConnection => 155,
            CtpError::CloseOptionNoMoney => 156,
            CtpError::CancelUnknownOrderUnsupported => 157,
            CtpError::OverInfoCntLimit => 158,
            CtpError::OverInvstParkedOrderLimit => 159,
            CtpError::OverBrokerParkedOrderLimit => 160,
            CtpError::ParkedOrderWrongType => 161,
            CtpError::SendInstitutionCodeError => 1000,
            CtpError::NoGetPlatformSn => 1001,
            CtpError::IllegalTransferBank => 1002,
            CtpError::AlreadyOpenAccount => 1003,
            CtpError::NotOpenAccount => 1004,
            CtpError::Processing => 1005,
            CtpError::Overtime => 1006,
            CtpError::RecordNotFound => 1007,
            CtpError::NoFoundReversalOriginalTransaction => 1008,
            CtpError::ConnectHostFailed => 1009,
            CtpError::SendFailed => 1010,
            CtpError::LateResponse => 1011,
            CtpError::ReversalBankidNotMatch => 1012,
            CtpError::ReversalBankaccountNotMatch => 1013,
            CtpError::ReversalBrokeridNotMatch => 1014,
            CtpError::ReversalAccountidNotMatch => 1015,
            CtpError::ReversalAmountNotMatch => 1016,
            CtpError::DbOperationFailed => 1017,
            CtpError::SendAspFailure => 1018,
            CtpError::NotSignin => 1019,
            CtpError::AlreadySignin => 1020,
            CtpError::AmountOrTimesOver => 1021,
            CtpError::NotInTransferTime => 1022,
            CtpError::BankServerError => 1023,
            CtpError::BankSerialIsRepealed => 1024,
            CtpError::BankSerialNotExist => 1025,
            CtpError::NotOrganMap => 1026,
            CtpError::ExistTransfer => 1027,
            CtpError::BankForbidReversal => 1028,
            CtpError::DupBankSerial => 1029,
            CtpError::FbtSystemBusy => 1030,
            CtpError::MackeySyncing => 1031,
            CtpError::AccountidAlreadyRegister => 1032,
            CtpError::BankaccountAlreadyRegister => 1033,
            CtpError::DupBankSerialRedoOk => 1034,
            CtpError::CurrencyidNotSupported => 1035,
            CtpError::InvalidMac => 1036,
            CtpError::NotSupportSecagentByBank => 1037,
            CtpError::PinkeySyncing => 1038,
            CtpError::SecagentQueryByCcb => 1039,
            CtpError::BankaccountNotEmpty => 1040,
            CtpError::InvalidReserveOpenAccount => 1041,
            CtpError::OpenAccountNotDefaultAccount => 1042,
            CtpError::BankSystemInternalError => 1043,
            CtpError::OfferLocaltimeOffsetIsTooLarge => 1044,
            CtpError::FutureserialHasBeenProcessed => 1045,
            CtpError::NoValidBankofferAvailable => 2000,
            CtpError::PasswordMismatch => 2001,
            CtpError::DuplationBankSerial => 2004,
            CtpError::DuplationOfferSerial => 2005,
            CtpError::SerialNotExsit => 2006,
            CtpError::SerialIsRepealed => 2007,
            CtpError::SerialMismatch => 2008,
            CtpError::IdentifiedcardnoMismatch => 2009,
            CtpError::AccountNotFund => 2011,
            CtpError::AccountNotActive => 2012,
            CtpError::NotAllowRepealBymanual => 2013,
            CtpError::AmountOutoftheway => 2014,
            CtpError::ExchangerateNotFound => 2015,
            CtpError::ReserveOpenAccountNotFund => 2016,
            CtpError::DuplicateReserveOpenAccountNotFund => 2017,
            CtpError::PwPassword => 2050,
            CtpError::AlAmountLimitation => 2051,
            CtpError::AcAuthorityControl => 2052,
            CtpError::DcDataContext => 2053,
            CtpError::CeContentError => 2054,
            CtpError::DoDuplicateOperation => 2055,
            CtpError::TmTime => 2056,
            CtpError::RcRiskControl => 2057,
            CtpError::BlBusinessLogic => 2058,
            CtpError::NaNa => 2059,
            CtpError::HwHardware => 2060,
            CtpError::IoIo => 2062,
            CtpError::DbDatabase => 2063,
            CtpError::NcNetworkCommunication => 2064,
            CtpError::SsSecurityService => 2065,
            CtpError::CmComponents => 2066,
            CtpError::FcFlowControl => 2067,
            CtpError::TlTechnicalLogic => 2069,
            CtpError::AtAbsoluteTechnique => 2070,
            CtpError::FbeNoGetPlatformSn => 3001,
            CtpError::FbeIllegalTransferBank => 3002,
            CtpError::FbeProcessing => 3005,
            CtpError::FbeOvertime => 3006,
            CtpError::FbeRecordNotFound => 3007,
            CtpError::FbeConnectHostFailed => 3009,
            CtpError::FbeSendFailed => 3010,
            CtpError::FbeLateResponse => 3011,
            CtpError::FbeDbOperationFailed => 3017,
            CtpError::FbeNotSignin => 3019,
            CtpError::FbeAlreadySignin => 3020,
            CtpError::FbeAmountOrTimesOver => 3021,
            CtpError::FbeNotInTransferTime => 3022,
            CtpError::FbeBankServerError => 3023,
            CtpError::FbeNotOrganMap => 3026,
            CtpError::FbeSystemBusy => 3030,
            CtpError::FbeCurrencyidNotSupported => 3035,
            CtpError::FbeWrongBankAccount => 3036,
            CtpError::FbeBankAccountNoFunds => 3037,
            CtpError::FbeDupCertNo => 3038,
            CtpError::ApiUnsupportedVersion => 3039,
            CtpError::ApiInvalidKey => 3040,
            CtpError::OptionSelfCloseNotOption => 3041,
            CtpError::OptionSelfCloseDuplicateRef => 3042,
            CtpError::OptionSelfCloseBadField => 3043,
            CtpError::OptionSelfCloseRecNotFound => 3044,
            CtpError::OptionSelfCloseStatusErr => 3045,
            CtpError::OptionSelfCloseDoubleSetErr => 3046,
            CtpError::QuoteWrongHedgeType => 3047,
            CtpError::ApiFrontShakeHandErr => 4040,
            CtpError::DuplicateSubmit => 4041,
            CtpError::AuthipCheckErr => 4042,
            CtpError::AuthuserCheckErr => 4043,
            CtpError::QuoteWrongRepalaceSysid => 4050,
            CtpError::AuthIpForbidden => 4060,
            CtpError::MortgageNotBalance => 4061,
            CtpError::ScapiSslConnectErr => 4100,
            CtpError::ScapiWrongUseridorname => 4101,
            CtpError::ScapiCertVerifyFailed => 4102,
            CtpError::ScapiCertProcessTimeout => 4103,
            CtpError::ScapiLoginError => 4104,
            CtpError::RcamsCombproductinfoNotFound => 5000,
            CtpError::RcamsShortoptadjustparamNotFound => 5001,
            CtpError::WaitingOfferRsp => 999999,
            CtpError::Unknown(code) => *code,
        }
    }

    /// 获取错误消息
    pub fn message(&self) -> &'static str {
        match self {
            CtpError::None => "CTP:正确",
            CtpError::InvalidDataSyncStatus => "CTP:不在已同步状态",
            CtpError::InconsistentInformation => "CTP:会话信息不一致",
            CtpError::InvalidLogin => "CTP:不合法的登录",
            CtpError::UserNotActive => "CTP:用户不活跃",
            CtpError::DuplicateLogin => "CTP:重复的登录",
            CtpError::NotLoginYet => "CTP:还没有登录",
            CtpError::NotInited => "CTP:还没有初始化",
            CtpError::FrontNotActive => "CTP:前置不活跃",
            CtpError::NoPrivilege => "CTP:无此权限",
            CtpError::ChangeOtherPassword => "CTP:修改别人的口令",
            CtpError::UserNotFound => "CTP:找不到该用户",
            CtpError::BrokerNotFound => "CTP:找不到该经纪公司",
            CtpError::InvestorNotFound => "CTP:找不到投资者",
            CtpError::OldPasswordMismatch => "CTP:原口令不匹配",
            CtpError::BadField => "CTP:报单字段有误",
            CtpError::InstrumentNotFound => "CTP:找不到合约",
            CtpError::InstrumentNotTrading => "CTP:合约不能交易",
            CtpError::NotExchangeParticipant => "CTP:经纪公司不是交易所的会员",
            CtpError::InvestorNotActive => "CTP:投资者不活跃",
            CtpError::NotExchangeClient => "CTP:投资者未在交易所开户",
            CtpError::NoValidTraderAvailable => "CTP:该交易席位未连接到交易所",
            CtpError::DuplicateOrderRef => "CTP:报单错误：不允许重复报单",
            CtpError::BadOrderActionField => "CTP:错误的报单操作字段",
            CtpError::DuplicateOrderActionRef => "CTP:撤单已报送，不允许重复撤单",
            CtpError::OrderNotFound => "CTP:撤单找不到相应报单",
            CtpError::InsuitableOrderStatus => "CTP:报单已全成交或已撤销，不能再撤",
            CtpError::UnsupportedFunction => "CTP:不支持的功能",
            CtpError::NoTradingRight => "CTP:没有报单交易权限",
            CtpError::CloseOnly => "CTP:只能平仓",
            CtpError::OverClosePosition => "CTP:平仓量超过持仓量",
            CtpError::InsufficientMoney => "CTP:资金不足",
            CtpError::DuplicatePk => "CTP:主键重复",
            CtpError::CannotFindPk => "CTP:找不到主键",
            CtpError::CanNotInactiveBroker => "CTP:设置经纪公司不活跃状态失败",
            CtpError::BrokerSynchronizing => "CTP:经纪公司正在同步",
            CtpError::BrokerSynchronized => "CTP:经纪公司已同步",
            CtpError::ShortSell => "CTP:现货交易不能卖空",
            CtpError::InvalidSettlementRef => "CTP:不合法的结算引用",
            CtpError::CffexNetworkError => "CTP:交易所网络连接失败",
            CtpError::CffexOverRequest => "CTP:交易所未处理请求超过许可数",
            CtpError::CffexOverRequestPerSecond => "CTP:交易所每秒发送请求数超过许可数",
            CtpError::SettlementInfoNotConfirmed => "CTP:结算结果未确认",
            CtpError::DepositNotFound => "CTP:没有对应的入金记录",
            CtpError::ExchangTrading => "CTP:交易所已经进入连续交易状态",
            CtpError::ParkedorderNotFound => "CTP:找不到预埋（撤单）单",
            CtpError::ParkedorderHassended => "CTP:预埋（撤单）单已经发送",
            CtpError::ParkedorderHasdelete => "CTP:预埋（撤单）单已经删除",
            CtpError::InvalidInvestoridorpassword => "CTP:无效的投资者或者密码",
            CtpError::InvalidLoginIpaddress => "CTP:不合法的登录IP地址",
            CtpError::OverClosetodayPosition => "CTP:平今仓位不足",
            CtpError::OverCloseyesterdayPosition => "CTP:平昨仓位不足",
            CtpError::BrokerNotEnoughCondorder => "CTP:经纪公司没有足够可用的条件单数量",
            CtpError::InvestorNotEnoughCondorder => "CTP:投资者没有足够可用的条件单数量",
            CtpError::BrokerNotSupportCondorder => "CTP:经纪公司不支持条件单",
            CtpError::ResendOrderBrokerinvestorNotmatch => "CTP:重发未知单经纪公司/投资者不匹配",
            CtpError::SycOtpFailed => "CTP:同步动态令牌失败",
            CtpError::OtpMismatch => "CTP:动态令牌校验错误",
            CtpError::OtpparamNotFound => "CTP:找不到动态令牌配置信息",
            CtpError::UnsupportedOtptype => "CTP:不支持的动态令牌类型",
            CtpError::SingleusersessionExceedLimit => "CTP:用户在线会话超出上限",
            CtpError::ExchangeUnsupportedArbitrage => "CTP:该交易所不支持套利/做市商类型报单",
            CtpError::NoConditionalOrderRight => "CTP:没有条件单交易权限",
            CtpError::AuthFailed => "CTP:客户端认证失败",
            CtpError::NotAuthent => "CTP:客户端未认证",
            CtpError::SwaporderUnsupported => "CTP:该合约不支持互换类型报单",
            CtpError::OptionsOnlySupportSpec => "CTP:该期权合约只支持投机类型报单",
            CtpError::DuplicateExecorderRef => "CTP:执行宣告错误，不允许重复执行",
            CtpError::ResendExecorderBrokerinvestorNotmatch => "CTP:重发未知执行宣告经纪公司/投资者不匹配",
            CtpError::ExecorderNotoptions => "CTP:只有期权合约可执行",
            CtpError::OptionsNotSupportExec => "CTP:该期权合约不支持执行",
            CtpError::BadExecorderActionField => "CTP:执行宣告字段有误",
            CtpError::DuplicateExecorderActionRef => "CTP:执行宣告撤单已报送，不允许重复撤单",
            CtpError::ExecorderNotFound => "CTP:执行宣告撤单找不到相应执行宣告",
            CtpError::OverExecutePosition => "CTP:执行仓位不足",
            CtpError::LoginForbidden => "CTP:连续登录失败次数超限，登录被禁止",
            CtpError::InvalidTransferAgent => "CTP:非法银期代理关系",
            CtpError::NoFoundFunction => "CTP:无此功能",
            CtpError::SendExchangeorderFailed => "CTP:发送报单失败",
            CtpError::SendExchangeorderactionFailed => "CTP:发送报单操作失败",
            CtpError::PricetypeNotsupportByexchange => "CTP:交易所不支持的价格类型",
            CtpError::BadExecuteType => "CTP:错误的执行类型",
            CtpError::BadOptionInstr => "CTP:无效的组合合约",
            CtpError::InstrNotsupportForquote => "CTP:该合约不支持询价",
            CtpError::ResendQuoteBrokerinvestorNotmatch => "CTP:重发未知报价经纪公司/投资者不匹配",
            CtpError::InstrNotsupportQuote => "CTP:该合约不支持报价",
            CtpError::QuoteNotFound => "CTP:报价撤单找不到相应报价",
            CtpError::OptionsNotSupportAbandon => "CTP:该期权合约不支持放弃执行",
            CtpError::ComboptionsSupportIocOnly => "CTP:该组合期权合约只支持IOC",
            CtpError::OpenFileFailed => "CTP:打开文件失败",
            CtpError::NeedRetry => "CTP：查询未就绪，请稍后重试",
            CtpError::ExchangeRtnerror => "CTP：交易所返回的错误",
            CtpError::QuoteDerivedorderActionerror => "CTP:报价衍生单要等待交易所返回才能撤单",
            CtpError::InstrumentmapNotFound => "CTP:找不到组合合约映射",
            CtpError::CancellationOfOtcDerivedOrderNotAllowed => "CTP:不允许撤销OTC衍生报单",
            CtpError::BadPriceValue => "CTP：不支持的价格",
            CtpError::SpbmfutparamNotFound => "CTP:找不到SPBM期货合约参数",
            CtpError::SpbmoptparamNotFound => "CTP:找不到SPBM期权合约参数",
            CtpError::SpbmintraparamNotFound => "CTP:找不到SPBM品种内对锁仓折扣参数",
            CtpError::RuleinstrparamNotFound => "CTP:找不到RULE合约参数",
            CtpError::RuleintraparamNotFound => "CTP:找不到RULE品种内对锁仓折扣参数",
            CtpError::NoTradingRightInSepcDr => "CTP:用户在本系统没有报单权限",
            CtpError::NoDrNo => "CTP:系统缺少灾备标示号",
            CtpError::BatchactionNosupport => "CTP:该交易所不支持批量撤单",
            CtpError::PosiLimit => "CTP:投资者限仓",
            CtpError::OutOfTimeinterval => "CTP:当前时间禁止询价",
            CtpError::OutOfPriceinterval => "CTP:当前价差禁止询价",
            CtpError::OrderFreqLimit => "CTP:下单频率限制",
            CtpError::WeakPassword => "CTP：您当前密码为弱密码，请修改成强密码后重新登录",
            CtpError::ExecForbiddenTime => "CTP:当前时间禁止行权",
            CtpError::FirstLogin => "CTP:首次登录必须修改密码，请修改密码后重新登录",
            CtpError::PwdOutOfDate => "CTP:您当前密码已过期，请修改后登录",
            CtpError::PwdMustDiff => "CTP:修改密码失败。新密码不允许与旧密码相同",
            CtpError::IpForbidden => "CTP:您登录失败次数过多，IP被禁止登入CTP",
            CtpError::IpBlack => "CTP:您当前IP在黑名单中，被禁止登录和认证",
            CtpError::NoAuthRightInSepcDr => "CTP:终端在本系统没有认证权限",
            CtpError::InvestorIdIsMissing => "CTP:缺少InvestorID字段，请填入InvestorID",
            CtpError::ExchangeIdIsMissing => "CTP:缺少ExchangeID字段，请填入ExchangeID",
            CtpError::ExchangeIdIsInvalid => "CTP:无效的ExchangeID字段，请填入正确的ExchangeID",
            CtpError::AccountIdIsMissing => "CTP:缺少AccountID字段，请填入AccountID",
            CtpError::ExchangeIdIsWrong => "CTP:交易所代码错误",
            CtpError::DelCombActionNoRec => "CTP:删除拆分组合指令：没有找到该记录",
            CtpError::DelCombActionTooFast => "CTP:删除拆分组合指令：原指令需要等待30s 才能删除",
            CtpError::CombActionShortMoney => "CTP:拆分组合钱不足",
            CtpError::QkBusy => "CTP:查询核心忙 请稍后重试",
            CtpError::CfmmcNoConnection => "CTP:未连接监控中心",
            CtpError::CloseOptionNoMoney => "CTP:平期权多头后资金为负（收益小于平仓手续费），只可由风控人员强平",
            CtpError::CancelUnknownOrderUnsupported => "CTP:该交易所不支持撤销未知单",
            CtpError::OverInfoCntLimit => "CTP:超过信息量限制",
            CtpError::OverInvstParkedOrderLimit => "CTP:超过个人预埋单最大量限制",
            CtpError::OverBrokerParkedOrderLimit => "CTP:超过经纪公司预埋单最大量限制",
            CtpError::ParkedOrderWrongType => "CTP:预埋单:不支持的触发类型",
            CtpError::SendInstitutionCodeError => "CTP:银期转账：发送机构代码错误",
            CtpError::NoGetPlatformSn => "CTP:银期转账：取平台流水号错误",
            CtpError::IllegalTransferBank => "CTP:银期转账：不合法的转账银行",
            CtpError::AlreadyOpenAccount => "CTP:银期转账：已经开户",
            CtpError::NotOpenAccount => "CTP:银期转账：未开户",
            CtpError::Processing => "CTP:银期转账：处理中",
            CtpError::Overtime => "CTP:银期转账：交易超时",
            CtpError::RecordNotFound => "CTP:银期转账：找不到记录",
            CtpError::NoFoundReversalOriginalTransaction => "CTP:银期转账：找不到被冲正的原始交易",
            CtpError::ConnectHostFailed => "CTP:银期转账：连接主机失败",
            CtpError::SendFailed => "CTP:银期转账：发送失败",
            CtpError::LateResponse => "CTP:银期转账：迟到应答",
            CtpError::ReversalBankidNotMatch => "CTP:银期转账：冲正交易银行代码错误",
            CtpError::ReversalBankaccountNotMatch => "CTP:银期转账：冲正交易银行账户错误",
            CtpError::ReversalBrokeridNotMatch => "CTP:银期转账：冲正交易经纪公司代码错误",
            CtpError::ReversalAccountidNotMatch => "CTP:银期转账：冲正交易资金账户错误",
            CtpError::ReversalAmountNotMatch => "CTP:银期转账：冲正交易交易金额错误",
            CtpError::DbOperationFailed => "CTP:银期转账：数据库操作错误",
            CtpError::SendAspFailure => "CTP:银期转账：发送到交易系统失败",
            CtpError::NotSignin => "CTP:银期转账：没有签到",
            CtpError::AlreadySignin => "CTP:银期转账：已经签到",
            CtpError::AmountOrTimesOver => "CTP:银期转账：金额或次数超限",
            CtpError::NotInTransferTime => "CTP:银期转账：这一时间段不能转账",
            CtpError::BankServerError => "银行主机错",
            CtpError::BankSerialIsRepealed => "CTP:银期转账：银行已经冲正",
            CtpError::BankSerialNotExist => "CTP:银期转账：银行流水不存在",
            CtpError::NotOrganMap => "CTP:银期转账：机构没有签约",
            CtpError::ExistTransfer => "CTP:银期转账：存在转账，不能销户",
            CtpError::BankForbidReversal => "CTP:银期转账：银行不支持冲正",
            CtpError::DupBankSerial => "CTP:银期转账：重复的银行流水",
            CtpError::FbtSystemBusy => "CTP:银期转账：转账系统忙，稍后再试",
            CtpError::MackeySyncing => "CTP:银期转账：MAC密钥正在同步",
            CtpError::AccountidAlreadyRegister => "CTP:银期转账：资金账户已经登记",
            CtpError::BankaccountAlreadyRegister => "CTP:银期转账：银行账户已经登记",
            CtpError::DupBankSerialRedoOk => "CTP:银期转账：重复的银行流水,重发成功",
            CtpError::CurrencyidNotSupported => "CTP:银期转账：该币种代码不支持",
            CtpError::InvalidMac => "CTP:银期转账：MAC值验证失败",
            CtpError::NotSupportSecagentByBank => "CTP:银期转账：不支持银行端发起的二级代理商转账和查询",
            CtpError::PinkeySyncing => "CTP:银期转账：PIN密钥正在同步",
            CtpError::SecagentQueryByCcb => "CTP:银期转账：建行发起的二级代理商查询",
            CtpError::BankaccountNotEmpty => "CTP:银期转账：银行账户不能为空",
            CtpError::InvalidReserveOpenAccount => "CTP:银期转账：资金账户存在，预约开户失败",
            CtpError::OpenAccountNotDefaultAccount => "CTP:银期转账：开户请求的银行卡号和预留的账号不同",
            CtpError::BankSystemInternalError => "银行系统内部错误",
            CtpError::OfferLocaltimeOffsetIsTooLarge => "银期转账：银期报盘机器时间偏移太大",
            CtpError::FutureserialHasBeenProcessed => "银期转账：该期货流水号已经处理过",
            CtpError::NoValidBankofferAvailable => "CTP:该报盘未连接到银行",
            CtpError::PasswordMismatch => "CTP:资金密码错误",
            CtpError::DuplationBankSerial => "CTP:银行流水号重复",
            CtpError::DuplationOfferSerial => "CTP:报盘流水号重复",
            CtpError::SerialNotExsit => "CTP:被冲正流水不存在(冲正交易)",
            CtpError::SerialIsRepealed => "CTP:原流水已冲正(冲正交易)",
            CtpError::SerialMismatch => "CTP:与原流水信息不符(冲正交易)",
            CtpError::IdentifiedcardnoMismatch => "CTP:证件号码或类型错误",
            CtpError::AccountNotFund => "CTP:资金账户不存在",
            CtpError::AccountNotActive => "CTP:资金账户已经销户",
            CtpError::NotAllowRepealBymanual => "CTP:该交易不能执行手工冲正",
            CtpError::AmountOutoftheway => "CTP:转帐金额错误",
            CtpError::ExchangerateNotFound => "CTP:找不到汇率",
            CtpError::ReserveOpenAccountNotFund => "CTP:找不到预约开户请求",
            CtpError::DuplicateReserveOpenAccountNotFund => "CTP:重复的预约开户请求",
            CtpError::PwPassword => "建行银期：密码与认证(业务错误)",
            CtpError::AlAmountLimitation => "建行银期：数量与限额(业务错误)",
            CtpError::AcAuthorityControl => "建行银期：权限控制(业务错误)",
            CtpError::DcDataContext => "建行银期：信息滥缺(业务错误)或者数据内容相关(技术错误)",
            CtpError::CeContentError => "建行银期：内容非法(业务错误)",
            CtpError::DoDuplicateOperation => "建行银期：重复交易(业务错误)",
            CtpError::TmTime => "建行银期： 时间与期限(业务错误)",
            CtpError::RcRiskControl => "建行银期：风险控制(业务错误)",
            CtpError::BlBusinessLogic => "建行银期：业务逻辑(业务错误)",
            CtpError::NaNa => "建行银期： 不确定交易结果(技术错误)",
            CtpError::HwHardware => "建行银期： 硬件错误(技术错误)",
            CtpError::IoIo => "建行银期： 读写相关(技术错误)",
            CtpError::DbDatabase => "建行银期： 数据库相关(技术错误)",
            CtpError::NcNetworkCommunication => "建行银期：网络通讯(技术错误)",
            CtpError::SsSecurityService => "建行银期：安全服务(技术错误)",
            CtpError::CmComponents => "建行银期： 组件模块(技术错误)",
            CtpError::FcFlowControl => "建行银期：流量控制(技术错误)",
            CtpError::TlTechnicalLogic => "建行银期：技术逻辑(技术错误)",
            CtpError::AtAbsoluteTechnique => "建行银期：纯技术性错误(技术错误)",
            CtpError::FbeNoGetPlatformSn => "CTP:银期换汇：取平台流水号错误",
            CtpError::FbeIllegalTransferBank => "CTP:银期换汇：不合法的转账银行",
            CtpError::FbeProcessing => "CTP:银期换汇：处理中",
            CtpError::FbeOvertime => "CTP:银期换汇：交易超时",
            CtpError::FbeRecordNotFound => "CTP:银期换汇：找不到记录",
            CtpError::FbeConnectHostFailed => "CTP:银期换汇：连接主机失败",
            CtpError::FbeSendFailed => "CTP:银期换汇：发送失败",
            CtpError::FbeLateResponse => "CTP:银期换汇：迟到应答",
            CtpError::FbeDbOperationFailed => "CTP:银期换汇：数据库操作错误",
            CtpError::FbeNotSignin => "CTP:银期换汇：没有签到",
            CtpError::FbeAlreadySignin => "CTP:银期换汇：已经签到",
            CtpError::FbeAmountOrTimesOver => "CTP:银期换汇：金额或次数超限",
            CtpError::FbeNotInTransferTime => "CTP:银期换汇：这一时间段不能换汇",
            CtpError::FbeBankServerError => "CTP:银期换汇：银行主机错",
            CtpError::FbeNotOrganMap => "CTP:银期换汇：机构没有签约",
            CtpError::FbeSystemBusy => "CTP:银期换汇：换汇系统忙，稍后再试",
            CtpError::FbeCurrencyidNotSupported => "CTP:银期换汇：该币种代码不支持",
            CtpError::FbeWrongBankAccount => "CTP:银期换汇：银行帐号不正确",
            CtpError::FbeBankAccountNoFunds => "CTP:银期换汇：银行帐户余额不足",
            CtpError::FbeDupCertNo => "CTP:银期换汇：凭证号重复",
            CtpError::ApiUnsupportedVersion => "CTP: 不支持该API版本",
            CtpError::ApiInvalidKey => "CTP: 无效的API KEY",
            CtpError::OptionSelfCloseNotOption => "CTP:期权对冲,履约对冲:非期权合约",
            CtpError::OptionSelfCloseDuplicateRef => "CTP:期权对冲,履约对冲:请求引用不合法",
            CtpError::OptionSelfCloseBadField => "CTP:期权对冲,履约对冲:非法字段 ",
            CtpError::OptionSelfCloseRecNotFound => "CTP:期权对冲,履约对冲:撤销未找到记录",
            CtpError::OptionSelfCloseStatusErr => "CTP:期权对冲,履约对冲:对冲状态不对，不能撤销",
            CtpError::OptionSelfCloseDoubleSetErr => "CTP:期权对冲,履约对冲:不能重复设置，只能先撤销再设置",
            CtpError::QuoteWrongHedgeType => "CTP:报价不支持改投机套保类型",
            CtpError::ApiFrontShakeHandErr => "CTP:API Front shake hand err",
            CtpError::DuplicateSubmit => "CTP:DUPLICATE_SUBMIT",
            CtpError::AuthipCheckErr => "CTP:IP授权验证失败",
            CtpError::AuthuserCheckErr => "CTP:用户与客户端授权验证失败",
            CtpError::QuoteWrongRepalaceSysid => "CTP:报价指定的顶单编号不合法（中金所）",
            CtpError::AuthIpForbidden => "CTP:您认证失败次数过多，IP进入认证禁止列表",
            CtpError::MortgageNotBalance => "CTP:未满足质押配比要求",
            CtpError::ScapiSslConnectErr => "CTP:SSL Connect Error.",
            CtpError::ScapiWrongUseridorname => "CTP:Wrong User ID or Name.",
            CtpError::ScapiCertVerifyFailed => "CTP:Cert Verify Failed.",
            CtpError::ScapiCertProcessTimeout => "CTP:Cert Process Timeout.",
            CtpError::ScapiLoginError => "CTP:Login Error.",
            CtpError::RcamsCombproductinfoNotFound => "CTP:找不到RCAMS产品组合信息",
            CtpError::RcamsShortoptadjustparamNotFound => "CTP:找不到RCAMS空头期权风险调整参数",
            CtpError::WaitingOfferRsp => "CTP:等待银期报盘处理结果",
            CtpError::Unknown(_) => "CTP:未知错误",
        }
    }
}

impl fmt::Display for CtpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.message(), self.code())
    }
}

impl StdError for CtpError {}


// 实现从i32转换为CtpError
impl From<i32> for CtpError {
    fn from(code: i32) -> Self {
        CtpError::from_code(code)
    }
}

