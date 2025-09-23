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
