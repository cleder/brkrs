#!/usr/bin/env python3
"""Simple static analysis to detect MessageWriter<T> usage alongside immediate
side-effects (commands.spawn, audio.play, commands.entity, etc.) which
likely indicates misuse of buffered messages for immediate logic.

This is intentionally conservative (may produce false positives) and is
meant to catch likely policy violations for human review.

Exit codes:
 - 0: no violations found
 - 2: violations found
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SRC_GLOBS = ["src/**/*.rs", "specs/**/*.rs", "tests/**/*.rs"]

# Patterns to treat as "immediate side-effects" (indicative of logic that
# should probably use an observer/Trigger<T> instead of MessageWriter).
SIDE_EFFECT_PATTERNS = [
    re.compile(r"\bcommands\.spawn\b"),
    re.compile(r"\bcommands\.entity\b"),
    re.compile(r"\bcommands\.insert_resource\b"),
    re.compile(r"\bcommands\.remove_resource\b"),
    re.compile(r"\bcommands\.despawn\b"),
    re.compile(r"\basset_server\.load\s*\(") ,
    re.compile(r"\b\.play\s*\("),
    re.compile(r"\baudio\.play\b"),
    re.compile(r"\bcommands\.spawn_batch\b"),
]

MSG_WRITER_RE = re.compile(r"MessageWriter\s*<")


def find_fn_start(lines, i):
    # Scan upwards to find nearest line that looks like a fn signature
    for k in range(i, -1, -1):
        if re.search(r"\bfn\s+[_A-Za-z][_0-9A-Za-z]*", lines[k]):
            return k
    return None


def find_fn_end(lines, start_idx):
    # Find the end of the function by balancing braces starting from the
    # first '{' after the signature line. Returns inclusive end index.
    brace_count = 0
    started = False
    for k in range(start_idx, len(lines)):
        line = lines[k]
        for ch in line:
            if ch == '{':
                brace_count += 1
                started = True
            elif ch == '}':
                brace_count -= 1
        if started and brace_count == 0:
            return k
    return None


def extract_fn_name(signature_line):
    m = re.search(r"\bfn\s+([_A-Za-z][_0-9A-Za-z]*)", signature_line)
    return m.group(1) if m else "<unknown>"


def scan_file(path):
    text = path.read_text()
    lines = text.splitlines()
    violations = []

    for i, line in enumerate(lines):
        if MSG_WRITER_RE.search(line):
            # Find enclosing function
            start = find_fn_start(lines, i)
            if start is None:
                # top-level; treat whole file as context
                start = 0
            end = find_fn_end(lines, start)
            if end is None:
                # couldn't find function end; skip conservative
                context = "\n".join(lines[start:start + 40])
                fn_name = extract_fn_name(lines[start]) if start < len(lines) else "<file>"
            else:
                context = "\n".join(lines[start:end + 1])
                fn_name = extract_fn_name(lines[start])

            for pat in SIDE_EFFECT_PATTERNS:
                if pat.search(context):
                    # find line numbers for snippet
                    violations.append((path, i + 1, fn_name, pat.pattern))
                    break

    return violations


def main():
    files = []
    for g in SRC_GLOBS:
        for p in ROOT.glob(g):
            if p.is_file():
                files.append(p)

    total_violations = []
    for f in files:
        v = scan_file(f)
        total_violations.extend(v)

    if not total_violations:
        print("Message/Event lint: no problems found.")
        return 0

    print("Message/Event lint: potential misuse found:")
    for path, line_num, fn_name, pattern in total_violations:
        print(f"- {path}:{line_num} in fn `{fn_name}` — matched side-effect pattern: {pattern}")

    print("Guidance: Systems that use `MessageWriter<T>` are buffered and should NOT be used for immediate side-effects.")
    print("Consider using an observer pattern (Trigger<T> or `commands.observe()`) or move the immediate effect into a separate observer system.")
    return 2


if __name__ == '__main__':
    rc = main()
    sys.exit(rc)
