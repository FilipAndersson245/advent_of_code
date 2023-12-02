use anyhow::{Ok, Result};
use clap::Parser;
use copy_dir::copy_dir;
use std::path::PathBuf;
use ureq::get;
use walkdir::WalkDir;

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

            let text = std::fs::read_to_string(path).unwrap();
            let text = text.replace("__YEAR__", &cli.year.to_string());
            let text = text.replace("__DAY__", &cli.day.to_string());
            std::fs::write(path, text).unwrap();
        });

    Ok(())
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
