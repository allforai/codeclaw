# Regression Workflow

## Steps

1. **Pull latest changes** -- Fetch and pull the latest changes from the main branch. Log the current HEAD commit hash and the list of new commits since the last regression run. Store the commit range in memory so the report can reference exactly what was tested.
   - tools: git_operations, shell, memory_store
   - requires_confirmation: false

2. **Build the project** -- Run the full project build process including all compile steps and asset generation. Capture build warnings and errors. If the build fails, store the error output in memory and skip to the report step with a build-failure status.
   - tools: shell, memory_store
   - requires_confirmation: false

3. **Run full test suite** -- Execute all unit tests, integration tests, and end-to-end tests. Capture the complete test output including pass/fail counts, timing, and any failure details. Store a structured summary of results in memory for the final report.
   - tools: shell, memory_store
   - requires_confirmation: false

4. **Start development server** -- Launch the development server in the background and wait for it to become ready. Verify the server responds to a health-check request. If the server fails to start, capture the error output and note the failure for the report.
   - tools: shell, memory_store
   - requires_confirmation: false

5. **Browser patrol** -- Navigate the browser through all key pages and critical user flows: landing page, authentication, dashboard, and primary feature screens. Take a screenshot of each page. Check for JavaScript console errors, broken layouts, and missing assets. Record any anomalies found.
   - tools: browser, screenshot, memory_store
   - requires_confirmation: false

6. **Generate regression report** -- Compile the full regression report including: commit range tested, build status, test results summary, browser patrol findings with screenshots, and an overall pass/fail verdict. Store the report in memory and output it. If any step failed, flag the report as requiring human review.
   - tools: memory_store, shell
   - requires_confirmation: false
