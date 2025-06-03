use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref TWITTER_BOT_REGEX: Regex =
        Regex::new(r"(?i)^\s*(?:\P{L}*\s*)?(\w+)\s+(Tweeted|Retweeted|Replied)").unwrap();
}
