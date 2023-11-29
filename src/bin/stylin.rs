use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use stylin::Stylin;

#[derive(Parser)]
#[command(about, version, arg_required_else_help = true)]
struct Cli {
    /// Debug
    #[arg(short, hide = true)]
    debug: bool,

    /// Configuration file
    #[arg(short, value_name = "PATH", default_value = "stylin.json")]
    config: PathBuf,

    /// Print readme
    #[arg(short)]
    readme: bool,

    /// Input file(s); use `-` for stdin
    #[arg(value_name = "PATH")]
    input_files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    if cli.readme {
        print!("{}", include_str!("../../README.md"));
        std::process::exit(0);
    }
    let s = Stylin::from_path(&cli.config).unwrap_or_default();
    for input_file in &cli.input_files {
        let input = if input_file.as_os_str() == "-" && !input_file.exists() {
            std::io::read_to_string(std::io::stdin())?
        } else {
            std::fs::read_to_string(input_file)?
        };
        let blocks = s.convert(&input)?;
        if cli.debug {
            println!("{blocks:#?}");
        } else {
            for block in blocks {
                print!("{block}");
            }
        }
    }
    Ok(())
}
