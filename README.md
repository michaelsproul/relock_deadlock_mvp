Relocking Deadlock MVP
======================

This repo contains minimum viable programs for double locking bugs affecting `RwLock`,
prompted by https://github.com/sigp/lighthouse/issues/2245.

The program `bad.rs` naively uses two calls to `RwLock::read` on the same thread,
which can deadlock if a write is performed concurrently, per this execution:

```
$ cargo run --release --bin bad
start
t1 trying first read
t2 trying write
t1 trying second read
```

i.e. the deadlock occurs when thread 1 already holding the lock attempts to re-lock it,
at the same time that thread 2 is trying to obtain a write lock.

It's also possible for the "bad" program to run without deadlocking, e.g.

```
start
t1 trying first read
t1 trying second read
t1 obtained two read locks
t2 trying write
t2 obtained write lock
```

Running it in a loop will quickly surface the deadlock (try `./loop_bad.sh`).

In contrast, the program `good.rs` uses `RwLock::read_recursive` for the 2nd read lock and
will never deadlock (try `./loop_good.sh`).

The trade-off is that `read_recursive` is prone to starving writers.
