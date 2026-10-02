"""Label the exported upstream playground as Brokk's maintained Scala grammar."""

from pathlib import Path
import re
import sys

page = Path(sys.argv[1])
html = page.read_text()
html, count = re.subn(
    r"<title>.*?</title>",
    "<title>Brokk Scala Grammar Playground</title>",
    html,
    count=1,
    flags=re.DOTALL,
)
assert count == 1, "Exported playground must have one title"
base_url = 'LANGUAGE_BASE_URL = "";'
assert html.count(base_url) == 1, "Expected the pinned CLI's language base URL"
html = html.replace(base_url, 'LANGUAGE_BASE_URL = ".";')
wasm = page.with_name("tree-sitter-parser.wasm")
assert wasm.is_file(), "Expected the pinned CLI's exported grammar WASM"
assert wasm.read_bytes()[:4] == b"\0asm", "Export must contain a WASM module"
wasm.rename(page.with_name("tree-sitter-scala.wasm"))
page.write_text(html)
