# Feature Workflow

## Steps

1. **Parse requirements** -- Read the feature request or issue description and break it down into concrete implementation tasks. Identify the affected modules, required API changes, new data structures, and UI components. Search the codebase for existing patterns and conventions to follow. Store the structured requirements in memory.
   - tools: file_read, content_search, memory_store
   - requires_confirmation: false

2. **Implement the feature** -- Delegate to claude_code agent with the structured requirements and a list of files to create or modify. The agent should follow existing code conventions, add appropriate error handling, and keep changes focused on the feature scope. Create a new git branch for the work if one does not already exist.
   - tools: delegate, git_operations, memory_store
   - requires_confirmation: true

3. **Add tests** -- Delegate to claude_code agent to write unit tests and integration tests covering the new feature. Tests should cover the happy path, edge cases, and error conditions. Ensure test file placement follows the project's existing test structure and naming conventions.
   - tools: delegate, content_search, file_read
   - requires_confirmation: true

4. **Run test suite** -- Execute the full test suite including the newly added tests. Verify all new tests pass and no existing tests have been broken. If any tests fail, capture the output and loop back to fix the implementation before proceeding.
   - tools: shell, memory_store
   - requires_confirmation: false

5. **Browser walkthrough** -- If the feature has a UI component, start the development server and navigate through the new feature in the browser. Take screenshots of each key state and interaction. Verify the layout, responsiveness, and functionality match the requirements.
   - tools: browser, screenshot, shell
   - requires_confirmation: true

6. **Changeset summary** -- Produce a detailed summary of all changes: new files created, existing files modified, tests added, and any configuration changes. Include a diff stat from git. Store the summary in memory and output it as the final report for pull request preparation.
   - tools: git_operations, memory_store, shell
   - requires_confirmation: false
