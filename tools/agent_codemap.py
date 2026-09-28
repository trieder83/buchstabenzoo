#!/usr/bin/env python3
"""Generate .agent/CODEMAP.md: where things are, so agents read only the relevant lines.

- every source file (Rust crates, web host, Blender tools) with its line count and the
  first line of its module doc comment,
- for big files: their public items / impl blocks with line numbers,
- test files with the spec test IDs they cover,
- big specs with their section headings and line numbers (read with offset/limit).

    python3 tools/agent_codemap.py          # write .agent/CODEMAP.md
    python3 tools/agent_codemap.py --check  # exit 1 if stale
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / ".agent" / "CODEMAP.md"
BIG_CODE = 700       # lines: list items of files bigger than this
BIG_SPEC = 300       # lines: list headings of specs bigger than this
ITEM_RE = re.compile(
    r"^(pub(?:\([a-z]+\))? (?:fn|struct|enum|trait|const|mod|type) \w+|impl(?:<[^>]*>)? [\w:<>, ]+|"
    r"export (?:function|class|const|interface|type) \w+|def \w+|class \w+)")
ID_RE = re.compile(r"\b[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*-\d{3}\b")


def rel(p: Path) -> str:
    return p.relative_to(ROOT).as_posix()


def header(lines):
    for ln in lines[:15]:
        s = ln.strip()
        for pre in ("//!", "//", "#", '"""'):
            if s.startswith(pre) and not s.startswith("#!") and not s.startswith("#["):
                t = s[len(pre):].strip().strip('"')
                if t:
                    return t[:110]
    return ""


def code_files():
    pats = ["crates/*/src/**/*.rs", "web/src/*.ts", "tools/**/*.py", "tools/**/*.mjs",
            "scripts/*.sh"]
    seen = []
    for pat in pats:
        for p in sorted(ROOT.glob(pat)):
            if "node_modules" in p.parts or "target" in p.parts or p.name.endswith(".test.ts"):
                continue
            seen.append(p)
    return seen


def id_ranges(ids):
    """Compress FEED-001, FEED-002, FEED-003 → FEED-001…003."""
    by = {}
    for i in ids:
        pre, n = i.rsplit("-", 1)
        by.setdefault(pre, set()).add(int(n))
    out = []
    for pre in sorted(by):
        nums = sorted(by[pre])
        start = prev = nums[0]
        for n in nums[1:] + [None]:
            if n is not None and n == prev + 1:
                prev = n
                continue
            out.append(f"{pre}-{start:03d}" + (f"…{prev:03d}" if prev != start else ""))
            if n is not None:
                start = prev = n
    return ", ".join(out)


def render() -> str:
    out = ["# Code & spec map (generated — do not edit)", "",
           "Regenerate: `python3 tools/agent_codemap.py`. Use it to open **only the lines you "
           "need** (Read with offset/limit, or `grep -n`), instead of whole files.", "",
           "## Source files", "", "| File | Lines | What |", "|---|---|---|"]
    big = []
    for p in code_files():
        lines = p.read_text(encoding="utf-8", errors="replace").splitlines()
        out.append(f"| `{rel(p)}` | {len(lines)} | {header(lines)} |")
        if len(lines) > BIG_CODE:
            big.append((p, lines))
    out += ["", "## Items in big files (line numbers)", ""]
    for p, lines in big:
        items = [f"{i + 1}:{ITEM_RE.match(l).group(1).strip()}"
                 for i, l in enumerate(lines) if ITEM_RE.match(l)]
        if len(items) > 60:
            items = items[:60] + [f"… (+{len(items) - 60} more — grep -n)"]
        out.append(f"- **`{rel(p)}`**: " + " · ".join(items))
    out += ["", "## Tests → spec test IDs", ""]
    tests = sorted(ROOT.glob("crates/*/tests/*.rs")) + sorted(ROOT.glob("web/src/*.test.ts")) \
        + sorted(ROOT.glob("web/tests/e2e/**/*.ts"))
    for p in tests:
        ids = sorted(set(ID_RE.findall(p.read_text(encoding="utf-8", errors="replace"))))
        ids = [i for i in ids if not i.startswith("Q-") and not i.startswith("FIX-")]
        out.append(f"- `{rel(p)}`: {id_ranges(ids) if ids else '—'}")
    out += ["", "## Big specs: sections (line numbers)", ""]
    for p in sorted(ROOT.glob("specs/**/*.md")):
        if "fixes" in p.parts or p.name in ("INDEX.md",):
            continue
        lines = p.read_text(encoding="utf-8").splitlines()
        if len(lines) <= BIG_SPEC:
            continue
        heads = [f"{i + 1}:{l.lstrip('#').strip()[:40]}" for i, l in enumerate(lines)
                 if re.match(r"^#{2,3} ", l)]
        out.append(f"- **`{rel(p)}`** ({len(lines)} lines): " + " · ".join(heads))
    return "\n".join(out) + "\n"


def main():
    text = render()
    if "--check" in sys.argv:
        ok = OUT.exists() and OUT.read_text(encoding="utf-8") == text
        print("CODEMAP.md up to date" if ok else "CODEMAP.md is stale")
        sys.exit(0 if ok else 1)
    OUT.parent.mkdir(exist_ok=True)
    OUT.write_text(text, encoding="utf-8")
    print(f"wrote {rel(OUT)} ({len(text)} bytes)")


if __name__ == "__main__":
    main()
