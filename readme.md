# Dynnax

![Dynnax Logo](frontend/public/dynnax.svg)

# IMPORTANT: Still in development.

## Overview

A self-hosted application that allows users to snipe Solana token addresses that are shilled in channels, groups or users.
The application leverages the Pumpfun Lightning Transaction API to automatically make trades when a valid Solana token address is detected.

## Features

- **Snipe Telegram Dialogs**: Snipe Channel shill, shill by user in a private group or DMs.

- **Snipe Twitter Post**: With the help of Redacted Systems Bot, whether using paid or free version, snipes token from Twitter/X post.

- **Manage Sniping Targets**: Dynamically modify how much should be the slippage, buy amount, and other parameters.


## Installation

- **TODO**: Everything related to Rust programming language & Postgres.

1. Clone the repository.

```
git clone git@github.com:diloytte/Dynnax.git
cd Dynnax
```

2. Add `.env` configuration. (Check bellow)

3. Run `cargo run` for development build.

4. Run `cargo build --release` for release build.

5. Start release build from `target` folder.

## Configuration

- **API_ID**: Your Telegram API ID.

- **API_HASH**: Your Telegram API hash.

- **PHONE_NUMBER**: The phone number associated with your Telegram account.

- **PASSWORD**: The password for logging into your Telegram account (if applicable).

- **PUMPFUN_PORTAL_API_KEY**: Your Pumpfun Lightning Transaction API key.

## Security

The application is designed with user privacy and security in mind. We do not store, share, or misuse any personal data provided in the .env file. Your API_ID, API_HASH, PHONE_NUMBER, PASSWORD, and PUMPFUN_PORTAL_API_KEY are only used locally to interact with the Telegram API and the Pumpfun Lightning Transaction API.

No external data storage: The application does not store user data or credentials in external databases or servers. Everything is processed locally.

No data sharing: Your sensitive data, such as Telegram credentials, will not be shared with third parties.

Local usage: The application is designed for local usage only, so no user data is transmitted over the internet, ensuring privacy and security.

# App help.

1. Running a full app (production build)
 - Run command: 
 ```
 cargo build --release --features production
 ```
2. Start the app with your local command: `./target/release/dynnax`

# Optional developer's message.

Many components in this project could have been simplified. For example, a database isn't strictly necessary — everything could have been handled with a collection of JSON configuration files. However, to turn this into a more comprehensive learning experience and to build something closer to a real-world product, I intentionally "overengineered" it.

Due to time constraints, if you’d like to try and test this app yourself, please feel free to do so — but kindly refrain from asking too many questions. I have a long to-do list to complete for this project and won’t be available to respond to questions, issues, or usage inquiries until the app is finished.