use reqwest::Error;
use rss::{Channel, Item};
use serde_json::Value;
use memos::find_text_between_brackets;
use memos::edit_memo;
use memos::get_feed;

#[path = "../src/memos.rs"] mod memos;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        assert_eq!(1 + 1, 2);
    }

    #[tokio::test]
    async fn test_get_feed() {
        let result = get_feed().await;
        assert!(result.is_ok());
        let channel = result.unwrap();
        assert!(channel.items().len() > 0);
    }

    #[tokio::test]
    async fn test_edit_memo() {
        let item = Item::default(); // Create a default Item for testing
        let result = edit_memo(&item).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_find_text_between_brackets() {
        let text = "This is a [test] string with [multiple] brackets.";
        let result = find_text_between_brackets(&text.to_string());
        assert_eq!(result, vec!["test", "multiple"]);
    }
}