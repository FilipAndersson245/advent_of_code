use anyhow::{Ok, Result};
use clap::Parser;
use copy_dir::copy_dir;
use std::path::PathBuf;
use ureq::get;
use walkdir::WalkDir;

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    year: u32,

    /// Sets a custom config file
    #[arg(short, long)]
    day: u8,

    #[arg(short, long, default_value = "./")]
    output: PathBuf,

    #[arg(short, long, env)]
    aoc_token: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut template_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    template_dir.push("template");

    let input = get_input(&cli)?;

    copy_dir(template_dir, &cli.output).unwrap();

    let mut input_path = cli.output.clone();
    input_path.push(std::path::Path::new("input.txt"));
    std::fs::write(input_path, input)?;

    WalkDir::new(&cli.output)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .for_each(|e| {
            let path = e.path();
            if path
                .components()
                .any(|a| a.as_os_str() != "target" || a.as_os_str() != "Cargo.lock")
            {
            } else {
                let text = std::fs::read_to_string(path).unwrap();
                let text = replace(regex!("__YEAR__"), &cli.year.to_string(), &text).unwrap();
                let text = replace(regex!("__DAY__"), &cli.day.to_string(), &text).unwrap();
                std::fs::write(path, text).unwrap();
            }
        });

    Ok(())
}

use regex::Regex;

fn replace(re: &Regex, replace_with: &str, text: &str) -> Result<String> {
    Ok(re.replace_all(text, replace_with).to_string())
}

fn get_input(cli: &Cli) -> Result<String> {
    let text: String = get(&format!(
        "https://adventofcode.com/{}/day/{}/input",
        cli.year, cli.day
    ))
    .set("Cookie", &cli.aoc_token)
    .call()?
    .into_string()?;

    Ok(text)
}
