use zeroclaw::config::ProjectConfig;
use zeroclaw::project::ProjectRegistry;

/// Load project configs from the test fixture TOML and verify parsing.
fn load_test_projects() -> Vec<ProjectConfig> {
    let content =
        std::fs::read_to_string("tests/fixtures/codeclaw-test.toml").expect("fixture must exist");

    #[derive(serde::Deserialize)]
    struct Fixture {
        projects: Vec<ProjectConfig>,
    }
    let fixture: Fixture = toml::from_str(&content).expect("fixture must parse");
    fixture.projects
}

#[test]
fn integration_parse_fixture_config() {
    let projects = load_test_projects();
    assert_eq!(projects.len(), 2);

    let web = &projects[0];
    assert_eq!(web.id, "web-admin");
    assert_eq!(web.name, "Admin Dashboard");
    assert_eq!(web.local_path, "/tmp/codeclaw-test-web");
    assert_eq!(web.tech_stack, vec!["typescript", "react"]);
    assert_eq!(web.test_cmd.as_deref(), Some("npm test"));
    assert_eq!(
        web.browser_base_url.as_deref(),
        Some("http://localhost:3000")
    );
    assert_eq!(web.preferred_agents, vec!["claude_code"]);

    let api = &projects[1];
    assert_eq!(api.id, "api-server");
    assert_eq!(api.tech_stack, vec!["go"]);
    assert_eq!(api.test_cmd.as_deref(), Some("go test ./..."));
    assert_eq!(api.preferred_agents, vec!["codex", "claude_code"]);
}

#[test]
fn integration_registry_from_config() {
    let projects = load_test_projects();
    let mut registry = ProjectRegistry::new(projects);

    // Initially no active project
    assert!(registry.active().is_none());

    // List all projects
    assert_eq!(registry.list().len(), 2);

    // Switch to web-admin
    registry.set_active("web-admin").unwrap();
    let active = registry.active().unwrap();
    assert_eq!(active.id, "web-admin");
    assert_eq!(active.build_cmd.as_deref(), Some("npm run build"));

    // Switch to api-server
    registry.set_active("api-server").unwrap();
    let active = registry.active().unwrap();
    assert_eq!(active.id, "api-server");
    assert_eq!(active.build_cmd.as_deref(), Some("go build ./..."));

    // Get by ID
    assert!(registry.get("web-admin").is_some());
    assert!(registry.get("nonexistent").is_none());

    // Switch to nonexistent project fails
    assert!(registry.set_active("nonexistent").is_err());
}

#[test]
fn integration_default_branch_applied() {
    let projects = load_test_projects();
    for project in &projects {
        assert_eq!(project.default_branch, "main");
    }
}
