use clap::{Parser, Subcommand, Args};
use std::str::FromStr;

/// Fbdb-webscrape command-line interface
///
/// Defines the command-line interface for the fbdb-webscrape CLI
#[derive(Parser)]
#[command(name="The Football Database Web Scraper")]
#[command(author="whatsacomputertho")]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
pub struct FbdbWebScrapeCli {
    /// The boxscores subcommand
    #[command(subcommand)]
    pub command: FbdbSubcommand
}

impl FbdbWebScrapeCli {
    pub fn command(&self) -> FbdbSubcommand {
        self.command.clone()
    }
}

/// The common arguments accepted by all fbdb-webscrape subcommands
#[derive(Args, Clone)]
pub struct FbdbSubcommandArgs {
    /// The format to output
    #[arg(short='o')]
    #[arg(long="output")]
    pub output_format: Option<String>,

    /// The file to write to
    #[arg(short='f')]
    #[arg(long="file")]
    pub output_file: Option<String>,

    /// The year of scores to retrieve
    #[arg(short='y')]
    #[arg(long="year")]
    #[arg(default_value_t=2024)]
    pub year: u32,
}

/// The subcommands of the fbdb-webscrape CLI
#[derive(Subcommand, Clone)]
pub enum FbdbSubcommand {
    #[command(about="Retrieve historic box scores from the football database")]
    Boxscores(FbdbSubcommandArgs)
}

/// Enum into which the output format argument is parsed
#[derive(Debug,PartialEq)]
pub enum OutputFormat {
    Json,
    Default
}
impl FromStr for OutputFormat {
    type Err = ();
    fn from_str(input: &str) -> Result<OutputFormat, Self::Err> {
        match input {
            "json"      => Ok(OutputFormat::Json),
            _           => Ok(OutputFormat::Default),
        }
    }
}
impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_str = match self {
            OutputFormat::Json => "json",
            OutputFormat::Default => "default"
        };
        f.write_str(&fmt_str)
    }
}
