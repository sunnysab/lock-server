# 机协之锁 服务端（Lock-server）

本程序用于上海应用技术大学机器人爱好者社团。

## 简介

本程序是一个授权服务器。通过 UDP 协议接收来自门锁的开门请求，经过数据库验证后，返回（或不返回）开门指令。
此外，它还提供了一套 RESTful API，用于查看日志、远程开锁和账户管理。

当前项目**已废弃**，有同学 [@Zhangzqs]() 基于 Golang 完成了一个功能更多的后端，但是我觉得本项目结构更好一点hhh

- Golang 门锁后端 [Zhangzqs/smartlock-server](https://github.com/Zhangzqs/smartlock-server)
- ESP32 单片机代码 [Zhangzqs/ESP32-Smart-Lock](https://github.com/Zhangzqs/ESP32-Smart-Lock)

## 环境及依赖

- Debian 10 bluster
- rustc 1.47.0

## 客户端实现

- [ESP8266实现](https://github.com/Zhangzqs/lock-client-esp8266)
- [ESP32实现(未完成)](https://github.com/Zhangzqs/Lock-ESP32)

## 开源协议

Copyright © 2020 sunnysab  
Permission granted under [MIT Licence](https://mit-license.org/)
