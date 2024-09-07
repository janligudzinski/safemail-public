# `Safemail`

`safemail` by Jan Ligudziński is a partial backend implementation of a hypothetical internet messaging protocol postulated by [Lars Wirzenius in his 2022 article "Re-thinking electronic mail"](https://liw.fi/rethinking-email/).

## Basic concept

The backend application implemented here allows its users to register accounts and log into them using no password known to the server, but using cryptographic signatures based on private keys known only to them; the server needs to know only the public signature and encryption keys of a user. Users may, on their own, issue signed stamps that authorize other users to send them mail over a given period of time

## System design

The system is designed according to Domain-Driven Design principles, with separate projects handling domain concerns and entity definitions, infrastructural concerns like handling database access and concrete cryptographic operations, application logic and a web API layer written in `axum`.

The database layer uses PostgreSQL through the `sqlx` library. Cryptographic operations are handled by the `ring` library. The application logic strictly applies CQRS principles, defining command and query types for each relevant entity.

## Features

- the platform is spam-resistant, nearly spam-proof by way of requiring either a preexisting stamp or a solved proof of work riddle, analogous to the systems used in cryptocurrencies, to make the sending of unsolicited mail costly yet still possible when receiving mail from strangers is desirable
- highly secure messaging: no message content or metadata other than the recipient's identifier and the global order in which it was received is kept in plaintext

## Possibilities for extension

- the implementation of a frontend is left as an exercise to interested developers