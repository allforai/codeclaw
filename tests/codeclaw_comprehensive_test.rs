//! Comprehensive test suite for CodeClaw extensions.
//!
//! Tests cover:
//! 1. ProjectConfig: parsing, defaults, edge cases
//! 2. ProjectRegistry: lifecycle, edge cases, error handling
//! 3. Config integration: projects field on main Config struct
//! 4. SOP file validation: actual SOP templates load correctly from disk
//! 5. Skills file validation: skill manifests exist and parse
//! 6. Rebrand verification: binary name and config paths use "codeclaw"

use zeroclaw::config::{Config, ProjectConfig};
use zeroclaw::project::ProjectRegistry;

// ═══════════════════════════════════════════════════════════════
// 1. ProjectConfig parsing
// ═══════════════════════════════════════════════════════════════

#[test]
fn project_config_minimal_fields() {
    let toml_str = r#"
        [[projects]]
        id = "minimal"
        name = "Minimal"
        local_path = "/tmp/minimal"
    "#;

    #[derive(serde::Deserialize)]
    struct W {
        projects: Vec<ProjectConfig>,
    }
    let w: W = toml::from_str(toml_str).unwrap();
    let p = &w.projects[0];

    assert_eq!(p.id, "minimal");
    assert_eq!(p.default_branch, "main"); // default
    assert!(p.repo_url.is_none());
    assert!(p.build_cmd.is_none());
    assert!(p.test_cmd.is_none());
    assert!(p.dev_cmd.is_none());
    assert!(p.browser_base_url.is_none());
    assert!(p.tech_stack.is_empty());
    assert!(p.preferred_agents.is_empty());
}

#[test]
fn project_config_all_fields() {
    let toml_str = r#"
        [[projects]]
        id = "full"
        name = "Full Project"
        repo_url = "git@github.com:org/repo.git"
        local_path = "/home/dev/full"
        tech_stack = ["rust", "typescript", "docker"]
        default_branch = "develop"
        build_cmd = "cargo build"
        test_cmd = "cargo test"
        dev_cmd = "cargo run"
        browser_base_url = "http://localhost:8080"
        preferred_agents = ["claude_code", "codex"]
    "#;

    #[derive(serde::Deserialize)]
    struct W {
        projects: Vec<ProjectConfig>,
    }
    let w: W = toml::from_str(toml_str).unwrap();
    let p = &w.projects[0];

    assert_eq!(p.id, "full");
    assert_eq!(p.repo_url.as_deref(), Some("git@github.com:org/repo.git"));
    assert_eq!(p.default_branch, "develop");
    assert_eq!(p.tech_stack.len(), 3);
    assert_eq!(p.preferred_agents, vec!["claude_code", "codex"]);
}

#[test]
fn project_config_multiple_projects() {
    let toml_str = r#"
        [[projects]]
        id = "alpha"
        name = "Alpha"
        local_path = "/tmp/alpha"

        [[projects]]
        id = "beta"
        name = "Beta"
        local_path = "/tmp/beta"
        tech_stack = ["python"]

        [[projects]]
        id = "gamma"
        name = "Gamma"
        local_path = "/tmp/gamma"
    "#;

    #[derive(serde::Deserialize)]
    struct W {
        projects: Vec<ProjectConfig>,
    }
    let w: W = toml::from_str(toml_str).unwrap();
    assert_eq!(w.projects.len(), 3);
    assert_eq!(w.projects[0].id, "alpha");
    assert_eq!(w.projects[1].id, "beta");
    assert_eq!(w.projects[2].id, "gamma");
}

#[test]
fn project_config_empty_list() {
    let toml_str = "";

    #[derive(serde::Deserialize)]
    struct W {
        #[serde(default)]
        projects: Vec<ProjectConfig>,
    }
    let w: W = toml::from_str(toml_str).unwrap();
    assert!(w.projects.is_empty());
}

#[test]
fn project_config_default_trait() {
    let p = ProjectConfig::default();
    assert!(p.id.is_empty());
    assert!(p.name.is_empty());
    assert!(p.local_path.is_empty());
    assert_eq!(p.default_branch, "main");
    assert!(p.tech_stack.is_empty());
    assert!(p.preferred_agents.is_empty());
}

// ═══════════════════════════════════════════════════════════════
// 2. ProjectRegistry lifecycle and edge cases
// ═══════════════════════════════════════════════════════════════

fn make_project(id: &str) -> ProjectConfig {
    ProjectConfig {
        id: id.into(),
        name: format!("Project {id}"),
        local_path: format!("/tmp/{id}"),
        ..Default::default()
    }
}

#[test]
fn registry_empty() {
    let registry = ProjectRegistry::new(vec![]);
    assert!(registry.active().is_none());
    assert!(registry.list().is_empty());
    assert!(registry.get("anything").is_none());
}

#[test]
fn registry_set_active_empty_fails() {
    let mut registry = ProjectRegistry::new(vec![]);
    assert!(registry.set_active("x").is_err());
}

#[test]
fn registry_single_project() {
    let mut registry = ProjectRegistry::new(vec![make_project("solo")]);

    assert_eq!(registry.list().len(), 1);
    assert!(registry.active().is_none());

    registry.set_active("solo").unwrap();
    assert_eq!(registry.active().unwrap().id, "solo");
    assert_eq!(registry.active().unwrap().name, "Project solo");
}

#[test]
fn registry_switch_between_projects() {
    let mut registry = ProjectRegistry::new(vec![
        make_project("a"),
        make_project("b"),
        make_project("c"),
    ]);

    // Switch through all projects
    for id in &["a", "b", "c", "a", "c", "b"] {
        registry.set_active(id).unwrap();
        assert_eq!(registry.active().unwrap().id, *id);
    }
}

#[test]
fn registry_get_by_id() {
    let registry = ProjectRegistry::new(vec![
        make_project("x"),
        make_project("y"),
    ]);

    assert_eq!(registry.get("x").unwrap().id, "x");
    assert_eq!(registry.get("y").unwrap().id, "y");
    assert!(registry.get("z").is_none());
}

#[test]
fn registry_nonexistent_project_error_message() {
    let mut registry = ProjectRegistry::new(vec![make_project("real")]);
    let err = registry.set_active("fake").unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("fake"), "Error should mention the project ID: {msg}");
}

#[test]
fn registry_list_preserves_order() {
    let registry = ProjectRegistry::new(vec![
        make_project("c"),
        make_project("a"),
        make_project("b"),
    ]);

    let ids: Vec<&str> = registry.list().iter().map(|p| p.id.as_str()).collect();
    assert_eq!(ids, vec!["c", "a", "b"]);
}

// ═══════════════════════════════════════════════════════════════
// 3. Config struct integration — projects field
// ═══════════════════════════════════════════════════════════════

#[test]
fn config_default_has_empty_projects() {
    let config = Config::default();
    assert!(config.projects.is_empty());
}

// ═══════════════════════════════════════════════════════════════
// 4. SOP template file validation (load from disk)
// ═══════════════════════════════════════════════════════════════

/// Verify that each SOP directory contains valid SOP.toml and SOP.md files.
#[test]
fn sop_templates_exist_and_parse() {
    let sops_dir = std::path::Path::new("sops");
    assert!(sops_dir.exists(), "sops/ directory must exist");

    let expected_sops = ["debug-workflow", "feature-workflow", "regression-workflow"];

    for sop_name in &expected_sops {
        let sop_dir = sops_dir.join(sop_name);
        assert!(sop_dir.exists(), "SOP directory {sop_name} must exist");

        // SOP.toml must exist and be valid TOML
        let toml_path = sop_dir.join("SOP.toml");
        assert!(toml_path.exists(), "{sop_name}/SOP.toml must exist");
        let toml_content = std::fs::read_to_string(&toml_path)
            .unwrap_or_else(|e| panic!("Failed to read {sop_name}/SOP.toml: {e}"));
        let toml_val: toml::Value = toml::from_str(&toml_content)
            .unwrap_or_else(|e| panic!("{sop_name}/SOP.toml is invalid TOML: {e}"));

        // Must have [sop] table with name and description
        let sop_table = toml_val.get("sop").expect(&format!("{sop_name}/SOP.toml must have [sop] table"));
        assert!(
            sop_table.get("name").is_some(),
            "{sop_name}/SOP.toml must have sop.name"
        );
        assert!(
            sop_table.get("description").is_some(),
            "{sop_name}/SOP.toml must have sop.description"
        );

        // SOP.md must exist and contain ## Steps
        let md_path = sop_dir.join("SOP.md");
        assert!(md_path.exists(), "{sop_name}/SOP.md must exist");
        let md_content = std::fs::read_to_string(&md_path)
            .unwrap_or_else(|e| panic!("Failed to read {sop_name}/SOP.md: {e}"));
        assert!(
            md_content.contains("## Steps"),
            "{sop_name}/SOP.md must contain '## Steps' heading"
        );
    }
}

/// Verify SOP.toml trigger types are valid.
#[test]
fn sop_templates_have_valid_triggers() {
    let sops_dir = std::path::Path::new("sops");

    for entry in std::fs::read_dir(sops_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let toml_path = path.join("SOP.toml");
        if !toml_path.exists() {
            continue;
        }

        let toml_content = std::fs::read_to_string(&toml_path).unwrap();
        let toml_val: toml::Value = toml::from_str(&toml_content).unwrap();

        if let Some(triggers) = toml_val.get("triggers").and_then(|t| t.as_array()) {
            for trigger in triggers {
                let trigger_type = trigger
                    .get("type")
                    .and_then(|t| t.as_str())
                    .expect("Each trigger must have a type");
                assert!(
                    ["manual", "webhook", "cron", "mqtt", "peripheral"].contains(&trigger_type),
                    "Invalid trigger type '{trigger_type}' in {}",
                    path.display()
                );
            }
        }
    }
}

/// Verify SOP.md steps are numbered sequentially.
#[test]
fn sop_templates_have_numbered_steps() {
    let sops_dir = std::path::Path::new("sops");

    for entry in std::fs::read_dir(sops_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let md_path = path.join("SOP.md");
        if !md_path.exists() {
            continue;
        }

        let content = std::fs::read_to_string(&md_path).unwrap();
        let mut in_steps = false;
        let mut step_count = 0u32;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.eq_ignore_ascii_case("## steps") || trimmed.eq_ignore_ascii_case("## Steps")
            {
                in_steps = true;
                continue;
            }
            if in_steps && trimmed.starts_with("## ") {
                break;
            }
            if in_steps {
                // Check for numbered items like "1. ", "2. "
                if let Some(dot_pos) = trimmed.find(". ") {
                    let prefix = &trimmed[..dot_pos];
                    if prefix.chars().all(|c| c.is_ascii_digit()) && !prefix.is_empty() {
                        step_count += 1;
                    }
                }
            }
        }

        assert!(
            step_count >= 1,
            "SOP {} must have at least 1 numbered step, found {}",
            path.display(),
            step_count
        );
    }
}

// ═══════════════════════════════════════════════════════════════
// 5. Skills file validation
// ═══════════════════════════════════════════════════════════════

#[test]
fn skill_manifests_exist_and_parse() {
    let skills_dir = std::path::Path::new("skills");
    assert!(skills_dir.exists(), "skills/ directory must exist");

    let expected_skills = ["code-review", "test-verify", "deploy-check", "project-switch"];

    for skill_name in &expected_skills {
        let skill_dir = skills_dir.join(skill_name);
        assert!(skill_dir.exists(), "Skill directory {skill_name} must exist");

        // SKILL.toml must exist and be valid TOML
        let toml_path = skill_dir.join("SKILL.toml");
        assert!(toml_path.exists(), "{skill_name}/SKILL.toml must exist");
        let toml_content = std::fs::read_to_string(&toml_path)
            .unwrap_or_else(|e| panic!("Failed to read {skill_name}/SKILL.toml: {e}"));
        let toml_val: toml::Value = toml::from_str(&toml_content)
            .unwrap_or_else(|e| panic!("{skill_name}/SKILL.toml is invalid TOML: {e}"));

        // Must have [skill] table with name
        let skill_table = toml_val
            .get("skill")
            .expect(&format!("{skill_name}/SKILL.toml must have [skill] table"));
        assert!(
            skill_table.get("name").is_some(),
            "{skill_name}/SKILL.toml must have skill.name"
        );

        // SKILL.md must exist
        let md_path = skill_dir.join("SKILL.md");
        assert!(md_path.exists(), "{skill_name}/SKILL.md must exist");
        let md_content = std::fs::read_to_string(&md_path).unwrap();
        assert!(
            !md_content.trim().is_empty(),
            "{skill_name}/SKILL.md must not be empty"
        );
    }
}

// ═══════════════════════════════════════════════════════════════
// 6. Config file validation — codeclaw.toml loads correctly
// ═══════════════════════════════════════════════════════════════

#[test]
fn codeclaw_config_file_is_valid_toml() {
    let config_path = std::path::Path::new("config/codeclaw.toml");
    assert!(config_path.exists(), "config/codeclaw.toml must exist");

    let content = std::fs::read_to_string(config_path).unwrap();
    let val: toml::Value = toml::from_str(&content)
        .unwrap_or_else(|e| panic!("config/codeclaw.toml is invalid TOML: {e}"));

    // Should have security section
    assert!(
        val.get("security").is_some(),
        "Config must have [security] section"
    );

    // Should have autonomy section
    assert!(
        val.get("autonomy").is_some(),
        "Config must have [autonomy] section"
    );
}

// ═══════════════════════════════════════════════════════════════
// 7. Integration: fixture → registry → workflow simulation
// ═══════════════════════════════════════════════════════════════

#[test]
fn integration_full_project_lifecycle() {
    // 1. Parse fixture
    let content =
        std::fs::read_to_string("tests/fixtures/codeclaw-test.toml").expect("fixture must exist");

    #[derive(serde::Deserialize)]
    struct Fixture {
        projects: Vec<ProjectConfig>,
    }
    let fixture: Fixture = toml::from_str(&content).unwrap();

    // 2. Create registry
    let mut registry = ProjectRegistry::new(fixture.projects);
    assert_eq!(registry.list().len(), 2);

    // 3. Switch to web-admin, simulate gathering project info for delegation
    registry.set_active("web-admin").unwrap();
    let active = registry.active().unwrap();
    assert_eq!(active.test_cmd.as_deref(), Some("npm test"));
    assert_eq!(
        active.browser_base_url.as_deref(),
        Some("http://localhost:3000")
    );
    assert_eq!(active.preferred_agents, vec!["claude_code"]);

    // 4. Switch to api-server
    registry.set_active("api-server").unwrap();
    let active = registry.active().unwrap();
    assert_eq!(active.test_cmd.as_deref(), Some("go test ./..."));
    assert!(active.browser_base_url.is_none());

    // 5. Lookup by ID still works while another is active
    let web = registry.get("web-admin").unwrap();
    assert_eq!(web.name, "Admin Dashboard");
}

// ═══════════════════════════════════════════════════════════════
// 8. Rebrand verification
// ═══════════════════════════════════════════════════════════════

#[test]
fn cargo_toml_uses_codeclaw_binary_name() {
    let content = std::fs::read_to_string("Cargo.toml").unwrap();
    let val: toml::Value = toml::from_str(&content).unwrap();

    // [[bin]] name should be "codeclaw"
    let bins = val.get("bin").and_then(|b| b.as_array()).unwrap();
    let bin_name = bins[0].get("name").and_then(|n| n.as_str()).unwrap();
    assert_eq!(bin_name, "codeclaw");

    // [package] name should be "codeclaw"
    let pkg_name = val["package"]["name"].as_str().unwrap();
    assert_eq!(pkg_name, "codeclaw");
}
