# CodeClaw Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fork ZeroClaw and configure it as a development/testing/ops-focused Agent OS called CodeClaw.

**Architecture:** Fork ZeroClaw (Rust), apply configuration presets for relaxed dev security, add CLI agent delegation configs, write SOP workflow templates and skill packs, then extend Rust code minimally for multi-project registry, SOP step conditions/retry, and memory tagging.

**Tech Stack:** Rust (ZeroClaw core), TOML (config), YAML (SOP/skills), Git

---

### Task 1: Fork ZeroClaw Repository

**Step 1: Fork the repo on GitHub**

Run:
```bash
gh repo fork zeroclaw-labs/zeroclaw --clone=false --remote=false
```
Expected: Fork created under your GitHub account.

**Step 2: Clone your fork into the codeclaw directory**

Run:
```bash
cd /Users/aa/Documents
rm -rf codeclaw  # Current empty repo
gh repo clone <your-github-username>/zeroclaw codeclaw
cd codeclaw
```
Expected: Full ZeroClaw source cloned.

**Step 3: Verify build**

Run:
```bash
cargo build
```
Expected: Successful compilation.

**Step 4: Commit**

```bash
git log --oneline -1
```
Expected: ZeroClaw's latest commit as HEAD.

---

### Task 2: Explore and Map ZeroClaw Codebase

Before making any changes, map the exact structure. This task produces a reference document.

**Step 1: Map directory structure**

Run:
```bash
find src/ -name "*.rs" | head -80
ls src/tools/
ls src/sop/ 2>/dev/null || echo "SOP dir not found"
ls src/memory/
ls src/config/ 2>/dev/null || ls src/workspace/ 2>/dev/null
```
Expected: Exact file listing. Record in notes for subsequent tasks.

**Step 2: Find tool registration pattern**

Run:
```bash
grep -rn "trait Tool" src/ | head -5
grep -rn "fn name" src/tools/mod.rs | head -10
```
Expected: Understand how tools are registered (factory, trait impl, etc.)

**Step 3: Find SOP/workflow structure**

Run:
```bash
grep -rn "struct Sop\|struct Step\|struct Trigger" src/ | head -10
find . -name "*.sop" -o -name "*.yaml" | grep -i sop | head -10
```
Expected: Understand SOP data model and file format.

**Step 4: Find memory entry structure**

Run:
```bash
grep -rn "struct Memory\|struct Entry\|struct Observation" src/memory/ | head -10
```
Expected: Understand memory entry fields (what tags/metadata exist already).

**Step 5: Find config schema**

Run:
```bash
grep -rn "struct Config\|struct Security\|sandbox" src/config/ | head -10
cat config/default.toml 2>/dev/null || cat config.toml 2>/dev/null | head -50
```
Expected: Understand current config structure and security settings.

**Step 6: Find delegate tool interface**

Run:
```bash
cat src/tools/delegate.rs | head -80
```
Expected: Understand how delegate tool routes tasks to agents.

**Step 7: Write codebase map document**

Create: `docs/zeroclaw-map.md`

Document:
- Exact file paths for each module
- Tool trait signature
- SOP data model
- Memory entry fields
- Config schema fields
- Delegate tool interface

**Step 8: Commit**

```bash
git add docs/zeroclaw-map.md
git commit -m "docs: add ZeroClaw codebase map for CodeClaw extension work"
```

---

### Task 3: Apply Security Configuration Preset

Relax ZeroClaw's default security for development use.

**Files:**
- Modify: `~/.zeroclaw/config.toml` (or the default config template in repo)
- Locate the actual default config file first: `find . -name "config.toml" -o -name "default.toml" | head -5`

**Step 1: Find current security config**

Run:
```bash
grep -rn "sandbox\|allowlist\|denylist\|shell_sandbox\|file_access\|network_access" src/config/ config/ | head -20
```
Expected: Locate security-related config fields.

**Step 2: Create CodeClaw config overlay**

Create: `config/codeclaw.toml` (or modify default config)

```toml
# CodeClaw: Development-focused security preset
[security]
shell_sandbox = "permissive"
file_access = "workspace"
network_access = "unrestricted"

[security.command_denylist]
commands = ["rm -rf /", "mkfs", "dd if=/dev/zero"]

[security.confirmation_policy]
auto_approve = ["git push", "docker build", "docker run", "npm install", "cargo build"]
require_approval = ["git push --force", "aws ec2 terminate-instances", "docker system prune"]
```

NOTE: Adapt field names to match actual ZeroClaw config schema discovered in Task 2.

**Step 3: Verify config loads**

Run:
```bash
cargo run -- --config config/codeclaw.toml --help 2>&1 | head -20
```
Expected: No parse errors.

**Step 4: Commit**

```bash
git add config/codeclaw.toml
git commit -m "feat: add CodeClaw dev-focused security configuration preset"
```

---

### Task 4: Configure CLI Agent Delegation

Set up Claude Code, Codex, OpenCode, and Cursor as delegate targets.

**Files:**
- Modify: `config/codeclaw.toml`
- Reference: `src/tools/delegate.rs` (read-only, understand interface)

**Step 1: Understand delegate agent config format**

Run:
```bash
grep -rn "agent\|delegate\|spawn\|subprocess" src/tools/delegate.rs | head -20
grep -rn "\[delegate\]" config/ | head -5
```
Expected: Understand how delegate agents are configured.

**Step 2: Add CLI agent configurations**

Append to `config/codeclaw.toml`:

```toml
[delegate.agents.claude_code]
name = "Claude Code"
cmd = "claude"
args = ["--print"]
description = "Anthropic Claude Code CLI for coding tasks"

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

NOTE: Adapt field names and structure to match actual delegate tool config schema discovered in Task 2.

**Step 3: Add infrastructure tool allowlist**

Append to `config/codeclaw.toml`:

```toml
[tools.shell.allowed_commands]
docker = true
aws = true
gcloud = true
kubectl = true
terraform = true
vercel = true
```

**Step 4: Verify config**

Run:
```bash
cargo run -- --config config/codeclaw.toml --help 2>&1 | head -20
```
Expected: No parse errors.

**Step 5: Commit**

```bash
git add config/codeclaw.toml
git commit -m "feat: add CLI agent delegation and infra tool configs"
```

---

### Task 5: Create Debug SOP Workflow

**Files:**
- Create: `sops/debug-workflow.yaml` (or wherever ZeroClaw expects SOP files)
- Reference: `src/sop/` (read-only)

**Step 1: Find SOP file format and location**

Run:
```bash
find . -name "*.sop" -o -name "*.yaml" | grep -i sop
grep -rn "sop_dir\|sop_path\|sops" src/config/ config/ | head -10
```
Expected: Know exact format and directory.

**Step 2: Create debug workflow**

Create: `sops/debug-workflow.yaml`

```yaml
name: "Debug Workflow"
id: "debug-workflow"
trigger:
  type: "manual"
mode: "supervised"

steps:
  - id: "collect-error"
    action: "shell"
    description: "Collect error context from logs"
    args:
      cmd: "tail -n 200 {{project.log_path}}"

  - id: "analyze-code"
    action: "delegate"
    description: "Analyze relevant code files for root cause"
    args:
      agent: "claude_code"
      task: "Analyze this error and identify the root cause:\n{{steps.collect-error.output}}"

  - id: "apply-fix"
    action: "delegate"
    description: "Delegate fix implementation"
    args:
      agent: "claude_code"
      task: "Implement the fix described in the analysis:\n{{steps.analyze-code.output}}"

  - id: "run-tests"
    action: "shell"
    description: "Run unit tests"
    args:
      cmd: "{{project.test_cmd}}"

  - id: "browser-verify"
    action: "browser"
    description: "Verify fix in browser"
    args:
      url: "{{project.browser_base_url}}"
      screenshot: true

  - id: "summarize"
    action: "delegate"
    description: "Summarize changes and evidence"
    args:
      agent: "claude_code"
      task: "Summarize the debug session: error, root cause, fix applied, test results, and browser verification."
```

NOTE: Adapt YAML schema to match actual ZeroClaw SOP format discovered in Task 2.

**Step 3: Commit**

```bash
git add sops/
git commit -m "feat: add debug SOP workflow template"
```

---

### Task 6: Create Feature and Regression SOP Workflows

**Files:**
- Create: `sops/feature-workflow.yaml`
- Create: `sops/regression-workflow.yaml`

**Step 1: Create feature workflow**

Create: `sops/feature-workflow.yaml`

```yaml
name: "Feature Workflow"
id: "feature-workflow"
trigger:
  type: "manual"
mode: "supervised"

steps:
  - id: "parse-requirements"
    action: "delegate"
    description: "Parse and clarify requirements"
    args:
      agent: "claude_code"
      task: "Parse these requirements and create an implementation plan:\n{{input.requirements}}"

  - id: "implement"
    action: "delegate"
    description: "Implement the feature"
    args:
      agent: "claude_code"
      task: "Implement according to plan:\n{{steps.parse-requirements.output}}"

  - id: "add-tests"
    action: "delegate"
    description: "Add or update tests"
    args:
      agent: "claude_code"
      task: "Add tests for the implementation:\n{{steps.implement.output}}"

  - id: "run-tests"
    action: "shell"
    description: "Run test suite"
    args:
      cmd: "{{project.test_cmd}}"

  - id: "browser-walkthrough"
    action: "browser"
    description: "Browser walkthrough with screenshots"
    args:
      url: "{{project.browser_base_url}}"
      screenshot: true
      multi_page: true

  - id: "changeset-summary"
    action: "delegate"
    description: "Generate changeset summary"
    args:
      agent: "claude_code"
      task: "Generate a changeset summary for this feature implementation."
```

**Step 2: Create regression workflow**

Create: `sops/regression-workflow.yaml`

```yaml
name: "Regression Workflow"
id: "regression-workflow"
trigger:
  type: "cron"
  cron: "0 2 * * *"
mode: "auto"

steps:
  - id: "pull-latest"
    action: "shell"
    description: "Pull latest changes"
    args:
      cmd: "cd {{project.local_path}} && git pull origin {{project.default_branch}}"

  - id: "build"
    action: "shell"
    description: "Build project"
    args:
      cmd: "cd {{project.local_path}} && {{project.build_cmd}}"

  - id: "run-tests"
    action: "shell"
    description: "Run full test suite"
    args:
      cmd: "cd {{project.local_path}} && {{project.test_cmd}}"

  - id: "start-server"
    action: "shell"
    description: "Start dev server"
    args:
      cmd: "cd {{project.local_path}} && {{project.dev_cmd}} &"
      timeout: 10

  - id: "browser-patrol"
    action: "browser"
    description: "Visit all key pages, screenshot, collect errors"
    args:
      urls:
        - "{{project.browser_base_url}}"
        - "{{project.browser_base_url}}/login"
        - "{{project.browser_base_url}}/dashboard"
      screenshot: true
      collect_console_errors: true

  - id: "report"
    action: "delegate"
    description: "Generate failure report if any step failed"
    args:
      agent: "claude_code"
      task: "Analyze regression results and generate report:\nTests: {{steps.run-tests.output}}\nBrowser: {{steps.browser-patrol.output}}"

notifications:
  - channel: "telegram"
    on: "complete"
    message: "Regression run complete: {{status}}"
```

**Step 3: Commit**

```bash
git add sops/
git commit -m "feat: add feature and regression SOP workflow templates"
```

---

### Task 7: Create Skills Pack

**Files:**
- Create: `skills/code-review/skill.yaml`
- Create: `skills/test-verify/skill.yaml`
- Create: `skills/deploy-check/skill.yaml`
- Create: `skills/project-switch/skill.yaml`

**Step 1: Find skill format**

Run:
```bash
find . -path "*/skills/*" -name "*.yaml" -o -name "*.toml" | head -10
cat skills/*/skill.yaml 2>/dev/null | head -30
```
Expected: Understand exact skill schema.

**Step 2: Create code-review skill**

Create: `skills/code-review/skill.yaml`

```yaml
name: "Code Review"
id: "code-review"
version: "1.0.0"
description: "Read git diff, analyze risk, output structured review"
trigger: "manual"

inputs:
  - name: "diff"
    type: "string"
    description: "Git diff to review (or 'HEAD' for latest commit)"
    default: "HEAD"

steps:
  - action: "shell"
    args:
      cmd: "git diff {{input.diff}}"
  - action: "delegate"
    args:
      agent: "claude_code"
      task: "Review this diff for: security issues, logic bugs, performance problems, style violations. Output structured review with risk level (low/medium/high)."

outputs:
  - name: "review"
    type: "string"
  - name: "risk_level"
    type: "string"
```

**Step 3: Create test-verify skill**

Create: `skills/test-verify/skill.yaml`

```yaml
name: "Test Verify"
id: "test-verify"
version: "1.0.0"
description: "Run tests, collect results, screenshot on failure"
trigger: "manual"

steps:
  - action: "shell"
    args:
      cmd: "{{project.test_cmd}}"
  - action: "browser"
    condition: "{{prev.exit_code}} != 0"
    args:
      url: "{{project.browser_base_url}}"
      screenshot: true

outputs:
  - name: "test_result"
    type: "string"
  - name: "screenshot"
    type: "file"
    optional: true
```

**Step 4: Create deploy-check skill**

Create: `skills/deploy-check/skill.yaml`

```yaml
name: "Deploy Check"
id: "deploy-check"
version: "1.0.0"
description: "Pre-deployment checklist verification"
trigger: "manual"

steps:
  - action: "shell"
    description: "Check for uncommitted changes"
    args:
      cmd: "git status --porcelain"
  - action: "shell"
    description: "Run full test suite"
    args:
      cmd: "{{project.test_cmd}}"
  - action: "shell"
    description: "Check for vulnerable dependencies"
    args:
      cmd: "npm audit 2>/dev/null || cargo audit 2>/dev/null || pip-audit 2>/dev/null || echo 'No audit tool found'"
  - action: "delegate"
    description: "Final deploy readiness assessment"
    args:
      agent: "claude_code"
      task: "Assess deployment readiness based on: git status, test results, and dependency audit."
```

**Step 5: Create project-switch skill**

Create: `skills/project-switch/skill.yaml`

```yaml
name: "Project Switch"
id: "project-switch"
version: "1.0.0"
description: "Switch active project context"
trigger: "manual"

inputs:
  - name: "project_id"
    type: "string"
    description: "Project ID to switch to"
    required: true

steps:
  - action: "memory"
    description: "Store active project"
    args:
      operation: "store"
      key: "active_project"
      value: "{{input.project_id}}"
  - action: "shell"
    description: "Navigate to project directory"
    args:
      cmd: "cd {{projects[input.project_id].local_path}}"
```

**Step 6: Commit**

```bash
git add skills/
git commit -m "feat: add CodeClaw skills pack (code-review, test-verify, deploy-check, project-switch)"
```

---

### Task 8: Rebrand to CodeClaw

**Files:**
- Modify: `Cargo.toml` (package name)
- Modify: `README.md` (or create new)
- Modify: Any branding strings in source

**Step 1: Update Cargo.toml package name**

Change `name = "zeroclaw"` to `name = "codeclaw"` in root `Cargo.toml`.

**Step 2: Find and update binary name references**

Run:
```bash
grep -rn "zeroclaw" Cargo.toml src/main.rs | head -20
```

Update binary name in `Cargo.toml`:
```toml
[[bin]]
name = "codeclaw"
path = "src/main.rs"
```

**Step 3: Update user-facing strings**

Run:
```bash
grep -rn '"zeroclaw"\|"ZeroClaw"' src/ | grep -v test | head -20
```

Replace user-facing brand strings (CLI help text, welcome message, etc.) from "ZeroClaw" to "CodeClaw".

**Step 4: Verify build**

Run:
```bash
cargo build
./target/debug/codeclaw --help
```
Expected: Binary works, shows "CodeClaw" branding.

**Step 5: Commit**

```bash
git add -A
git commit -m "feat: rebrand ZeroClaw to CodeClaw"
```

---

### Task 9: Extend SOP with Step Conditions and Retry (Rust)

This is the first Rust code extension task. Requires understanding from Task 2.

**Files:**
- Modify: `src/sop/schema.rs` (or equivalent data model file)
- Modify: `src/sop/executor.rs` (or equivalent step runner)
- Test: `tests/sop_extensions_test.rs`

**Step 1: Write failing test for step conditions**

Create: `tests/sop_step_condition_test.rs`

```rust
#[test]
fn test_step_with_condition_skips_when_false() {
    // Parse a SOP with a step that has condition: "{{prev.exit_code}} == 0"
    // When prev step exit_code is 1, step should be skipped
    let sop = parse_sop(r#"
        steps:
          - id: "step1"
            action: "shell"
            args:
              cmd: "exit 1"
          - id: "step2"
            action: "shell"
            condition: "{{steps.step1.exit_code}} == 0"
            args:
              cmd: "echo should_not_run"
    "#);

    let result = execute_sop(sop);
    assert_eq!(result.steps["step2"].status, StepStatus::Skipped);
}

#[test]
fn test_step_with_condition_runs_when_true() {
    let sop = parse_sop(r#"
        steps:
          - id: "step1"
            action: "shell"
            args:
              cmd: "exit 0"
          - id: "step2"
            action: "shell"
            condition: "{{steps.step1.exit_code}} == 0"
            args:
              cmd: "echo should_run"
    "#);

    let result = execute_sop(sop);
    assert_eq!(result.steps["step2"].status, StepStatus::Completed);
}
```

NOTE: Adapt imports, struct names, and parse functions to match actual ZeroClaw SOP code discovered in Task 2.

**Step 2: Run test to verify it fails**

Run: `cargo test test_step_with_condition -- --nocapture`
Expected: FAIL (condition field not recognized or not evaluated)

**Step 3: Add `condition` field to Step struct**

In the SOP schema file, add:
```rust
pub struct Step {
    // ... existing fields
    pub condition: Option<String>,  // NEW: expression to evaluate
}
```

**Step 4: Implement condition evaluation in executor**

In the SOP executor, before running a step:
```rust
if let Some(condition) = &step.condition {
    let evaluated = template_engine.render(condition, &context)?;
    if !eval_condition(&evaluated) {
        step_result.status = StepStatus::Skipped;
        continue;
    }
}
```

**Step 5: Run test to verify it passes**

Run: `cargo test test_step_with_condition -- --nocapture`
Expected: PASS

**Step 6: Write failing test for retry**

Add to test file:
```rust
#[test]
fn test_step_with_retry_retries_on_failure() {
    let sop = parse_sop(r#"
        steps:
          - id: "flaky-step"
            action: "shell"
            retry:
              max_attempts: 3
              backoff_ms: 100
            args:
              cmd: "test -f /tmp/flaky_marker && echo ok || (touch /tmp/flaky_marker && exit 1)"
    "#);

    let result = execute_sop(sop);
    assert_eq!(result.steps["flaky-step"].status, StepStatus::Completed);
    assert!(result.steps["flaky-step"].attempts > 1);
}
```

**Step 7: Run test to verify it fails**

Run: `cargo test test_step_with_retry -- --nocapture`
Expected: FAIL

**Step 8: Add retry fields to Step struct**

```rust
#[derive(Default)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub backoff_ms: u64,
}

pub struct Step {
    // ... existing fields
    pub condition: Option<String>,
    pub retry: Option<RetryPolicy>,  // NEW
}
```

**Step 9: Implement retry logic in executor**

```rust
let max_attempts = step.retry.as_ref().map(|r| r.max_attempts).unwrap_or(1);
let backoff = step.retry.as_ref().map(|r| r.backoff_ms).unwrap_or(0);

for attempt in 1..=max_attempts {
    let result = execute_step_action(&step)?;
    if result.success || attempt == max_attempts {
        step_result.attempts = attempt;
        step_result.status = if result.success { StepStatus::Completed } else { StepStatus::Failed };
        break;
    }
    tokio::time::sleep(Duration::from_millis(backoff * attempt as u64)).await;
}
```

**Step 10: Run test to verify it passes**

Run: `cargo test test_step_with_retry -- --nocapture`
Expected: PASS

**Step 11: Commit**

```bash
git add src/sop/ tests/
git commit -m "feat: add step-level conditions and retry to SOP system"
```

---

### Task 10: Add Multi-Project Registry (Rust)

**Files:**
- Modify: `src/config/` (add project config parsing)
- Create: `src/project/mod.rs` (or extend existing workspace module)
- Test: `tests/project_registry_test.rs`

**Step 1: Write failing test for project parsing**

Create: `tests/project_registry_test.rs`

```rust
#[test]
fn test_parse_project_config() {
    let config_str = r#"
        [[codeclaw.projects]]
        id = "web-admin"
        name = "Admin Dashboard"
        local_path = "/tmp/test-project"
        tech_stack = ["typescript", "react"]
        default_branch = "main"
        build_cmd = "npm run build"
        test_cmd = "npm test"
        dev_cmd = "npm run dev"
        browser_base_url = "http://localhost:3000"
    "#;

    let config: CodeClawConfig = toml::from_str(config_str).unwrap();
    assert_eq!(config.projects.len(), 1);
    assert_eq!(config.projects[0].id, "web-admin");
    assert_eq!(config.projects[0].test_cmd, "npm test");
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_parse_project_config -- --nocapture`
Expected: FAIL (CodeClawConfig struct doesn't exist)

**Step 3: Create project data model**

Create or extend config module:

```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub repo_url: Option<String>,
    pub local_path: String,
    #[serde(default)]
    pub tech_stack: Vec<String>,
    #[serde(default = "default_branch")]
    pub default_branch: String,
    #[serde(default)]
    pub build_cmd: Option<String>,
    #[serde(default)]
    pub test_cmd: Option<String>,
    #[serde(default)]
    pub dev_cmd: Option<String>,
    #[serde(default)]
    pub browser_base_url: Option<String>,
    #[serde(default)]
    pub preferred_executors: Vec<String>,
}

fn default_branch() -> String {
    "main".to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodeClawConfig {
    #[serde(default)]
    pub projects: Vec<Project>,
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_parse_project_config -- --nocapture`
Expected: PASS

**Step 5: Write failing test for active project switching**

```rust
#[test]
fn test_project_registry_switch_active() {
    let mut registry = ProjectRegistry::new(vec![
        Project { id: "a".into(), name: "A".into(), local_path: "/tmp/a".into(), ..Default::default() },
        Project { id: "b".into(), name: "B".into(), local_path: "/tmp/b".into(), ..Default::default() },
    ]);

    assert!(registry.active().is_none());
    registry.set_active("a").unwrap();
    assert_eq!(registry.active().unwrap().id, "a");
    registry.set_active("b").unwrap();
    assert_eq!(registry.active().unwrap().id, "b");
    assert!(registry.set_active("nonexistent").is_err());
}
```

**Step 6: Run test, verify fails**

Run: `cargo test test_project_registry_switch -- --nocapture`
Expected: FAIL

**Step 7: Implement ProjectRegistry**

```rust
pub struct ProjectRegistry {
    projects: Vec<Project>,
    active_id: Option<String>,
}

impl ProjectRegistry {
    pub fn new(projects: Vec<Project>) -> Self {
        Self { projects, active_id: None }
    }

    pub fn active(&self) -> Option<&Project> {
        self.active_id.as_ref()
            .and_then(|id| self.projects.iter().find(|p| &p.id == id))
    }

    pub fn set_active(&mut self, id: &str) -> Result<()> {
        if self.projects.iter().any(|p| p.id == id) {
            self.active_id = Some(id.to_string());
            Ok(())
        } else {
            Err(anyhow!("Project '{}' not found", id))
        }
    }

    pub fn list(&self) -> &[Project] {
        &self.projects
    }

    pub fn get(&self, id: &str) -> Option<&Project> {
        self.projects.iter().find(|p| p.id == id)
    }
}
```

**Step 8: Run test, verify passes**

Run: `cargo test test_project_registry -- --nocapture`
Expected: PASS

**Step 9: Commit**

```bash
git add src/ tests/
git commit -m "feat: add multi-project registry with config parsing and active project switching"
```

---

### Task 11: Extend Memory with Project/Session Tags (Rust)

**Files:**
- Modify: `src/memory/` (extend entry struct and queries)
- Test: `tests/memory_tagging_test.rs`

**Step 1: Find current memory entry struct**

Run:
```bash
grep -rn "struct.*Entry\|struct.*Observation\|struct.*Memory" src/memory/ | head -10
```
Expected: Identify the exact struct to extend.

**Step 2: Write failing test for tagged memory**

Create: `tests/memory_tagging_test.rs`

```rust
#[test]
fn test_store_and_search_with_project_tag() {
    let mut memory = create_test_memory();

    memory.store(MemoryEntry {
        content: "Always run linting before commit".into(),
        tags: MemoryTags {
            project_id: Some("web-admin".into()),
            memory_type: Some(MemoryType::ProjectRule),
            ..Default::default()
        },
        ..Default::default()
    }).unwrap();

    memory.store(MemoryEntry {
        content: "Use Go test -race flag".into(),
        tags: MemoryTags {
            project_id: Some("api-server".into()),
            memory_type: Some(MemoryType::ProjectRule),
            ..Default::default()
        },
        ..Default::default()
    }).unwrap();

    // Search within project scope
    let results = memory.search_with_tags("linting", &MemoryTagFilter {
        project_id: Some("web-admin".into()),
        ..Default::default()
    }, 10).unwrap();

    assert_eq!(results.len(), 1);
    assert!(results[0].content.contains("linting"));
}
```

NOTE: Adapt to actual memory struct names from Task 2 exploration.

**Step 3: Run test, verify fails**

Run: `cargo test test_store_and_search_with_project_tag -- --nocapture`
Expected: FAIL

**Step 4: Add tag fields to memory entry**

```rust
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemoryTags {
    pub project_id: Option<String>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub artifact_id: Option<String>,
    pub memory_type: Option<MemoryType>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MemoryType {
    ProjectRule,
    UserPreference,
    SessionContext,
    Event,
    ArtifactLink,
}
```

Add `tags: MemoryTags` field to existing memory entry struct.

**Step 5: Extend SQLite schema**

Add columns to memory table:
```sql
ALTER TABLE memories ADD COLUMN project_id TEXT;
ALTER TABLE memories ADD COLUMN user_id TEXT;
ALTER TABLE memories ADD COLUMN session_id TEXT;
ALTER TABLE memories ADD COLUMN artifact_id TEXT;
ALTER TABLE memories ADD COLUMN memory_type TEXT;
```

Create index:
```sql
CREATE INDEX idx_memories_project ON memories(project_id);
CREATE INDEX idx_memories_type ON memories(memory_type);
```

**Step 6: Extend search to filter by tags**

Add `search_with_tags` method that appends WHERE clauses for non-None tag fields.

**Step 7: Run test, verify passes**

Run: `cargo test test_store_and_search_with_project_tag -- --nocapture`
Expected: PASS

**Step 8: Commit**

```bash
git add src/memory/ tests/
git commit -m "feat: add project/user/session memory tagging with filtered search"
```

---

### Task 12: Integration Test - End-to-End Workflow

**Files:**
- Create: `tests/integration_test.rs`

**Step 1: Write integration test**

```rust
#[tokio::test]
async fn test_codeclaw_project_switch_and_workflow() {
    // 1. Load config with two projects
    let config = load_config("tests/fixtures/codeclaw-test.toml");

    // 2. Initialize project registry
    let mut registry = ProjectRegistry::new(config.projects);
    registry.set_active("test-project").unwrap();

    // 3. Verify active project
    let active = registry.active().unwrap();
    assert_eq!(active.id, "test-project");
    assert_eq!(active.test_cmd.as_deref(), Some("echo tests_pass"));

    // 4. Store project-scoped memory
    let memory = create_memory();
    memory.store(MemoryEntry {
        content: "Test rule for this project".into(),
        tags: MemoryTags {
            project_id: Some("test-project".into()),
            memory_type: Some(MemoryType::ProjectRule),
            ..Default::default()
        },
        ..Default::default()
    }).unwrap();

    // 5. Verify scoped search
    let results = memory.search_with_tags("rule", &MemoryTagFilter {
        project_id: Some("test-project".into()),
        ..Default::default()
    }, 10).unwrap();
    assert_eq!(results.len(), 1);
}
```

**Step 2: Create test fixture**

Create: `tests/fixtures/codeclaw-test.toml`

```toml
[[codeclaw.projects]]
id = "test-project"
name = "Test Project"
local_path = "/tmp/codeclaw-test"
tech_stack = ["rust"]
default_branch = "main"
test_cmd = "echo tests_pass"
```

**Step 3: Run integration test**

Run: `cargo test test_codeclaw_project_switch_and_workflow -- --nocapture`
Expected: PASS

**Step 4: Commit**

```bash
git add tests/
git commit -m "test: add integration test for project registry + memory tagging"
```

---

### Task 13: Final Verification and Tag

**Step 1: Run full test suite**

Run:
```bash
cargo test
```
Expected: All tests pass.

**Step 2: Verify binary builds and runs**

Run:
```bash
cargo build --release
./target/release/codeclaw --help
```
Expected: Shows CodeClaw branding, no errors.

**Step 3: Verify config loads**

Run:
```bash
./target/release/codeclaw --config config/codeclaw.toml --help
```
Expected: Config accepted.

**Step 4: Tag release**

```bash
git tag -a v0.1.0 -m "CodeClaw v0.1.0: ZeroClaw fork with dev/test/ops focus"
```

**Step 5: Final commit (if any remaining changes)**

```bash
git status
# If clean, skip. Otherwise:
git add -A
git commit -m "chore: final cleanup for v0.1.0"
```
