use std::env;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = args.next().ok_or("Insufficient parameters")?;

        let file_path = args.next().ok_or("Insufficient parameters")?;

        if file_path.is_empty() {
            return Err("file path must not be empty");
        }

        if args.next().is_some() {
            return Err("too many arguments");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let lines = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in lines {
        println!("{line}");
    }
    Ok(())
}

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

pub fn search_case_insensitive_with_line_number<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<(usize, &'a str)> {
    vec![]
}
#[cfg(test)]
mod tests {

    use super::{Config, search, search_case_insensitive, search_with_line_numbers};

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
        assert!(Config::build(args.into_iter().map(String::from)).is_err());
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
    fn ignores_case_and_preserves() {
        let query = "rUsT";
        let contents = "Python\nRust is fast.\n\nTrust me.\nRust is useful.";
        assert_eq!(vec![(2, "")], search_with_line_numbers(query, contents))
    }
}
