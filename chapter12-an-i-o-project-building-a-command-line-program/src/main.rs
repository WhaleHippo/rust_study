#[derive(Debug, PartialEq, Eq)]
// 파싱 경계에서 가능한 실패 원인을 열거형으로 제한해 호출자가 모든 경우를 구분할 수 있게 한다.
enum ConfigError {
    MissingQuery,
    MissingContents,
    TooManyArguments,
}

#[derive(Debug, PartialEq, Eq)]
// `'text`는 쿼리와 내용이 원본 인수보다 오래 살 수 없음을 표현한다. Config는 문자열을 소유하지 않고 빌린다.
struct Config<'text> {
    query: &'text str,
    contents: &'text str,
    case_sensitive: bool,
}

impl<'text> Config<'text> {
    fn parse(arguments: &'text [&'text str]) -> Result<Self, ConfigError> {
        // 슬라이스 패턴은 인수 개수와 `--ignore-case`의 정확한 위치를 동시에 검사한다.
        // 어느 패턴에도 맞지 않는 입력은 각각의 오류 변형으로 경계에서 즉시 반환한다.
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
    // `lines()`가 만든 각 줄은 `contents`의 부분 슬라이스다. 반환 Vec는 새 문자열을 만들지 않고 그 참조만 보관한다.
    // 일치하는 줄이 하나도 없으면 `filter`는 아무 값도 내보내지 않으며, `collect`는 이를 정상적인 빈 Vec로 표현한다.
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'text>(query: &str, contents: &'text str) -> Vec<&'text str> {
    // `to_lowercase()`는 유니코드 대소문자 비교를 위해 새 `String`을 할당한다. 줄마다 임시 소문자 문자열을 만들지만,
    // 일치한 결과는 원본 `contents`에서 빌린 줄 그대로라 출력의 대소문자와 수명은 유지된다.
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

fn parse_and_search<'text>(arguments: &'text [&'text str]) -> Result<Vec<&'text str>, ConfigError> {
    // `?`는 파싱 성공 시 Config를 꺼내고, 실패 시 같은 ConfigError를 즉시 호출자에게 전파한다.
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
    // 실제 인수ㆍ파일ㆍ환경ㆍ네트워크 대신 고정 배열을 주입하므로 실행마다 같은 입력과 결과를 관찰한다.
    for line in parse_and_search(&sample)? {
        println!("case-insensitive match: {line}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Config, ConfigError, parse_and_search, search, search_case_insensitive};

    const SAMPLE: &str = "Rust:\nsafe, fast, productive.\nDuct tape.";
    // 고정 입력과 `lines()`의 원본 순서 보존 덕분에 일치 결과의 순서도 결정적이며, 일치가 없으면 빈 Vec가 된다.

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

    // [학습 실습: C12-01]
    // 목표: 일치가 없는 검색에서 빈 Vec가 오류가 아닌 필터링 결과임을 이해한다.
    // 학습자 행동: `search`에 일치가 없을 때 `matches.push("no matches")`로 폴백 줄을 잠시 추가한다.
    // RED: 워크스페이스 루트에서 `cargo test -p chapter12-an-i-o-project-building-a-command-line-program search_when_query_has_no_match`를 실행하면 빈 Vec 기대값과 폴백 줄이 달라 실패한다.
    // GREEN: 폴백을 제거하고 `filter`를 통과한 줄만 `collect`하여 빈 Vec를 복원한다.
    // 힌트: `filter` 뒤의 `collect::<Vec<_>>()`는 일치한 줄이 없을 때도 유효한 빈 Vec를 만든다.
    #[test]
    fn search_when_query_has_no_match() {
        // 준비: 질의가 없는 결정적 인메모리 텍스트를 준비한다.
        let contents = SAMPLE;
        // 실행: 대소문자를 구분해 검색한다.
        let matches = search("missing", contents);
        // 검증: 반환된 줄이 없어 빈 Vec가 된다.
        // 빈 결과도 오류가 아닌 정상 검색 결과이므로 `search`는 빈 Vec를 그대로 반환한다.
        // 같은 결과를 `parse_and_search`로 얻으면 파싱 성공을 나타내는 `Ok`가 이 Vec를 감싼다.
        assert!(matches.is_empty());
    }
}
