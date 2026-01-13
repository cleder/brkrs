//! Level format normalization module
//!
//! Provides matrix normalization utilities for level loading,
//! ensuring level matrices conform to expected 20x20 dimensions.

use bevy::prelude::*;

/// Target dimensions for normalized level matrices.
pub const TARGET_ROWS: usize = 20;
pub const TARGET_COLS: usize = 20;

/// Canonical tile index used for simple (destructible) bricks when authoring new levels.
/// Historically the project used `3` for a simple brick; new levels should prefer `20`.
pub const SIMPLE_BRICK: u8 = 20;

/// Tile index reserved for indestructible bricks. Indestructible bricks collide and render
/// like regular bricks but do NOT count toward level completion.
pub const INDESTRUCTIBLE_BRICK: u8 = 90;

/// Multi-hit brick index 10: needs 1 more hit to become simple stone (index 20).
pub const MULTI_HIT_BRICK_1: u8 = 10;

/// Multi-hit brick index 11: needs 2 more hits to be destroyed.
pub const MULTI_HIT_BRICK_2: u8 = 11;

/// Multi-hit brick index 12: needs 3 more hits to be destroyed.
pub const MULTI_HIT_BRICK_3: u8 = 12;

/// Multi-hit brick index 13: needs 4 more hits to be destroyed (maximum durability).
pub const MULTI_HIT_BRICK_4: u8 = 13;

/// Extra Life brick index 41: awards +1 player life when destroyed, no points.
/// This brick is destructible (durability 1) and plays a unique destruction sound.
pub const EXTRA_LIFE_BRICK: u8 = 41;

/// Returns `true` if the given type ID represents a multi-hit brick (indices 10-13).
///
/// Multi-hit bricks require multiple ball collisions to destroy. Each hit decrements
/// the index by 1, until reaching index 10 which transitions to a simple stone (index 20).
///
/// # Examples
///
/// ```no_run
/// use brkrs::level_format::is_multi_hit_brick;
///
/// assert!(is_multi_hit_brick(10));  // MULTI_HIT_BRICK_1
/// assert!(is_multi_hit_brick(13));  // MULTI_HIT_BRICK_4
/// assert!(!is_multi_hit_brick(20)); // Simple stone
/// assert!(!is_multi_hit_brick(90)); // Indestructible
/// ```
#[inline]
pub fn is_multi_hit_brick(type_id: u8) -> bool {
    (MULTI_HIT_BRICK_1..=MULTI_HIT_BRICK_4).contains(&type_id)
}

/// Returns true if the brick type is paddle-destroyable (type 57).
///
/// Paddle-destroyable bricks are destroyed only by paddle contact,
/// not by ball collisions. The ball bounces off these bricks without
/// destroying them.
#[inline]
pub fn is_paddle_destroyable_brick(type_id: u8) -> bool {
    type_id == 57
}

/// Metrics collected during matrix normalization.
///
/// These metrics indicate how the input matrix was adjusted to fit
/// the target 20x20 dimensions.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NormalizationMetrics {
    /// Number of rows that were padded (added as empty rows).
    pub padded_rows: usize,
    /// Number of rows that were truncated (removed from end).
    pub truncated_rows: usize,
    /// Total number of columns padded across all rows (cumulative count).
    pub padded_cols: usize,
    /// Total number of columns truncated across all rows (cumulative count).
    pub truncated_cols: usize,
}

/// Result of normalizing a level matrix.
#[derive(Debug, Clone)]
pub struct NormalizationResult {
    /// The normalized 20x20 matrix.
    pub matrix: Vec<Vec<u8>>,
    /// Metrics describing the normalization operations performed.
    pub metrics: NormalizationMetrics,
}

/// Normalize a level matrix to 20x20 dimensions with padding/truncation.
///
/// Returns the normalized matrix along with metrics describing the
/// normalization operations performed. Logs warnings for dimension mismatches.
///
/// # Arguments
///
/// * `matrix` - The input matrix to normalize.
///
/// # Returns
///
/// A [`NormalizationResult`] containing the normalized 20x20 matrix and metrics.
pub fn normalize_matrix(mut matrix: Vec<Vec<u8>>) -> NormalizationResult {
    let mut metrics = NormalizationMetrics::default();

    let original_rows = matrix.len();
    let original_cols = matrix.first().map_or(0, |r| r.len());

    // Log warning if dimensions don't match
    if original_rows != TARGET_ROWS || original_cols != TARGET_COLS {
        warn!(
            "Level matrix wrong dimensions; expected 20x20, got {}x{}",
            original_rows, original_cols
        );
    }

    // Pad rows if needed
    if matrix.len() < TARGET_ROWS {
        metrics.padded_rows = TARGET_ROWS - matrix.len();
        while matrix.len() < TARGET_ROWS {
            matrix.push(vec![0; TARGET_COLS]);
        }
    }

    // Truncate rows if needed
    if matrix.len() > TARGET_ROWS {
        metrics.truncated_rows = matrix.len() - TARGET_ROWS;
        warn!(
            "Level matrix has {} rows; truncating to {}",
            matrix.len(),
            TARGET_ROWS
        );
        matrix.truncate(TARGET_ROWS);
    }

    // Pad/truncate columns
    for (i, row) in matrix.iter_mut().enumerate() {
        let original_row_len = row.len();

        // Pad columns if needed
        if row.len() < TARGET_COLS {
            metrics.padded_cols += TARGET_COLS - row.len();
            while row.len() < TARGET_COLS {
                row.push(0);
            }
        }

        // Truncate columns if needed
        if original_row_len > TARGET_COLS {
            metrics.truncated_cols += original_row_len - TARGET_COLS;
            warn!(
                "Row {} has {} columns; truncating to {}",
                i, original_row_len, TARGET_COLS
            );
            row.truncate(TARGET_COLS);
        }
    }

    NormalizationResult { matrix, metrics }
}

/// Convenience function to normalize matrix and return only the matrix.
///
/// This is a backward-compatible wrapper that calls [`normalize_matrix`]
/// and returns only the normalized matrix, discarding metrics.
pub fn normalize_matrix_simple(matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    normalize_matrix(matrix).matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_padding_rows_and_cols() {
        // 18 rows, each 19 cols -> should pad to 20x20
        let input = vec![vec![1u8; 19]; 18];
        let result = normalize_matrix(input.clone());
        assert_eq!(result.matrix.len(), 20, "row count padded to 20");
        for row in &result.matrix {
            assert_eq!(row.len(), 20, "col count padded to 20");
        }
        // Check metrics
        assert_eq!(result.metrics.padded_rows, 2);
        assert_eq!(result.metrics.truncated_rows, 0);
        // Each of 18 rows had 19 cols, needs 1 padding each = 18
        // Plus 2 new rows each get 20 cols (but they're created with 20, so no padding counted)
        assert_eq!(result.metrics.padded_cols, 18);
        assert_eq!(result.metrics.truncated_cols, 0);

        // Original data preserved in leading rows/cols
        for (r, row) in result.matrix.iter().enumerate().take(18) {
            for (c, &val) in row.iter().enumerate().take(19) {
                assert_eq!(val, 1, "row {r} col {c} should preserve value 1");
            }
        }
        // Padded cells zeroed
        for (r, row) in result.matrix.iter().enumerate().take(18) {
            assert_eq!(row[19], 0, "row {r} col 19 should be padded zero");
        }
        for (r, row) in result.matrix.iter().enumerate().skip(18).take(2) {
            for (c, &val) in row.iter().enumerate() {
                assert_eq!(val, 0, "row {r} col {c} should be padded zero");
            }
        }
    }

    #[test]
    fn normalize_truncates_rows_and_cols() {
        // 22 rows of 24 cols -> truncates to first 20 rows/cols
        let input = vec![vec![2u8; 24]; 22];
        let result = normalize_matrix(input.clone());
        assert_eq!(result.matrix.len(), 20);
        for row in &result.matrix {
            assert_eq!(row.len(), 20);
        }
        // Check metrics
        assert_eq!(result.metrics.padded_rows, 0);
        assert_eq!(result.metrics.truncated_rows, 2);
        assert_eq!(result.metrics.padded_cols, 0);
        // 20 rows each had 24 cols, truncated 4 each = 80
        assert_eq!(result.metrics.truncated_cols, 80);

        // Leading preserved
        for r in 0..20 {
            for c in 0..20 {
                assert_eq!(result.matrix[r][c], 2);
            }
        }
    }

    #[test]
    fn normalize_irregular_row_lengths() {
        // Mixture: some short, some long
        let mut input: Vec<Vec<u8>> = Vec::new();
        for i in 0..22 {
            // exceed target rows to test truncation
            let len = match i % 3 {
                0 => 10,
                1 => 25,
                _ => 20,
            }; // various lengths
            input.push(vec![3u8; len]);
        }
        let result = normalize_matrix(input);
        assert_eq!(result.matrix.len(), 20);
        for (r, row) in result.matrix.iter().enumerate() {
            assert_eq!(row.len(), 20, "row {} not normalized to 20 cols", r);
            let original_len = match r % 3 {
                0 => 10,
                1 => 25,
                _ => 20,
            };
            let preserved = original_len.min(20);
            for (c, &val) in row.iter().enumerate().take(preserved) {
                assert_eq!(val, 3, "row {r} col {c} should preserve value 3");
            }
            for (c, &val) in row.iter().enumerate().skip(preserved).take(20 - preserved) {
                assert_eq!(val, 0, "row {r} col {c} should be padded zero");
            }
        }
        // Check metrics
        assert_eq!(result.metrics.truncated_rows, 2);
        assert_eq!(result.metrics.padded_rows, 0);
        // Rows 0, 3, 6, 9, 12, 15, 18 have 10 cols -> need 10 padding each
        // That's 7 rows with 10 padding = 70
        assert_eq!(result.metrics.padded_cols, 70);
        // Rows 1, 4, 7, 10, 13, 16, 19 have 25 cols -> truncate 5 each
        // That's 7 rows with 5 truncation = 35
        assert_eq!(result.metrics.truncated_cols, 35);
    }

    #[test]
    fn normalize_empty_matrix() {
        let result = normalize_matrix(Vec::new());
        assert_eq!(result.matrix.len(), 20);
        for row in &result.matrix {
            assert_eq!(row.len(), 20);
            for c in row {
                assert_eq!(*c, 0);
            }
        }
        // Check metrics
        assert_eq!(result.metrics.padded_rows, 20);
        assert_eq!(result.metrics.truncated_rows, 0);
        assert_eq!(result.metrics.padded_cols, 0); // New rows created with 20 cols
        assert_eq!(result.metrics.truncated_cols, 0);
    }

    #[test]
    fn normalize_exact_dimensions_unchanged() {
        let mut input = vec![vec![5u8; 20]; 20];
        input[0][0] = 7;
        let result = normalize_matrix(input.clone());
        assert_eq!(result.matrix.len(), 20);
        for row in &result.matrix {
            assert_eq!(row.len(), 20);
        }
        assert_eq!(result.matrix[0][0], 7);
        // Ensure no unintended zeroing
        for (r, row) in result.matrix.iter().enumerate().take(20) {
            for (c, &val) in row.iter().enumerate().take(20) {
                assert_eq!(val, input[r][c]);
            }
        }
        // Check metrics - no changes
        assert_eq!(result.metrics.padded_rows, 0);
        assert_eq!(result.metrics.truncated_rows, 0);
        assert_eq!(result.metrics.padded_cols, 0);
        assert_eq!(result.metrics.truncated_cols, 0);
    }

    #[test]
    fn metrics_track_individual_row_padding() {
        // 20 rows but varying column counts
        let mut input: Vec<Vec<u8>> = Vec::new();
        for i in 0..20 {
            let len = if i < 10 { 15 } else { 20 }; // First 10 rows need 5 padding each
            input.push(vec![1u8; len]);
        }
        let result = normalize_matrix(input);
        assert_eq!(result.metrics.padded_rows, 0);
        assert_eq!(result.metrics.padded_cols, 50); // 10 rows * 5 padding
        assert_eq!(result.metrics.truncated_rows, 0);
        assert_eq!(result.metrics.truncated_cols, 0);
    }

    #[test]
    fn metrics_track_individual_row_truncation() {
        // 20 rows but varying column counts
        let mut input: Vec<Vec<u8>> = Vec::new();
        for i in 0..20 {
            let len = if i < 5 { 25 } else { 20 }; // First 5 rows need 5 truncation each
            input.push(vec![1u8; len]);
        }
        let result = normalize_matrix(input);
        assert_eq!(result.metrics.padded_rows, 0);
        assert_eq!(result.metrics.padded_cols, 0);
        assert_eq!(result.metrics.truncated_rows, 0);
        assert_eq!(result.metrics.truncated_cols, 25); // 5 rows * 5 truncation
    }
}
