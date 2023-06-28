# ForNet

[English](./README.md) | 中文

ForNet 基于 [BoringTun](https://github.com/cloudflare/boringtun)：WireGuard Rust 实现，做了第三层协议组网工具。

目前, 它还处于 `实验` 阶段。

### 特性
- 客户端由 Rust 编写，100% 代码开源。
- 可自行构建、自行部署，提供 web 界面管理客户端，与 Keycloak SSO 集成简便。
- 支持 UDP/TCP 双协议，解决特殊地区对 UDP 的封锁。
- 发布了 Linux 和 macOS 平台二进制客户端和 Linux Docker 镜像，方便部署，其他平台正在加紧开发中。

### 快速开始
1. 打开控制平台网页： [ForNet Admin](https://sso.fornetcode.com), 注册账号，并前往邮箱激活。
2. 激活后，进入控制平台，创建网络，获得加入该网络邀请链接。
3. 前往 [Github Release](https://github.com/ForNetCode/fornet/releases)，按平台下载对应客户端，并解压到特定目录。
4. sudo fornet join xxx, 加入网路，并前往控制平台，激活该设备。

### 文档
[文档地址](https://fornetcode.github.io/documentation)，快速上手请参见 [Quick Start](https://fornetcode.github.io/documentation/guide/quick-start)。 
若您还想了解本项目未来发展方向，请参看 [项目规划](https://fornetcode.github.io/documentation/plan) 。

### License
[BSL 1.1](https://github.com/fornetcode/fornet/blob/main/LICENSE)

<sub>WireGuard is a registered trademark of Jason A. Donenfeld. ForNet is not sponsored or endorsed by Jason A. Donenfeld.</sub>