use std::env;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub show_line_numbers: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigError {
    MissingQuery,
    MissingFilePath,
    EmptyFilePath,
    TooManyArguments,
}
impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ConfigError::MissingQuery => "missing query",
            ConfigError::MissingFilePath => "missing file path",
            ConfigError::EmptyFilePath => "file path must not be empty",
            ConfigError::TooManyArguments => "too many arguments",
        };
        write!(f, "{message}")
    }
}
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, ConfigError> {
        args.next();
        let query = args.next().ok_or(ConfigError::MissingQuery)?;

        let file_path = args.next().ok_or(ConfigError::MissingFilePath)?;

        let show_line_numbers = env::var("SHOW_LINE_NUMBERS").is_ok();

        if file_path.is_empty() {
            return Err(ConfigError::EmptyFilePath);
        }

        if args.next().is_some() {
            return Err(ConfigError::TooManyArguments);
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            file_path,
            ignore_case,
            show_line_numbers,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    if config.show_line_numbers {
        let lines = if config.ignore_case {
            search_case_insensitive_with_line_numbers(&config.query, &contents)
        } else {
            search_with_line_numbers(&config.query, &contents)
        };
        for (number, line) in lines {
            println!("{number}:{line}");
        }
    } else {
        let lines = if config.ignore_case {
            search_case_insensitive(&config.query, &contents)
        } else {
            search(&config.query, &contents)
        };
        for line in lines {
            println!("{line}");
        }
    }

    Ok(())
}
/// 返回包含查询词的原始行,区分大小写,保持原有顺序
///
/// # Example
/// ```
/// use minigrep::search;
///
/// let contents = "Rust is fast.\nPython is readable.";
/// let results = search("Rust", contents);
///
/// assert_eq!(results, vec!["Rust is fast."]);
/// ```
///
/// 返回的行切片借自 contents, 不复制每行的字符串内容
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lowercase_query = query.to_lowercase();
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(lowercase_query.as_str()) {
    //         results.push(line);
    //     }
    // }
    // results
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(lowercase_query.as_str()))
        .collect()
}

pub fn search_with_line_numbers<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .map(|(index, line)| (index + 1, line))
        .collect()
}

pub fn search_case_insensitive_with_line_numbers<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<(usize, &'a str)> {
    let lowercase_query = query.to_lowercase();
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(lowercase_query.as_str()))
        .map(|(index, line)| (index + 1, line))
        .collect()
}
#[cfg(test)]
mod tests {

    use super::{
        Config, ConfigError, search, search_case_insensitive,
        search_case_insensitive_with_line_numbers, search_with_line_numbers,
    };

    #[test]
    fn find_lines_containing_query() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn returns_empty_when_no_lines_match() {
        let query = "Python";
        let contents = "Rust:\nsafe, fast, productive.";

        assert!(search(query, contents).is_empty());
    }

    #[test]
    fn returns_matching_lines_in_original_order() {
        let query = "Rust";
        let contents = "Rust is fast.\nPython is readable.\nI like Rust.";

        assert_eq!(
            vec!["Rust is fast.", "I like Rust."],
            search(query, contents)
        );
    }

    #[test]
    fn ignore_case() {
        let query = "rUsT";
        let contents = "Rust is fast.\nTrust me.\nRUST is useful.\nPython is readable.";

        assert_eq!(
            vec!["Rust is fast.", "Trust me.", "RUST is useful."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn missing_path() {
        let args = ["minigrep", "nobody"];
        assert!(Config::build(args.into_iter().map(String::from)).is_err());
    }

    #[test]
    fn parses_query_and_file_path() {
        let args = ["minigrep", "nobody", "poem.txt"];
        let config = Config::build(args.into_iter().map(String::from)).expect("完整参数应解析成功");
        assert_eq!(config.query, "nobody");
        assert_eq!(config.file_path, "poem.txt");
    }

    #[test]
    fn return_err_when_query_missing() {
        let args = ["minigrep"];
        // assert!(Config::build(args.into_iter().map(String::from)).is_err());
        let error = Config::build(args.into_iter().map(String::from))
            .err()
            .expect("缺少查询参数应返回错误");
        assert_eq!(error, ConfigError::MissingQuery);
    }

    #[test]
    fn rejects_extra_arguments() {
        let args = ["minigrep", "nobody", "poem.txt", "extra"];
        assert!(Config::build(args.into_iter().map(String::from)).is_err());
    }

    #[test]
    fn reject_empty_file_path() {
        let args = ["minigrep", "nobody", ""];
        assert!(Config::build(args.into_iter().map(String::from)).is_err());
    }

    #[test]
    fn empty_query_matches_all_lines() {
        let query = "";
        let contents = "Rust\n\nPython";
        assert_eq!(vec!["Rust", "", "Python"], search(query, contents));
    }

    #[test]
    fn preserves_original_line_numbers() {
        let query = "Rust";
        let contents = "Rust\nPython\nRust again";
        assert_eq!(
            vec![(1, "Rust"), (3, "Rust again")],
            search_with_line_numbers(query, contents)
        )
    }

    #[test]
    fn empty_line_with_line_numbers() {
        let query = "Rust";
        let contents = "Python\n\nRust";
        assert_eq!(vec![(3, "Rust")], search_with_line_numbers(query, contents))
    }

    #[test]
    fn ignores_case_and_preserves_line_numbers() {
        let query = "rUsT";
        let contents = "Python\nRust is fast.\n\nTrust me.\nRUST is useful.";
        assert_eq!(
            vec![
                (2, "Rust is fast."),
                (4, "Trust me."),
                (5, "RUST is useful.")
            ],
            search_case_insensitive_with_line_numbers(query, contents)
        )
    }
}
