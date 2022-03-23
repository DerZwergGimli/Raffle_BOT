# DDRB - DisocrdRustRaffleBot

![bot-icon](./bot_icon.png)

#### This is an implementation of a Raffle Management bot for discord servers.

## Dependencies
It this bot is not standalone it will need a backend API.

## Commands

- Raffle Management
  - `/r add_raffle <title> <ticket_amount> <price_per_ticket> <description>`
- Ticket Management
  - `/r add_ticket <raffle_id> <spl-transaction>`
  - `/r remove_ticket <ticket_id>`