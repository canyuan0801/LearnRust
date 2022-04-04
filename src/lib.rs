

mod lifecycle;

/**
 *  
 *
 * @author  canyuan
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe,fast,productive.
        Pick three.";
        assert_eq!(
            vec!["safe,fast,productive."],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:\
safe,fast,productive.\
Pick three.\
Trust me.";

        assert_eq!(
            vec!["Rust", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    /**
    'a是一个显式生命周期
     */
    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();
        for line in contents.lines() {
            // do something with line
            if line.contains(query) {
                // 使用line执行某些操作
                results.push(line)
            }
        }
        results
    }

    pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let query = query.to_lowercase();
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line)
            }
        }

        results
    }
}