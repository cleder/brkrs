#!/usr/bin/env python3
"""Verify level files have exactly 20 rows with 20 columns each."""

import sys
from pathlib import Path


def verify_level_dimensions(level_path):
    """Verify a level file has 20x20 dimensions."""
    content = level_path.read_text()

    # Find the matrix section
    if "matrix: [" not in content:
        return None, None, "No matrix found"

    # Extract matrix content between "matrix: [" and the closing "],"
    matrix_start = content.find("matrix: [")
    matrix_section = content[matrix_start:]

    # Count rows - each row starts with "    [" (4 spaces + bracket)
    rows = matrix_section.count("\n    [")

    # Extract first row to count columns
    first_row_start = matrix_section.find("[", matrix_section.find("[") + 1)
    first_row_end = matrix_section.find("]", first_row_start)
    first_row = matrix_section[first_row_start:first_row_end]

    # Count comma-separated values
    cols = first_row.count(",") + 1 if first_row.strip() else 0

    return rows, cols, None


def main():
    levels_dir = Path("assets/levels")
    all_pass = True

    print("Verifying level dimensions...\n")

    for level_file in sorted(levels_dir.glob("level_*.ron")):
        print(f"Checking {level_file}:")

        rows, cols, error = verify_level_dimensions(level_file)

        if error:
            print(f"  ❌ ERROR: {error}")
            all_pass = False
        else:
            print(f"  Rows: {rows}")
            print(f"  Columns: {cols}")

            if rows == 20 and cols == 20:
                print("  ✅ PASS")
            else:
                print(f"  ❌ FAIL - Expected 20x20")
                all_pass = False

        print()

    print("Done.")
    sys.exit(0 if all_pass else 1)


if __name__ == "__main__":
    main()
