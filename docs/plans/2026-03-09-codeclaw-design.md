# CodeClaw Design Document

Date: 2026-03-09

## 1. Positioning

CodeClaw is a **fork of ZeroClaw** specialized for multi-project development, testing, and infrastructure operations.

It retains ZeroClaw's full agent runtime (channels, memory, tools, providers, config, security) and adds:

- Small Rust extensions (~1000-2000 lines)
- Configuration presets for development/ops workflows
- SOP templates and skill packs for dev/test automation

## 2. Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Base | Fork ZeroClaw | 70%+ capabilities already exist; no point rewriting |
| Language | Rust (ZeroClaw native) | Avoids cross-language complexity; minimal new code |
| Architecture | Monolith binary (ZeroClaw default) | Single binary, embedded SQLite, simple deployment |
| Persistence | SQLite (vector + FTS5) | ZeroClaw standard |
| Config format | TOML with hot-reload | ZeroClaw standard |
| Plugin model | Rust trait / Go interface equivalent | ZeroClaw standard |
| Security posture | Relaxed from ZeroClaw defaults | Dev environments need broader command access |

## 3. What ZeroClaw Already Provides

Retained as-is, no modifications:

- **Runtime**: Main loop, daemon mode
- **17+ Channels**: CLI, Telegram, Discord, Slack, WhatsApp, Signal, iMessage, Matrix, etc.
- **28+ LLM Providers**: Anthropic, OpenAI, Google, DeepSeek, Qwen, Ollama, OpenRouter, etc.
- **12+ Tools**: shell, file_read, file_write, git, http_request, cron, browser (WebDriver), screenshot, delegate, memory, composio, hardware
- **Memory**: Hybrid vector + FTS5 search, SQLite/PostgreSQL/Markdown/Qdrant backends, LRU embedding cache
- **Delegate Tool**: Multi-agent delegation with model routing, load balancing, depth limiting
- **SOP System**: Step sequencing, trigger types (manual/webhook/MQTT/cron), execution modes (auto/supervised/manual)
- **MCP**: Full Model Context Protocol support
- **SkillForge**: Auto-discovery and quality evaluation engine
- **CLI Discovery**: Automatic detection of system CLIs (git, python, node, docker, etc.)
- **Security**: Sandbox, domain allowlist, encrypted secret storage, deny-by-default policy
- **Config**: TOML parsing, hot-reload, workspace resolution

## 4. Rust Code Extensions

### 4.1 SOP Enhancements

Extend the existing SOP system with:

- **Step-level conditions**: Branch logic within a workflow (not just trigger-level)
- **Retry policy**: Per-step retry count, backoff, timeout
- **Artifact output**: Steps can produce typed artifacts (screenshot, diff, report, log)
- **Human checkpoints**: Pause execution for human review at designated steps

Location: Extend `src/sop/`

### 4.2 Multi-Project Registry

Extend workspace configuration to support multiple project contexts:

- Project registration with repo URL, local path, tech stack, build/test/dev commands, browser base URL
- Active project switching via CLI command
- Per-project memory isolation (via tags)
- Per-project executor preferences

Location: Extend `src/config/` or `src/workspace/`

```toml
# Example project configuration
[[codeclaw.projects]]
id = "web-admin"
name = "Admin Dashboard"
repo_url = "git@github.com:org/web-admin.git"
local_path = "/home/user/projects/web-admin"
tech_stack = ["typescript", "react", "node"]
default_branch = "main"
build_cmd = "npm run build"
test_cmd = "npm test"
dev_cmd = "npm run dev"
browser_base_url = "http://localhost:3000"
preferred_executors = ["claude_code", "shell"]

[[codeclaw.projects]]
id = "api-server"
name = "Backend API"
repo_url = "git@github.com:org/api-server.git"
local_path = "/home/user/projects/api-server"
tech_stack = ["go"]
default_branch = "main"
build_cmd = "go build ./..."
test_cmd = "go test ./..."
dev_cmd = "go run ./cmd/server"
```

### 4.3 Memory Tagging Extension

Extend memory entries with structured tags for layered retrieval:

- `project_id`: Which project this memory belongs to
- `user_id`: Which user's preference/habit
- `session_id`: Current session context
- `artifact_id`: Link to a specific artifact/run
- `memory_type`: One of `project_rule`, `user_preference`, `session_context`, `event`, `artifact_link`

Location: Extend `src/memory/`

### 4.4 Playwright Browser Backend (Optional)

Add Playwright as an additional browser tool backend alongside existing WebDriver and agent-browser:

- Leverages the existing pluggable browser backend architecture
- Provides Playwright-specific capabilities (network interception, multi-browser, trace recording)
- Falls back to existing backends if Playwright is not installed

Location: Extend `src/tools/browser.rs` or add `src/tools/browser_playwright.rs`

## 5. Configuration Presets

### 5.1 Security Policy (Relaxed for Dev)

```toml
[security]
# Relax sandbox for development operations
shell_sandbox = "permissive"     # Allow broader command execution
file_access = "workspace"         # Full workspace access
network_access = "unrestricted"   # No domain allowlist restriction

[security.command_denylist]
# Only block truly dangerous commands
commands = ["rm -rf /", "mkfs", "dd if=/dev/zero"]

[security.confirmation_policy]
# Reduce confirmation prompts for dev workflow
auto_approve = ["git push", "docker build", "docker run", "npm install"]
require_approval = ["git push --force", "aws ec2 terminate-instances", "docker system prune"]
```

### 5.2 CLI Agent Delegation

```toml
[delegate.agents.claude_code]
name = "Claude Code"
cmd = "claude"
args = ["--print"]
description = "Anthropic Claude Code CLI for coding tasks"
use_api_key = true  # Not OAuth tokens

[delegate.agents.codex]
name = "Codex"
cmd = "codex"
args = ["exec"]
description = "OpenAI Codex CLI for code generation"

[delegate.agents.opencode]
name = "OpenCode"
cmd = "opencode"
args = []
description = "OpenCode CLI agent"

[delegate.agents.cursor]
name = "Cursor"
cmd = "cursor"
args = ["--cli"]
description = "Cursor editor CLI mode"
```

### 5.3 Infrastructure Operations

```toml
[tools.shell.allowed_commands]
# Explicitly allow infrastructure CLIs
docker = true
aws = true
gcloud = true
kubectl = true
terraform = true
vercel = true
```

## 6. SOP Templates (Dev Workflows)

### 6.1 Debug Workflow

```markdown
# Debug Workflow
trigger: manual
mode: supervised

## Steps
1. Collect error context (logs, stack trace, reproduction steps)
2. Analyze relevant code files
3. Delegate fix to Claude Code / Codex
4. Run unit tests
5. Start dev server
6. Browser verification (navigate, screenshot)
7. Summarize changes and evidence
```

### 6.2 Feature Workflow

```markdown
# Feature Workflow
trigger: manual
mode: supervised

## Steps
1. Parse requirements
2. Delegate implementation to Claude Code
3. Add/update tests
4. Run test suite
5. Start dev server
6. Browser walkthrough (multi-page, screenshot each)
7. Generate changeset summary
```

### 6.3 Regression Workflow

```markdown
# Regression Workflow
trigger: cron("0 2 * * *")
mode: auto

## Steps
1. Pull latest changes
2. Build project
3. Run full test suite
4. Start dev server
5. Browser patrol (visit all key pages, screenshot, collect errors)
6. Compare screenshots with baseline
7. Output failure report (if any)
8. Notify via channel (Telegram/Slack)
```

## 7. Skills Pack

Development-focused skills to add to `~/.zeroclaw/workspace/skills/`:

| Skill | Purpose |
|-------|---------|
| `code-review` | Read diff, analyze risk, output review |
| `test-verify` | Run tests, collect results, screenshot on failure |
| `screenshot-compare` | Compare current vs baseline screenshots |
| `deploy-check` | Pre-deployment checklist verification |
| `dependency-audit` | Check for vulnerable dependencies |
| `project-switch` | Switch active project context |
| `run-history` | Query past workflow runs and artifacts |

## 8. Implementation Phases

### Phase 1: Fork + Configure

- Fork ZeroClaw repository
- Apply security configuration presets (relaxed for dev)
- Configure CLI agent delegation (Claude Code, Codex, OpenCode, Cursor)
- Configure infrastructure tool access (Docker, AWS)
- Rename/rebrand to CodeClaw

### Phase 2: SOP + Skills

- Create debug/feature/regression SOP templates
- Write skill pack (code-review, test-verify, screenshot-compare, etc.)
- Test workflows end-to-end

### Phase 3: Rust Extensions

- Extend SOP with step-level conditions, retry, artifact output
- Add multi-project registry
- Add memory tagging (project_id, user_id, session_id)
- Optional: Add Playwright browser backend

### Phase 4: Native Development Support (Future)

- XcodeExecutor (build, test, simulator management, screenshot)
- GradleExecutor (Android build/test)
- Simulator lifecycle management
- Code signing / provisioning profile handling

## 9. Architecture: Driver/Car Model

ZeroClaw = driver (orchestration), Claude Code = car (execution). They are independent systems with separate API keys, configs, accounts, and permissions.

### Responsibility Split

| Layer | ZeroClaw (Driver) | Claude Code (Car) |
|-------|-------------------|-------------------|
| **Tools** | shell, cron, channels, memory, browser/screenshot, delegate | Read/Write/Edit, Grep/Glob, Bash(git), Playwright MCP |
| **Memory** | Scheduling-level: which project failed last, run history | Project-level: code conventions, bug history (.claude/) |
| **Workflows** | Coarse-grained SOPs: route task → delegate → collect artifacts → notify | Fine-grained: analyze → fix → test → verify (internal) |
| **Permissions** | Controls what gets delegated | Controls what gets executed |
| **Config** | ~/.codeclaw/config.toml | ~/.claude/settings.json + CLAUDE.md |

### Permission Bridge: Hooks + HTTP API

When Claude Code blocks an operation (permission prompt), ZeroClaw acts as approval proxy via Claude Code's hook system:

```
Claude Code hook (PreToolUse)
  → HTTP call to ZeroClaw gateway: POST /approve?tool=Bash&command=git+push+--force
  → ZeroClaw checks policy:
      - auto_approve list → return allow
      - require_approval list → forward to human via channel (Telegram/Slack)
      - human responds → ZeroClaw returns allow/deny
  → Claude Code proceeds or aborts
```

Implementation:

1. **ZeroClaw side**: Add `/approve` endpoint to existing axum gateway. Check against `autonomy.auto_approve` and `autonomy.always_ask` config. For human-approval items, send to active channel and wait for response.

2. **Claude Code side**: Configure hook in `.claude/settings.json`:
```json
{
  "hooks": {
    "PreToolUse": [{
      "command": "curl -sf http://localhost:${CODECLAW_PORT}/approve -d '{\"tool\":\"$CLAUDE_TOOL_NAME\",\"input\":\"$CLAUDE_TOOL_INPUT\"}'"
    }]
  }
}
```

3. **Channel flow**: ZeroClaw receives approval request → sends to Telegram/Slack → waits for human reply → returns result to hook.

This bridges the driver and car: ZeroClaw's security policy governs Claude Code's execution without the two systems needing to share config or credentials.

## 10. Design Principles

1. **Driver/Car separation** - ZeroClaw orchestrates, CLI agents execute. Don't micromanage.
2. **Minimal diff from ZeroClaw** - Keep fork maintainable, easy to sync upstream
3. **Configuration over code** - Prefer config changes over new Rust modules
4. **Project-first** - Everything organized around projects, not conversations
5. **Executor-agnostic** - Claude Code / Codex / Cursor are interchangeable tools
6. **Artifact-driven** - Results must have evidence (screenshots, diffs, logs)
7. **Permissive security** - Trust the developer, block only truly dangerous operations
8. **Memory split** - ZeroClaw owns scheduling memory, Claude Code owns project memory
