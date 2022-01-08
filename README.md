# rodosol-telegram-bot

#### [Add this bot to your contacts list](https://t.me/rodosol_bot)

This bot is a quick scraper that gets the pictures from the rodosol "De olho na via" feature and sends it to the desired telegram channel.

## Usage

This bot is deployed into my Google Cloud account and is available to use without deployment.

Add the @rodosol_bot to your chat and use the following commands:

- `/tp_now`: sends pictures from the Terceira Ponte in the channel
- `/rodosol_now`: sends picture from Rodosol in the channel

You may also send these commands directly to the bot instead of adding it to a chat.

## Running it locally

Add a `.env` file with the `TELEGRAM_BOT_TOKEN` defined and run `cargo run`.

Or use `docker-compose up` or `docker run` manually with the `Dockerfile` provided.

You should be all set.

## Support

No support whatsoever is given to this application. Use it in your risk.

## Credits

Credits go to [@guilhermelimak](https://github.com/guilhermelimak) for creating the [first version of this bot years ago in Go](https://github.com/guilhermelimak/Terceira-ponte-bot).

I literally made it in a few hours while quarantining in the middle of a trip to Brazil because of COVID-19.