# Feature Workflow

## Steps

1. **Prepare context** -- Load project config and feature requirements from memory or the triggering input. Assemble all relevant context (existing conventions, affected modules, acceptance criteria) into a single brief for the coding agent. No implementation happens here -- only context gathering.
   - tools: memory_recall
   - requires_confirmation: false

2. **Delegate to coding agent** -- Hand the complete feature task to the coding agent with the assembled brief. The coding agent handles implementation, test writing, test execution, and code-level verification internally. ZeroClaw does not split these into separate steps because the agent manages its own development workflow end-to-end.
   - tools: delegate
   - requires_confirmation: true

3. **Collect artifacts and report** -- Gather the changeset from git (diff stat, new/modified files). If the feature has a UI component, run a browser walkthrough and take screenshots as evidence. Store all artifacts (diff, test results, screenshots) in memory and notify via the configured channel. This is the orchestration layer's responsibility: artifact collection and delivery.
   - tools: shell, browser, screenshot, memory_store
   - requires_confirmation: false
