#!/usr/bin/env python3
"""Generate .agent/DECISIONS.md from specs/open-questions.md (token-saving digest).

Agents read the short digest instead of the full question table (~120 KB). The table stays
the source of truth; this file is regenerated, never edited by hand.

    python3 tools/agent_state.py          # write .agent/DECISIONS.md
    python3 tools/agent_state.py --check  # exit 1 if the digest is out of date
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "specs" / "open-questions.md"
OUT = ROOT / ".agent" / "DECISIONS.md"


def short(text: str, limit: int) -> str:
    text = re.sub(r"\*\*|`", "", text).strip()
    text = re.sub(r"\s+", " ", text)
    if len(text) <= limit:
        return text
    cut = text[:limit].rsplit(" ", 1)[0]
    return cut.rstrip(",;:") + " …"


def first_sentence(text: str) -> str:
    text = re.sub(r"\*\*|`", "", text).strip()
    m = re.match(r"(.{25,}?[?.])(\s|$)", text)
    return m.group(1) if m else text


def parse():
    rows = []
    for line in SRC.read_text(encoding="utf-8").splitlines():
        m = re.match(r"\|\s*(Q-\d{3})\s*\|", line)
        if not m:
            continue
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) < 6:
            continue
        qid, question, affects, status, answer = (
            cells[0], cells[1], cells[2], cells[-2], cells[-1])
        rows.append(dict(id=qid, q=question, affects=affects, status=status.lower(),
                         answer=answer))
    return rows


def render(rows) -> str:
    nums = [int(r["id"][2:]) for r in rows]
    next_free = max(nums) + 1 if nums else 1
    open_rows = [r for r in rows if r["status"] == "open"]
    partly = [r for r in rows if r["status"] not in ("open", "answered")]
    answered = [r for r in rows if r["status"] == "answered"]
    out = [
        "# Decisions digest (generated — do not edit)",
        "",
        f"Source: `specs/open-questions.md` (the full text). Regenerate: "
        f"`python3 tools/agent_state.py`. **Next free question number: Q-{next_free:03d}** "
        f"(parallel agents: use the range assigned in `.agent/STATE.md`).",
        "",
        f"{len(answered)} answered · {len(open_rows)} open · {len(partly)} other "
        f"(partly answered / proposed / superseded).",
        "",
        "## Open",
        "",
    ]
    out += [f"- {r['id']} — {short(first_sentence(r['q']), 140)} ({short(r['affects'], 50)})"
            for r in open_rows]
    out += ["", "## Partly answered / other", ""]
    out += [f"- {r['id']} [{r['status']}] — {short(first_sentence(r['q']), 100)} → "
            f"{short(r['answer'], 160)}" for r in partly]
    out += ["", "## Answered (newest first)", ""]
    for r in reversed(answered):
        ans = re.sub(r"^User,?\s*", "", r["answer"])
        out.append(f"- {r['id']} — {short(first_sentence(r['q']), 70)} → {short(ans, 130)}")
    return "\n".join(out) + "\n"


def main():
    text = render(parse())
    if "--check" in sys.argv:
        ok = OUT.exists() and OUT.read_text(encoding="utf-8") == text
        print("DECISIONS.md up to date" if ok else "DECISIONS.md is stale")
        sys.exit(0 if ok else 1)
    OUT.parent.mkdir(exist_ok=True)
    OUT.write_text(text, encoding="utf-8")
    print(f"wrote {OUT.relative_to(ROOT)} ({len(text)} bytes)")


if __name__ == "__main__":
    main()
