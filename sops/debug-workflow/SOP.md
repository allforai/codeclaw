# Debug Workflow

## Steps

1. **Prepare context** -- Read project config from memory and collect the full error logs, stack traces, and relevant surrounding context. This step gathers everything the coding agent will need so the delegation in the next step is self-contained. No analysis or fixing happens here -- only context assembly.
   - tools: memory_recall, shell
   - requires_confirmation: false

2. **Delegate to coding agent** -- Hand the complete debug task to the coding agent with all collected context. The coding agent handles the full debug cycle internally: root-cause analysis, fix implementation, test execution, and verification. ZeroClaw does not micromanage these sub-steps because they are the agent's core competency.
   - tools: delegate
   - requires_confirmation: true

3. **Collect artifacts and report** -- Gather the agent's output including the diff and test results. If the fix affects a web-facing feature, take a browser screenshot as visual evidence. Store the final results (diff, test output, screenshots) in memory and notify via the configured channel. This is ZeroClaw's job: infrastructure wrap-up that the coding agent cannot do.
   - tools: shell, browser, screenshot, memory_store
   - requires_confirmation: false
