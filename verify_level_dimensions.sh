#!/usr/bin/env zsh
# Verify level files have exactly 20 rows with 20 columns each

echo "Verifying level dimensions..."
echo ""

for level in assets/levels/level_*.ron; do
    echo "Checking $level:"

    # Extract matrix content and count rows
    rows=$(grep -oP '\[\s*\[' "$level" | wc -l)

    # Count columns in first row
    first_row=$(grep -m1 '\[' "$level" | grep -o '\[' | wc -l)
    cols=$((first_row - 1))  # Subtract 1 for outer bracket

    echo "  Rows: $rows"
    echo "  Columns: $cols"

    if [[ $rows -eq 20 ]] && [[ $cols -eq 20 ]]; then
        echo "  ✅ PASS"
    else
        echo "  ❌ FAIL - Expected 20x20"
    fi
    echo ""
done

echo "Done."
