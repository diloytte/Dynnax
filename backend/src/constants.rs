use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref TWITTER_BOT_REGEX: Regex = Regex::new(
        r"(?i)\b(?:tweet|quote|reply|retweet)\sfrom\s(.+)"
    ).unwrap();
}

lazy_static! {
    pub static ref GLOBALY_BLOCKED_CAS:Vec<String> = vec![
        String::from("43SXvpf4c41t2uErsw7aL6w5qhnie6BXSSPqiTcTpump").to_lowercase(),
        String::from("FtUEW73K6vEYHfbkfpdBZfWpxgQar2HipGdbutEhpump").to_lowercase()
    ];
}