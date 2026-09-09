#[derive(Debug, PartialEq, Eq)]
enum ConfigError {
    MissingQuery,
    MissingContents,
    TooManyArguments,
}

#[derive(Debug, PartialEq, Eq)]
struct Config<'text> {
    query: &'text str,
    contents: &'text str,
    case_sensitive: bool,
}

impl<'text> Config<'text> {
    fn parse(arguments: &'text [&'text str]) -> Result<Self, ConfigError> {
        match arguments {
            [query, contents] => Ok(Self {
                query,
                contents,
                case_sensitive: true,
            }),
            [query, contents, "--ignore-case"] => Ok(Self {
                query,
                contents,
                case_sensitive: false,
            }),
            [] => Err(ConfigError::MissingQuery),
            [_] => Err(ConfigError::MissingContents),
            _ => Err(ConfigError::TooManyArguments),
        }
    }
}

fn search<'text>(query: &str, contents: &'text str) -> Vec<&'text str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'text>(query: &str, contents: &'text str) -> Vec<&'text str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

fn parse_and_search<'text>(arguments: &'text [&'text str]) -> Result<Vec<&'text str>, ConfigError> {
    let config = Config::parse(arguments)?;
    if config.case_sensitive {
        Ok(search(config.query, config.contents))
    } else {
        Ok(search_case_insensitive(config.query, config.contents))
    }
}

fn main() -> Result<(), ConfigError> {
    println!("Chapter 12: An I/O Project: Building a Command Line Program");
    println!(
        "input: deterministic values injected in memory (no process, file, env, or network I/O)"
    );
    let sample = [
        "duct",
        "Rust:\nsafe, fast, productive.\nDuct tape.",
        "--ignore-case",
    ];
    for line in parse_and_search(&sample)? {
        println!("case-insensitive match: {line}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Config, ConfigError, parse_and_search, search, search_case_insensitive};

    const SAMPLE: &str = "Rust:\nsafe, fast, productive.\nDuct tape.";

    #[test]
    fn config_parse_when_ignore_case_is_requested() {
        // Given: injected command-like values with the case option.
        let arguments = ["duct", SAMPLE, "--ignore-case"];
        // When: Config is built without reading process inputs.
        let config = Config::parse(&arguments);
        // Then: the deterministic search mode is case-insensitive.
        assert_eq!(
            config,
            Ok(Config {
                query: "duct",
                contents: SAMPLE,
                case_sensitive: false,
            })
        );
    }

    #[test]
    fn config_parse_when_contents_are_missing() {
        // Given: only a query value.
        let arguments = ["duct"];
        // When: Config parsing is attempted.
        let config = Config::parse(&arguments);
        // Then: the boundary error identifies the missing contents.
        assert_eq!(config, Err(ConfigError::MissingContents));
    }

    #[test]
    fn search_when_query_matches_a_line() {
        // Given: source text containing the query.
        let contents = SAMPLE;
        // When: a case-sensitive search is performed.
        let matches = search("duct", contents);
        // Then: the matching line is returned.
        assert_eq!(matches, vec!["safe, fast, productive."]);
    }

    #[test]
    fn search_case_insensitive_when_line_starts_with_an_uppercase_match() {
        // Given: text with both lowercase and uppercase query matches.
        let contents = SAMPLE;
        // When: a case-insensitive search is performed.
        let matches = search_case_insensitive("duct", contents);
        // Then: source order is retained for all matching lines.
        assert_eq!(matches, vec!["safe, fast, productive.", "Duct tape."]);
    }

    #[test]
    fn parse_and_search_when_parse_succeeds_composes_with_question_mark() {
        // Given: injected values requesting a case-insensitive search.
        let arguments = ["duct", SAMPLE, "--ignore-case"];
        // When: parsing and search are composed.
        let matches = parse_and_search(&arguments);
        // Then: the successful result contains deterministic matches.
        assert_eq!(matches, Ok(vec!["safe, fast, productive.", "Duct tape."]));
    }

    #[test]
    fn search_when_query_has_no_match() {
        // Given: deterministic in-memory text without the query.
        let contents = SAMPLE;
        // When: a case-sensitive search is performed.
        let matches = search("missing", contents);
        // Then: no lines are returned.
        assert!(matches.is_empty());
    }
}
