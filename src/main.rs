use anyhow::{Context, Result, anyhow};
use clap::Parser;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// md2typst — Markdown → Typst (→ PDF)
///
/// Examples:
///   md2typst README.md                  # to stdout (.typ code)
///   md2typst README.md --output out.typ # to Typst file
///   md2typst README.md --output out.pdf # full pipeline to PDF
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Input Markdown file (default: stdin)
    input: Option<String>,

    /// Output file: .typ (Typst source) or .pdf (full pipeline)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Add a simple Typst preamble at top
    #[arg(long)]
    preamble: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // ---- Read input as bytes → decode (lossy) to guaranteed-UTF-8 String ----
    let md: String = if let Some(path) = &cli.input {
        String::from_utf8_lossy(&fs::read(path)?).into_owned()
    } else {
        let mut bytes = Vec::new();
        io::stdin().read_to_end(&mut bytes)?;
        String::from_utf8_lossy(&bytes).into_owned()
    };
    let md = md.replace("\r\n", "\n").replace('\r', "\n");

    // ---- Translate to Typst ----
    let mut typ = md2typ::translate(&md, cli.preamble)?;
    typ = md2typ::sanitize_text(&typ);

    match &cli.output {
        None => {
            // stdout
            let mut stdout = io::stdout().lock();
            stdout.write_all(typ.as_bytes())?;
            stdout.flush()?;
        }
        Some(out_path) => {
            if out_path.extension().map(|e| e == "pdf").unwrap_or(false) {
                // User wants PDF directly
                let typ_path = out_path.with_extension("typ");
                write_utf8(&typ_path, &typ)?;
                ensure_typst()?;
                run_typst_compile(&typ_path, out_path)?;
                eprintln!("PDF written to {}", out_path.display());
            } else {
                // Write .typ
                write_utf8(out_path, &typ)?;
            }
        }
    }

    Ok(())
}

/// Ensure Typst is on PATH; if not, `cargo install typst`.
fn ensure_typst() -> Result<()> {
    let check = Command::new("typst")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    let needs_install = match check {
        Ok(status) => !status.success(),
        Err(_) => true,
    };

    if needs_install {
        eprintln!("Typst not found — installing via Cargo…");
        let install = Command::new("cargo")
            .args(["install", "typst"])
            .status()
            .context("failed to run `cargo install typst`")?;
        if !install.success() {
            return Err(anyhow!("`cargo install typst` failed"));
        }
    }
    Ok(())
}

/// Invoke `typst compile <in.typ> <out.pdf>`
fn run_typst_compile(typ_path: &Path, pdf_path: &Path) -> Result<()> {
    let status = Command::new("typst")
        .args([
            "compile",
            &typ_path.display().to_string(),
            &pdf_path.display().to_string(),
        ])
        .status()
        .context("failed to run `typst compile`")?;

    if !status.success() {
        return Err(anyhow!("`typst compile` failed"));
    }
    Ok(())
}

/// Write UTF-8 text with normalized newlines.
fn write_utf8(path: &Path, s: &str) -> Result<()> {
    let clean = s.replace("\r\n", "\n").replace('\r', "\n");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, clean.as_bytes()).with_context(|| format!("failed to write {}", path.display()))
}
