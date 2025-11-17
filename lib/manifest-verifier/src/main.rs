use clap::Parser;
use manifest_verifier::{Result, Rule};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enables the template checker
    #[arg(short = 't', long)]
    is_template: bool,

    /// Enables the proprietary checker
    #[arg(short = 'p', long)]
    is_proprietary: bool,

    /// A _RELATIVE_ path to the manifest file
    path: String,
}

fn on_error(path: &str, rule: &Rule) {
    // Format: "::error file={name},col={col},endColumn={endColumn},line={line},endLine={endLine},title={title}::{message}"
    // Example: "::error file=app.js,line=1,col=5,endColumn=7,title=YOUR-TITLE::Missing semicolon"

    let message = rule.message.as_ref().map(|m| urlencoding::encode(m));
    let parts = vec![
        Some(format!("file={}", path)),
        rule.location.as_ref().map(|l| format!("col={}", l.column)),
        rule.location
            .as_ref()
            .and_then(|l| l.end_column)
            .map(|c| format!("endColumn={}", c)),
        rule.location.as_ref().map(|l| format!("line={}", l.line)),
        rule.location
            .as_ref()
            .and_then(|l| l.end_line)
            .map(|l| format!("endLine={}", l)),
        rule.title
            .as_ref()
            .map(|t| format!("title={}", urlencoding::encode(t))),
    ];

    let args = parts.into_iter().flatten().collect::<Vec<_>>().join(",");
    let text = format!("::error {}::{}", args, message.unwrap_or_default());

    println!("{}", text);
}

fn main() -> Result<()> {
    let args = Args::parse();

    let flags = manifest_verifier::RunFlags {
        is_template: args.is_template,
        is_proprietary: args.is_proprietary,
    };

    // use std::path::Path;
    // if !Path::new(&args.path).is_relative() {
    //     return Err(Error::PathNotRelative);
    // }

    let on_error = |rule: &Rule| on_error(&args.path, rule);
    let has_error = manifest_verifier::run_from_file(&args.path, on_error, flags)?;

    match has_error {
        true => std::process::exit(1),
        false => Ok(()),
    }
}
