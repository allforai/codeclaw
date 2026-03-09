# Deploy Check Skill

## Purpose

Run a comprehensive pre-deployment checklist to ensure the codebase is safe and ready to deploy. This skill validates that there are no uncommitted changes, all tests pass, dependencies are free of known vulnerabilities, and the branch is in sync with the remote.

## Workflow

1. **Check git status** -- Ensure no uncommitted or untracked changes exist.
2. **Verify branch** -- Confirm the current branch and check for unpushed commits.
3. **Run tests** -- Execute the full test suite and require all tests to pass.
4. **Audit dependencies** -- Scan for known security vulnerabilities in dependencies.
5. **Report** -- Present a pass/fail checklist.

## Prompt: Pre-Deploy Checklist

When asked to run a deploy check, execute all checks and present results:

- Run `check-git-status`. If the output is non-empty, the check FAILS (uncommitted changes exist).
- Run `check-branch` to list any commits not yet pushed to `origin/main`.
- Run `run-tests`. If any test fails, the check FAILS.
- Run `audit-dependencies`. If any vulnerabilities are found, the check FAILS (or WARNS for low severity).

Format the output as:

```
## Deploy Readiness Report

| Check               | Status | Details                    |
|---------------------|--------|----------------------------|
| Clean working tree  | PASS/FAIL | N uncommitted files     |
| Branch synced       | PASS/FAIL | N unpushed commits      |
| Tests passing       | PASS/FAIL | N passed, N failed      |
| Dependency audit    | PASS/FAIL | N vulnerabilities found |

### Overall: READY / NOT READY

### Action Items (if not ready)
- ...
```

## Prompt: Quick Status

When asked for a quick deploy status:

- Run only `check-git-status` and `check-branch`.
- Report a brief one-line summary of readiness.

## Guidelines

- All four checks must pass for a READY status.
- If `cargo audit` is not installed, warn and skip that check rather than failing entirely.
- For non-Rust projects, adapt the test and audit commands to the appropriate package manager.
- Always verify the deploy target branch (usually `main`) is up to date with `git fetch` before checking sync status.
- Treat any HIGH or CRITICAL vulnerability as a deployment blocker.
