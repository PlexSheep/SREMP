<div align="center">

# 🦀 `GRRSMP`

**Great Routed Rust Secure Messaging Protocol**

_Relay Enhanced, secure, direct, messaging_

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Rust CI](https://github.com/PlexSheep/grrsmp/actions/workflows/cargo.yaml/badge.svg)](https://github.com/PlexSheep/grrsmp/actions/workflows/cargo.yaml)

| Component                                                          | API-Documentation                                     | Description                                | Version                                                         |
| ------------------------------------------------------------------ | ----------------------------------------------------- | ------------------------------------------ | --------------------------------------------------------------- |
| [📦 **`core`** ](https://crates.io/crates/grrsmp-core)             | [📖 Documentation](https://docs.rs/grrsmp-core)       | backend, networking and cryptography       | ![Crates.io](https://img.shields.io/crates/v/grrsmp-core)       |
| [📦 **`gtk`**](https://crates.io/crates/grrsmp-gtk)                | [📖 Documentation](https://docs.rs/grrsmp-gtk)        | GTK4-based desktop client                  | ![Crates.io](https://img.shields.io/crates/v/grrsmp-gtk)        |
| [📦 **`relay`** ](https://crates.io/crates/grrsmp-relay)           | [📖 Documentation](https://docs.rs/grrsmp-relay)      | temporary message storage, message routing | ![Crates.io](https://img.shields.io/crates/v/grrsmp-relay)      |
| [📦 **`rendezvous`** ](https://crates.io/crates/grrsmp-rendezvous) | [📖 Documentation](https://docs.rs/grrsmp-rendezvous) | Public list of contacts                    | ![Crates.io](https://img.shields.io/crates/v/grrsmp-rendezvous) |

</div>

## Introduction

`GRRSMP` is a secure messaging protocol for chats. It tries to be secure and
decentralized, with modern cryptography.

Your identity is a cryptographic key, and conversations ideally happen in a
direct (_peer-to-peer_) connection without any server involved. If a direct
connection cannot be made, you can use a relay server to connect indirectly.

Your messages are _always_ encrypted end to end, and with
_perfect forward secrecy_, messages can't even be decrypted when your identity
is compromised and all traffic you ever sent was saved (Unless the cryptographic
primitives are broken, but that is pretty unlikely).

The key difference from other chat systems is that the
components of its infrastructure are separated. There are 3 kinds of actors in
`GRRSMP`:

- **Clients**: These are your frontend. They store your messages for you
  to see, hold your cryptographic identity, and so on. Theoretically, that is
  all you need.
- **Relay Servers**: Clients can only receive messages when they are online. They also
  may have difficulties talking to other clients that are not on the same network.
  That is why _Relays_ exist. They are like a mailbox that accepts messages to
  you even when you are not online or directly reachable. They can't look at
  your message's contents.
- **Rendezvous Servers**: Since you probably don't know the network address of
  people you want to talk to, clients can register themselves in rendezvous
  servers. This server acts like a phone book, telling you where you need to
  establish a connection to in order to chat with who you're looking for. They
  don't even know that you send messages.

Each of these components can be self-hosted. You do not need to rely on
centralized infrastructure at all. You don't even need anything besides
clients if you can make a direct connection, for maximum paranoia.

## ❓ Why

> Why does this exist?

**Technology**

I believe that `GRRSMP` may offer a legitimate, albeit niche, advantage over
those amazing established protocols: It's distributed in the sense that
components can be self-hosted by anyone. You can host your own rendezvous
server (basically holds a contact list of who is online), your own relay server
(accepts your messages when you are offline and helps with NAT) and your own
clients too of course. That is its advantage over signal. Matrix and IRC just
don't feel right to me personally, so maybe that's an advantage over them too.

**Personal**

That aside, it all started when I wanted to learn GTK with Rust. I needed some project to
code a GUI for. I had had the idea to create a basic chat application for a long
time, just as an exercise or for fun, and so I started working on `grrsmp-gtk`.

I don't know why, but somehow, I got really involved in the backend. I think it
started when I discovered the amazing Noise Protocol Framework, then the double
ratchet algorithm for actual end-to-end encryption beyond the Transport Layer.

> What is the median amount of contributes per open source project? It's either one or zero. I'm not sure.

To be realistic, you should just use Signal. Signal is so good. Or Matrix, or
XMPP. But I have started building a little emotional connection to this
project. It's the first personal project that I _really_ care about (at least at
the time of writing. God I hope I don't abandon this project in two weeks, that
would be so corny.). I mean just look at the name. `GRRSMP`? Really?

I am developing this from a perspective that nobody will ever use this, and
especially not contribute to it. I will write a specification for myself,
Requests-For-Comments that will be only commented by me. But I want to do
it right. Worst case, I have a semi-professional Open-Source Project that
I learned a ton with.

**Thank you**

If you're reading this, I would really appreciate it if you gave this project
a star if you feel it matters. Or report issues, or if you are feeling extra
generous, I would appreciate code review, specification review, or patches the
most.

## 📖 Specification

I am still working on the first version of the `GRRSMP`
[specification](./docs/specification.md), but this document goes into the
technical details of how `GRRSMP` should work in theory.

## 🚧 Project Status

**Current Status**: Development (Alpha)

`GRRSMP` is currently in early development. The core protocol and basic P2P messaging
functionality are still being implemented, and the project is not yet ready
for real use.

## 🛠️ Development

### Prerequisites

- **Rust**: 1.85.1 or later (MSRV)
- **GTK4**: 4.10 or later
- **Platform**: Linux

Windows and macOS are currently not officially supported, but may work if you
install GTK4.

#### Linux (Debian/Ubuntu)

```bash
sudo apt-get update
sudo apt-get install libgtk-4-dev build-essential pkg-config
```

## 📜 License

`GRRSMP` is free software, and will always be free.

This project is licensed under the **GNU Affero General Public License v3.0** (AGPL-3.0).

This copyleft license ensures that:

- You can freely use, modify, and distribute this software
- Any derivative works must also be licensed under AGPL-3.0
- **Network services using this code must provide source code to users**
- The community benefits from all improvements

See the [LICENSE](LICENSE) file for the full license text.
