use indoc::indoc;
use manifest::Manifest;
use manifest_verifier_lib::Checker;
use marked_yaml::from_yaml;

mod error;
mod manifest;

pub use error::{Error, Result};
pub use manifest_verifier_lib::{Location, Rule};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunFlags {
    pub is_template: bool,
    pub is_proprietary: bool,
}

fn add_rules(checker: &mut Checker, lines: Vec<&str>, manifest: &Manifest, flags: RunFlags) {
    if flags.is_proprietary {
        checker
            .add_rule()
            .with_title("Manifest must begin with a comment to the specs")
            .with_message(
                "First line must be `# See: https://asimov-specs.github.io/module-manifest/`",
            )
            .with_condition(lines[0] == "# See: https://asimov-specs.github.io/module-manifest/")
            .with_location(Location::from_line_and_col(1, 0))
            .build();

        checker
            .add_rule()
            .with_title("Manifest must begin with a YAML document separator (---)")
            .with_message("Second line must be `---`")
            .with_condition(lines[1] == "---")
            .with_location(Location::from_line_and_col(2, 0))
            .build();
    } else {
        let first_non_comment_line = lines.iter().find(|line| !line.starts_with('#'));

        if let Some(first_non_comment_line) = first_non_comment_line {
            checker
                .add_rule()
                .with_title("Manifest must begin with a YAML document separator (---)")
                .with_message("First non-comment line must be `---`")
                .with_condition(*first_non_comment_line == "---")
                .with_location(Location::from_line_and_col(2, 0))
                .build();
        }
    }

    checker
        .add_rule()
        .with_title("Name must be present")
        .with_message("`name` is a required field")
        .with_condition(manifest.name.is_some())
        .build();

    if let Some(name) = &manifest.name {
        const DESCRIPTION: &str = indoc! {"
            The `name` field MUST contain a string that uniquely identifies the module within the ASIMOV Platform. The name:
            - MUST consist only of lowercase letters, digits, and hyphens
            - MUST start with a letter
            - MUST NOT exceed 64 characters in length
            - SHOULD be descriptive and follow the pattern category-provider-type (e.g., search-google-fetcher)
        "};

        if !flags.is_template {
            checker
                .add_rule()
                .with_title("Name must be changed from the template")
                .with_message(DESCRIPTION)
                .with_property(name)
                .with_condition(*name != "template")
                .build();
        }

        checker
            .add_rule()
            .with_title("Name must not be empty")
            .with_message(DESCRIPTION)
            .with_property(name)
            .with_condition(!name.trim().is_empty())
            .build();

        checker
            .add_rule()
            .with_title("Name must consist only of lowercase letters, digits, and hyphens")
            .with_message(DESCRIPTION)
            .with_property(name)
            .with_condition(name.as_str().chars().all(|c| c.is_ascii_lowercase()))
            .build();

        checker
            .add_rule()
            .with_title("Name must start with a letter")
            .with_message(DESCRIPTION)
            .with_property(name)
            .with_condition(
                name.as_str()
                    .chars()
                    .next()
                    .map(|c| c.is_alphabetic())
                    .unwrap_or(true),
            )
            .build();

        checker
            .add_rule()
            .with_title("Name must not exceed 64 characters")
            .with_message(DESCRIPTION)
            .with_property(name)
            .with_condition(name.as_str().len() <= 64)
            .build();
    }

    if flags.is_proprietary {
        checker
            .add_rule()
            .with_title("Label must be present")
            .with_message("`label` is a required field")
            .with_condition(manifest.label.is_some())
            .build();
    }

    if let Some(label) = &manifest.label {
        const DESCRIPTION: &str = "The `label` field contains an optional human-readable display name for the module. If provided, it SHOULD be concise and suitable for display in user interfaces.";

        if !flags.is_template {
            checker
                .add_rule()
                .with_title("Label must be changed from the template")
                .with_message(DESCRIPTION)
                .with_property(label)
                .with_condition(*label != "Template")
                .build();
        }

        checker
            .add_rule()
            .with_title("Label must not be empty")
            .with_message(DESCRIPTION)
            .with_property(label)
            .with_condition(!label.trim().is_empty())
            .build();

        checker
            .add_rule()
            .with_title("Label must start with a capital letter")
            .with_message(DESCRIPTION)
            .with_property(label)
            .with_condition(
                label
                    .as_str()
                    .chars()
                    .next()
                    .map(|c| c.is_uppercase())
                    .unwrap_or(true),
            )
            .build();
    }

    if let Some(summary) = &manifest.summary {
        const DESCRIPTION: &str = "The `summary` field contains an optional brief description of the module’s purpose and functionality. This SHOULD be a single sentence that clearly explains what the module does.";

        if !flags.is_template {
            checker
                .add_rule()
                .with_title("Summary must be changed from the template")
                .with_message(DESCRIPTION)
                .with_property(summary)
                .with_condition(*summary != "ASIMOV Template Module")
                .build();
        }

        checker
            .add_rule()
            .with_title("Summary must not be empty")
            .with_message(DESCRIPTION)
            .with_property(summary)
            .with_condition(!summary.trim().is_empty())
            .build();

        checker
            .add_rule()
            .with_title("Summary must start with a capital letter")
            .with_message(DESCRIPTION)
            .with_property(summary)
            .with_condition(
                summary
                    .as_str()
                    .chars()
                    .next()
                    .map(|c| c.is_uppercase())
                    .unwrap_or(true),
            )
            .build();

        checker
            .add_rule()
            .with_title("Summary must end with a period or exclamation mark")
            .with_message(DESCRIPTION)
            .with_property(summary)
            .with_condition(matches!(
                summary.as_str().chars().last(),
                Some('.') | Some('!')
            ))
            .build();
    }
}

pub fn run<S, F>(content: S, error_fn: F, flags: RunFlags) -> Result<bool>
where
    S: AsRef<str>,
    F: Fn(&Rule),
{
    let content = content.as_ref();
    let lines = content.lines().collect::<Vec<_>>();
    let manifest: Manifest = from_yaml(0, content)?;

    let mut checker = Checker::new();
    add_rules(&mut checker, lines, &manifest, flags);

    let has_error = checker.run(error_fn);
    Ok(has_error)
}

pub fn run_from_file<S, F>(path: S, error_fn: F, flags: RunFlags) -> Result<bool>
where
    S: AsRef<std::path::Path>,
    F: Fn(&Rule),
{
    let content = std::fs::read_to_string(path)?;
    run(content, error_fn, flags)
}
