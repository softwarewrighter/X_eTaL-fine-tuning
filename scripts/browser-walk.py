# Run with scripts/py (the uv venv .venv), never a system python.
"""Walk one demo's built page by keyboard alone and by touch, in a real
browser (scripts/browser-check.py calls this; scripts/browser-walk.sh
runs it alone).

Headless Chrome opens pages/<slug>/ (or the live site with XETAL_LIVE)
through its DevTools protocol, waits for the page's WebAssembly to run
X_eTaL, and then:

  - lists every control a reader can act on (buttons, inputs, links,
    the keyboard-reachable figures) and fails on one without a name a
    screen reader can speak (text, a label, aria-label or a title);
  - requires every heat map to carry its color legend and its note on
    the scale, and every slider to sit in a label that shows its value;
  - presses Tab until the focus returns to the page and requires every
    control to have been reached, in order, by keyboard alone;
  - acts by keyboard: Enter on a word that opens an explanation (the
    note appears), Enter on the stepper's Next and Back (the stage
    moves and returns), an arrow key on a heat map (the inspector
    reads the next cell);
  - acts by touch: a tap on a heat map's cell, on a word that explains,
    on the stepper's Next, with the same effects;
  - replays: with the stepper at a stage, the numbers on the page are
    recorded; Play, Pause, Next, Back, and back to that stage; the
    numbers must be the same. Animation changes no value. And each
    slider is moved one notch by keyboard and moved back: the page
    reruns its program twice and must read exactly as before (the
    runs are deterministic: fixed seeds, settings as data).

Prints one line per check, the same each run, so reg-rs can keep it as
a baseline with the browser check's lines.

  scripts/py scripts/browser-walk.py SLUG
"""
import base64
import json
import os
import re
import socket
import subprocess
import sys
import tempfile
import time
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CHROME = os.environ.get("CHROME", "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")

# What a reader can act on. Disabled buttons are skipped by Tab, as a
# browser does; they still need a name.
CONTROLS = 'button, input, select, textarea, a[href], summary, [tabindex="0"]'


class WebSocket:
    """A client for the DevTools protocol's socket: the standard library
    only (the handshake, text frames, masking), enough for a local Chrome."""

    def __init__(self, url):
        m = re.match(r"ws://([^:/]+):(\d+)(/.*)", url)
        host, port, path = m.group(1), int(m.group(2)), m.group(3)
        self.sock = socket.create_connection((host, port), timeout=60)
        key = base64.b64encode(os.urandom(16)).decode()
        self.sock.sendall(
            f"GET {path} HTTP/1.1\r\nHost: {host}:{port}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\n"
            f"Sec-WebSocket-Key: {key}\r\nSec-WebSocket-Version: 13\r\n\r\n".encode()
        )
        head = b""
        while b"\r\n\r\n" not in head:
            head += self.sock.recv(4096)
        if b" 101 " not in head.split(b"\r\n")[0]:
            raise RuntimeError("browser-walk: the DevTools socket refused the handshake")
        self.buf = head.split(b"\r\n\r\n", 1)[1]

    def _read(self, n):
        while len(self.buf) < n:
            chunk = self.sock.recv(65536)
            if not chunk:
                raise RuntimeError("browser-walk: Chrome closed the socket")
            self.buf += chunk
        out, self.buf = self.buf[:n], self.buf[n:]
        return out

    def send(self, text):
        data = text.encode()
        head = bytearray([0x81])
        n = len(data)
        if n < 126:
            head.append(0x80 | n)
        elif n < 65536:
            head += bytes([0x80 | 126]) + n.to_bytes(2, "big")
        else:
            head += bytes([0x80 | 127]) + n.to_bytes(8, "big")
        mask = os.urandom(4)
        self.sock.sendall(bytes(head) + mask + bytes(b ^ mask[i % 4] for i, b in enumerate(data)))

    def recv(self):
        while True:
            b0, b1 = self._read(2)
            op, n = b0 & 0x0F, b1 & 0x7F
            if n == 126:
                n = int.from_bytes(self._read(2), "big")
            elif n == 127:
                n = int.from_bytes(self._read(8), "big")
            if b1 & 0x80:
                self._read(4)
            data = self._read(n)
            if op == 1:
                return data.decode()
            if op == 8:
                raise RuntimeError("browser-walk: Chrome closed the socket")


class Chrome:
    """Headless Chrome with one page, driven through the DevTools protocol."""

    def __init__(self, url):
        self.tmp = tempfile.mkdtemp()
        self.proc = subprocess.Popen(
            [CHROME, "--headless=new", "--disable-gpu", "--no-first-run", "--no-default-browser-check",
             "--remote-debugging-port=0", f"--user-data-dir={self.tmp}/profile", "--window-size=1200,2400", "about:blank"],
            stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True,
        )
        ws = None
        while ws is None:
            line = self.proc.stderr.readline()
            if not line:
                raise RuntimeError("browser-walk: Chrome gave no DevTools address")
            m = re.search(r"DevTools listening on (ws://\S+)", line)
            if m:
                ws = m.group(1)
        self.ws = WebSocket(ws)
        self.id = 0
        self.session = None
        target = self.call("Target.createTarget", url="about:blank")["targetId"]
        self.session = self.call("Target.attachToTarget", targetId=target, flatten=True)["sessionId"]
        self.call("Page.enable")
        self.call("Runtime.enable")
        self.call("Page.navigate", url=url)

    def call(self, method, **params):
        self.id += 1
        msg = {"id": self.id, "method": method, "params": params}
        if self.session:
            msg["sessionId"] = self.session
        self.ws.send(json.dumps(msg))
        while True:
            got = json.loads(self.ws.recv())
            if got.get("id") == self.id:
                if "error" in got:
                    raise RuntimeError(f"browser-walk: {method}: {got['error']}")
                return got.get("result", {})

    def js(self, code):
        """Evaluate an expression; its JSON value."""
        r = self.call("Runtime.evaluate", expression=code, returnByValue=True, awaitPromise=True)
        if "exceptionDetails" in r:
            raise RuntimeError(f"browser-walk: js: {r['exceptionDetails'].get('text')}: {code[:80]}")
        return r["result"].get("value")

    def settle(self, ms=120):
        """Let the page react (Yew renders after the event)."""
        time.sleep(ms / 1000)

    def key(self, key, code=None, text=None):
        vk = {"Tab": 9, "Enter": 13, " ": 32, "ArrowRight": 39, "ArrowLeft": 37, "ArrowDown": 40, "ArrowUp": 38, "Home": 36, "End": 35}
        params = {"key": key, "code": code or key, "windowsVirtualKeyCode": vk.get(key, 0)}
        if text is not None:
            params["text"] = text
        self.call("Input.dispatchKeyEvent", type="keyDown" if text is None else "keyDown", **params)
        self.call("Input.dispatchKeyEvent", type="keyUp", **params)
        self.settle()

    def enter(self):
        self.key("Enter", text="\r")

    def tap(self, x, y):
        self.call("Input.dispatchTouchEvent", type="touchStart", touchPoints=[{"x": x, "y": y}])
        self.call("Input.dispatchTouchEvent", type="touchEnd", touchPoints=[])
        self.settle()

    def close(self):
        try:
            self.call("Browser.close")
        except Exception:
            pass
        self.proc.kill()
        self.proc.wait()
        subprocess.run(["rm", "-rf", self.tmp], check=False)


# JavaScript helpers, defined once in the page.
HELPERS = r"""
window.__walk = {
  visible(e) {
    if (e.closest("[hidden]")) return false;
    // What a closed <details> holds is shown only once it opens (its summary is shown).
    for (let d = e.closest("details"); d; d = d.parentElement && d.parentElement.closest("details")) {
      if (!d.open && !(e.tagName === "SUMMARY" && e.parentElement === d)) return false;
    }
    const r = e.getBoundingClientRect();
    return r.width > 0 && r.height > 0 && getComputedStyle(e).visibility !== "hidden";
  },
  name(e) {
    // As a screen reader names it: aria-label, a label, the text, and a title last.
    const own = (e.getAttribute("aria-label") || "").trim();
    if (own) return own;
    const by = e.getAttribute("aria-labelledby");
    if (by) { const t = document.getElementById(by); if (t) return t.textContent.trim(); }
    if (e.tagName === "INPUT" || e.tagName === "SELECT" || e.tagName === "TEXTAREA") {
      const l = e.closest("label") || (e.id && document.querySelector(`label[for="${e.id}"]`));
      if (l) return l.textContent.trim();
    }
    return (e.textContent || "").trim() || (e.getAttribute("title") || "").trim();
  },
  controls() {
    return [...document.querySelectorAll(CONTROLS)].filter(e => this.visible(e));
  },
  describe(e) {
    const tag = e.tagName.toLowerCase();
    const kind = tag === "input" ? `input[${e.type}]` : tag;
    return `${kind} "${this.name(e).replace(/\s+/g, " ").slice(0, 60)}"`;
  },
  center(e) {
    const r = e.getBoundingClientRect();
    return [r.left + r.width / 2, r.top + r.height / 2];
  },
  numbers() {
    // Every number the page shows: the cells of its heat maps, its tables, its charts, its sliders'
    // values and the stepper's place. Not the narration, which may compare the run with the one
    // before it, and not which cell a reader has picked, which is theirs, not the run's.
    return [...document.querySelectorAll("main .heat text.num, main table td, main output, main .where, main .charts img, main .charts svg text")].map(e => e.tagName === "IMG" ? e.src.length : e.textContent).join("|");
  },
  stage() {
    const w = document.querySelector(".stepper .where");
    return w ? w.textContent : "";
  },
  focusPath() {
    const a = document.activeElement;
    if (!a || a === document.body) return null;
    return this.describe(a);
  }
};
""".replace("CONTROLS", json.dumps(CONTROLS))


def main(slug):
    live = bool(os.environ.get("XETAL_LIVE"))
    if live:
        url = f"https://softwarewrighter.github.io/X_eTaL-fine-tuning/{slug}/"
        server = None
    else:
        if not (ROOT / "pages" / slug / "index.html").is_file():
            sys.exit(f"browser-walk: no pages/{slug} (just pages)")
        s = socket.socket()
        s.bind(("127.0.0.1", 0))
        port = s.getsockname()[1]
        s.close()
        site = Path(tempfile.mkdtemp()) / "site"
        site.mkdir()
        (site / "X_eTaL-fine-tuning").symlink_to(ROOT / "pages")
        server = subprocess.Popen([sys.executable, "-m", "http.server", str(port), "--bind", "127.0.0.1", "--directory", str(site)],
                                  stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        for _ in range(50):
            try:
                urllib.request.urlopen(f"http://127.0.0.1:{port}/", timeout=1)
                break
            except Exception:
                time.sleep(0.1)
        url = f"http://127.0.0.1:{port}/X_eTaL-fine-tuning/{slug}/"
    ok = True
    chrome = None
    try:
        chrome = Chrome(url)
        # X_eTaL has run once the summary frame and the main content are there and no error is shown.
        for _ in range(300):
            ready = chrome.js('!!document.querySelector(\'[data-frame="summary"]\') && !!document.querySelector("main") && document.body.textContent.length > 2000')
            if ready:
                break
            time.sleep(0.1)
        chrome.js(HELPERS)
        print("walk: loaded" if ready else "walk: NOT LOADED")
        ok &= bool(ready)
        if not ready:
            return 1
        # Reduced motion keeps the walk the same each run.
        chrome.call("Emulation.setEmulatedMedia", features=[{"name": "prefers-reduced-motion", "value": "reduce"}])

        if chrome.js('!!document.querySelector("fieldset.modes input[type=radio]:checked")'):
            before = chrome.js('document.querySelector("fieldset.modes .mode-about").textContent')
            chrome.js('document.querySelector("fieldset.modes input[type=radio]:checked").focus()')
            chrome.key("ArrowRight")
            after = chrome.js('document.querySelector("fieldset.modes .mode-about").textContent')
            chrome.key("ArrowLeft")
            print("keyboard: an arrow changed how much is shown, and back" if before != after and chrome.js('document.querySelector("fieldset.modes .mode-about").textContent') == before else "keyboard: the arrow did NOT change the mode")
            ok &= before != after
            # The rest of the walk sees everything the page can show.
            chrome.js('[...document.querySelectorAll("fieldset.modes input[type=radio]")].pop().click()')
            chrome.settle(400)
            print("walk: at the deepest mode, everything shown")

        # 1. Every control has a name a screen reader can speak.
        kinds = chrome.js("(() => { const c = __walk.controls().map(e => __walk.describe(e)); return c; })()")
        unnamed = [k for k in kinds if k.endswith(' ""')]
        print(f"controls: {len(kinds)}, without a spoken name: " + (", ".join(unnamed) if unnamed else "none"))
        ok &= not unnamed

        # 2. Every heat map has its legend and scale note; every slider its label and value.
        heats = chrome.js('document.querySelectorAll("main figure.heat").length')
        if heats:
            keyed = chrome.js('[...document.querySelectorAll("main figure.heat")].filter(f => (f.querySelector(".legend") && f.querySelector(".legend-note")) || (f.closest(".heats, .maps, section, .panel") || f.parentElement).querySelector(".heat-key .legend-note")).length')
            print(f"heat maps: {heats}, with a legend and a scale note: {keyed}")
            ok &= keyed == heats
        sliders = chrome.js('document.querySelectorAll("main input[type=range]").length')
        if sliders:
            labeled = chrome.js('[...document.querySelectorAll("main input[type=range]")].filter(i => i.closest("label") && i.closest("label").querySelector("output") && __walk.name(i)).length')
            print(f"sliders: {sliders}, in a label that shows the value: {labeled}")
            ok &= labeled == sliders

        # 2a. Every symbol marked in a drawn formula has a letter button, and each letter, picked by keyboard, is explained.
        if chrome.js('!!document.querySelector("main .letters")'):
            marked = chrome.js('[...new Set([...document.querySelectorAll("main .formulas .sym")].flatMap(e => [...e.classList].filter(c => c.startsWith("sym-")).map(c => c.slice(4))))].sort()')
            letters = chrome.js('[...document.querySelectorAll("main .letters button")].map(b => b.textContent.trim())')
            unexplained = [m for m in marked if m not in letters]
            said = 0
            for i in range(len(letters)):
                chrome.js(f'document.querySelectorAll("main .letters button")[{i}].focus()')
                chrome.enter()
                said += chrome.js(f'(() => {{ const s = document.querySelector("main .letters ~ .said"); return !!s && s.textContent.startsWith({json.dumps(letters[i])} + ":"); }})()')
            print(f"symbols: {len(marked)} marked in the formulas, {len(letters)} letters, explained when picked: {said}" + (", without a letter: " + ", ".join(unexplained) if unexplained else ""))
            ok &= not unexplained and said == len(letters)

        # 3. Tab reaches every control, by keyboard alone.
        # Tab starts from the top of the page: a mark there, out of the tab order itself, takes the focus first.
        chrome.js('(() => { const m = document.createElement("span"); m.id = "walk-start"; m.tabIndex = -1; document.body.prepend(m); m.focus(); window.scrollTo(0, 0); })()')
        # A group of radio buttons is one Tab stop (its checked one); arrows move within it.
        reachable = chrome.js("__walk.controls().filter(e => !e.disabled && !(e.type === 'radio' && !e.checked)).map(e => __walk.describe(e))")
        seen = []
        for _ in range(len(reachable) + 50):
            chrome.key("Tab")
            at = chrome.js("__walk.focusPath()")
            if at is None and seen:
                break
            if at is not None:
                seen.append(at)
        missed = [r for r in reachable if r not in seen]
        print(f"keyboard: {len(reachable)} controls, reached by Tab: {len(reachable) - len(missed)}" + (", not reached: " + ", ".join(missed[:5]) if missed else ""))
        ok &= not missed

        # 4. Keyboard actions.
        if chrome.js('!!document.querySelector("main button.term")'):
            chrome.js('document.querySelector("main button.term").focus()')
            chrome.enter()
            note = chrome.js('(() => { const n = document.querySelector("main button.term.open + .eli5 b, main .eli5 b"); return n ? n.textContent.replace(/:\\s*$/, "") : ""; })()')
            print(f'keyboard: Enter on a word opened "{note}"' if note else "keyboard: Enter on a word opened NOTHING")
            ok &= bool(note)
            chrome.enter()
        if chrome.js('!!document.querySelector(".stepper")'):
            before = chrome.js("__walk.stage()")
            chrome.js('document.querySelector(".stepper button:nth-of-type(4)").focus()')
            chrome.enter()
            after = chrome.js("__walk.stage()")
            chrome.js('document.querySelector(".stepper button:nth-of-type(2)").focus()')
            chrome.enter()
            back = chrome.js("__walk.stage()")
            moved = before != after and back == before
            print(f'keyboard: Next moved "{before}" to "{after}", Back returned' if moved else f'keyboard: Next or Back FAILED ("{before}", "{after}", "{back}")')
            ok &= moved
        if heats:
            chrome.js('document.querySelector("main figure.heat svg[tabindex]").focus()')
            chrome.settle()
            first = chrome.js('document.querySelector("main figure.heat .inspector").textContent')
            # Along the map's longer side: a column has only a row below.
            arrow = chrome.js('/(\\d+) rows and (\\d+) columns/.exec(document.querySelector("main figure.heat svg[tabindex]").getAttribute("aria-label")).slice(1).map(Number)')
            chrome.key("ArrowRight" if arrow[1] > 1 else "ArrowDown")
            second = chrome.js('document.querySelector("main figure.heat .inspector").textContent')
            print(f'keyboard: an arrow moved the inspector to "{second[:50]}"' if first and second and first != second else f'keyboard: the arrow key did NOT move the inspector ("{first[:40]}")')
            ok &= bool(first) and bool(second) and first != second

        # 5. Touch.
        if heats:
            x, y = chrome.js('(() => { const c = document.querySelectorAll("main figure.heat rect.cell")[3]; c.scrollIntoView({block: "center"}); return __walk.center(c); })()')
            chrome.tap(x, y)
            read = chrome.js('document.querySelector("main figure.heat .inspector").textContent')
            want = chrome.js('document.querySelectorAll("main figure.heat rect.cell")[3].querySelector("title").textContent')
            print(f'touch: a tap on a cell read "{read[:50]}"' if want and read.startswith(want) else f'touch: the tap did NOT read the cell ("{read[:40]}" for "{want[:40]}")')
            ok &= bool(want) and read.startswith(want)
        if chrome.js('!!document.querySelector("main button.term")'):
            x, y = chrome.js('(() => { const t = document.querySelector("main button.term"); t.scrollIntoView({block: "center"}); return __walk.center(t); })()')
            chrome.tap(x, y)
            opened = chrome.js('!!document.querySelector("main button.term.open")')
            print("touch: a tap on a word opened its explanation" if opened else "touch: a tap on a word opened NOTHING")
            ok &= opened
            chrome.tap(x, y)
        if chrome.js('!!document.querySelector(".stepper")'):
            before = chrome.js("__walk.stage()")
            x, y = chrome.js('(() => { const b = document.querySelector(".stepper button:nth-of-type(4)"); b.scrollIntoView({block: "center"}); return __walk.center(b); })()')
            chrome.tap(x, y)
            after = chrome.js("__walk.stage()")
            print(f'touch: a tap on Next moved "{before}" to "{after}"' if before != after else f'touch: the tap on Next did NOT move the stage ("{before}")')
            ok &= before != after

        # 6. Replay: play, pause and step change no number.
        if chrome.js('!!document.querySelector(".stepper")'):
            chrome.js('document.querySelector(".stepper button:nth-of-type(1)").click()')
            chrome.settle(200)
            chrome.js('document.querySelector(".stepper button:nth-of-type(4)").click()')
            chrome.settle(200)
            stage = chrome.js("__walk.stage()")
            recorded = chrome.js("__walk.numbers()")
            chrome.js('document.querySelector(".stepper button.play").click()')
            chrome.settle(600)
            chrome.js('document.querySelector(".stepper button.play").click()')
            chrome.settle(200)
            chrome.js('document.querySelector(".stepper button:nth-of-type(4)").click()')
            chrome.settle(200)
            chrome.js('document.querySelector(".stepper button:nth-of-type(1)").click()')
            chrome.settle(200)
            chrome.js('document.querySelector(".stepper button:nth-of-type(4)").click()')
            chrome.settle(400)
            again = chrome.js("__walk.numbers()")
            same = chrome.js("__walk.stage()") == stage and recorded == again and len(recorded) > 0
            print(f'replay: play, pause and step left every number at "{stage}" the same ({len(recorded)} characters)' if same else f'replay: the numbers at "{stage}" CHANGED')
            ok &= same
        if sliders:
            same = 0
            for i in range(sliders):
                before = chrome.js("__walk.numbers()")
                # Toward the side with room: a slider may start at its end.
                room = chrome.js(f'(() => {{ const e = document.querySelectorAll("main input[type=range]")[{i}]; e.focus(); return Number(e.value) < Number(e.max); }})()')
                there, back = ("ArrowRight", "ArrowLeft") if room else ("ArrowLeft", "ArrowRight")
                chrome.key(there)
                chrome.settle(400)
                moved = chrome.js("__walk.numbers()")
                chrome.key(back)
                chrome.settle(400)
                after = chrome.js("__walk.numbers()")
                same += moved != before and after == before
                if os.environ.get("XETAL_WALK_DEBUG"):
                    import difflib
                    print(f"debug: slider {i}: moved {moved != before}, same after {after == before}, playing {chrome.js('(document.querySelector(\".stepper button.play\") || {}).getAttribute && document.querySelector(\".stepper button.play\").getAttribute(\"aria-pressed\")')}", file=sys.stderr)
                    for d in difflib.unified_diff(before.split("|"), after.split("|"), lineterm="", n=0):
                        print("debug:   " + d[:160], file=sys.stderr)
            print(f"replay: {sliders} sliders moved a notch and back, every number the same after: {same}")
            ok &= same == sliders
    finally:
        if chrome:
            chrome.close()
        if server:
            server.kill()
            server.wait()
    return 0 if ok else 1


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit("usage: scripts/py scripts/browser-walk.py SLUG")
    sys.exit(main(sys.argv[1]))
