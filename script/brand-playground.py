"""Label the exported upstream playground as Brokk's maintained Scala grammar."""

from pathlib import Path
import hashlib
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
module = wasm.read_bytes()
assert module[:4] == b"\0asm", "Export must contain a WASM module"
revision = hashlib.sha256(module).hexdigest()
wasm.rename(page.with_name("tree-sitter-scala.wasm"))
loader = page.with_name("playground.js")
javascript = loader.read_text()
grammar_url = "${LANGUAGE_BASE_URL}/tree-sitter-${newLanguageName}.wasm"
assert javascript.count(grammar_url) == 1, "Expected the pinned CLI's grammar loader"
loader.write_text(javascript.replace(grammar_url, f"{grammar_url}?v={revision}"))
script = 'src="playground.js"'
assert html.count(script) == 1, "Expected one playground script"
html = html.replace(script, f'src="playground.js?v={revision}"')
page.write_text(html)
