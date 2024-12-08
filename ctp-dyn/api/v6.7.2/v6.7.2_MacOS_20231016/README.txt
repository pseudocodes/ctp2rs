1. 说明

本次更新内容:
交易API新增接口如下：
   1）新增接口：SPBM附加跨品种抵扣参数查询，ReqQrySPBMAddOnInterParameter。
   2）新增接口：RCAMS产品组合信息查询，ReqQryRCAMSCombProductInfo。
   3）新增接口：RCAMS同合约风险对冲参数查询，ReqQryRCAMSInstrParameter。
   4）新增接口：RCAMS品种内风险对冲参数查询，ReqQryRCAMSIntraParameter。
   5）新增接口：RCAMS跨品种风险折抵参数查询，ReqQryRCAMSInterParameter。
   6）新增接口：RCAMS空头期权风险调整参数查询，ReqQryRCAMSShortOptAdjustParam。
   7）新增接口：RCAMS策略组合持仓查询，ReqQryRCAMSInvestorCombPosition。
   8）新增接口：投资者品种RCAMS保证金查询，ReqQryInvestorProdRCAMSMargin。
   9）新增接口：RULE合约保证金参数查询，ReqQryRULEInstrParameter。
  10）新增接口：RULE品种内对锁仓折扣参数查询，ReqQryRULEIntraParameter。
  11）新增接口：RULE跨品种抵扣参数查询，ReqQryRULEInterParameter。
  12）新增接口：投资者产品RULE保证金查询，ReqQryInvestorProdRULEMargin。
2. 开发环境要求

开发者需要配置Xcode开发环境，否则采集模块可能无法正常使用。

3. 操作系统要求

要求操作系统为MacOS 10.15以上版本。

4. 应用权限配置

信息采集模块采集信息时，需要应用具备某些权限。若应用不具备某些权限，
可能会导致获取到对应的采集项为空。因此在使用信息采集模块前， 请使用者申请权限。
包括允许应用访问网络权限。


5. 支持架构

SDK支持x86_64和arm64。

6. 如何使用API和采集模块

将交易api和行情api以及信息采集模块导入项目中。


7. 替换动态库可能出现的问题

 7.1. 头文件找不到
修改引入头文件如下：
#import "thosttraderapi_se/ThostFtdcTraderApi.h"

 7.2. 程序运行失败
选择 TARGETAS > General > Frameworks,Libraries,and Embedded Content
设置 SDK Embed 为 Embed & Sign。





