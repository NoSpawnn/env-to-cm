use std::path::PathBuf;

use clap::{ArgAction, Parser};
use env_to_cm::ParseError;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Input .env file
    infile: PathBuf,

    /// Output .yaml, if not provided will print to stdout
    outfile: Option<PathBuf>,

    /// Name to put in the metadata.name field of the generated configmap, defaults to outfile filename, falls back to empty
    #[arg(short = 'n')]
    configmap_name: Option<String>,

    /// Do not preserve whitespace spacing in the generated configmap
    #[arg(short = 'w', long, action = ArgAction::SetFalse)]
    no_preserve_whitespace: bool,

    /// Do not preserve comments in the generated configmap
    #[arg(short = 'c', long, action = ArgAction::SetFalse)]
    no_preserve_comments: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let data = std::fs::read_to_string(&args.infile)?;

    let parse_config = env_to_cm::ParseConfig {
        preserve_comments: args.no_preserve_comments,
        preserve_whitespace: args.no_preserve_whitespace,
    };

    let values = env_to_cm::parse(&data, parse_config).map_err(|e| match e {
        ParseError::InvalidFormat((line_no, line)) => {
            anyhow::anyhow!(
                "Failed parsing {}:{}\n  '{}'",
                &args.infile.display(),
                line_no,
                line
            )
        }
        ParseError::Empty => anyhow::anyhow!("Input was empty, nothing to convert."),
    })?;

    let template_config = env_to_cm::TemplateConfig {
        values,
        configmap_name: match args.configmap_name {
            Some(name) => name,
            None => args
                .outfile
                .as_ref()
                .and_then(|p| p.file_prefix())
                .and_then(|p| p.to_str())
                .unwrap_or("")
                .to_string(),
        },
    };

    if let Some(outfile) = &args.outfile {
        let write_config = env_to_cm::WriteConfig {
            template_config,
            outfile,
        };
        env_to_cm::write_to_file(write_config)?;
    } else {
        let formatted = env_to_cm::template(template_config);
        print!("{formatted}");
    }

    Ok(())
}
