# SPL Risk (Community Edition)

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

JSON example:

```bash
spl-risk <MINT_ADDRESS> --json
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

Пример JSON-режима:

```bash
spl-risk <MINT_ADDRESS> --json
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
