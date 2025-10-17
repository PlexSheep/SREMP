# Dev Log

## Foreword

To try and keep the motivation high and improve project transparency, I am
starting a development log. This document contains some information about the
development sessions of the authors. In case this project becomes actually
bigger, this will likely get archived as impractical to maintain, but while it's
just me, I think it makes sense.

The first entry in this logbook is not the first session of developing this
project.

Also, since I'm a night person, I will count sessions continuing into the next
day as part of the previous one. It's just more convenient that way.

## 2025-10-17 (plexsheep)

### Progress

- Refactored the gui a bit
- First tofu trust dialog + sremp doesnt crash after connection established
- The connected chat actually appears in the gui

### Decisions

- Purely functional gui isn't great. Having special widgets in structs is neat.
- Start the listener on 33399 so that i dont always have to write a random
  port. This should be a random port in the actual release.

### Notes

### Mood

- A bit annoyed at GUI code but it does work

## 2025-10-16 (plexsheep)

It's been a while since I last coded on SREMP. I wasn't really busy, more like
I was busy doing nothing. Playing some games, going on some activities with
friends and family, getting my 12 to 16 hours of sleep a day, the stuff you do
when you have a month of mostly free time after studying and working for years.

Last time, I found the problem that we try to use Ed25519 Keys for the noise
static key, but the noise static key must be an X25519 Key. Today, I will try to
implement the changed identity and key management so that we actually use
cryptographically compatible keys.

### Progress

- Added the fields to identity and added verification and signature mechanism
- Added some access methods and such, looks really professional
- Split the identity module into more manageable smaller ones
- Changed how identity is used in the core crate
- Applied refactoring for the changed identity structure everywhere else
- Added a simple noise static key rotation function
- The problem still exists: `Public key of peer (127.0.0.1:54036) is inval id: signature error: Cannot decompress Edwards point`
- The first full handshake and identity exchange has been made! Commit: `9a97f81c928e5fbbe5e509f8c0dac5891a0c2b72`
- Put a lot of things in `Arc`s so that we can send them around for cheap.

### Decisions

- Data to be verified must be serialized with MessagePack. That way, we dont
  need to use `std::mem::transmute` and it's probably more consistent than
  in-memory representation, maybe?
- Larger data that should be sent over events and commands should always be in
  an `Arc`. The source of truth should probably also just contain Arced
  data. See #32
- Decided to arc the UserIdentity too. Hope this does not come back to bite me
  in the ass when I change it's fields.

### Notes

Shortened log of the first handshake by one of the peers:

```

[2025-10-16T16:25:41Z INFO  sremp_core::domain] Network domain has been started
[2025-10-16T16:25:41Z INFO  sremp_client::domain] Application domain has started
[2025-10-16T16:25:41Z TRACE sremp_core::domain] sremp_core::domain::NetworkDomain::run::{{closure}}
[2025-10-16T16:25:41Z TRACE sremp_gtk::domain::listen] sremp_gtk::domain::listen::<impl sremp_gtk::domain::UiDomain>::fmt_listen_status -> "
Listener is not active"
[2025-10-16T16:25:45Z INFO  sremp_gtk::domain] Sending ui command: Set working copy of user identity to E5D7A449F2E5CEE86CF8D252114EEA6366C5
0D8AB2694DB85FFA5E51E0C22C21 (plex)
[2025-10-16T16:25:45Z TRACE sremp_gtk::domain] ui command sent
[2025-10-16T16:25:45Z TRACE sremp_client::domain::jobs] sremp_client::domain::jobs::<impl sremp_client::domain::ClientDomain>::process_ui_co
mmand::{{closure}}
[2025-10-16T16:25:45Z INFO  sremp_client::domain::jobs] Processing Ui Command: Set working copy of user identity to E5D7A449F2E5CEE86CF8D252
114EEA6366C50D8AB2694DB85FFA5E51E0C22C21 (plex)
[2025-10-16T16:25:45Z TRACE sremp_client::domain::jobs] sremp_client::domain::jobs::<impl sremp_client::domain::ClientDomain>::set_identity:
:{{closure}}
[2025-10-16T16:25:45Z TRACE sremp_core::domain::jobs] sremp_core::domain::jobs::<impl sremp_core::domain::NetworkDomain>::process_network_co
mmand::{{closure}}
[2025-10-16T16:25:45Z INFO  sremp_core::domain::jobs] Processing Network Command: Set working copy of user identity to E5D7A449F2E5CEE86CF8D
252114EEA6366C50D8AB2694DB85FFA5E51E0C22C21 (plex)
[2025-10-16T16:25:45Z INFO  sremp_gtk::jobs] Processing network event: working copy of user identity was set to E5D7A449F2E5CEE86CF8D252114E
EA6366C50D8AB2694DB85FFA5E51E0C22C21 (plex)
[2025-10-16T16:25:45Z TRACE sremp_gtk::jobs] borrowing mutable ui domain state
[2025-10-16T16:25:45Z INFO  sremp_gtk::domain] Created new user identity for username 'plex': E5D7A449F2E5CEE86CF8D252114EEA6366C50D8AB2694D
B85FFA5E51E0C22C21
[2025-10-16T16:25:45Z DEBUG sremp_gtk::gui::identity] Showing identity created success window
[2025-10-16T16:26:05Z INFO  sremp_gtk::domain] Sending ui command: Connect to 127.0.0.1:46207
[2025-10-16T16:26:05Z TRACE sremp_gtk::domain] ui command sent
[2025-10-16T16:26:05Z TRACE sremp_client::domain::jobs] sremp_client::domain::jobs::<impl sremp_client::domain::ClientDomain>::process_ui_co
mmand::{{closure}}
[2025-10-16T16:26:05Z INFO  sremp_client::domain::jobs] Processing Ui Command: Connect to 127.0.0.1:46207
[2025-10-16T16:26:05Z TRACE sremp_client::domain::jobs] sremp_client::domain::jobs::<impl sremp_client::domain::ClientDomain>::connect::{{cl
osure}}
[2025-10-16T16:26:05Z TRACE sremp_core::domain::jobs] sremp_core::domain::jobs::<impl sremp_core::domain::NetworkDomain>::process_network_co
mmand::{{closure}}
[2025-10-16T16:26:05Z INFO  sremp_core::domain::jobs] Processing Network Command: Connect to 127.0.0.1:46207
[2025-10-16T16:26:05Z TRACE sremp_core::domain::jobs] sremp_core::domain::jobs::<impl sremp_core::domain::NetworkDomain>::connect_to::{{clos
ure}}
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection] sremp_core::net::connection::Connection::connect_to::{{closure}}
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection] sremp_core::net::connection::P2PConnection::connect_to::{{closure}}
[2025-10-16T16:26:05Z DEBUG sremp_core::net::connection] Tcp Connection Established
[2025-10-16T16:26:05Z DEBUG sremp_core::net::connection] Beginning noise handshake as initiator
[2025-10-16T16:26:05Z DEBUG sremp_core::net::connection] Sending Noise: `XX: --> e`
[2025-10-16T16:26:05Z DEBUG sremp_core::net::connection::frame] Sending Frame
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Sending Length
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Sending version
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Sending Data
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Sending Finished
[2025-10-16T16:26:05Z DEBUG sremp_core::net::connection] Receiving: `XX: <-- e, ee, s, es`
[2025-10-16T16:26:05Z DEBUG sremp_core::net::connection::frame] Receiving Frame
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Reading Length
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Length: 110
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Reading version
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Version: SREMP_DIRECT v0.1
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Reading Data
[2025-10-16T16:26:05Z DEBUG sremp_core::net::connection] Sending Noise: `XX: --> s, se`
[2025-10-16T16:26:05Z DEBUG sremp_core::net::connection::frame] Sending Frame
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Sending Length
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Sending version
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Sending Data
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Sending Finished
[2025-10-16T16:26:05Z DEBUG sremp_core::net::connection] Finished noise handshake
[2025-10-16T16:26:05Z DEBUG sremp_core::net::connection] Finished noise handshake
[2025-10-16T16:26:05Z DEBUG sremp_core::net::connection] Sending identity to peer
[2025-10-16T16:26:05Z DEBUG sremp_core::net::connection::frame] Sending Frame
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Sending Length
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Sending version
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Sending Data
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Sending Finished
[2025-10-16T16:26:05Z DEBUG sremp_core::net::connection] Receiving identity from peer
[2025-10-16T16:26:05Z DEBUG sremp_core::net::connection::frame] Receiving Frame
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Reading Length
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Length: 269
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Reading version
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Version: SREMP_DIRECT v0.1
[2025-10-16T16:26:05Z TRACE sremp_core::net::connection::frame] Reading Data
[2025-10-16T16:26:05Z DEBUG sremp_core::net::connection] Received (unverified) Identity: Identity {...}
[2025-10-16T16:26:05Z DEBUG sremp_core::identity::crypto] Verifying Identity B2BE45C042A564F4884FD34F6BC4C400FC710027DC149BF4A4D6DDD81592924D.
[2025-10-16T16:26:05Z DEBUG sremp_core::identity::crypto] Signature is valid
[2025-10-16T16:26:05Z DEBUG sremp_core::identity::crypto] Username is valid
[2025-10-16T16:26:05Z DEBUG sremp_core::net::connection] Noise Handshake and identity exchange with peer 127.0.0.1:46207 successful
[2025-10-16T16:26:05Z TRACE sremp_core::domain::jobs] sremp_core::domain::jobs::<impl sremp_core::domain::NetworkDomain>::init_connection::{{closure}}
[2025-10-16T16:26:05Z DEBUG sremp_core::domain::jobs] Initializing TLS connection for 127.0.0.1:46207
[2025-10-16T16:26:05Z INFO  sremp_core::domain] Emitting net event: Connection established with 127.0.0.1:46207 (B2BE45C042A564F4884FD34F6BC4C400FC710027DC149BF
4A4D6DDD81592924D)
[2025-10-16T16:26:05Z TRACE sremp_client::domain::jobs] sremp_client::domain::jobs::<impl sremp_client::domain::ClientDomain>::process_net_event::{{closure}}
[2025-10-16T16:26:05Z INFO  sremp_client::domain::jobs] Processing Net Event: Connection established with 127.0.0.1:46207 (B2BE45C042A564F4884FD34F6BC4C400FC710
027DC149BF4A4D6DDD81592924D)
[2025-10-16T16:26:05Z INFO  sremp_client::domain] Emitting ui event: Connection established with 127.0.0.1:46207 (B2BE45C042A564F4884FD34F6BC4C400FC710027DC149B
F4A4D6DDD81592924D)
[2025-10-16T16:26:05Z INFO  sremp_gtk::jobs] Processing network event: Connection established with 127.0.0.1:46207 (B2BE45C042A564F4884FD34F6BC4C400FC710027DC14
9BF4A4D6DDD81592924D)
[2025-10-16T16:26:05Z WARN  sremp_gtk::jobs] Received unimplemented Ui event: Connection established with 127.0.0.1:46207 (B2BE45C042A564F4884FD34F6BC4C400FC710
027DC149BF4A4D6DDD81592924D)
```

### Mood

- A bit sleepy but actually well rested
- In the zone about identities
- Happy because handshake and identity exchange works now :)

## 2025-10-03 (plexsheep)

### Progress

- I think I fixed the clippy ci, which sometimes went boom when pushing automatic changes
- The identity exchange fails because the specification is wrong: We try to
  use an Ed25519 Key (identity key we called it so far) for the noise static
  key, but that seems to be the wrong key type (need X25519 apperently?)
- I made a lot of progress with the specification. The identity is now better
  specified and should actually hold up in reality. The security is also
  improved with cryptographic signatures over the identities.

### Decisions

- Needed to change identity specification because Ed25519 keys cannot be used as
  static noise keys (without major hacks).

### Notes

### Mood

- A bit tired

## 2025-10-02 (plexsheep)

### Progress

- I'm still trying to debug that nasty deadlock when trying to create an
  identity. The UI Domain can't be gotten mutably for some reason.
- Yay i fixed the deadlock :) It was just some stupid lock being held in the
  event processor job, while we also tried to borrow mutably. Easy bug but somehow
  i missed it for quite some time.
- The listener status in the gtk crate was not actually updated when a listener
  start event was being processed. Fixed that.
- We got the half handshake working again, but the handler for the incomming
  connection does not spawn, not even when running two processes, since gtk
  spawns only one process even if we run it twice.
- After fixing #19, I got the first noise handshake between two separate
  processes running on my local machine :) The identity exchange worked
  partially: `Public key of peer (127.0.0.1:44961) is inval id: signature error`
- Added the `VersionHeader` to the `Frame`, but i think it somehow fucked up the
  length calculation or deserialization somewhere.

### Decisions

- We cannot simultaneously handle an incoming connection and connect to a remote
- Therefore, I am switching some methonds in core to use a `NetworkDomainSync`
  instead of a `&[mut] NetworkDomain`.
- I also want to enable running the program as two separate processes if
  possible #19.

### Notes

I'm attaching the log of the first handshake because I want to do so and you
can't stop me.

```
[2025-10-02T21:16:13Z INFO  sremp_core::domain] Network domain has been started
[2025-10-02T21:16:13Z INFO  sremp_client::domain] Application domain has started
[2025-10-02T21:16:13Z TRACE sremp_core::domain] sremp_core::domain::NetworkDomain::run::{{closure}}
[2025-10-02T21:16:13Z TRACE sremp_gtk::domain::listen] sremp_gtk::domain::listen::<impl sremp_gtk::domain::UiDomain>::fmt_listen_status -> "Listener is not acti
ve"
[2025-10-02T21:16:45Z INFO  sremp_gtk::domain] Sending ui command: Set working copy of user identity to B9AF56313C1D681C7506D449A6C7E76EBE95CBCC5317F8F7CE66E260
BCE491A5 (plexsheep)
[2025-10-02T21:16:45Z TRACE sremp_gtk::domain] ui command sent
[2025-10-02T21:16:45Z TRACE sremp_client::domain::jobs] sremp_client::domain::jobs::<impl sremp_client::domain::ClientDomain>::process_ui_command::{{closure}}
[2025-10-02T21:16:45Z INFO  sremp_client::domain::jobs] Processing Ui Command: Set working copy of user identity to B9AF56313C1D681C7506D449A6C7E76EBE95CBCC5317
F8F7CE66E260BCE491A5 (plexsheep)
[2025-10-02T21:16:45Z TRACE sremp_client::domain::jobs] sremp_client::domain::jobs::<impl sremp_client::domain::ClientDomain>::set_identity::{{closure}}
[2025-10-02T21:16:45Z TRACE sremp_core::domain::jobs] sremp_core::domain::jobs::<impl sremp_core::domain::NetworkDomain>::process_network_command::{{closure}}
[2025-10-02T21:16:45Z INFO  sremp_core::domain::jobs] Processing Network Command: Set working copy of user identity to B9AF56313C1D681C7506D449A6C7E76EBE95CBCC5
317F8F7CE66E260BCE491A5 (plexsheep)
[2025-10-02T21:16:45Z INFO  sremp_gtk::jobs] Processing network event: working copy of user identity was set to B9AF56313C1D681C7506D449A6C7E76EBE95CBCC5317F8F7
CE66E260BCE491A5 (plexsheep)
[2025-10-02T21:16:45Z TRACE sremp_gtk::jobs] borrowing mutable ui domain state
[2025-10-02T21:16:45Z INFO  sremp_gtk::domain] Created new user identity for username 'plexsheep': B9AF56313C1D681C7506D449A6C7E76EBE95CBCC5317F8F7CE66E260BCE49
1A5
[2025-10-02T21:16:45Z DEBUG sremp_gtk::gui::identity] Showing identity created success window
[2025-10-02T21:17:05Z INFO  sremp_gtk::domain] Sending ui command: Connect to 127.0.0.1:44961
[2025-10-02T21:17:05Z TRACE sremp_gtk::domain] ui command sent
[2025-10-02T21:17:05Z TRACE sremp_client::domain::jobs] sremp_client::domain::jobs::<impl sremp_client::domain::ClientDomain>::process_ui_command::{{closure}}
[2025-10-02T21:17:05Z INFO  sremp_client::domain::jobs] Processing Ui Command: Connect to 127.0.0.1:44961
[2025-10-02T21:17:05Z TRACE sremp_client::domain::jobs] sremp_client::domain::jobs::<impl sremp_client::domain::ClientDomain>::connect::{{closure}}
[2025-10-02T21:17:05Z TRACE sremp_core::domain::jobs] sremp_core::domain::jobs::<impl sremp_core::domain::NetworkDomain>::process_network_command::{{closure}}
[2025-10-02T21:17:05Z INFO  sremp_core::domain::jobs] Processing Network Command: Connect to 127.0.0.1:44961
[2025-10-02T21:17:05Z TRACE sremp_core::domain::jobs] sremp_core::domain::jobs::<impl sremp_core::domain::NetworkDomain>::connect_to::{{closure}}
[2025-10-02T21:17:05Z TRACE sremp_core::net::connection] sremp_core::net::connection::Connection::connect_to::{{closure}}
[2025-10-02T21:17:05Z TRACE sremp_core::net::connection] sremp_core::net::connection::P2PConnection::connect_to::{{closure}}
[2025-10-02T21:17:05Z DEBUG sremp_core::net::connection] Tcp Connection Established
[2025-10-02T21:17:05Z DEBUG sremp_core::net::connection] Beginning noise handshake as initiator
[2025-10-02T21:17:05Z DEBUG sremp_core::net::connection] Sending Noise: `XX: --> e`
[2025-10-02T21:17:05Z DEBUG sremp_core::net::connection::frame] Sending Frame
[2025-10-02T21:17:05Z TRACE sremp_core::net::connection::frame] Sending Length
[2025-10-02T21:17:05Z TRACE sremp_core::net::connection::frame] Sending Data
[2025-10-02T21:17:05Z TRACE sremp_core::net::connection::frame] Sending Finished
[2025-10-02T21:17:05Z DEBUG sremp_core::net::connection] Receiving: `XX: <-- e, ee, s, es`
[2025-10-02T21:17:05Z DEBUG sremp_core::net::connection::frame] Receiving Frame
[2025-10-02T21:17:05Z TRACE sremp_core::net::connection::frame] Reading Length
[2025-10-02T21:17:05Z TRACE sremp_core::net::connection::frame] Length: 96
[2025-10-02T21:17:05Z TRACE sremp_core::net::connection::frame] Reading Data
[2025-10-02T21:17:05Z TRACE sremp_core::net::connection::frame] Data: [7d, 1d, fa, 8c, 53, ba, a3, 45, 5, 6e, 6, f5, 5, 2b, dc, bf, 8c, 35, f6, 58, 2, af, a1, e
6, f2, 33, 6d, 37, 7d, 6e, 48, 13, 92, e7, fb, b4, c1, f4, 29, a6, aa, 3e, 68, 68, 68, 5c, 4a, 19, b1, c9, 25, 0, 7e, 4d, 9c, 22, 74, 9d, 35, 96, b5, 7c, 11, 66
, f0, f6, 37, 2c, f3, 79, 1b, c3, 58, 3e, 85, a, 4a, d, 2e, 74, e7, 33, 3, 73, c8, 85, 72, b, cf, ae, d1, 6c, 40, c0, 5f, 89]
[2025-10-02T21:17:05Z DEBUG sremp_core::net::connection] Sending Noise: `XX: --> s, se`
[2025-10-02T21:17:05Z DEBUG sremp_core::net::connection::frame] Sending Frame
[2025-10-02T21:17:05Z TRACE sremp_core::net::connection::frame] Sending Length
[2025-10-02T21:17:05Z TRACE sremp_core::net::connection::frame] Sending Data
[2025-10-02T21:17:05Z TRACE sremp_core::net::connection::frame] Sending Finished
[2025-10-02T21:17:05Z DEBUG sremp_core::net::connection] Finished noise handshake
[2025-10-02T21:17:05Z DEBUG sremp_core::net::connection] Sending identity to peer
[2025-10-02T21:17:05Z DEBUG sremp_core::net::connection::frame] Sending Frame
[2025-10-02T21:17:05Z TRACE sremp_core::net::connection::frame] Sending Length
[2025-10-02T21:17:05Z TRACE sremp_core::net::connection::frame] Sending Data
[2025-10-02T21:17:05Z TRACE sremp_core::net::connection::frame] Sending Finished
[2025-10-02T21:17:05Z DEBUG sremp_core::net::connection] Receiving identity from peer
[2025-10-02T21:17:05Z DEBUG sremp_core::net::connection::frame] Receiving Frame
[2025-10-02T21:17:05Z TRACE sremp_core::net::connection::frame] Reading Length
[2025-10-02T21:17:05Z TRACE sremp_core::net::connection::frame] Length: 60
[2025-10-02T21:17:05Z TRACE sremp_core::net::connection::frame] Reading Data
[2025-10-02T21:17:05Z TRACE sremp_core::net::connection::frame] Data: [2d, db, 94, f9, 79, ea, 25, 7e, 4a, a5, ae, 60, c6, ef, 16, 9a, b7, 93, 57, 7e, 3f, 14, 7
, 50, 90, f7, 84, eb, 2e, e7, 84, e7, dc, f, ce, de, 6a, e6, fe, 7d, 82, 2e, 1e, 4f, 42, 8b, b, a0, c1, 47, de, b1, 8a, c9, fb, a0, 20, ad, 4b, 3a]
[2025-10-02T21:17:05Z WARN  sremp_core::net::connection] Error while handling a Connection, cutting the TcpStream: Public key of peer (127.0.0.1:44961) is inval
id: signature error
```

The other peer has a similar log of course.

### Mood

- Frustrated by deadlocks
- Happy about the first proper handshake having worked

## 2025-09-25 (plexsheep)

### Progress

- I think the identity gets passed down the domains now
- Found another deadlock in the gtk crate, but I don't yet know how to fix it

### Decisions

### Notes

### Mood

- Watching some videos nearby, have a lot of free time right now which is cool
- Writing gui code is cond of tedious, but I guess I need a proper gui

## 2025-09-23 (plexsheep)

### Progress

- I found out why the main loop of the core crate deadlocks
- Worked on processing the network events in the application domain and the ui domain

### Decisions

### Notes

### Mood

- Had a good day. Coding is fun

## 2025-09-20 (plexsheep)

### Progress

- I'm currently reworking the gtk crate for the 3 domain architecture. Which
  isn't too easy, since I need to change a lot of code.
- After the 3 domain refactor, the application loads up again
- After all this work... Now i have a deadlock again. In the client crate.

### Decisions

- the error variants of async_channel send errors were rather large, so they are
  now stored as boxes in `CoreError` and `ClientError`.

### Notes

### Mood

- A bit sleepy, but had a good day
- Frustrated about deadlocks

## 2025-09-19 (plexsheep)

### Progress

- Thought about the software architecture for a few days. What we have is
  crap and makes deadlocks and bugs easy.
- Wrote a concept for that 3 domain architecture. Yeah, I didn't write it myself
  but used an LLM. I want to code, not write tons of text, okay?!

### Decisions

- Decided on the 3 domain layout: Network domain, Application domain, UI Domain.
- The domains communicate with specific interfaces only: Commands and events. Commands go down and events go up.
- Application domain goes into a client crate. All clients may use the client domain and simply program another UI domain, as clients mostly need to do the same.

### Notes

- I did the refactoring of client and core. No idea how to implement the system with the UI domain though.

### Mood

- Neutral, but Getting that UI domain to work with commands and events seems
  kind of difficult

## 2025-09-16 (plexsheep)

### Progress

- Specified the rust like notation
- The deadlock comes from the TCP listener in the job for it. The job gets
  a mutable reference to the TCP listener by locking the core state,
  meaning no other thread can use the core state. The bad news is that we need
  a mutable ref to that thing...
- WAIT `tokio::net::TcpListener::accept` does _not_ need a mutable reference!
  I can just use regular ones and it should work!
- I also added a timeout to the TCP listener job, so that there are points in
  time when no one holds a reference, so that getting a mutable reference (lock)
  is possible (which is needed by the identity creation GUI). #9
- I now got the first partial noise handshake! We start a listener, create an identity,
  then connect to our own listener. Sadly, the listener does not actually reply
  with noise protocol messages for the handshake yet for some reason.
- Ah amazing, it's another deadlock...

### Notes

- I need to do something about those deadlocks. The application starts hanging
  when some actions are combined, forever
- I already inline pretty much all lock actions, never hold across await
  (I deny that clippy warning actually), but still.
- Having Synchronous GUI code might be part of the issue. Does the whole tokio
  runtime block when I use block_on to get a lock?
- While I was able to fix my immediate deadlock problem, the architecture is
  suboptimal and will lead to more deadlocks. I should improve the state system in
  a way that makes deadlocks impossible or at least much less likely. I should
  also be sure to remember to add timeouts and be critical of await when I have
  a lock.

### Mood

Annoyed and frustrated at those damn deadlocks that I probably wouldn't have if
I could have multiple mutable references.

## 2025-09-15 (plexsheep)

### Progress

- Slop specification was written
- Removed non-critical stuff from the README, and text is now all authentic
- Fancy network stack diagram for spec
- Create identity in GUI #4

### Decisions

- Rebranded to SREMP because GRRSMP was way too unprofessional, and this project is weirdly important to me
- GitHub over git.cscherr.de (forgejo), for amazing CI infrastructure and discoverability
- Removed version and multi_device flag from identity in spec

### Notes

- Working with a specification is awesome actually! I can define ideas with natural language and refer to them without needing to code them first.
- It seems like we have some deadlocks in the application... Some may be from GTK, and some may be, even worse, from the core state.

### Mood

Motivation high, I'm not just writing git commits anymore. I am
_committing_. Maybe.
