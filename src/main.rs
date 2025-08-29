use anyhow::Result;
use clap::Parser;
use std::fs;
use std::io::{self, Read, Write};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    input: Option<String>,
    #[arg(short, long)]
    output: Option<String>,
    #[arg(long)]
    preamble: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let md = if let Some(path) = &cli.input {
        String::from_utf8_lossy(&fs::read(path)?).into_owned()
    } else {
        let mut bytes = Vec::new();
        io::stdin().read_to_end(&mut bytes)?;
        String::from_utf8_lossy(&bytes).into_owned()
    };

    let md = md.replace("\r\n", "\n").replace('\r', "\n");
    let mut out = md2typ::translate(&md, cli.preamble)?;
    out = md2typ::sanitize_text(&out);

    if let Some(path) = &cli.output {
        fs::write(path, out.as_bytes())?;
    } else {
        let mut stdout = io::stdout().lock();
        stdout.write_all(out.as_bytes())?;
        stdout.flush()?;
    }
    Ok(())
}
