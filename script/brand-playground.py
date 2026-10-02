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
page.write_text(html)
