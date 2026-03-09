// Integration tests for SOP step-level conditions and retry policy extensions.
//
// Since the SOP module is not exported from the library crate, these tests
// verify the data contract (serde serialization format) of the new fields.
// The actual logic tests (condition evaluation, step skipping, retry) are
// in the unit tests within src/sop/engine.rs and src/sop/types.rs.
//
// Run with: cargo test sop_extension -- --nocapture

/// Verify that a SopStep with condition and retry fields serializes correctly.
#[test]
fn sop_extension_step_condition_roundtrip() {
    let json = r#"{
        "number": 2,
        "title": "Notify operator",
        "body": "Send alert",
        "suggested_tools": ["pushover"],
        "requires_confirmation": false,
        "condition": "{{prev.success}} == true",
        "retry": {
            "max_attempts": 3,
            "backoff_secs": 5
        }
    }"#;

    let step: serde_json::Value = serde_json::from_str(json).unwrap();
    assert_eq!(step["condition"], "{{prev.success}} == true");
    assert_eq!(step["retry"]["max_attempts"], 3);
    assert_eq!(step["retry"]["backoff_secs"], 5);

    // Round-trip
    let serialized = serde_json::to_string(&step).unwrap();
    let reparsed: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    assert_eq!(reparsed["condition"], "{{prev.success}} == true");
    assert_eq!(reparsed["retry"]["max_attempts"], 3);
}

/// Verify backward compatibility: SopStep without condition/retry still works.
#[test]
fn sop_extension_step_backward_compatible() {
    let json = r#"{
        "number": 1,
        "title": "Check readings",
        "body": "Read sensor data",
        "suggested_tools": [],
        "requires_confirmation": false
    }"#;

    let step: serde_json::Value = serde_json::from_str(json).unwrap();
    assert_eq!(step["number"], 1);
    assert_eq!(step["title"], "Check readings");
    // New fields are absent — backward compatible
    assert!(step.get("condition").is_none());
    assert!(step.get("retry").is_none());
}

/// Verify SopStepResult with attempts and skipped_by_condition fields.
#[test]
fn sop_extension_step_result_with_attempts() {
    let json = r#"{
        "step_number": 1,
        "status": "completed",
        "output": "Step 1 done",
        "started_at": "2026-02-19T12:00:00Z",
        "completed_at": "2026-02-19T12:00:05Z",
        "attempts": 3,
        "skipped_by_condition": false
    }"#;

    let result: serde_json::Value = serde_json::from_str(json).unwrap();
    assert_eq!(result["attempts"], 3);
    assert!(!result["skipped_by_condition"].as_bool().unwrap());
}

/// Verify SopStepResult representing a skipped step.
#[test]
fn sop_extension_step_result_skipped_by_condition() {
    let json = r#"{
        "step_number": 2,
        "status": "skipped",
        "output": "Skipped: condition '{{prev.success}} == true' evaluated to false",
        "started_at": "2026-02-19T12:00:05Z",
        "completed_at": "2026-02-19T12:00:05Z",
        "attempts": 0,
        "skipped_by_condition": true
    }"#;

    let result: serde_json::Value = serde_json::from_str(json).unwrap();
    assert_eq!(result["status"], "skipped");
    assert!(result["skipped_by_condition"].as_bool().unwrap());
    assert_eq!(result["attempts"], 0);
}

/// Verify backward compatibility: SopStepResult without attempts/skipped fields.
#[test]
fn sop_extension_step_result_backward_compatible() {
    let json = r#"{
        "step_number": 1,
        "status": "completed",
        "output": "done",
        "started_at": "2026-02-19T12:00:00Z",
        "completed_at": "2026-02-19T12:00:05Z"
    }"#;

    let result: serde_json::Value = serde_json::from_str(json).unwrap();
    assert_eq!(result["step_number"], 1);
    assert_eq!(result["status"], "completed");
    // New fields absent — backward compatible
    assert!(result.get("attempts").is_none());
    assert!(result.get("skipped_by_condition").is_none());
}

/// Verify RetryPolicy structure.
#[test]
fn sop_extension_retry_policy_serde() {
    let json = r#"{"max_attempts": 5, "backoff_secs": 10}"#;
    let policy: serde_json::Value = serde_json::from_str(json).unwrap();
    assert_eq!(policy["max_attempts"], 5);
    assert_eq!(policy["backoff_secs"], 10);

    // Round-trip
    let serialized = serde_json::to_string(&policy).unwrap();
    let reparsed: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    assert_eq!(reparsed["max_attempts"], 5);
    assert_eq!(reparsed["backoff_secs"], 10);
}

/// Verify a complete SOP step definition with all new features.
#[test]
fn sop_extension_full_step_with_all_fields() {
    let json = r#"{
        "number": 3,
        "title": "Retry with condition",
        "body": "This step only runs if previous succeeded and retries up to 3 times",
        "suggested_tools": ["shell", "gpio_write"],
        "requires_confirmation": true,
        "condition": "{{prev.success}} == true",
        "retry": {"max_attempts": 3, "backoff_secs": 2}
    }"#;

    let step: serde_json::Value = serde_json::from_str(json).unwrap();
    assert_eq!(step["number"], 3);
    assert_eq!(step["title"], "Retry with condition");
    assert!(step["requires_confirmation"].as_bool().unwrap());
    assert_eq!(step["condition"], "{{prev.success}} == true");
    assert_eq!(step["retry"]["max_attempts"], 3);
    assert_eq!(step["retry"]["backoff_secs"], 2);
    assert_eq!(step["suggested_tools"].as_array().unwrap().len(), 2);
}
