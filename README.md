# Telegram Bot with Teloxide

This is a simple Telegram bot created using the [Teloxide](https://github.com/teloxide/teloxide) library in Rust. The bot responds to any text message received with the phrase "Dijiste: ...", repeating what the user sent.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) installed.
- A Telegram bot created via [@BotFather](https://t.me/BotFather) to obtain the access token.
- A `.env` file configured with the bot token.

## Setup

1. **Create the `.env` file**

   In the root of the project, create a file named `.env` and add the following:

   ```env
   TELEGRAM_BOT_TOKEN=your-token-here
   RUST_LOG=info
   ```

   - `TELEGRAM_BOT_TOKEN`: The token obtained from [@BotFather](https://t.me/BotFather).
   - `RUST_LOG=info`: Configuration to display informational messages in the console.

2. **Install dependencies**

   Make sure to add the following dependencies to the `Cargo.toml` file:

   ```toml
   [dependencies]
   teloxide = { version = "0.9", features = ["macros"] }
   tokio = { version = "1", features = ["full"] }
   dotenv = "0.15"
   pretty_env_logger = "0.4"
   log = "0.4"
   ```

## Running

1. **Compile and run the bot**

   Run the following command to compile and execute the bot:

   ```sh
   cargo run
   ```

   If everything is set up correctly, you should see the message `Bot iniciado...` in the console.

2. **Interact with the bot**

   Send a message to the bot on Telegram, and it will reply by repeating the message with the prefix "Dijiste: ...".

## Code Explanation

- **Load configurations**: Uses `dotenv` to load the configurations from the `.env` file containing the bot token.
- **Initialize the bot**: Creates a bot with `Bot::new(bot_token)` using the token from the `.env` file. Then `auto_send()` is used to handle sending messages automatically.
- **Logger setup**: Configures a logger (`pretty_env_logger`) to display messages in the console, such as when the bot starts.
- **Dispatcher**: A `Dispatcher` is created to handle incoming messages. It routes each message to the appropriate function.
- **Function `responder_a_mensaje`**: This function handles each received message by responding with a simple "Dijiste: ...".

## Notes

- Do not share your bot token publicly, as it allows access to your bot.
- You can modify the `responder_a_mensaje` function to add more logic and make the bot more interactive.

## Common Issues

- **Token not found**: Ensure the `.env` file is properly configured and contains the correct token.
- **Logger not showing messages**: Ensure `RUST_LOG` is properly set (e.g., `RUST_LOG=info` or `RUST_LOG=debug`).

## Contributing

To contribute, you can fork the repository and submit a pull request with improvements or new features. Contributions are always welcome!

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.
