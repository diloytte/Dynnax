use crate::constants::TWITTER_BOT_REGEX;

pub fn extract_twitter_sender(text: &str) -> Option<String> {
    TWITTER_BOT_REGEX
        .captures(text)
        .map(|caps| caps[1].trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twitter_bot_regex() {
        let cases = [
            ("🖼️🔄 Retweet from fz7", "fz7"),
            ("💬 Quote from patty_fi", "patty_fi"),
            ("📝 Tweet from Ga__ke", "Ga__ke"),
            ("🔁 Reply from johnny", "johnny"),
        ];

        for (input, expected_user) in cases {
            match TWITTER_BOT_REGEX.captures(input) {
                Some(caps) => {
                    assert_eq!(&caps[1], expected_user, "User mismatch on input: {}", input);
                }
                None => panic!("Failed on input: {}", input),
            }
        }
    }

    #[test]
    fn test_valid_inputs() {
        let cases = [
            ("🖼️🔄 Retweet from fz7", "fz7"),
            ("💬 Quote from patty_fi", "patty_fi"),
            ("📝 Tweet from Ga__ke", "Ga__ke"),
            ("🔁 Reply from johnny", "johnny"),
            ("🔄 Retweet from ElonMusk", "ElonMusk"),
            ("tweet from Satoshi", "Satoshi"),
            ("RETWEET from DOGE_maxi", "DOGE_maxi"), // case-insensitive test
            ("quote from vitalik.eth", "vitalik.eth"),
        ];

        for (input, expected) in cases {
            let actual = extract_twitter_sender(input);
            assert_eq!(
                actual,
                Some(expected.to_string()),
                "Input failed: {}",
                input
            );
        }
    }

    #[test]
    fn test_invalid_inputs() {
        let invalids = [
            "🖼️🔄 repost from fz7",        // not a keyword
            "📝 Message from Ga__ke",      // not a keyword
            "from patty_fi",               // missing action
            "Retweet by user123",          // wrong preposition
            "This is just a random tweet", // no structure
            "Quote: @john",                // wrong format
            "tweetform Ga__ke",            // typo
            "tweet from",                  // no username
        ];

        for input in invalids {
            let result = extract_twitter_sender(input);
            assert_eq!(result, None, "Should not match: {}", input);
        }
    }

    #[test]
    fn test_trimmed_output() {
        let input = "Retweet from  spaced_user  ";
        let result = extract_twitter_sender(input);
        assert_eq!(result, Some("spaced_user".to_string()));
    }
}
