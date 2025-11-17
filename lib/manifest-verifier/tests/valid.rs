use indoc::indoc;
use manifest_verifier::{RunFlags, run};

fn on_error_to_void(_: &manifest_verifier::Rule) {}

#[test]
fn test_valid() {
    const MANIFEST: &str = indoc! {"
        # See: https://asimov-specs.github.io/module-manifest/
        ---
        name: luma
        label: Luma
        title: ASIMOV Luma Module
        summary: ASIMOV module for importing luma events.
        links:
        - https://github.com/asimov-modules/asimov-luma-module
        - https://crates.io/crates/asimov-luma-module

        provides:
        programs:
            - asimov-luma-module
            - asimov-luma-fetcher

        handles:
        url_prefixes:
            - https://lu.ma/
    "};

    let flags = RunFlags {
        is_template: false,
        is_proprietary: true,
    };

    let has_error = run(MANIFEST, on_error_to_void, flags);
    assert_eq!(has_error.unwrap(), false);
}
