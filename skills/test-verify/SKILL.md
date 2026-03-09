# Test Verify Skill

## Purpose

Run the project test suite, collect structured pass/fail results, and capture visual evidence (browser screenshots) when failures occur. This skill is designed for both local development and CI pipelines.

## Workflow

1. **Run tests** -- Use `run-tests` to execute the full suite, or `run-tests-filtered` to target specific tests.
2. **Parse results** -- Extract pass/fail counts and identify failing test names.
3. **Capture on failure** -- If any tests fail and the project has a UI component, use `screenshot-failure` to capture the current browser state for debugging.
4. **Report** -- Present a structured summary.

## Prompt: Run Full Test Suite

When asked to run tests:

- Execute `run-tests` to run the complete test suite.
- Parse the output to determine:
  - Total tests run
  - Tests passed
  - Tests failed
  - Tests ignored/skipped
- If all tests pass, report success concisely.
- If any tests fail, list each failing test with its error output.

Format the output as:

```
## Test Results

**Status**: PASS / FAIL
**Total**: N | Passed: N | Failed: N | Skipped: N

### Failures (if any)

#### test_name
Error output...
```

## Prompt: Run Specific Tests

When asked to run specific tests:

- Use `run-tests-filtered` with the provided filter string.
- Follow the same reporting format as the full suite.

## Prompt: Capture Failure Screenshot

When a UI-related test fails:

- Use `screenshot-failure` to take a browser screenshot.
- Save the screenshot to `./test-artifacts/` with a timestamp-based filename.
- Include the screenshot path in the test failure report.

## Guidelines

- Always run tests from the project root directory.
- If `cargo test` is not appropriate (e.g., a JS project), detect the project type and use the correct test runner (npm test, pytest, etc.).
- On CI, ensure test artifacts are saved to a path that will be collected by the CI system.
- Do not ignore flaky tests silently -- report them as warnings.
- If tests take longer than 5 minutes, warn about potential performance issues.
