# md2typ: Markdown → Typst translator

A small Rust CLI tool that converts Markdown documents into Typst markup.  
It’s useful if you want to keep writing in Markdown but generate clean Typst for typesetting and PDF export.

---

## ✨ Features

- Converts core Markdown constructs into Typst equivalents:
  - Headings → `=`, `==`, `===`
  - Emphasis → `#emph[]`, strong → `#strong[]`, strikethrough → `#strike[]`
  - Links → `#link("url")[text]`
  - Images → `#image("file.png")`
  - Inline code → `` `code` ``
  - Code blocks → ` ```code``` `
  - Lists (unordered, ordered, nested)
  - Blockquotes → `#quote[...]`
  - Horizontal rules → `#line(length: 100%)`
  - Tables → `#table(columns: (...))[ ... ]`
- Outputs Typst to `stdout` (so you can pipe/redirect).
- Optional `--preamble` flag adds a simple Typst page setup at the top.

---

## 📦 Get Started

Clone, build, and run tests:

```shell
# Build
git clone https://github.com/nervosys/md2typ.git
cd md2typ
cargo build --release

# Test
cargo test
./target/release/md2typ TEST.md --output test.typ
typst compile test.typ test.pdf
```

Install `md2typ` binary to your path:

```shell
cargo install --path .
```
