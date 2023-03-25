use regex::Regex;

// Rules:
// 1. Start with an alphabet character
// 2. Have one or more alphanumeric characters, or `_` or `-`
// 3. End with an alphanumeric character
const PATTERN: &str = r"^[a-zA-Z][-_a-zA-Z0-9]+[a-zA-Z0-9]$";

fn matches_pattern(input_string: &str, pattern_var: &str) -> bool {
    let pattern_regex = Regex::new(pattern_var).unwrap();
    pattern_regex.is_match(input_string)
}
pub fn is_valid_name(name: &str) -> bool {
    let matches: bool = matches_pattern(name, PATTERN);
    if !matches {
        return false;
    }
    // Instead of making the regex more complicated,
    // lets handle the additional rules here
    // (there is only one at the moment)

    // we check if string contains `--` because the urls are delemeted with this.
    // So if a name has -- in it, that will break the url rules.
    if name.contains("--") {
        return false;
    }

    true
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
        assert_eq!(is_valid_name(input_string), should_match);
    }
}
