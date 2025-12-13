use std::{io, path::PathBuf};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    infile: PathBuf,
    outfile: Option<PathBuf>,

    #[arg(short = 'n')]
    configmap_name: Option<String>,

    #[arg(long)]
    ignore_whitespace: bool,
    #[arg(long)]
    ignore_comments: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let data = std::fs::read_to_string(&args.infile)?;

    let parse_config = env_to_cm::ParseConfig {
        ignore_comments: args.ignore_comments,
        ignore_whitespace: args.ignore_whitespace,
    };

    let values = match env_to_cm::parse(&data, parse_config) {
        Ok(v) => v,
        Err(e) => match e {
            env_to_cm::ParseError::InvalidFormat((line_no, line)) => {
                panic!(
                    "Failed to parse line {} in {}: '{}'",
                    line_no,
                    &args.infile.display(),
                    line
                )
            }
            env_to_cm::ParseError::Empty => {
                panic!("Input was empty, nothing to convert")
            }
        },
    };

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
        println!("{formatted}");
    }

    Ok(())
}
