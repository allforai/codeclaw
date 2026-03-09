# Code Review Skill

## Purpose

Perform a structured code review on the current working tree or branch diff. The review identifies risk levels, security concerns, code quality issues, and provides actionable recommendations.

## Workflow

1. **Collect the diff** -- Use `git-diff` (for unstaged/staged changes) or `git-diff-branch` (for branch-level comparison) to capture the changeset.
2. **Analyze the diff** -- Pass the captured diff to `analyze-review` for structured analysis.
3. **Output the report** -- Present findings in a consistent format.

## Prompt: Review Current Changes

When asked to review code, follow these steps:

- Run `git-diff` with the `--cached` flag to capture staged changes. If nothing is staged, run without the flag to capture unstaged changes.
- For each file in the diff, assess:
  - **Risk Level**: LOW / MEDIUM / HIGH / CRITICAL
  - **Category**: security, performance, correctness, style, documentation
  - **Finding**: A concise description of the issue
  - **Suggestion**: A concrete fix or improvement

Format the output as:

```
## Code Review Report

### Summary
- Files reviewed: N
- Total findings: N
- Critical: N | High: N | Medium: N | Low: N

### Findings

#### [RISK] filename:line -- Category
Finding description.
> Suggestion: ...
```

## Prompt: Review Branch Diff

When asked to review a branch:

- Run `git-diff-branch` with the target branch (default: `main`).
- Follow the same analysis and output format as above.
- Additionally, summarize the overall intent of the branch changes at the top of the report.

## Guidelines

- Always check for hardcoded secrets, credentials, or API keys.
- Flag any TODO or FIXME comments introduced in the diff.
- Note functions that exceed 50 lines or have high cyclomatic complexity.
- Identify missing error handling or unchecked return values.
- Call out any changes to public APIs or breaking interface changes.
