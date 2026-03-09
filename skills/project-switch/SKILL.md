# Project Switch Skill

## Purpose

Switch the active project context. This updates the agent's memory store with the new project identity and navigates to the correct project directory, ensuring all subsequent commands operate in the right context.

## Workflow

1. **Identify target project** -- Accept the project name or path from the user.
2. **Update memory** -- Use `set-active-project` to store the project name and path in memory.
3. **Navigate** -- Use `navigate-project` to change to the project directory.
4. **Load config** -- Use `load-project-config` to read the project's configuration and restore any project-specific settings.
5. **Confirm** -- Report the active project context to the user.

## Prompt: Switch Project

When asked to switch projects:

- Accept the project name or path as input.
- If only a name is given, look up the path from known projects in memory (key: `known_projects`).
- If the path does not exist, report an error and do not switch.
- Run `set-active-project` with the project name and resolved path.
- Run `navigate-project` to change to the project directory.
- Run `load-project-config` to read project-level configuration (if the config file exists).
- Report the switch:

```
## Project Context Switched

**Active Project**: project-name
**Path**: /absolute/path/to/project
**Config**: Loaded / Not found
```

## Prompt: Show Current Project

When asked what the current project is:

- Read `active_project` from the memory store.
- Report the project name and path.
- If no project is set, inform the user and suggest they switch to one.

## Prompt: List Known Projects

When asked to list projects:

- Read `known_projects` from the memory store.
- Display each project with its name and path.
- If none are registered, instruct the user on how to add one.

## Guidelines

- Always use absolute paths when storing project directories.
- Verify the target directory exists before switching.
- If a project has a `zeroclaw.toml` or similar config file, load it to restore project-specific agent settings.
- The memory store key `active_project` should contain both `name` and `path` fields.
- When switching projects, clear any stale context from the previous project to avoid cross-contamination.
