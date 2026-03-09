# Regression Workflow

## Steps

1. **Prepare environment** -- Pull the latest code from the main branch and load the project config from memory. Log the current HEAD commit and the commit range since the last regression run. This ensures the coding agent works against the newest code and the report references exactly what was tested.
   - tools: shell, memory_recall
   - requires_confirmation: false

2. **Delegate regression run** -- Hand the complete test-and-verify task to the coding agent: build the project, run the full test suite, start the dev server, check key pages, and report any failures. The coding agent manages this entire sequence internally because build-test-verify is a single logical unit that the agent can iterate on without orchestrator involvement.
   - tools: delegate
   - requires_confirmation: false

3. **Collect results and alert** -- Gather the test report and any screenshots from the agent's output. Compare results against the stored baseline from the previous run. Store the new results in memory. If any failures are detected, notify via the configured channel with a summary and flag for human review.
   - tools: shell, screenshot, memory_store
   - requires_confirmation: false
