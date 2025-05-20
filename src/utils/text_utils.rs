use crate::clients::ticktick_client::TaskBody;
use regex::Regex;

pub fn parse_task(input: &str) -> TaskBody {
    // TODO: check regex for hashes in middle of words
    let tag_regex = Regex::new(r"#([\w-]+)").unwrap();

    let tags: Vec<String> = tag_regex
        .captures_iter(input)
        .map(|cap| cap[1].to_string())
        .collect();

    let title = tag_regex.replace_all(input, "").trim().to_string();

    TaskBody {
        title,
        id: None,
        tags: if tags.is_empty() { None } else { Some(tags) },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_tags_and_title_correctly() {
        let input = "Finish homework #school #urgent-task";
        let task = parse_task(input);

        assert_eq!(task.title, "Finish homework");
        assert_eq!(
            task.tags,
            Some(vec!["school".to_string(), "urgent-task".to_string()])
        );
    }

    #[test]
    fn handles_no_tags() {
        let input = "Just a regular task";
        let task = parse_task(input);

        assert_eq!(task.title, "Just a regular task");
        assert_eq!(task.tags, None);
    }

    #[test]
    fn trims_title_properly() {
        let input = "  Call mom  #family   ";
        let task = parse_task(input);

        assert_eq!(task.title, "Call mom");
        assert_eq!(task.tags, Some(vec!["family".to_string()]));
    }

    #[test]
    fn allows_numbers_and_dashes_in_tags() {
        let input = "Check server logs #dev-ops #2025Q2";
        let task = parse_task(input);

        assert_eq!(task.title, "Check server logs");
        assert_eq!(
            task.tags,
            Some(vec!["dev-ops".to_string(), "2025Q2".to_string()])
        );
    }

    #[test]
    fn ignores_hashes_in_middle_of_words() {
        let input = "Discuss #ideas and a#weirdcase";
        let task = parse_task(input);

        assert_eq!(task.title, "Discuss and a#weirdcase");
        assert_eq!(task.tags, Some(vec!["ideas".to_string()]));
    }
}
