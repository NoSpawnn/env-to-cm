use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

#[derive(Debug)]
pub enum Error<'a> {
    InvalidFile,
    ParseError(ParseError<'a>),
}

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
    pub ignore_comments: bool,
    pub ignore_whitespace: bool,
}

pub fn parse<'a>(
    s: &'a str,
    config: ParseConfig,
) -> Result<Vec<(String, String)>, self::ParseError<'a>> {
    if s.trim().is_empty() {
        return Err(ParseError::Empty);
    }

    let res: Result<Vec<(String, String)>, ParseError> = s
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty() && !line.starts_with('#'))
        .map(|(idx, line)| {
            line.split_once('=')
                .ok_or(ParseError::InvalidFormat((idx, line)))
                .map(|(key, value)| (key.into(), value.into()))
        })
        .collect();

    Ok(res?)
}

pub struct WriteConfig<'a> {
    pub template_config: TemplateConfig,
    pub outfile: &'a Path,
}

pub fn write_to_file(config: WriteConfig) -> io::Result<()> {
    let mut f = File::create(config.outfile)?;
    let formatted = template(config.template_config);
    f.write(formatted.as_bytes())?;
    Ok(())
}

pub struct TemplateConfig {
    pub values: Vec<(String, String)>,
    pub configmap_name: String,
}

pub fn template(config: TemplateConfig) -> String {
    [
        format_header!(config.configmap_name),
        config
            .values
            .iter()
            .map(|(k, v)| format!("  {k}: {}\n", if v.is_empty() { "\"\"" } else { v }))
            .collect(),
    ]
    .join("")
}
