# SPL Risk (Community Edition)

[![Rust 1.75+](https://img.shields.io/badge/Rust-1.75%2B-informational)](bin/spl-risk/Cargo.toml)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Edition: Community](https://img.shields.io/badge/Edition-Community-blue)](crates/spl-risk-community/src/heuristics/mod.rs)
[![Status: As-Is](https://img.shields.io/badge/Status-As--Is-lightgrey)](README.md)

**TL;DR (EN):** Free, deterministic SPL token risk CLI. Community edition is unsigned/unaudited and shipped as-is.

**TL;DR (RU):** Бесплатный детерминированный CLI-анализатор риска SPL-токенов. Community-версия без подписи/аудита и поставляется "как есть".

Free CLI risk analyzer for SPL tokens on Solana. It reads on-chain data via RPC, applies a small set of deterministic heuristics, and returns a risk score plus a human-readable explanation.

Important: this is the free community edition, provided "as is". Releases may be unsigned and unaudited, and come with no guarantees. Use at your own risk and always DYOR.

## English

### Features

- Computes a risk score in the 0-100 range.
- Shows a clear rule breakdown with `--verbose`.
- Supports risk profiles: `conservative`, `balanced`, `degenerate`.
- Emits JSON output for scripts and CI.
- Caches RPC data and can print cache stats.

### Community Heuristics (Current)

- Whether mint authority is active.
- Whether freeze authority is active.
- Supply concentration for creator / top holder.
- Whether the creator is also an authority.
- Creator wallet age (based on top holders).
- Low holder count.
- Metadata presence and verification.

## Download Prebuilt Binary

You can download a ready-to-use precompiled binary for your operating system from **GitHub Releases** — no Rust toolchain required.

1. Go to the **Releases** page:
   https://github.com/solana-labs/spl-risk-community/releases

2. Download the archive for your platform:

   - **Linux (glibc)**: `spl-risk-vX.Y.Z-linux-x86_64.tar.gz`
   - **Linux (static, musl)**: `spl-risk-vX.Y.Z-linux-musl-x86_64.tar.gz`
   - **Linux (ARM64)**: `spl-risk-vX.Y.Z-linux-aarch64.tar.gz`
   - **macOS (Intel)**: `spl-risk-vX.Y.Z-macos-x86_64.tar.gz`
   - **macOS (Apple Silicon)**: `spl-risk-vX.Y.Z-macos-aarch64.tar.gz`
   - **Windows**: `spl-risk-vX.Y.Z-windows-x86_64.zip`

3. Extract the archive and make the binary executable (Linux/macOS):

   ```bash
   tar -xzf spl-risk-*.tar.gz
   chmod +x spl-risk
   ./spl-risk <MINT_ADDRESS>
   ```

   On Windows, unzip the archive and run `spl-risk.exe` from PowerShell or CMD.

> Note: Community Edition binaries are provided **as-is**, may be unsigned, and are not audited. Always verify checksums and DYOR.

## Скачать готовый бинарник

Вы можете скачать **готовый скомпилированный бинарник** под вашу операционную систему со страницы **GitHub Releases** — установка Rust не требуется.

1. Перейдите на страницу релизов:
   https://github.com/solana-labs/spl-risk-community/releases

2. Скачайте архив под вашу платформу:

   - **Linux (glibc)**: `spl-risk-vX.Y.Z-linux-x86_64.tar.gz`
   - **Linux (static, musl)**: `spl-risk-vX.Y.Z-linux-musl-x86_64.tar.gz`
   - **Linux (ARM64)**: `spl-risk-vX.Y.Z-linux-aarch64.tar.gz`
   - **macOS (Intel)**: `spl-risk-vX.Y.Z-macos-x86_64.tar.gz`
   - **macOS (Apple Silicon)**: `spl-risk-vX.Y.Z-macos-aarch64.tar.gz`
   - **Windows**: `spl-risk-vX.Y.Z-windows-x86_64.zip`

3. Распакуйте архив и запустите бинарник.

   **Linux / macOS:**
   ```bash
   tar -xzf spl-risk-*.tar.gz
   chmod +x spl-risk
   ./spl-risk <MINT_ADDRESS>
   ```

   **Windows:** распакуйте архив и запустите `spl-risk.exe` через PowerShell или CMD.

> Примечание: Community Edition поставляется **"как есть"**, бинарники могут быть без подписи и без аудита. Всегда проверяйте контрольные суммы и делайте DYOR.

### Quick Start

Requirements:

- Rust 1.75+.
- Access to a Solana RPC endpoint (public mainnet RPC is the default).

Build:

```bash
cargo build --release -p spl-risk
```

Run:

```bash
./target/release/spl-risk <MINT_ADDRESS>
```

Example:

```bash
./target/release/spl-risk EPjFWdd5AufqSSqeM2q1xzybapC8G4wEGGkZwyTDt1v -v
```

### Install As a CLI

```bash
cargo install --path bin/spl-risk
```

Then:

```bash
spl-risk <MINT_ADDRESS>
```

### Usage

Help:

```bash
spl-risk --help
```

Main options:

- `-r, --rpc-url` - RPC URL (or use `SOLANA_RPC_URL`).
- `-p, --profile` - risk profile: `conservative | balanced | degenerate`.
- `-j, --json` - JSON output.
- `-v, --verbose` - detailed breakdown.
- `-t, --timeout` - RPC timeout in seconds.
- `--no-cache` - disable cache.
- `--cache-stats` - print cache stats.

### Free RPC Key (Helius)

You can register with Helius and get a free API key, then pass it via `--rpc-url`.

Example:

```bash
spl-risk <MINT_ADDRESS> --rpc-url "https://mainnet.helius-rpc.com/?api-key=YOUR_API_KEY"
```

### Sample Output (Human)

Note: this is a representative example of the current formatter.

```text
═══════════════════════════════════════════════════════════
SPL TOKEN RISK ANALYSIS
═══════════════════════════════════════════════════════════

TOKEN: EPjFWdd5AufqSSqeM2q1xzybapC8G4wEGGkZwyTDt1v
PROFILE: balanced

RISK SCORE: 45% [MEDIUM]
CONFIDENCE: 86% [MEDIUM]

BREAKDOWN:
───────────────────────────────────────────────────────────
 ── mint authority active     : active           (+30)  Mint authority is active - owner can create unlimited tokens
 ── freeze authority active   : active           (+25)  Freeze authority is active - owner can freeze token accounts
 ── creator supply            : low              (-15)  Top holder has only 8.2% (well distributed)
 ── holder count              : low               (+5)  Low holder count (24) - early stage or limited adoption
 ── verified metadata         : verified           (0)  Metadata is verified

METRICS:
───────────────────────────────────────────────────────────
  Total Supply               : 1,000,000,000
  Decimals                   : 6
  Creator Supply             : 8.20%
  Holders                    : 24
  Top Holder                 : 8.20%
  Wallet Age                 : 120 days ≈ 0.3 years

DATA SOURCES:
───────────────────────────────────────────────────────────
  RPC          : ✓ OK
  Metadata     : ✓ OK
  Holders      : ✓ OK
  Wallet Age   : ✓ OK

EDITION LIMITATIONS:
───────────────────────────────────────────────────────────
  ✗ Liquidity pool analysis
  ✗ LP lock / burn detection
  ✗ Historical transaction patterns
  ✗ Unlimited batch processing

SUMMARY:
───────────────────────────────────────────────────────────
  Medium risk. Multiple risk factors detected. DYOR recommended.

DISCLAIMER:
───────────────────────────────────────────────────────────
  Probabilistic assessment based on on-chain heuristics only.
  NOT financial advice. Always DYOR (Do Your Own Research).
═══════════════════════════════════════════════════════════
```

### Sample Output (JSON)

Note: the exact fields may evolve, but this is the current shape.

```json
{
  "mint": "EPjFWdd5AufqSSqeM2q1xzybapC8G4wEGGkZwyTDt1v",
  "risk_score": 45,
  "confidence_score": 0.86,
  "profile": "balanced",
  "flags": {
    "mint_authority": true,
    "freeze_authority": true
  },
  "metrics": {
    "creator_supply_pct": 8.2,
    "wallet_age_days": 120,
    "holders": 24,
    "decimals": 6,
    "total_supply": 1000000000,
    "top_holder_pct": 8.2
  },
  "breakdown": [
    {
      "rule": "mint_authority_active",
      "weight": 30,
      "description": "Mint authority is active - owner can create unlimited tokens",
      "status": "active"
    },
    {
      "rule": "freeze_authority_active",
      "weight": 25,
      "description": "Freeze authority is active - owner can freeze token accounts",
      "status": "active"
    },
    {
      "rule": "supply_distributed",
      "weight": -15,
      "description": "Top holder has only 8.2% (well distributed)",
      "status": "low"
    },
    {
      "rule": "low_holders",
      "weight": 5,
      "description": "Low holder count (24) - early stage or limited adoption",
      "status": "low"
    },
    {
      "rule": "verified_metadata",
      "weight": 0,
      "description": "Metadata is verified",
      "status": "verified"
    }
  ],
  "summary": "Medium risk. Multiple risk factors detected. DYOR recommended.",
  "warnings": [],
  "data_sources": {
    "rpc": "ok",
    "metadata": "ok",
    "holders": "ok",
    "wallet_age": "ok"
  }
}
```

### How To Interpret Results

Approximate scale:

- `0-20` - LOW.
- `21-40` - LOW-MEDIUM.
- `41-60` - MEDIUM.
- `61-80` - HIGH.
- `81-100` - CRITICAL.

Exit codes (useful for CI / scripts):

- `0` - low risk (< 40).
- `1` - medium risk (40-69).
- `2` - high risk (>= 70).

### Community Edition Limits

This version is intentionally simple and free. In particular:

- No release signature verification.
- No audits and no formal correctness guarantees.
- No deep LP / liquidity analysis (unless added separately).
- RPC limits can lead to incomplete data.

### Disclaimer

This is not financial advice. The tool provides a heuristic on-chain risk assessment and can be wrong. You are responsible for your own decisions.

---

## Русская версия

### Что это

Бесплатный CLI-анализатор риска для SPL-токенов в сети Solana. Он читает ончейн-данные через RPC, применяет набор простых детерминированных эвристик и выдает риск-скор вместе с объяснением.

Важно: это бесплатная community-версия "как есть". Релизы могут быть без подписи и без аудита, без каких-либо гарантий. Используйте на свой риск и всегда делайте DYOR.

### Возможности

- Считает риск-скор в диапазоне 0-100.
- Показывает понятный breakdown правил при `--verbose`.
- Поддерживает профили риска: `conservative`, `balanced`, `degenerate`.
- Умеет печатать JSON (удобно для скриптов и CI).
- Кэширует RPC-данные и показывает статистику кэша.

### Эвристики community (сейчас)

- Активна ли mint authority.
- Активна ли freeze authority.
- Концентрация саплая у создателя / топ-холдера.
- Совпадает ли создатель с authority.
- Возраст кошелька создателя (по топ-холдерам).
- Низкое число холдеров.
- Наличие и верификация metadata.

### Быстрый старт

Требования:

- Rust 1.75+.
- Доступ к Solana RPC (по умолчанию используется публичный mainnet RPC).

Сборка:

```bash
cargo build --release -p spl-risk
```

Запуск:

```bash
./target/release/spl-risk <MINT_ADDRESS>
```

Пример:

```bash
./target/release/spl-risk EPjFWdd5AufqSSqeM2q1xzybapC8G4wEGGkZwyTDt1v -v
```

### Установка как CLI

```bash
cargo install --path bin/spl-risk
```

После этого:

```bash
spl-risk <MINT_ADDRESS>
```

### Использование

Справка:

```bash
spl-risk --help
```

Основные опции:

- `-r, --rpc-url` - URL RPC (или через `SOLANA_RPC_URL`).
- `-p, --profile` - профиль риска: `conservative | balanced | degenerate`.
- `-j, --json` - вывод в JSON.
- `-v, --verbose` - подробный breakdown.
- `-t, --timeout` - таймаут RPC в секундах.
- `--no-cache` - отключить кэш.
- `--cache-stats` - показать статистику кэша.

### Бесплатный RPC-ключ (Helius)

Можно зарегистрироваться в Helius, получить бесплатный API-ключ и передать его через `--rpc-url`.

Пример:

```bash
spl-risk <MINT_ADDRESS> --rpc-url "https://mainnet.helius-rpc.com/?api-key=YOUR_API_KEY"
```

### Пример вывода (human)

Важно: это репрезентативный пример текущего форматтера.

```text
═══════════════════════════════════════════════════════════
SPL TOKEN RISK ANALYSIS
═══════════════════════════════════════════════════════════

TOKEN: EPjFWdd5AufqSSqeM2q1xzybapC8G4wEGGkZwyTDt1v
PROFILE: balanced

RISK SCORE: 45% [MEDIUM]
CONFIDENCE: 86% [MEDIUM]

BREAKDOWN:
───────────────────────────────────────────────────────────
 ── mint authority active     : active           (+30)  Mint authority is active - owner can create unlimited tokens
 ── freeze authority active   : active           (+25)  Freeze authority is active - owner can freeze token accounts
 ── creator supply            : low              (-15)  Top holder has only 8.2% (well distributed)
 ── holder count              : low               (+5)  Low holder count (24) - early stage or limited adoption
 ── verified metadata         : verified           (0)  Metadata is verified

METRICS:
───────────────────────────────────────────────────────────
  Total Supply               : 1,000,000,000
  Decimals                   : 6
  Creator Supply             : 8.20%
  Holders                    : 24
  Top Holder                 : 8.20%
  Wallet Age                 : 120 days ≈ 0.3 years

DATA SOURCES:
───────────────────────────────────────────────────────────
  RPC          : ✓ OK
  Metadata     : ✓ OK
  Holders      : ✓ OK
  Wallet Age   : ✓ OK

EDITION LIMITATIONS:
───────────────────────────────────────────────────────────
  ✗ Liquidity pool analysis
  ✗ LP lock / burn detection
  ✗ Historical transaction patterns
  ✗ Unlimited batch processing

SUMMARY:
───────────────────────────────────────────────────────────
  Medium risk. Multiple risk factors detected. DYOR recommended.

DISCLAIMER:
───────────────────────────────────────────────────────────
  Probabilistic assessment based on on-chain heuristics only.
  NOT financial advice. Always DYOR (Do Your Own Research).
═══════════════════════════════════════════════════════════
```

### Пример вывода (JSON)

Важно: поля могут меняться, но форма сейчас такая.

```json
{
  "mint": "EPjFWdd5AufqSSqeM2q1xzybapC8G4wEGGkZwyTDt1v",
  "risk_score": 45,
  "confidence_score": 0.86,
  "profile": "balanced",
  "flags": {
    "mint_authority": true,
    "freeze_authority": true
  },
  "metrics": {
    "creator_supply_pct": 8.2,
    "wallet_age_days": 120,
    "holders": 24,
    "decimals": 6,
    "total_supply": 1000000000,
    "top_holder_pct": 8.2
  },
  "breakdown": [
    {
      "rule": "mint_authority_active",
      "weight": 30,
      "description": "Mint authority is active - owner can create unlimited tokens",
      "status": "active"
    },
    {
      "rule": "freeze_authority_active",
      "weight": 25,
      "description": "Freeze authority is active - owner can freeze token accounts",
      "status": "active"
    },
    {
      "rule": "supply_distributed",
      "weight": -15,
      "description": "Top holder has only 8.2% (well distributed)",
      "status": "low"
    },
    {
      "rule": "low_holders",
      "weight": 5,
      "description": "Low holder count (24) - early stage or limited adoption",
      "status": "low"
    },
    {
      "rule": "verified_metadata",
      "weight": 0,
      "description": "Metadata is verified",
      "status": "verified"
    }
  ],
  "summary": "Medium risk. Multiple risk factors detected. DYOR recommended.",
  "warnings": [],
  "data_sources": {
    "rpc": "ok",
    "metadata": "ok",
    "holders": "ok",
    "wallet_age": "ok"
  }
}
```

### Интерпретация результата

Примерная шкала:

- `0-20` - LOW.
- `21-40` - LOW-MEDIUM.
- `41-60` - MEDIUM.
- `61-80` - HIGH.
- `81-100` - CRITICAL.

Коды выхода (удобно для CI / скриптов):

- `0` - низкий риск (< 40).
- `1` - средний риск (40-69).
- `2` - высокий риск (>= 70).

### Ограничения community-версии

Эта версия намеренно простая и бесплатная. В частности:

- Нет проверки подписи релизов.
- Нет аудита и формальных гарантий корректности.
- Нет глубокого LP / liquidity-анализа (если не добавлен отдельно).
- Из-за лимитов RPC данные могут быть неполными.

### Дисклеймер

Это не финансовый совет. Инструмент дает эвристическую оценку риска на основе ончейн-данных и может ошибаться. Все решения вы принимаете самостоятельно.

---

If this is useful, feel free to use it, fork it, and adapt it to your needs.  
Если инструмент полезен - пользуйтесь, форкайте и дорабатывайте под себя.
