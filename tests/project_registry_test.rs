use codeclaw::config::ProjectConfig;
use codeclaw::project::ProjectRegistry;

#[test]
fn test_parse_project_config() {
    let config_str = r#"
        [[projects]]
        id = "web-admin"
        name = "Admin Dashboard"
        local_path = "/tmp/test-project"
        tech_stack = ["typescript", "react"]
        default_branch = "main"
        build_cmd = "npm run build"
        test_cmd = "npm test"
        dev_cmd = "npm run dev"
        browser_base_url = "http://localhost:3000"
    "#;

    #[derive(serde::Deserialize)]
    struct Wrapper {
        projects: Vec<ProjectConfig>,
    }
    let wrapper: Wrapper = toml::from_str(config_str).unwrap();
    let projects = wrapper.projects;

    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].id, "web-admin");
    assert_eq!(projects[0].name, "Admin Dashboard");
    assert_eq!(projects[0].local_path, "/tmp/test-project");
    assert_eq!(projects[0].tech_stack, vec!["typescript", "react"]);
    assert_eq!(projects[0].default_branch, "main");
    assert_eq!(projects[0].build_cmd.as_deref(), Some("npm run build"));
    assert_eq!(projects[0].test_cmd.as_deref(), Some("npm test"));
    assert_eq!(projects[0].dev_cmd.as_deref(), Some("npm run dev"));
    assert_eq!(
        projects[0].browser_base_url.as_deref(),
        Some("http://localhost:3000")
    );
}

#[test]
fn test_project_registry_switch_active() {
    let mut registry = ProjectRegistry::new(vec![
        ProjectConfig {
            id: "a".into(),
            name: "A".into(),
            local_path: "/tmp/a".into(),
            ..Default::default()
        },
        ProjectConfig {
            id: "b".into(),
            name: "B".into(),
            local_path: "/tmp/b".into(),
            ..Default::default()
        },
    ]);

    assert!(registry.active().is_none());
    registry.set_active("a").unwrap();
    assert_eq!(registry.active().unwrap().id, "a");
    registry.set_active("b").unwrap();
    assert_eq!(registry.active().unwrap().id, "b");
    assert!(registry.set_active("nonexistent").is_err());
}

#[test]
fn test_project_registry_list_and_get() {
    let registry = ProjectRegistry::new(vec![ProjectConfig {
        id: "x".into(),
        name: "X".into(),
        local_path: "/tmp/x".into(),
        ..Default::default()
    }]);

    assert_eq!(registry.list().len(), 1);
    assert!(registry.get("x").is_some());
    assert!(registry.get("y").is_none());
}
