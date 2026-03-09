# Debug Workflow

## Steps

1. **Collect error context** -- Gather the full error message, stack trace, and any relevant log output. Capture the exact command or action that triggered the error and note the environment details (branch, commit, OS). Store the error context in memory for reference by subsequent steps.
   - tools: shell, memory_store, file_read
   - requires_confirmation: false

2. **Analyze relevant code** -- Search the codebase for the files and functions referenced in the error. Read the source code around the failure point, trace the call chain, and identify the root cause. Check git history for recent changes to the affected files that may have introduced the bug.
   - tools: content_search, file_read, git_operations, shell
   - requires_confirmation: false

3. **Delegate fix implementation** -- Delegate to claude_code agent with a precise description of the root cause and the specific files to modify. Include the error context from memory and the analysis findings. The agent should implement the minimal fix required to resolve the issue without introducing regressions.
   - tools: delegate, memory_store
   - requires_confirmation: true

4. **Run test suite** -- Execute the project's test suite to verify the fix resolves the original error and does not break any existing tests. Run both unit tests and integration tests relevant to the affected module. Capture and store the full test output.
   - tools: shell, memory_store
   - requires_confirmation: false

5. **Browser verification** -- If the bug affects a web-facing feature, start the development server and use the browser to navigate to the affected page or endpoint. Take a screenshot to visually confirm the fix. Verify that the UI renders correctly and no console errors appear.
   - tools: browser, screenshot, shell
   - requires_confirmation: true

6. **Summarize findings** -- Produce a concise summary including: the original error, root cause analysis, changes made, test results, and browser verification outcome. Store the summary in memory and output it as the final report for review.
   - tools: memory_store
   - requires_confirmation: false
