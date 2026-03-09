# ZeroClaw Codebase Map

Comprehensive reference for CodeClaw extension work. All paths are relative to
the repository root (`/Users/aa/Documents/codeclaw`).

---

## 1. Top-Level Directory Structure

```
.
├── benches/              # Criterion benchmarks
├── crates/
│   └── robot-kit/        # Sub-crate for robotic hardware abstraction
├── dev/                  # Developer tooling / scripts
├── docs/                 # Documentation (66+ files)
├── examples/             # Example configs / usage
├── firmware/             # Arduino, ESP32, Nucleo firmware
│   ├── zeroclaw-arduino/
│   ├── zeroclaw-esp32/
│   ├── zeroclaw-esp32-ui/
│   ├── zeroclaw-nucleo/
│   └── zeroclaw-uno-q-bridge/
├── fuzz/                 # Fuzz testing
├── python/               # Python SDK (zeroclaw_tools)
├── scripts/              # Build / release scripts
├── src/                  # Main Rust source (see below)
├── test_helpers/         # Shared test helpers
├── tests/                # Integration tests
├── web/                  # Embedded web dashboard (rust-embed)
├── Cargo.toml            # Workspace root
├── CLAUDE.md / AGENTS.md # Agent coding guides
└── config.toml           # (user) default config template
```

### Workspace (Cargo.toml)

```toml
[workspace]
members = [".", "crates/robot-kit"]
resolver = "2"

[package]
name = "zeroclaw"
version = "0.1.7"
edition = "2021"
rust-version = "1.87"
```

---

## 2. Source Code Structure (`src/`)

### Top-level modules

| Directory/File        | Purpose                                         |
|-----------------------|-------------------------------------------------|
| `src/main.rs`         | CLI entry point, clap, subcommand dispatch       |
| `src/lib.rs`          | Library root, re-exports                         |
| `src/identity.rs`     | Identity formatting (OpenClaw/AIEOS)             |
| `src/migration.rs`    | Memory backend migration                         |
| `src/multimodal.rs`   | Image handling for LLM vision                    |
| `src/util.rs`         | Shared utilities                                 |
| `src/agent/`          | Agent loop, tool-call execution                  |
| `src/approval/`       | Interactive approval prompts                     |
| `src/auth/`           | OAuth flows (Anthropic, Gemini, OpenAI)          |
| `src/channels/`       | Messaging integrations (Telegram, Discord, etc.) |
| `src/config/`         | Configuration schema, loading, traits            |
| `src/cost/`           | Cost tracking and budget enforcement             |
| `src/cron/`           | Cron job scheduler                               |
| `src/daemon/`         | Background daemon mode                           |
| `src/doctor/`         | System health checks                             |
| `src/gateway/`        | HTTP gateway (axum-based)                        |
| `src/hardware/`       | Hardware discovery and registry                  |
| `src/health/`         | Health endpoint                                  |
| `src/heartbeat/`      | Periodic health pings                            |
| `src/hooks/`          | Lifecycle hooks                                  |
| `src/integrations/`   | External integrations                            |
| `src/memory/`         | Persistent memory subsystem                      |
| `src/observability/`  | Tracing, metrics, OTLP export                    |
| `src/onboard/`        | Onboarding wizard                                |
| `src/peripherals/`    | Peripheral board communication                   |
| `src/providers/`      | LLM provider adapters                            |
| `src/rag/`            | RAG / datasheet indexing                         |
| `src/runtime/`        | Runtime adapters (native, Docker)                |
| `src/security/`       | Security policy, sandbox, audit, secrets         |
| `src/service/`        | Service management                               |
| `src/skillforge/`     | Skill discovery and evaluation                   |
| `src/skills/`         | Skill definitions, community loading             |
| `src/sop/`            | Standard Operating Procedures engine             |
| `src/tools/`          | Agent-callable tool implementations              |
| `src/tunnel/`         | Tunnel config for public exposure                |

---

## 3. Tool System

### 3.1 Tool Trait (`src/tools/traits.rs`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSpec {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters_schema(&self) -> serde_json::Value;
    async fn execute(&self, args: serde_json::Value) -> anyhow::Result<ToolResult>;
    fn spec(&self) -> ToolSpec { /* default impl builds from above */ }
}
```

### 3.2 All Tools (`src/tools/`)

| File                      | Tool Name               | Notes                          |
|---------------------------|-------------------------|--------------------------------|
| `shell.rs`                | `shell`                 | Shell command execution        |
| `file_read.rs`            | `file_read`             |                                |
| `file_write.rs`           | `file_write`            |                                |
| `file_edit.rs`            | `file_edit`             |                                |
| `glob_search.rs`          | `glob_search`           |                                |
| `content_search.rs`       | `content_search`        |                                |
| `memory_store.rs`         | `memory_store`          |                                |
| `memory_recall.rs`        | `memory_recall`         |                                |
| `memory_forget.rs`        | `memory_forget`         |                                |
| `cron_add.rs`             | `cron_add`              |                                |
| `cron_list.rs`            | `cron_list`             |                                |
| `cron_remove.rs`          | `cron_remove`           |                                |
| `cron_run.rs`             | `cron_run`              |                                |
| `cron_runs.rs`            | `cron_runs`             |                                |
| `cron_update.rs`          | `cron_update`           |                                |
| `schedule.rs`             | `schedule`              |                                |
| `delegate.rs`             | `delegate`              | Multi-agent delegation         |
| `browser.rs`              | `browser`               | Full browser automation        |
| `browser_open.rs`         | `browser_open`          | Simple URL opening             |
| `http_request.rs`         | `http_request`          |                                |
| `web_fetch.rs`            | `web_fetch`             |                                |
| `web_search_tool.rs`      | `web_search`            |                                |
| `git_operations.rs`       | `git_operations`        |                                |
| `screenshot.rs`           | `screenshot`            |                                |
| `image_info.rs`           | `image_info`            |                                |
| `pdf_read.rs`             | `pdf_read`              | Feature-gated: `rag-pdf`      |
| `pushover.rs`             | `pushover`              | Push notifications             |
| `composio.rs`             | `composio`              | Managed OAuth tools            |
| `model_routing_config.rs` | `model_routing_config`  |                                |
| `proxy_config.rs`         | `proxy_config`          |                                |
| `cli_discovery.rs`        | `cli_discovery`         |                                |
| `sop_execute.rs`          | `sop_execute`           | SOP tools                     |
| `sop_list.rs`             | `sop_list`              |                                |
| `sop_approve.rs`          | `sop_approve`           |                                |
| `sop_status.rs`           | `sop_status`            |                                |
| `sop_advance.rs`          | `sop_advance`           |                                |
| `hardware_board_info.rs`  | `hardware_board_info`   | Feature-gated: `hardware`     |
| `hardware_memory_map.rs`  | `hardware_memory_map`   | Feature-gated: `hardware`     |
| `hardware_memory_read.rs` | `hardware_memory_read`  | Feature-gated: `hardware`     |

Auxiliary files: `schema.rs` (schema cleaning), `traits.rs` (Tool trait).

### 3.3 Tool Registration Pattern (`src/tools/mod.rs`)

Tools are **manually registered** in two factory functions -- no macros, no
registry trait:

- `default_tools(security)` -- 6 core tools: shell, file_read, file_write, file_edit, glob_search, content_search
- `all_tools_with_runtime(config, security, runtime, memory, ...)` -- full set, conditionally includes browser, HTTP, web_fetch, web_search, PDF, composio, delegate tools based on config flags

Each tool is `Arc<dyn Tool>` pushed into a `Vec`, then wrapped via
`boxed_registry_from_arcs()`.

**To add a new tool:** implement `Tool` in a new submodule, add `pub mod` and
`pub use` in `mod.rs`, push into the `tool_arcs` vec in
`all_tools_with_runtime`.

---

## 4. SOP / Workflow System

### 4.1 Module Location

`src/sop/` with files:

| File            | Purpose                                    |
|-----------------|--------------------------------------------|
| `mod.rs`        | SOP loading, parsing, validation, CLI      |
| `types.rs`      | All type definitions                       |
| `engine.rs`     | SopEngine: trigger matching, run lifecycle |
| `condition.rs`  | Trigger condition evaluator (JSON path)    |
| `dispatch.rs`   | Unified event dispatch helpers             |
| `audit.rs`      | SOP audit logging                          |
| `metrics.rs`    | SOP metrics collection                     |
| `gates.rs`      | Gate evaluation (feature-gated: `ampersona-gates`) |

### 4.2 Core Structs (`src/sop/types.rs`)

```rust
pub struct SopStep {
    pub number: u32,
    pub title: String,
    pub body: String,
    pub suggested_tools: Vec<String>,       // default: []
    pub requires_confirmation: bool,        // default: false
}

pub struct Sop {
    pub name: String,
    pub description: String,
    pub version: String,
    pub priority: SopPriority,              // Low | Normal | High | Critical
    pub execution_mode: SopExecutionMode,   // Auto | Supervised | StepByStep | PriorityBased
    pub triggers: Vec<SopTrigger>,
    pub steps: Vec<SopStep>,
    pub cooldown_secs: u64,                 // default: 0
    pub max_concurrent: u32,                // default: 1
    pub location: Option<PathBuf>,          // skip serialization
}

pub enum SopTrigger {
    Mqtt { topic: String, condition: Option<String> },
    Webhook { path: String },
    Cron { expression: String },
    Peripheral { board: String, signal: String, condition: Option<String> },
    Manual,
}

pub struct SopRun {
    pub run_id: String,
    pub sop_name: String,
    pub trigger_event: SopEvent,
    pub status: SopRunStatus,               // Pending | Running | WaitingApproval | Completed | Failed | Cancelled
    pub current_step: u32,
    pub total_steps: u32,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub step_results: Vec<SopStepResult>,
    pub waiting_since: Option<String>,
}

pub enum SopRunAction {
    ExecuteStep { run_id, step, context },
    WaitApproval { run_id, step, context },
    Completed { run_id, sop_name },
    Failed { run_id, sop_name, reason },
}
```

### 4.3 SOP File Format

SOPs live in `<workspace>/sops/<sop-name>/` with two files:

**`SOP.toml`** (metadata + triggers):
```toml
[sop]
name = "my-sop"
description = "Description"
version = "1.0.0"
priority = "high"              # low | normal | high | critical
execution_mode = "auto"        # auto | supervised | step_by_step | priority_based
cooldown_secs = 60
max_concurrent = 1

[[triggers]]
type = "manual"

[[triggers]]
type = "webhook"
path = "/sop/my-sop"

[[triggers]]
type = "cron"
expression = "0 */5 * * *"
```

**`SOP.md`** (procedure steps, parsed by `parse_steps()`):
```markdown
# My SOP

## Steps

1. **Step title** -- Step body description.
   - tools: shell, memory_store
   - requires_confirmation: true

2. **Another step** -- More details.
   - tools: file_read
```

### 4.4 Step Conditions and Retry

**Step conditions DO NOT exist yet.** `SopStep` has no `condition` or `retry`
fields. The `condition.rs` module only handles **trigger-level** conditions
(evaluating JSON path expressions against event payloads), not step-level
conditions.

### 4.5 SopConfig -- MISSING

**CRITICAL FINDING:** `SopConfig` is referenced by `src/sop/engine.rs` and all
SOP tool files (`sop_execute.rs`, `sop_list.rs`, `sop_approve.rs`,
`sop_status.rs`, `sop_advance.rs`, `sop/dispatch.rs`) via
`crate::config::SopConfig`, but **the struct is not defined anywhere in the
codebase**. The `Config` struct in `src/config/schema.rs` has **no `sop` field**.

This means the SOP tools likely do not compile. A `SopConfig` struct must be
created and added to `Config` before SOP tools can function. Expected fields
(inferred from usage in `engine.rs`):

```rust
pub struct SopConfig {
    pub sops_dir: Option<String>,
    pub default_execution_mode: SopExecutionMode,
}
```

---

## 5. Memory System

### 5.1 Module Location

`src/memory/` with files:

| File               | Purpose                                      |
|--------------------|----------------------------------------------|
| `mod.rs`           | Factory functions, backend dispatch           |
| `traits.rs`        | Memory trait, MemoryEntry, MemoryCategory     |
| `sqlite.rs`        | SQLite backend (brain.db) -- primary          |
| `lucid.rs`         | Lucid memory (SQLite + workspace sync)        |
| `markdown.rs`      | Markdown file backend                         |
| `none.rs`          | No-op backend                                 |
| `postgres.rs`      | PostgreSQL backend (feature-gated)            |
| `qdrant.rs`        | Qdrant vector DB backend                      |
| `backend.rs`       | Backend classification / selection helpers    |
| `vector.rs`        | Cosine similarity, vector operations          |
| `embeddings.rs`    | Embedding providers (OpenAI, custom, noop)    |
| `chunker.rs`       | Text chunking for embeddings                  |
| `response_cache.rs`| Response caching layer                        |
| `snapshot.rs`      | Memory snapshot export/hydration              |
| `hygiene.rs`       | Retention cleanup and archiving               |
| `cli.rs`           | Memory CLI subcommands                        |

### 5.2 Core Structs (`src/memory/traits.rs`)

```rust
#[derive(Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: String,
    pub key: String,
    pub content: String,
    pub category: MemoryCategory,
    pub timestamp: String,
    pub session_id: Option<String>,
    pub score: Option<f64>,
}

pub enum MemoryCategory {
    Core,
    Daily,
    Conversation,
    Custom(String),
}
```

**NOTE:** There are NO tags, metadata, or project fields on `MemoryEntry`.
Only `category` and `session_id` provide any form of scoping. Adding
project/session tags (Task 11) will require extending this struct.

### 5.3 Memory Trait (`src/memory/traits.rs`)

```rust
#[async_trait]
pub trait Memory: Send + Sync {
    fn name(&self) -> &str;
    async fn store(&self, key: &str, content: &str, category: MemoryCategory, session_id: Option<&str>) -> anyhow::Result<()>;
    async fn recall(&self, query: &str, limit: usize, session_id: Option<&str>) -> anyhow::Result<Vec<MemoryEntry>>;
    async fn get(&self, key: &str) -> anyhow::Result<Option<MemoryEntry>>;
    async fn list(&self, category: Option<&MemoryCategory>, session_id: Option<&str>) -> anyhow::Result<Vec<MemoryEntry>>;
    async fn forget(&self, key: &str) -> anyhow::Result<bool>;
    async fn count(&self) -> anyhow::Result<usize>;
    async fn health_check(&self) -> bool;
}
```

### 5.4 SQLite Schema (`src/memory/sqlite.rs`)

```sql
CREATE TABLE IF NOT EXISTS memories (
    id          TEXT PRIMARY KEY,
    key         TEXT NOT NULL UNIQUE,
    content     TEXT NOT NULL,
    category    TEXT NOT NULL DEFAULT 'core',
    embedding   BLOB,
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_memories_category ON memories(category);
CREATE INDEX IF NOT EXISTS idx_memories_key ON memories(key);

-- Migration: session_id added dynamically if missing
ALTER TABLE memories ADD COLUMN session_id TEXT;
CREATE INDEX IF NOT EXISTS idx_memories_session ON memories(session_id);

-- FTS5 full-text search (BM25 scoring)
CREATE VIRTUAL TABLE IF NOT EXISTS memories_fts USING fts5(
    key, content, content=memories, content_rowid=rowid
);
-- Triggers keep FTS in sync with memories table (insert, delete, update)

-- Embedding cache with LRU eviction
CREATE TABLE IF NOT EXISTS embedding_cache (
    content_hash TEXT PRIMARY KEY,
    embedding    BLOB NOT NULL,
    created_at   TEXT NOT NULL,
    accessed_at  TEXT NOT NULL
);
```

Database path: `<workspace>/memory/brain.db`

### 5.5 Memory Backends

| Backend    | Config value | Feature gate       | Notes                         |
|------------|--------------|--------------------|-------------------------------|
| SQLite     | `sqlite`     | (default)          | Primary, hybrid vector+FTS5   |
| Lucid      | `lucid`      | (default)          | SQLite + workspace sync       |
| Markdown   | `markdown`   | (default)          | Flat file fallback            |
| None       | `none`       | (default)          | Explicit no-op                |
| PostgreSQL | `postgres`   | `memory-postgres`  | Requires `[storage]` config   |
| Qdrant     | `qdrant`     | (default)          | External vector DB            |

---

## 6. Config System

### 6.1 Config Struct (`src/config/schema.rs`)

The `Config` struct is the top-level configuration loaded from `config.toml`.
All sections:

```rust
pub struct Config {
    pub workspace_dir: PathBuf,                        // skip serde
    pub config_path: PathBuf,                          // skip serde
    pub api_key: Option<String>,
    pub api_url: Option<String>,
    pub default_provider: Option<String>,              // default: "openrouter"
    pub default_model: Option<String>,                 // default: "anthropic/claude-sonnet-4.6"
    pub model_providers: HashMap<String, ModelProviderConfig>,
    pub default_temperature: f64,                      // default: 0.7
    pub observability: ObservabilityConfig,
    pub autonomy: AutonomyConfig,
    pub security: SecurityConfig,
    pub runtime: RuntimeConfig,
    pub reliability: ReliabilityConfig,
    pub scheduler: SchedulerConfig,
    pub agent: AgentConfig,
    pub skills: SkillsConfig,
    pub model_routes: Vec<ModelRouteConfig>,
    pub embedding_routes: Vec<EmbeddingRouteConfig>,
    pub query_classification: QueryClassificationConfig,
    pub heartbeat: HeartbeatConfig,
    pub cron: CronConfig,
    pub channels_config: ChannelsConfig,
    pub memory: MemoryConfig,
    pub storage: StorageConfig,
    pub tunnel: TunnelConfig,
    pub gateway: GatewayConfig,
    pub composio: ComposioConfig,
    pub secrets: SecretsConfig,
    pub browser: BrowserConfig,
    pub http_request: HttpRequestConfig,
    pub multimodal: MultimodalConfig,
    pub web_fetch: WebFetchConfig,
    pub web_search: WebSearchConfig,
    pub proxy: ProxyConfig,
    pub identity: IdentityConfig,
    pub cost: CostConfig,
    pub peripherals: PeripheralsConfig,
    pub agents: HashMap<String, DelegateAgentConfig>,
    pub hooks: HooksConfig,
    pub hardware: HardwareConfig,
    pub transcription: TranscriptionConfig,
    // NOTE: NO `sop` field exists -- SopConfig is MISSING
}
```

Config file location: `~/.zeroclaw/config.toml`
Workspace default: `~/.zeroclaw/workspace`

### 6.2 Security-Related Config

**`AutonomyConfig`** (`[autonomy]`):
```rust
pub struct AutonomyConfig {
    pub level: AutonomyLevel,                    // read_only | supervised | full
    pub workspace_only: bool,                    // default: true
    pub allowed_commands: Vec<String>,
    pub forbidden_paths: Vec<String>,
    pub max_actions_per_hour: u32,               // default: 100
    pub max_cost_per_day_cents: u32,             // default: 1000
    pub require_approval_for_medium_risk: bool,  // default: true
    pub block_high_risk_commands: bool,           // default: true
    pub shell_env_passthrough: Vec<String>,
    pub auto_approve: Vec<String>,               // tools that never need approval
    pub always_ask: Vec<String>,                 // tools that always need approval
    pub allowed_roots: Vec<String>,              // extra directory roots
}
```

**`SecurityConfig`** (`[security]`):
```rust
pub struct SecurityConfig {
    pub sandbox: SandboxConfig,
    pub resources: ResourceLimitsConfig,
    pub audit: AuditConfig,
    pub otp: OtpConfig,
    pub estop: EstopConfig,
}
```

**`SandboxConfig`** (`[security.sandbox]`):
```rust
pub struct SandboxConfig {
    pub enabled: Option<bool>,      // None = auto-detect
    pub backend: SandboxBackend,    // Auto | Landlock | Firejail | Bubblewrap | Docker | None
    pub firejail_args: Vec<String>,
}
```

**`ResourceLimitsConfig`** (`[security.resources]`):
```rust
pub struct ResourceLimitsConfig {
    pub max_memory_mb: u32,         // default: 512
    pub max_cpu_time_seconds: u64,  // default: 60
    pub max_subprocesses: u32,      // default: 10
    pub memory_monitoring: bool,    // default: true
}
```

**`SecurityPolicy`** (`src/security/policy.rs`) -- runtime enforcement:
```rust
pub struct SecurityPolicy {
    pub autonomy: AutonomyLevel,
    pub workspace_dir: PathBuf,
    pub workspace_only: bool,
    pub allowed_commands: Vec<String>,
    pub forbidden_paths: Vec<String>,
    pub allowed_roots: Vec<PathBuf>,
    pub max_actions_per_hour: u32,
    pub max_cost_per_day_cents: u32,
    pub require_approval_for_medium_risk: bool,
    pub block_high_risk_commands: bool,
    pub shell_env_passthrough: Vec<String>,
    pub tracker: ActionTracker,
}
```

Default allowed commands: `git`, `npm`, `cargo`, `ls`, `cat`, `grep`, `find`, `echo`, `pwd`.

### 6.3 Delegate Agent Config

```rust
pub struct DelegateAgentConfig {
    pub provider: String,
    pub model: String,
    pub system_prompt: Option<String>,
    pub api_key: Option<String>,
    pub temperature: Option<f64>,
    pub max_depth: u32,                // default: 3
    pub agentic: bool,                 // default: false
    pub allowed_tools: Vec<String>,    // for agentic mode
    pub max_iterations: usize,         // default: 10
}
```

Configured under `[agents.<name>]` in config.toml.

---

## 7. Delegate Tool (`src/tools/delegate.rs`)

### 7.1 Overview

The delegate tool enables multi-agent workflows. A primary agent can hand off
tasks to named sub-agents with different provider/model configurations.

### 7.2 Key Behaviors

- Agents configured in `Config.agents` HashMap
- Two modes:
  - **Simple**: single prompt/response via `provider.chat_with_system()`
  - **Agentic**: multi-turn tool-call loop via `run_tool_call_loop()` with filtered tool allowlist
- Recursion depth tracked per-instance (`self.depth`), checked against `agent_config.max_depth`
- Delegate tool **explicitly excluded** from sub-agent tool registries (prevents infinite recursion)
- Timeouts: 120s simple, 300s agentic
- Security: respects `ToolOperation::Act` enforcement, blocked in read-only mode

### 7.3 Can It Spawn Shell Subprocesses?

**Yes**, indirectly. In agentic mode, if `allowed_tools` includes `"shell"`,
the sub-agent gets access to the parent's `ShellTool` instance (which runs
commands through `SecurityPolicy`). The delegate tool itself does not directly
spawn processes.

---

## 8. Skills System

### 8.1 Skill Definitions (`src/skills/mod.rs`)

```rust
pub struct Skill {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: Option<String>,
    pub tags: Vec<String>,
    pub tools: Vec<SkillTool>,
    pub prompts: Vec<String>,
    pub location: Option<PathBuf>,       // skip serde
}

pub struct SkillTool {
    pub name: String,
    pub description: String,
    pub kind: String,                    // "shell" | "http" | "script"
    pub command: String,
    pub args: HashMap<String, String>,
}
```

### 8.2 Skill File Format

Skills live in `<workspace>/skills/<name>/` with:

- **`SKILL.toml`** -- manifest with `[skill]` metadata table + `[[tools]]` array
- **`SKILL.md`** -- markdown content with prompts (parsed as prompt blocks)

Community skills come from the `open-skills` repository
(`https://github.com/besoeasy/open-skills`), synced weekly when
`skills.open_skills_enabled = true`.

### 8.3 SkillsConfig

```rust
pub struct SkillsConfig {
    pub open_skills_enabled: bool,             // default: false
    pub open_skills_dir: Option<String>,
    pub prompt_injection_mode: SkillsPromptInjectionMode,  // full | compact
}
```

### 8.4 SkillForge (`src/skillforge/`)

Additional module for skill discovery and evaluation:
- `scout.rs` -- skill scouting/discovery
- `evaluate.rs` -- skill evaluation
- `integrate.rs` -- skill integration
- `mod.rs` -- module root

---

## 9. Provider System (`src/providers/`)

Supported providers: Anthropic, Bedrock, Copilot, Gemini, GLM, Ollama,
OpenAI Codex, OpenRouter, Telnyx, and `compatible` (generic OpenAI-compatible).
Routing handled by `router.rs`.

---

## 10. Crates Structure

```toml
[workspace]
members = [".", "crates/robot-kit"]
```

Only one sub-crate: `crates/robot-kit/` for robotic hardware abstraction
(drive, look, speak, listen, emote, sense, safety modules).

---

## 11. Key Findings for CodeClaw Implementation

### Must-Fix Before SOP Tools Work

1. **`SopConfig` does not exist.** Must create struct and add `pub sop: SopConfig`
   to `Config`, add to `Default` impl, and re-export from `config/mod.rs`.
   Minimum fields needed:
   ```rust
   pub struct SopConfig {
       pub sops_dir: Option<String>,
       pub default_execution_mode: SopExecutionMode,
   }
   ```

### Extension Points for CodeClaw

2. **Memory has no project/session tags.** `MemoryEntry` has `category` and
   `session_id` but no `tags`, `project_id`, or arbitrary metadata. The SQLite
   schema will need new columns.

3. **SOP steps have no conditions or retry.** `SopStep` has `number`, `title`,
   `body`, `suggested_tools`, `requires_confirmation` -- no `condition`,
   `retry_count`, `retry_delay`, or `on_failure` fields.

4. **Tool registration is manual.** No macro system or auto-discovery. New tools
   must be added to `all_tools_with_runtime` in `src/tools/mod.rs`.

5. **Security policy is runtime-only.** `SecurityPolicy` is built from
   `AutonomyConfig` at startup. Changing security posture requires config
   reload.

6. **SOP condition evaluator exists** (`src/sop/condition.rs`) but only for
   trigger-level JSON path conditions. Step-level conditions would need a new
   mechanism.

7. **Delegate tool supports agentic mode** with tool allowlists and max
   iterations -- suitable for CodeClaw's sub-agent needs.

8. **Config file is TOML** at `~/.zeroclaw/config.toml`. SOPs use
   TOML + Markdown. Skills use TOML + Markdown.

9. **Branding:** Package name is `zeroclaw`, config dir is `~/.zeroclaw/`,
   env vars prefixed `ZEROCLAW_`. Rebranding (Task 8) affects all of these.
