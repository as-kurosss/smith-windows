# 🏗️ smith-windows

**UI Automation API for Windows — Rust library based on `uiautomation` crate**

`smith-windows` — это библиотека на Rust для автоматизации Windows через UI Automation API. Проект является MVP-надстройкой над `uiautomation` crate (v0.24.4) для интеграции в экосистему `smith-core`.

## 📋 Table of Contents

- [Key Features](#-key-features)
- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [Modules](#-modules)
- [Architecture](#-architecture)
- [Development](#-development)
- [License](#-license)

## 🚀 Key Features

- **ClickTool**: Клик по UI-элементам через UI Automation API
- **TypeTool**: Ввод текста в UI-элементы через clipboard
- **InspectTool**: Интерактивный режим инспекции элементов (Ctrl+Hover для захвата)
- **AutomationSession**: Управление сессиями UI Automation
- **Idempotent Operations**: Повторные вызовы не ломают состояние
- **Zero Silent Failures**: Все ошибки явные через `Result`
- **COM Safety**: Все WinAPI/COM вызовы изолируются через `tokio::task::spawn_blocking`

## 📦 Installation
