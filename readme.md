# Sopaste

Rust port of Soder's Pastebin.


# Introduction

Soder's Pastebin was originally created by [Fredrik Soderlund] in 2007. The
original code runs on a now unsupported version of PHP, and its dependencies
have not been upgraded for quite some time. The pastebin is still under active
use despite this, so a gentle sunsetting is in order.

The goal of this project is to create a read-only version of Soder's Pastebin,
mimicking its original style and functionality. The plan is to go live as soon
as the minimal read-only features have been implemented. The creation of new
pastes will not be supported, at least not initially.

[Fredrik Soderlund]: https://github.com/sodr


# Running

This project requires a recent version of [Rust]. Simply invoke cargo to build
and start the local development server.

```
$ cargo run
```

When running, the pastebin should be available on [localhost]. Make sure to use
a release build instead for deployment to a public server.

```
$ cargo build --release
```

The executable server binary will be placed in the `target` directory.

[Rust]: https://www.rust-lang.org/
[localhost]: http://127.0.0.1:8000/


# License

Licensed under GNU Affero General Public License v3.0.

- Copyright (C) 2008, Fredrik Soderlund.
- Copyright (C) 2021, Joakim Soderlund.

See `license.txt` for more information.
