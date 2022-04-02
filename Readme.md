# DDRB - DisocrdRustRaffleBot

![bot-icon](./bot_icon.png)

#### This is an implementation of a Raffle Management bot for discord servers.

## Dependencies

It this bot is not standalone it will need a backend API.


---

## Commands

### Status Commands

| command        | result                                                       |
|----------------|--------------------------------------------------------------|
| `~status`      | prints info about status commands                            |
| `~status list` | prints out a short message about the raffles                 |
| `~status perm` | prints out a short message about the raffles but permanent   |
| `~status full` | prints out a full message about the raffles                  |

### Raffle Commands

| command           | result                            |
|-------------------|-----------------------------------|
| `~raffle`         | prints info about raffle commands |
| `~raffle add`     | adds a new raffle                 |
| `~raffle list`    | list all raffles                  |
| `~raffle delete`  | deletes a raffle                  |

### Ticket Commands

| command           | result                             |
|-------------------|------------------------------------|
| `~raffle`         | prints info about ticket commands  |
| `~raffle add`     | adds a new ticket                  |
| `~raffle list`    | list all ticket                    |
| `~raffle delete`  | deletes a ticket                   |

---

## Sample Commands:

1. Add a new raffle:
    1. `~raffle add <title> <ticket_price> <ticket_amount> <ticket_currency> "<description>"`
2. View your raffle:
    1. `~raffle list`
3. Add a Ticket:
    1. `~ticket add <raffle_id> <spl_tx_signature>`
4. View your ticket:
    1. `~ticket list`
5. Print a permanent Status view:
    1. `~status perm`

## Env
