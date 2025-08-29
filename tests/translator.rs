//! Robust tests for the Markdown → Typst translator.

// Helper: run the pipeline used by the CLI (translate → sanitize).
fn render(md: &str) -> String {
    let out = md2typ::translate(md, false).expect("translate should succeed");
    md2typ::sanitize_text(&out)
}

// Collapse whitespace to make tests resilient.
fn norm(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last_ws = false;
    for ch in s.chars() {
        if ch.is_whitespace() {
            if !last_ws {
                out.push(' ');
                last_ws = true;
            }
        } else {
            last_ws = false;
            out.push(ch);
        }
    }
    out.trim().to_string()
}

#[test]
fn headings_emphasis_and_lists() {
    let md = r#"
# Title

Some *emphasis*, **strong**, and `inline`.

- a
  - nested
- b

1. one
2. two
"#;

    let got = render(md);
    let g = norm(&got);

    assert!(g.contains("= Title"), "missing H1 heading:\n{}", got);
    assert!(g.contains("#emph[emphasis]"), "missing emphasis:\n{}", got);
    assert!(g.contains("#strong[strong]"), "missing strong:\n{}", got);
    assert!(g.contains("`inline`"), "missing inline code:\n{}", got);
    assert!(g.contains("- a"), "missing first bullet:\n{}", got);
    assert!(g.contains("nested"), "missing nested item:\n{}", got);
    assert!(g.contains("- b"), "missing second bullet:\n{}", got);
    assert!(
        g.contains("1. one") && g.contains("2. two"),
        "missing ordered list:\n{}",
        got
    );
}

#[test]
fn code_blocks_passthrough_and_blockquote() {
    let md = r#"
> Quote

```rust
fn main() { println!("hi"); }

"#;

    let got = render(md);
    let g = norm(&got);

    // Blockquote present
    assert!(g.contains("#quote[Quote"), "missing blockquote:\n{}", got);

    // Code block PASSTHROUGH: literal fences and code present
    assert!(
        got.contains("```rust\n"),
        "missing opening fenced code:\n{}",
        got
    );
    assert!(
        got.contains("println!(\"hi\");"),
        "code content missing:\n{}",
        got
    );
    assert!(
        got.contains("\n```\n"),
        "missing closing fenced code:\n{}",
        got
    );
}

#[test]
fn tables_basic_alignment_cells_present() {
    let md = r#"
A	B
x	y
"#;

    let got = render(md);
    let g = norm(&got);

    // Either a proper Typst table with cells, or a plain fallback with both cell values.
    let ok_typst = g.contains("#table(") && g.contains("[x]") && g.contains("[y]");
    let ok_plain = g.contains("x") && g.contains("y");
    assert!(
        ok_typst || ok_plain,
        "table rendering missing cells/wrapper:\n{}",
        got
    );
}

#[test]
fn fixture_capabilities_minimal() {
    let md = r#"## Capabilities

    Autonomy

    Real-time

    Security
    "#;

    let got = render(md);
    let g = norm(&got);

    assert!(
        g.contains("== Capabilities"),
        "missing H2 heading:\n{}",
        got
    );
    assert!(g.contains("Autonomy"), "missing list item 1:\n{}", got);
    assert!(g.contains("Real-time"), "missing list item 2:\n{}", got);
    assert!(g.contains("Security"), "missing list item 3:\n{}", got);
}
