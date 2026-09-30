"""Generate sound-effect candidates with Runway (eleven_text_to_sound_v2, POST /v1/sound_effect).
Key from env MF_RUNWAY_API_KEY (never printed/stored). Output: .run/sound-src/runway/<name>.* (not committed).
Usage: python3 tools/sound/runway_gen.py <name> "<prompt>" [duration_s]"""
import json, os, sys, time, urllib.request
BASE = "https://api.dev.runwayml.com"
H = {"Authorization": "Bearer " + os.environ["MF_RUNWAY_API_KEY"], "X-Runway-Version": "2024-11-06",
     "Content-Type": "application/json", "User-Agent": "buchstabenzoo-sound/0.1"}
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", ".run", "sound-src", "runway")

def call(path, body=None):
    req = urllib.request.Request(BASE + path, data=json.dumps(body).encode() if body else None, headers=H)
    return json.load(urllib.request.urlopen(req, timeout=60))

def main(name, prompt, dur=None):
    body = {"model": "eleven_text_to_sound_v2", "promptText": prompt}
    if dur: body["duration"] = float(dur)
    t = call("/v1/sound_effect", body)
    print("task", t["id"], "est credits", t["estimatedCost"])
    while True:
        time.sleep(3)
        s = call("/v1/tasks/" + t["id"])
        if s["status"] in ("SUCCEEDED", "FAILED", "CANCELLED"): break
    print(s["status"], s.get("failure"))
    if s["status"] == "SUCCEEDED":
        url = s["output"][0]
        ext = url.split("?")[0].rsplit(".", 1)[-1][:4]
        os.makedirs(OUT, exist_ok=True)
        p = os.path.join(OUT, f"{name}.{ext}")
        r = urllib.request.Request(url, headers={"User-Agent": "buchstabenzoo-sound/0.1"})
        open(p, "wb").write(urllib.request.urlopen(r).read())
        print("saved", p)

if __name__ == "__main__":
    main(*sys.argv[1:])
