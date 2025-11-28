use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use ron::de::from_str;

#[derive(serde::Deserialize, Debug, PartialEq, Eq)]
struct LevelDefinition {
    number: u32,
    #[serde(default)]
    matrix: Vec<Vec<u8>>,
}

fn replace_3_with_20(mut def: LevelDefinition) -> LevelDefinition {
    for row in def.matrix.iter_mut() {
        for v in row.iter_mut() {
            if *v == 3 {
                *v = 20;
            }
        }
    }
    def
}

#[test]
fn migration_preserves_parity_for_level_001() -> Result<(), Box<dyn Error>> {
    // locate the sample level
    let sample = PathBuf::from("assets/levels/level_001.ron");
    assert!(
        sample.exists(),
        "expected sample level to exist: {:?}",
        sample
    );

    // create a temporary copy to run the migration on
    let tmp_dir = PathBuf::from("target/test-migrations");
    fs::create_dir_all(&tmp_dir)?;
    let tmp = tmp_dir.join("level_001.tmp.ron");
    fs::copy(&sample, &tmp)?;

    // Build the migration binary first (manifest path to the tool crate)
    let status = Command::new("cargo")
        .args([
            "build",
            "--manifest-path",
            "tools/migrate-level-indices/Cargo.toml",
        ])
        .status()?;
    assert!(status.success(), "failed to build migration tool");

    // Run the migration tool on the temporary copy
    let run_status = Command::new("cargo")
        .args([
            "run",
            "--manifest-path",
            "tools/migrate-level-indices/Cargo.toml",
            "--",
            "--backup",
            "--from",
            "3",
            "--to",
            "20",
            tmp.to_str().unwrap(),
        ])
        .status()?;
    assert!(run_status.success(), "migration tool failed to run");

    // Read the original and migrated files
    let original = fs::read_to_string(&sample)?;
    let migrated = fs::read_to_string(&tmp)?;

    // Parse both as LevelDefinition
    let def_orig: LevelDefinition = from_str(&original)?;
    let def_migrated: LevelDefinition = from_str(&migrated)?;

    // In the original, replacing '3' with '20' should produce the migrated structure
    let def_expected = replace_3_with_20(def_orig);
    assert_eq!(
        def_expected, def_migrated,
        "Migration should preserve layout and convert 3->20"
    );

    // Ensure no standalone '3' tokens remain in the migrated text
    assert!(
        !migrated.contains(" 3"),
        "migrated file must not contain standalone tile 3"
    );

    // clean up
    let _ = fs::remove_file(&tmp);
    let _ = fs::remove_file(tmp.with_extension("tmp.ron.bak"));

    Ok(())
}
