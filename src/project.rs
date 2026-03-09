use crate::config::ProjectConfig;
use anyhow::{anyhow, Result};

/// Registry that holds all configured projects and tracks the currently active one.
pub struct ProjectRegistry {
    projects: Vec<ProjectConfig>,
    active_id: Option<String>,
}

impl ProjectRegistry {
    /// Create a new registry from a list of project configurations.
    pub fn new(projects: Vec<ProjectConfig>) -> Self {
        Self {
            projects,
            active_id: None,
        }
    }

    /// Return the currently active project, if one has been set.
    pub fn active(&self) -> Option<&ProjectConfig> {
        self.active_id
            .as_ref()
            .and_then(|id| self.projects.iter().find(|p| p.id == *id))
    }

    /// Switch the active project by ID.  Returns an error if the ID is not found.
    pub fn set_active(&mut self, id: &str) -> Result<()> {
        if self.projects.iter().any(|p| p.id == id) {
            self.active_id = Some(id.to_string());
            Ok(())
        } else {
            Err(anyhow!("Project '{}' not found in registry", id))
        }
    }

    /// List all registered projects.
    pub fn list(&self) -> &[ProjectConfig] {
        &self.projects
    }

    /// Look up a project by ID.
    pub fn get(&self, id: &str) -> Option<&ProjectConfig> {
        self.projects.iter().find(|p| p.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_project(id: &str, name: &str) -> ProjectConfig {
        ProjectConfig {
            id: id.into(),
            name: name.into(),
            local_path: format!("/tmp/{id}"),
            ..Default::default()
        }
    }

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
    fn test_registry_switch_active() {
        let mut registry = ProjectRegistry::new(vec![
            make_project("a", "A"),
            make_project("b", "B"),
        ]);

        assert!(registry.active().is_none());
        registry.set_active("a").unwrap();
        assert_eq!(registry.active().unwrap().id, "a");
        registry.set_active("b").unwrap();
        assert_eq!(registry.active().unwrap().id, "b");
        assert!(registry.set_active("nonexistent").is_err());
    }

    #[test]
    fn test_registry_list_and_get() {
        let registry = ProjectRegistry::new(vec![make_project("x", "X")]);

        assert_eq!(registry.list().len(), 1);
        assert!(registry.get("x").is_some());
        assert!(registry.get("y").is_none());
    }
}
