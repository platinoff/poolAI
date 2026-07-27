# Аналіз: grok-build (xAI / SpaceXAI) vs PoolAI

**Дата:** 2026-07-27  
**Тип:** технічний research (без маркетингу)  
**Джерела:** [xai-org/grok-build](https://github.com/xai-org/grok-build) (Apache-2.0), анонс [Grok Build is Now Open Source](https://x.ai/news/grok-build-open-source), публічні teardown/огляди архітектури; внутрішній зріз PoolAI (`src/`, `crates/`, `docs/ml/`, job/grid).  
**Статус:** research-only — **не** roadmap commit і **не** зміна `Cargo.toml`.

---

## 1. Вердикт (коротко)

| Питання | Відповідь |
|---------|-----------|
| Чи зливати репозиторії / vendor весь grok-build у PoolAI? | **Ні** |
| Чи сумісні ліцензії для drop-in копіювання коду? | **Ні як «просто MIT»** — потрібен NOTICE / attribution / patent grant Apache-2.0 |
| Чи приймає upstream PR від сторонніх? | **Ні** ([CONTRIBUTING.md](https://github.com/xai-org/grok-build/blob/main/CONTRIBUTING.md)) |
| Чи «Grok ML» = TurboQuant / тренування моделей? | **Ні** — harness агента + виклики моделі (локально або API) |
| Що реально корисне для PoolAI? | Ідеї протоколів (ACP/MCP), job/lease для довгих задач, sandbox-патерни — **окремими** PH-S*/FM, без merge дерев |

**Форки (~4.3k):** свідчать про інтерес спільноти. Вони **не** дають права на upstream merge і **не** знімають Apache-2.0 з копійованого коду.

---

## 2. Що таке grok-build

**Продукт:** terminal-native **coding agent harness** + fullscreen TUI (`grok` / `xai-grok-pager`).

**Опубліковано:** ~2026-07-15 як open source (синк з внутрішнього monorepo SpaceXAI; `SOURCE_REV` фіксує SHA).

### 2.1 Архітектурні шари (за публічним деревом)

| Шар | Приклад crates | Роль |
|-----|----------------|------|
| TUI | `xai-grok-pager`, ratatui/crossterm | UI клієнт |
| Agent runtime | `xai-grok-shell`, `xai-grok-agent` | сесії, tool dispatch, subagents |
| Північ (клієнти) | `xai-acp-lib`, `agent-client-protocol` | **ACP** — JSON-RPC «пульт» для TUI / IDE / CI / headless |
| Південь (інструменти) | `xai-grok-mcp`, MCP | **MCP** — зовнішні tool/data servers |
| Tools / workspace | `xai-grok-tools`, `xai-grok-workspace` | edit/search/shell, VCS, checkpoints |
| Ізоляція | `xai-grok-sandbox` | OS-level FS/network (Landlock/Seatbelt тощо; часто opt-in) |
| Інше | memory, hooks, plugins, telemetry, token-estimation | продуктовий harness |

**Ключова ідея:** TUI **не** володіє приватним backchannel до «мозку»; клієнти говорять **ACP**. MCP — окремий канал «зайвих рук» (інструменти). Leader process — shared local runtime для multi-client.

**Стек (перетин з PoolAI):** також Rust + `tokio` + `axum` 0.8 + `serde` + OTel/Prometheus у workspace — але **інша предметна область**.

**Edition:** workspace `edition = "2024"`; PoolAI — `edition = "2021"`.

---

## 3. Що таке PoolAI (релевантний зріз)

| Шар | Стан |
|-----|------|
| Продукт | Distributed **AI mining pool** / Galaxy Grid / RAID / VM / jobs / Solana sidecar |
| Ліцензія | **MIT** (`LICENSE`) |
| HTTP | `axum` 0.8, admin UI, OpenAPI |
| ML (`feature = "ml"`) | TurboQuant pack/unpack, pipeline, AutoML/federated/experiments — **стиснення / orchestration**, не coding-agent |
| Chat API | OpenAI-compatible completions (`services/chat_completion_service`, `ModelInterface`); token count — **евристика** (`len/4`), не BPE |
| Job kinds | `Inference \| Training \| FineTune \| Indexing \| Embeddings \| Memory \| System` |
| Worker | `poolai-worker` — virtual-node executor (ping/RAID/telegram…), **не** agent loop |
| Sandbox | `VmIsolation::ProcessSandbox` — **VM isolation**, не agent sandbox |
| MCP/ACP/TUI agent | **Відсутні** у product code (згадки MCP — лише Cursor ops research) |
| Solana | `crates/poolai-solana-adapter` — events на chain (devnet), не NLP tokenization |

PoolAI = **compute / grid / job platform**. grok-build = **developer coding agent platform**.

---

## 4. Ліцензії (обов’язково перед будь-яким кодом)

| Дія | Оцінка |
|-----|--------|
| Читати / збирати / запускати grok-build локально | OK під Apache-2.0 |
| `Cargo` dependency на Apache-2.0 crate (якщо з’явиться окремий опублікований crate) | Зазвичай OK для MIT-проєкту + notice в THIRD-PARTY |
| Скопіювати файли з grok-build у `src/` / `crates/` PoolAI | Потрібні **Apache-2.0 notice**, змінені файли §4(b), збереження patent grant; **не** «переліцензувати в MIT» |
| Злити git-історію / зробити PoolAI fork grok-build | Безсенсовно продуктово + юридично заплутано |
| PR у `xai-org/grok-build` | Відхиляються за політикою |

**Рекомендація:** будь-який future PH-S* — **clean-room** (власна реалізація за протоколами ACP/MCP specs) або явний Apache-notice sidecar crate; **не** copy-paste з дерева grok-build без юридичного огляду OWNER.

---

## 5. Порівняння «Grok у ML» vs PoolAI ML

| Тема | grok-build | PoolAI |
|------|------------|--------|
| Роль моделі | LLM для coding agent (BYOK / local / API) | Completions API + job Inference/Training; TurboQuant для **ваг/буферів** |
| Tokenization | `xai-token-estimation` / model path | евристика в chat; Gigatoken тощо — **поза** поточним кодом |
| Тренування / квантизація | не продукт harness | `src/ml/turboquant.rs`, pipeline quantization |
| «Tokenization» у Galaxy | — | **Solana / billing** (FM-010), не BPE |

Висновок: інтерес до grok-build **не** замінює і **не** прискорює TurboQuant. Це ортогональні осі.

---

## 6. Що **не** варто робити

1. Merge / subtree / vendor усього `grok-build` у PoolAI.  
2. Робити PoolAI «ще один Cursor/Grok TUI» — розмиває product-complete Galaxy Grid.  
3. Чекати upstream merge через форки.  
4. Плутати agent sandbox і `VmIsolation`.  
5. Підміняти `async-raft` / RAID / job lease логіку кодом з agent harness.

---

## 7. Що **можливо** зробити (пріоритезовано)

Рівні: **P0** = низький ризик / висока ясність · **P1** = корисний інкремент · **P2** = горизонт / окремий продукт.

### P0 — процес і знання (без коду продукту)

| # | Дія | Acceptance |
|---|-----|------------|
| A | Тримати цей документ як канон research | Посилання з INDEX |
| B | OWNER: політика «no vendoring Apache without NOTICE» | 1 рядок у security/docs policy за потреби |
| C | Локально (поза PoolAI tree) зібрати `grok` для навчання патернів | Опційно; не в CI PoolAI |

### P1 — інтеграція **зовні**, не merge

| # | Дія | Як стикується з PoolAI | Effort |
|---|-----|------------------------|--------|
| D | Документувати «operator uses grok/Cursor vs PoolAI stands» | HANDOFF / ops runbook | S |
| E | Headless agent у **CI** лише як зовнішній tool (не dependency) | `.github` optional workflow, secrets окремо | M |
| F | MCP **client** у майбутньому admin/ops (PoolAI як MCP *server* для метрик/jobs) | Новий FM; clean-room або crates.io MCP SDK | L |
| G | Розширити `JobKind` / worker task типом на кшталт `AgentSession` (метадані + lease), без TUI | `src/job/`, `poolai-worker` | L |

### P1 — протоколи (clean-room)

| # | Дія | Примітка |
|---|-----|----------|
| H | Вивчити публічну специфікацію **ACP**; опційний thin adapter «PoolAI operator ACP» | Не копипастити `xai-acp-lib` |
| I | MCP server: expose `GET /metrics`, job status, grid locality як tools | Синергія з Prometheus / job API |

### P2 — горизонт (окремий продукт / crate)

| # | Дія | Ризик |
|---|-----|-------|
| J | Окремий MIT/Apache dual crate `poolai-operator-agent` (workspace member) з ACP+MCP | Великий scope; конкурує з Cursor/Grok як UX |
| K | Підключити локальний inference через існуючий `ModelInterface` до agent loop | Потрібен реальний tokenizer / model load (FM-035+) |
| L | Gigatoken / sonic-rs — з **Speed Layer** research; не з grok-build | Інша вісь (див. окремі notes сесії) |

### Матриця «цінність × ризик»

```
Висока цінність, низький ризик:   A, B, D
Середня цінність, середній ризик: F, G, I
Висока цінність, високий ризик:   J, K  (окремий продукт)
Низька цінність / антипатерн:     merge, vendor без NOTICE, PR upstream
```

---

## 8. Рекомендована черговість (якщо OWNER відкриє токени)

1. **Не** чіпати `Cargo.toml` під grok-build.  
2. Зафіксувати FM-чернетку (наприклад **FM-0xx Operator agent / MCP**) у `FUNCTION_MANAGEMENT.md` §5.1 — тільки після явного запиту.  
3. Перший код: MCP **server** поверх існуючих REST/metrics (найменший overlap з TUI).  
4. Agent loop / ACP — лише якщо з’явиться окремий product brief (не «тому що форків багато»).

---

## 9. Посилання

- Upstream: https://github.com/xai-org/grok-build  
- Анонс: https://x.ai/news/grok-build-open-source  
- Документація продукту: https://docs.x.ai/build/overview · https://x.ai/cli  
- PoolAI ML: [`../ml/TURBOQUANT_INTEGRATION.md`](../ml/TURBOQUANT_INTEGRATION.md)  
- Job layer: [`JOB_LAYER_CONCEPT_2026-03-17.md`](./JOB_LAYER_CONCEPT_2026-03-17.md)  
- Solana adapter: [`SOLANA_ADAPTER_CONCEPT_2026-04-06.md`](./SOLANA_ADAPTER_CONCEPT_2026-04-06.md)  
- Cursor/MCP (IDE ops, не product): [`CURSOR_UPDATE_RESEARCH_2026-07-24.md`](./CURSOR_UPDATE_RESEARCH_2026-07-24.md)

---

## 10. Changelog документа

| Дата | Зміна |
|------|--------|
| 2026-07-27 | Перший глибокий зріз: no-merge, license, ML orthogonality, P0–P2 options |
