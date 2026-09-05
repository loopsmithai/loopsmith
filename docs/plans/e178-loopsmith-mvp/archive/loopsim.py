#!/usr/bin/env python3
"""
loopsim.py — a transpiler simulation / playtest.

Hardcoded SOURCE (one loop, one hat) → a set of TRANSPILE RULES applied one at a
time. Each step externalizes WHICH rule fired, WHAT part of the source it read,
and shows the ralph.yml accreting live. Ends with a validation pass.

Run:
    python3 loopsim.py            # interactive: press Enter to apply each rule
    python3 loopsim.py --auto     # auto-advance with a short delay
"""
import sys, time, textwrap

AUTO_DELAY = 0.0  # used in non-interactive / --auto mode

# ─────────────────────────────────────────────────────────────────────────────
# SOURCE  (the input — "the transpiler hardcodes the workflow")
#   three faces: BPMN graph (wiring) · content prose · setup config
# ─────────────────────────────────────────────────────────────────────────────
SOURCE = {
    "loop_id": "merge-recorder",
    # setup/blueprint config (NOT the graph)
    "driver":   {"engine": "ralph", "persistent": True, "max_iterations": 100,
                 "prompt_file": "PROMPT.md", "board_scanner": True},
    "actor":    {"performer": "agent", "coding_agent": "claude"},
    "guardrails": ["All issue operations use the github-project skill — no write-locks needed"],
    "skills":   ["github-project"],
    "activity": {
        "id": "record_merge",
        "name": "Record Merge",
        "description": "Records the PR name of each work item that reaches the merge status.",
        # ① BPMN graph — the wiring
        "graph": {
            "entry_status":  "snt:gate:merge",
            "trigger_event": "snt.record",
            "outcomes": [{"name": "RECORDED", "publish": "snt.record.done", "status_out": None}],
        },
        # ② content prose — pure, no wiring
        "content_prose": ("Read the linked PR's title from the work item. Append one line\n"
                          "to the local file `./merges.txt` in the form `<issue#> <pr-title>`."),
    },
}

# ─────────────────────────────────────────────────────────────────────────────
# tiny YAML emitter (so the demo has zero deps and renders literal blocks nicely)
# ─────────────────────────────────────────────────────────────────────────────
def _scalar(v):
    if isinstance(v, bool): return "true" if v else "false"
    if v is None: return "null"
    return str(v)

def to_yaml(obj, indent=0):
    pad = " " * indent
    out = []
    if isinstance(obj, dict):
        for k, v in obj.items():
            if isinstance(v, dict):
                out.append(f"{pad}{k}:")
                out.append(to_yaml(v, indent + 2))
            elif isinstance(v, list):
                out.append(f"{pad}{k}:")
                for item in v:
                    out.append(f"{pad}- {_scalar(item)}")
            elif isinstance(v, str) and "\n" in v:
                out.append(f"{pad}{k}: |")
                for line in v.rstrip("\n").split("\n"):
                    out.append(f"{pad}  {line}")
            else:
                out.append(f"{pad}{k}: {_scalar(v)}")
    return "\n".join(out)

# ─────────────────────────────────────────────────────────────────────────────
# storyboard (the BPMN graph), with the touched element highlighted
# ─────────────────────────────────────────────────────────────────────────────
def storyboard(mark=None):
    g = SOURCE["activity"]["graph"]; a = SOURCE["activity"]
    def m(t): return "►" if mark == t else " "
    o = g["outcomes"][0]
    return "\n".join([
        f"   STORYBOARD — BPMN graph   (loop: {SOURCE['loop_id']})",
        f"   {m('entry')} ◯ entry  @ {g['entry_status']}",
        f"        │",
        f"   {m('task')} ▭ {a['name']}      body → content prose ②",
        f"        │",
        f"   {m('end')} ◉ end → publish {o['publish']}   (status-out: {o['status_out']})",
    ])

# ─────────────────────────────────────────────────────────────────────────────
# TRANSPILE RULES  — each reads SOURCE, writes part of `ralph`, returns a log line
#   (ralph, scratch) -> log
# ─────────────────────────────────────────────────────────────────────────────
def R_event_loop(ralph, s):
    d = SOURCE["driver"]
    ralph["event_loop"] = {"prompt_file": d["prompt_file"],
                           "max_iterations": d["max_iterations"],
                           "persistent": d["persistent"]}
    return "driver  →  event_loop {prompt_file, max_iterations, persistent}"

def R_cli(ralph, s):
    ralph["cli"] = {"backend": SOURCE["actor"]["coding_agent"]}
    return f"actor.coding_agent='{SOURCE['actor']['coding_agent']}'  →  cli.backend"

def R_guardrails(ralph, s):
    ralph["core"] = {"guardrails": list(SOURCE["guardrails"])}
    return "loop.guardrails  →  core.guardrails"

def R_hat_open(ralph, s):
    a = SOURCE["activity"]
    ralph.setdefault("hats", {})[a["id"]] = {"name": a["name"], "description": a["description"]}
    return f"activity  →  hats.{a['id']} (name, description)"

def R_triggers(ralph, s):
    a = SOURCE["activity"]; g = a["graph"]
    ralph["hats"][a["id"]]["triggers"] = [g["trigger_event"]]
    return f"graph.entry(status {g['entry_status']})  →  triggers=[{g['trigger_event']}]"

def R_publishes(ralph, s):
    a = SOURCE["activity"]
    pubs = [o["publish"] for o in a["graph"]["outcomes"] if o["publish"]]
    ralph["hats"][a["id"]]["publishes"] = pubs
    return f"graph.outcomes  →  publishes={pubs}"

def R_wiring_to_prose(ralph, s):   # TRANSFORMER (deterministic)
    g = SOURCE["activity"]["graph"]
    lines = [f"Handle the work item currently at board status `{g['entry_status']}`."]
    for o in g["outcomes"]:
        seg = []
        if o["status_out"]: seg.append(f"set status to `{o['status_out']}`")
        if o["publish"]:    seg.append(f"publish `{o['publish']}`")
        tail = ", ".join(seg) + "." if seg else "do nothing further."
        lines.append(f"On {o['name']}: {tail} (no status change.)" if not o["status_out"]
                     else f"On {o['name']}: {tail}")
    s["wiring_prose"] = "\n".join(lines)
    return "TRANSFORMER ⚙  render BPMN wiring  →  prose"

def R_compose(ralph, s):           # TRANSFORMER (deterministic)
    a = SOURCE["activity"]
    skills_line = "Use the " + ", ".join(f"`{x}`" for x in SOURCE["skills"]) + " skill."
    body = (f"## {a['name']}\n\n{a['description']}\n\n"
            f"{s['wiring_prose']}\n\n"
            f"{a['content_prose']}\n\n{skills_line}\n")
    ralph["hats"][a["id"]]["instructions"] = body
    return "TRANSFORMER ⚙  compose  header + wiring-prose + content-prose  →  instructions"

def R_skills(ralph, s):
    ralph["skills"] = {"enabled": True,
                       "overrides": {"board-scanner": {"auto_inject": SOURCE["driver"]["board_scanner"]}}}
    return "driver.board_scanner  →  skills.overrides.board-scanner.auto_inject"

RULES = [
    ("R1", R_event_loop,     None),
    ("R2", R_cli,            None),
    ("R3", R_guardrails,     None),
    ("R4", R_hat_open,       "task"),
    ("R5", R_triggers,       "entry"),
    ("R6", R_publishes,      "end"),
    ("R7", R_wiring_to_prose,"task"),
    ("R8", R_compose,        "task"),
    ("R9", R_skills,         None),
]

# ─────────────────────────────────────────────────────────────────────────────
# rendering / driver
# ─────────────────────────────────────────────────────────────────────────────
def _tty():    return sys.stdout.isatty() and "--auto" not in sys.argv
def _clear():  sys.stdout.write("\033[2J\033[H") if _tty() else None
def _pause():
    if _tty():
        try: input("\n   … press Enter to apply next rule …")
        except EOFError: pass
    else:
        time.sleep(AUTO_DELAY)

def frame(i, total, rid, log, ralph, mark):
    bar = "═" * 74
    y = to_yaml(ralph) if ralph else "  (empty)"
    return "\n".join([
        bar,
        f" STEP {i}/{total}   [{rid}]",
        bar,
        storyboard(mark),
        "",
        f"   ▶ rule fired:  {log}",
        "",
        "   ── ralph.yml so far ────────────────────────────────────────────",
        textwrap.indent(y, "     "),
        "",
    ])

def intro():
    a = SOURCE["activity"]
    print("╔" + "═" * 72 + "╗")
    print("║  LOOPSIM — transpile playtest:  SOURCE (1 loop, 1 hat) → ralph.yml    ║")
    print("╚" + "═" * 72 + "╝")
    print()
    print(storyboard())
    print()
    print("   ② content prose (referenced by the task, carries NO wiring):")
    print(textwrap.indent(a["content_prose"], "        "))
    print()
    print("   setup config:  driver=ralph(persistent) · actor=claude ·"
          " skills=[github-project]")
    print()

def validate(ralph):
    a_id = SOURCE["activity"]["id"]
    h = ralph["hats"][a_id]
    checks = [
        ("event_loop.persistent == true",        ralph["event_loop"]["persistent"] is True),
        ("cli.backend == claude",                ralph["cli"]["backend"] == "claude"),
        ("core.guardrails present",              bool(ralph["core"]["guardrails"])),
        (f"hats.{a_id}.triggers == [snt.record]", h["triggers"] == ["snt.record"]),
        (f"hats.{a_id}.publishes == [snt.record.done]", h["publishes"] == ["snt.record.done"]),
        (f"hats.{a_id}.instructions non-empty",  bool(h.get("instructions"))),
        ("instructions mention the entry status", "snt:gate:merge" in h["instructions"]),
        ("instructions carry the content prose", "merges.txt" in h["instructions"]),
        ("board-scanner.auto_inject == true",    ralph["skills"]["overrides"]["board-scanner"]["auto_inject"] is True),
        ("hat key-set == real pr_gate key-set",
         set(h.keys()) == {"name", "description", "triggers", "publishes", "instructions"}),
    ]
    print("═" * 74)
    print(" VALIDATION — does the emitted ralph.yml hold?")
    print("═" * 74)
    ok = True
    for name, passed in checks:
        ok &= passed
        print(f"   [{'PASS' if passed else 'FAIL'}]  {name}")
    print()
    print("   RESULT:", "✅ ALL CHECKS PASS — valid ralph.yml" if ok else "❌ FAILED")
    return ok

def main():
    _clear(); intro(); _pause()
    ralph, scratch = {}, {}
    total = len(RULES)
    for i, (rid, fn, mark) in enumerate(RULES, 1):
        log = fn(ralph, scratch)
        _clear()
        print(frame(i, total, rid, log, ralph, mark))
        _pause()
    print("═" * 74)
    print(" FINAL ralph.yml")
    print("═" * 74)
    print(to_yaml(ralph))
    print()
    ok = validate(ralph)
    out = __file__.rsplit("/", 1)[0] + "/generated-ralph.yml"
    with open(out, "w") as f:
        f.write(to_yaml(ralph) + "\n")
    print(f"\n   written → {out}")
    sys.exit(0 if ok else 1)

if __name__ == "__main__":
    main()
