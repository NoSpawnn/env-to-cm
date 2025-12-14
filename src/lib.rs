use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    Empty,
    InvalidFormat((usize, &'a str)),
}

macro_rules! format_header {
    ($name:expr) => {
        format!(
            indoc::indoc! {"
            apiVersion: v1
            kind: ConfigMap
            metadata:
              name: {}
            data:
            "},
            $name
        )
    };
}

#[derive(Default)]
pub struct ParseConfig {
    pub preserve_comments: bool,
    pub preserve_whitespace: bool,
}

#[derive(PartialEq, Eq)]
pub enum EnvToken {
    Blank,
    Comment(String),
    Pair((String, String)),
}

pub fn parse<'a>(s: &'a str, config: ParseConfig) -> Result<Vec<EnvToken>, self::ParseError<'a>> {
    if s.trim().is_empty() {
        return Err(ParseError::Empty);
    }

    let res: Result<Vec<EnvToken>, ParseError> = s
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            if line.trim_start().starts_with('#') {
                if config.preserve_comments {
                    let text = line[line.find('#').unwrap() + 1..].trim_start(); // this is kind of yucky, do I actually need to do this?
                    Some(Ok(EnvToken::Comment(text.into())))
                } else {
                    None
                }
            } else if let Some((key, value)) = line.split_once('=') {
                Some(Ok(EnvToken::Pair((key.into(), value.into()))))
            } else if line.trim().is_empty() {
                if config.preserve_whitespace {
                    Some(Ok(EnvToken::Blank))
                } else {
                    None
                }
            } else {
                Some(Err(ParseError::InvalidFormat((idx, line))))
            }
        })
        .collect();

    res
}

pub struct WriteConfig<'a> {
    pub template_config: TemplateConfig,
    pub outfile: &'a Path,
}

pub fn write_to_file(config: WriteConfig) -> io::Result<()> {
    let mut f = File::create(config.outfile)?;
    let formatted = template(config.template_config);
    f.write_all(formatted.as_bytes())?;
    Ok(())
}

pub struct TemplateConfig {
    pub values: Vec<EnvToken>,
    pub configmap_name: String,
}

pub fn template(config: TemplateConfig) -> String {
    std::iter::once(format_header!(config.configmap_name))
        .chain(config.values.iter().map(|t| match t {
            EnvToken::Blank => String::from("\n"),
            EnvToken::Comment(c) => format!("  # {c}\n"),
            EnvToken::Pair((k, v)) => format!("  {k}: {}\n", if v.is_empty() { "\"\"" } else { v }),
        }))
        .collect()
}
