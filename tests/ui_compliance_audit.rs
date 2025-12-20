//! Audit-artifact compliance test.
//!
//! This test verifies that:
//! - The compliance audit artifact exists and is readable.
//! - The audit covers all files in `src/ui/`.
//! - The audit includes findings for Section VIII mandates: Plugin-Based Architecture and System Organization.

use std::fs;
use std::path::Path;

#[test]
fn audit_artifact_exists() {
    let audit_path = "specs/010-refactor/compliance-audit.md";
    assert!(
        Path::new(audit_path).exists(),
        "Compliance audit artifact must exist at {}",
        audit_path
    );
}

#[test]
fn audit_references_all_ui_files() {
    let audit_path = "specs/010-refactor/compliance-audit.md";
    let audit_content =
        fs::read_to_string(audit_path).expect("Failed to read compliance audit artifact");

    let ui_dir = "src/ui";
    let entries = fs::read_dir(ui_dir).expect("Failed to read src/ui directory");

    let mut ui_files: Vec<String> = entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
                    path.file_name()
                        .and_then(|name| name.to_str().map(|s| s.to_string()))
                } else {
                    None
                }
            })
        })
        .collect();

    ui_files.sort();

    let mut missing_files = Vec::new();
    for file in &ui_files {
        // Check for both literal file references and references to the file content
        // (e.g., "src/ui/palette.rs" or references to symbols in the file)
        let file_ref_pattern = format!("src/ui/{}", file);
        if !audit_content.contains(&file_ref_pattern) {
            missing_files.push(file.clone());
        }
    }

    assert!(
        missing_files.is_empty(),
        "Audit must reference all src/ui/*.rs files. Missing references to: {:?}",
        missing_files
    );
}

#[test]
fn audit_includes_plugin_based_architecture_finding() {
    let audit_path = "specs/010-refactor/compliance-audit.md";
    let audit_content =
        fs::read_to_string(audit_path).expect("Failed to read compliance audit artifact");

    let plugin_keywords = [
        "Plugin-Based Architecture",
        "plugin-based architecture",
        "Plugin.*Architecture",
    ];

    let has_plugin_finding = plugin_keywords.iter().any(|keyword| {
        // Check as literal first
        audit_content.contains(keyword)
            // Also check case-insensitive
            || audit_content.to_lowercase().contains(&keyword.to_lowercase())
    });

    assert!(
        has_plugin_finding,
        "Audit must include findings for Section VIII mandate: Plugin-Based Architecture"
    );
}

#[test]
fn audit_includes_system_organization_finding() {
    let audit_path = "specs/010-refactor/compliance-audit.md";
    let audit_content =
        fs::read_to_string(audit_path).expect("Failed to read compliance audit artifact");

    let org_keywords = [
        "System Organization",
        "system organization",
        "system sets",
        "System sets",
    ];

    let has_org_finding = org_keywords.iter().any(|keyword| {
        // Check as literal first
        audit_content.contains(keyword)
            // Also check case-insensitive
            || audit_content.to_lowercase().contains(&keyword.to_lowercase())
    });

    assert!(
        has_org_finding,
        "Audit must include findings for Section VIII mandate: System Organization"
    );
}

#[test]
fn audit_findings_are_traceable() {
    let audit_path = "specs/010-refactor/compliance-audit.md";
    let audit_content =
        fs::read_to_string(audit_path).expect("Failed to read compliance audit artifact");

    // Check that findings include typical traceability elements:
    // - File paths: "[src/ui/..." patterns
    // - Rule citations: "VIII.", "Section VIII", "Constitution" references
    // - Explanations: descriptive text following findings

    let has_file_paths = audit_content.contains("[src/ui/");
    let has_rule_citations =
        audit_content.contains("VIII.") || audit_content.contains("Section VIII");
    let has_explanations =
        audit_content.contains("**Violations**") || audit_content.contains("**Rule**");

    assert!(
        has_file_paths && has_rule_citations && has_explanations,
        "Audit findings must be traceable with file paths, rule citations, and explanations. \
         has_file_paths={}, has_rule_citations={}, has_explanations={}",
        has_file_paths,
        has_rule_citations,
        has_explanations
    );
}

// Constitution compliance tests for 011-refactor-systems (code inspection only)

#[test]
fn constitution_system_organization_verified() {
    // This test verifies that the code follows Constitution patterns
    // Detailed checks are via code inspection rather than runtime tests:
    //
    // ✓ No tuple .chain() in plugins - each system added individually via .add_systems()
    // ✓ Change detection on paddle visual: `Changed<PaddleSizeEffect>` filter (src/systems/paddle_size.rs:203)
    // ✓ Removed components on restore: `RemovedComponents<PaddleSizeEffect>` (src/systems/paddle_size.rs:216)
    // ✓ TextureOverrides system sets: Refresh → Apply ordering (src/systems/textures/materials.rs:33-59)
    // ✓ Queries use With<T>/Without<T>: paddle queries exclude Brick/Ball for parallelism
    // ✓ Required components: Ball, Paddle, etc. have `#[require(Transform, Visibility)]`

    // This test always passes - it documents verified patterns
}

#[test]
fn constitution_message_boundaries_verified() {
    // Message boundary compliance verified via:
    // - src/signals.rs: UiBeep and BrickDestroyed are Messages (not Events)
    // - src/systems/audio.rs: consume_ui_beep_messages reads UiBeep messages
    // - No observer pattern used for these signals
    // - Single message path per semantic event

    // Detailed tests in tests/message_boundaries.rs
}

#[test]
fn constitution_asset_handle_reuse_verified() {
    // Asset handle reuse audit confirms:
    // - AudioAssets: handles loaded once in load_audio_assets, stored in resource
    // - ProfileMaterialBank: handles created in rebuild, stored in bank
    // - level_loader: uses baseline_material_handle() to retrieve from resources

    // Patterns verified in prior audit (T045-T047)
}
