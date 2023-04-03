//! polyvalid is written to provide a single source of truth for validating
//! package names, usernames, namespace names, app names.

//! The library can then be used from python and JS.
mod bindings;
use anyhow::{bail, Result};
use regex::Regex;

// Rules:
// 1. Start with an alphabet character
// 2. Have one or more alphanumeric characters, or `_` or `-`
// 3. End with an alphanumeric character
const APP_PATTERN: &str = r"^[a-zA-Z][-_a-zA-Z0-9]+[a-zA-Z0-9]$";

fn matches_pattern(input_string: &str, pattern_var: &str) -> bool {
    let pattern_regex = Regex::new(pattern_var).unwrap();
    pattern_regex.is_match(input_string)
}

/// Checks if `name` is a valid identifier for username, package name, namespaces, and app names.
///
/// # Examples
///
/// ```rust
/// use polyvalid::is_app_name_valid;
///
///
/// let name = "ayush";
/// let result = is_app_name_valid(name.to_string());
///
/// assert!(result.is_ok());
///
/// ```
pub fn is_app_name_valid(name: String) -> Result<()> {
    if name.len() < 3 {
        bail!(format!(
            "App name `{name}` needs to be atleast 3 characters long."
        ))
    }
    let matches: bool = matches_pattern(&name, APP_PATTERN);
    if !matches {
        bail!(format!(
            r#"App name `{name}` does not conform to wasmer's app-name pattern.
The name should start with an alphabetical character,
followed by one or more alphanumeric characters or hyphen/underscore characters,
and end with an alphanumeric character.
Please modify the name to match the pattern."#
        ))
    }
    // Instead of making the regex more complicated,
    // lets handle the additional rules here
    // (there is only one at the moment)

    // we check if string contains `--` because the urls are delemeted with this.
    // So if a name has -- in it, that will break the url rules.
    if name.contains("--") {
        bail!(format!("The app name `{name}` cannot contain `--`. Please modify your app name to match the pattern."))
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
        // valid cases
        case::valid_start_with_uppercase("ValidName1", true),
        case::valid_all_small_letters("helloworld", true),
        case::valid_all_capital_letters("HELLOWORLD", true),
        case::valid_start_and_end_with_capital_letter("HelloworlD", true),
        case::valid_start_with_letter_all_nums("d981273", true),
        case::valid_capitalized_letters_seperated_by_dash("Valid-Name", true),
        case::name_with_dash_in_middle("hello-world", true),
        case::invalid_double_underscore_in_middle("hel__lo", true),
        // invalid cases
        case::invalid_starts_with_number("1InvalidName", false),
        case::invalid_name_with_at_symbol("Inva@lidName", false),
        case::invalid_name_with_exclamation_mark("InvalidName!", false),
        case::invalid_no_whitespace(" hello ", false),
        case::invalid_no_dashes_at_start("-hello", false),
        case::invalid_no_dashes_at_end("hello-", false),
        case::invalid_no_dashes_at_start_and_end("-hello-", false),
        case::invalid_no_underscorees_at_start("_hello", false),
        case::invalid_no_underscorees_at_end("hello_", false),
        case::invalid_double_dash_at_start("--hello", false),
        case::invalid_double_dash_at_end("hello--", false),
        case::invalid_double_dash_in_middle("hel--lo", false),
        case::invalid_double_dash_at_start("__hello", false),
        case::invalid_double_dash_at_end("hello__", false),
        case::invalid_no_underscorees_at_start_and_end("_hello_", false)
    )]
    fn validation_test(#[case] input_string: &str, #[case] should_match: bool) {
        assert_eq!(
            is_app_name_valid(input_string.to_string()).is_ok(),
            should_match
        );
    }
}
