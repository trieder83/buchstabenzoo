#!/usr/bin/env python3
"""Generate concept images from an art brief with the Gemini image API.

Reads the prompt and negative prompt (```text blocks) from a brief.md, sends them to
Gemini, saves the results next to the brief and appends a row to the brief's
"Generation log" table (ART-PIPELINE §7).

Usage:
  tools/gen_image.py art/environment/style_frame/brief.md --prompt 1 \
      --out style_frame_v1.png style_frame_v2.png style_frame_v3.png --aspect 16:9

  --prompt N       N-th full prompt block in the brief (1-based; negative prompts and
                   short fragments are skipped)
  --out FILE...    one image is generated per file name (saved in the brief's folder)
  --ref IMAGE...   optional reference images (e.g. the approved style frame, the front view)
  --extra TEXT     appended to the prompt (e.g. "Same character as in the second image.")

Needs GEMINI_API_KEY in the environment. Standard library only.
"""
import argparse, base64, concurrent.futures, datetime, json, mimetypes, os, re, sys, urllib.request

API = "https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent"
MIN_PROMPT_LEN = 700  # shorter blocks are fragments (e.g. alternative camera paragraphs)


STYLE_FILE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "art", "style", "style.md")


def read_blocks(brief):
    blocks = re.findall(r"```text\n(.*?)\n```", open(brief, encoding="utf-8").read(), re.S)
    # Negative prompts are the blocks ending with the NEGATIVE suffix of art/style/style.md.
    neg_suffix = re.findall(r"```text\n(.*?)\n```", open(STYLE_FILE, encoding="utf-8").read(), re.S)[-1]
    negatives = [b for b in blocks if b.rstrip().endswith(neg_suffix)]
    prompts = [b for b in blocks if b not in negatives and len(b) >= MIN_PROMPT_LEN]
    return prompts, negatives


def generate(model, prompt, aspect, size, refs, key):
    parts = [{"text": prompt}]
    for r in refs:
        mime = mimetypes.guess_type(r)[0] or "image/png"
        parts.append({"inline_data": {"mime_type": mime,
                                      "data": base64.b64encode(open(r, "rb").read()).decode()}})
    body = {"contents": [{"parts": parts}],
            "generationConfig": {"responseModalities": ["IMAGE"],
                                 "imageConfig": {"aspectRatio": aspect, "imageSize": size}}}
    req = urllib.request.Request(API.format(model=model), data=json.dumps(body).encode(),
                                 headers={"Content-Type": "application/json", "x-goog-api-key": key})
    with urllib.request.urlopen(req, timeout=300) as resp:
        data = json.load(resp)
    for part in data.get("candidates", [{}])[0].get("content", {}).get("parts", []):
        inline = part.get("inlineData") or part.get("inline_data")
        if inline:
            return base64.b64decode(inline["data"]), inline.get("mimeType", "image/png")
    raise RuntimeError("no image in response: " + json.dumps(data)[:500])


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("brief")
    ap.add_argument("--prompt", type=int, default=1)
    ap.add_argument("--out", nargs="+", required=True)
    ap.add_argument("--aspect", default="16:9")
    ap.add_argument("--size", default="1K", choices=["1K", "2K", "4K"])
    ap.add_argument("--model", default="gemini-3-pro-image")
    ap.add_argument("--ref", nargs="*", default=[])
    ap.add_argument("--extra", default="", help="text appended to the prompt, e.g. how to use the reference images")
    a = ap.parse_args()

    key = os.environ.get("GEMINI_API_KEY")
    if not key:
        sys.exit("GEMINI_API_KEY is not set")
    prompts, negatives = read_blocks(a.brief)
    if not 1 <= a.prompt <= len(prompts):
        sys.exit(f"brief has {len(prompts)} full prompts, --prompt {a.prompt} is out of range")
    prompt = prompts[a.prompt - 1]
    if a.extra:
        prompt += "\n\n" + a.extra
    if negatives:  # Gemini has no negative-prompt field
        prompt += "\n\nAvoid all of the following: " + negatives[0]

    folder = os.path.dirname(a.brief)
    with concurrent.futures.ThreadPoolExecutor(len(a.out)) as pool:
        jobs = {pool.submit(generate, a.model, prompt, a.aspect, a.size, a.ref, key): f for f in a.out}
        done = []
        for job in concurrent.futures.as_completed(jobs):
            name = jobs[job]
            try:
                img, mime = job.result()
            except Exception as e:  # keep the other variants
                print(f"FAILED {name}: {e}", file=sys.stderr)
                continue
            ext = mimetypes.guess_extension(mime) or ".png"
            if not name.endswith(ext) and ext in (".jpg", ".jpeg"):
                name = os.path.splitext(name)[0] + ".jpg"
            open(os.path.join(folder, name), "wb").write(img)
            done.append(name)
            print("saved", os.path.join(folder, name))

    if done:
        today = datetime.date.today().isoformat()
        refs = ", ref: " + ", ".join(os.path.basename(r) for r in a.ref) if a.ref else ""
        extra = " + extra text" if a.extra else ""
        rows = "".join(f"| {today} | {n} | {a.model} ({a.size}, {a.aspect}) | — | prompt {a.prompt} + negative as 'Avoid'{refs}{extra} | generated, to review |\n"
                       for n in sorted(done))
        with open(a.brief, "a", encoding="utf-8") as f:
            f.write(rows)
    sys.exit(0 if len(done) == len(a.out) else 1)


if __name__ == "__main__":
    main()
