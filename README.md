
# TRNovel

> **TRNovel (Terminal Reader for Novel)** 是一个专为小说爱好者设计的终端阅读器。

[![NPM Version](https://img.shields.io/npm/v/@trnovel/trnovel)](https://www.npmjs.com/package/@trnovel/trnovel) ![NPM Downloads](https://img.shields.io/npm/d18m/%40trnovel%2Ftrnovel?label=npm%20downloads) [![Crates.io Version](https://img.shields.io/crates/v/trnovel)](https://crates.io/crates/trnovel) ![Crates.io Total Downloads](https://img.shields.io/crates/d/trnovel?label=crates.io%20downloads)

## 目录

- [TRNovel](#trnovel)
  - [目录](#目录)
  - [简介](#简介)
  - [特性](#特性)
    - [安装指南](#安装指南)
    - [使用说明](#使用说明)
      - [声明](#声明)
      - [法律声明与用户协议](#法律声明与用户协议)
      - [安全与隐私保护](#安全与隐私保护)

## 简介

TRNovel 是一款基于终端的小说阅读应用程序，由 Rust 语言构建，并采用了 Ratatui 库来提供用户界面。它兼容 Windows、Linux 和 MacOS 操作系统，旨在为用户提供流畅的小说阅读体验。

## 特性

TRNovel 提供了以下功能：

- 支持本地 `.txt` 格式的小说文件。
- 支持网络小说，通过集成特定书源获取内容。
- 自动保存阅读历史记录，方便您继续未完成的故事。
- 提供个性化主题设置，定制您的阅读环境。

请注意，TRNovel 的网络小说功能与 [Legado](https://github.com/gedoor/legado) 的书源并不完全兼容。

### 安装指南

根据您的开发环境，您可以选择以下任意一种方式来安装 TRNovel：

**使用 Node.js 环境安装**

若您已安装 Node.js 环境，可以通过 npm 全局安装 TRNovel：

```shell
npm install -g @trnovel/trnovel
```

**使用 Rust 环境安装**

如果您有 Rust 工具链（包括 cargo），可以直接通过 Cargo 安装 TRNovel：

```shell
cargo install trnovel
```

**下载预编译二进制文件**

对于没有 Node.js 或 Rust 环境的用户，可以从 [Releases](https://github.com/yexiyue/TRNovel/releases/latest) 页面下载适合您操作系统的最新版本的可执行文件。请确保将下载的文件路径添加到您的环境变量中以便全局调用。

### 使用说明

安装完成后，您可以直接在命令行中输入 `trnovel` 来启动应用。如果您是通过 `npm`或者`cargo`安装的也可以使用命令别名`trn`来启动应用。

初次使用时，建议您先查看帮助信息以熟悉基本操作：![trnovel-help](https://yexiyue.github.io/picx-images-hosting/TRNovel/image.9gwnw0eued.webp)

**本地阅读**

TRNovel 支持读取本地存储的小说文本文件（.txt 格式）。要开始阅读本地文件，请进入 TRNovel 并按下 `s` 键，然后按照提示输入或选择您想要打开的文件路径。

![](https://yexiyue.github.io/picx-images-hosting/TRNovel/20250116183707_rec_.gif)

**查看快捷键信息**

您可以在每个页面按i键，查看该页面可用的快捷键信息。

![](https://yexiyue.github.io/picx-images-hosting/TRNovel/image.webp)

**网络阅读**

对于网络上的小说资源，TRNovel 提供了书源解析的能力，允许您在线阅读最新的章节内容。要使用此功能，您首先需要导入书源。按`s`键，然后输入书源的本地地址或者URL，接着选择您想要导入的书源即可。

![](https://yexiyue.github.io/picx-images-hosting/TRNovel/20250117103809_rec_.gif)

**历史记录**

历史记录会自动保存，您可以在历史记录页面按d键删除历史记录。

![](https://yexiyue.github.io/picx-images-hosting/TRNovel/20250117110758_rec_.gif)

**主题设置**

您可以在主题设置页面修改主题颜色。
设置完成后，需要重新启动`TRNovel`才能生效。

![](https://yexiyue.github.io/picx-images-hosting/TRNovel/20250117111013_rec_.gif)

**清理缓存**

`TRNovel`会在HOME目录或者命令所在目录下创建一个`.novel`文件夹, 用于存放缓存文件。您可以使用以下命令快速清理缓存。

```shell
trnovel clear
```

如果您想要重置主题，或者更高级的自定义主题，可以删除或修改`.novel/theme.json`文件。

**快速模式，接着上一次阅读的位置继续阅读**

您可以使用以下命令进入快速模式，接着上一次阅读的位置继续阅读。

```shell
trnovel quick
```

注意：快速模式需要有一个历史记录才能使用。

#### 声明

1. 请大家支持正版, 所有资源来自网上, 该软件不参与任何制作, 上传, 储存等内容, 禁止传播违法资源。
2. 该软件仅供学习交流使用, 禁止个人用于非法商业用途, 请于安装后 24 小时内删除。
3. 该软件是一款开源项目，所有源码均托管在 GitHub 上，欢迎开发者贡献代码。

#### 法律声明与用户协议

1. 若您不同意本声明的任何内容，请您立即停止使用本软件。一旦您开始使用本软件产品和服务，则表示您已同意本声明的所有内容。
2. 本软件仅供个人学习、研究和技术交流使用，仅提供展示功能，所有数据资源均由用户自身制作提供，包括但不限于小说、漫画、视频网站、媒体分享站点等。本软件无法控制这些资源的合法性、准确性、完整性或可用性，因此不对资源内容的真实性、合法性或适用性负责。
3. 您在使用本软件时需自行负责所有操作和使用结果。本软件不对您通过使用本软件获取的任何内容负责，包括但不限于媒体资源的准确性、版权合规性、完整性、安全性和可用性。对于任何因使用本软件导致的损失、损害或法律纠纷，不承担任何责任。
4. 您在使用本软件时必须遵守您所在国家/地区的相关法律法规，禁止使用本软件进行任何违反法律法规的活动，包括但不限于制作、上传、传播、存储任何违法、侵权、淫秽、诽谤、恶意软件等内容。如您违反相关法律法规，需自行承担法律责任。
5. 本免责声明适用于本软件的所有用户。本软件保留随时修改、更新本声明的权利，并以 GitHub Readme、软件更新等形式通知用户。请您定期查阅并遵守最新的免责声明。

#### 安全与隐私保护

1. 数据安全：TRNovel 会将缓存文件存储在`.novel`文件夹中，不会上传至任何服务器。
2. 隐私保护：TRNovel 不会收集用户的个人信息，除非用户主动提供。用户的历史记录和主题设置仅存储在本地，不会上传至任何服务器。
3. 本软件不会对用户行为进行监控，但建议用户遵守相关法律法规，避免上传或访问非法内容。
