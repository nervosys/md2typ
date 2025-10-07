# `md2typ`: Markdown to Typst Translator

**Markdown → Typst (→ PDF)** converter and CLI utility.

## Features

- **Markdown → Typst** translation with [CommonMark](https://commonmark.org/) + GFM support:
  - Headings, emphasis, strong, strikethrough
  - Lists: unordered, ordered, nested, and task lists
  - Tables with per-column alignment
  - Blockquotes, horizontal rules
  - Footnotes
- **Code blocks**: passed through unchanged as fenced Markdown
- **Smart CLI**:
  - `--output out.typ` → emits Typst source
  - `--output out.pdf` → compiles directly to PDF (auto-installs Typst if missing)
  - `--template file.typ` → applies custom Typst template for advanced styling
- **Template support**: Use custom Typst templates for professional formatting

## Installation

### From source

```bash
# Clone and build
git clone https://github.com/your-org/md2typ.git
cd md2typ
cargo build --release

# Install into your PATH
cargo install --path .
````

### From crates.io

```bash
cargo install md2typ
```

> Requires Rust 1.70+ and Cargo. If Typst is not already installed, the CLI will run
> `cargo install typst` on first PDF use.

## Usage

```bash
# Translate Markdown to Typst (stdout)
md2typ README.md

# Save Typst output
md2typ README.md --output out.typ

# Produce PDF directly
md2typ README.md --output out.pdf
```

### Options

```shell
USAGE:
    md2typst [OPTIONS] [INPUT]

ARGS:
    <INPUT>    Input Markdown file (default: stdin)

OPTIONS:
    -o, --output <PATH>       Output file: .typ (Typst source) or .pdf (compiled)
        --preamble            Add a simple Typst preamble at top
    -t, --template <PATH>     Use a Typst template file (overrides --preamble)
    -h, --help                Print help
    -V, --version             Print version
```

## Examples

Convert a Markdown design doc to Typst:

```bash
md2typ docs/design.md --output docs/design.typ
```

Convert a report directly to PDF:

```bash
md2typ report.md --output report.pdf
```

Pipe Markdown from stdin:

```bash
cat notes.md | md2typ --output notes.pdf
```

Use a custom template for professional formatting:

```bash
md2typ report.md --template examples/templates/academic.typ --output report.pdf
```

## Development

Run unit tests:

```bash
cargo test
```

Format code:

```bash
cargo fmt
```

Lint:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

## Roadmap

- [ ] Improve Typst table rendering
- [ ] Add more configurable preamble options
- [x] Allow themes/templates for PDF output
- [ ] CI/CD builds & release binaries

## Contributing

Pull requests are welcome!
Please open an issue to discuss changes or feature requests.

## License

Apache 2.0 © 2025 [Nervosys, LLC](https://github.com/nervosys/)
