# Redis-Rs

A Redis clone created in Rust.
This is a university project and is NOT a full re-implementation of Redis.

## Features
This is a basic engine implementation, there will be a server that is wrapping the engine up and making it accessible via tcp.

There are 4 types of commands being run:

1. ### Single-run commands
    - echo
    - ping
    - flushall

2. ### Key-Value operations
    - set
    - get
    - key
    - type
    - del
    - unlink
    - expire
    - rename

3. ### Lists
    - llen
    - lrem
    - lindex
    - lpop/rpop
    - lpush/rpush
    - lset

4. ### Hashes (TODO)
    - hget
    - hexists
    - hdel
    - hgetall
    - hkeys
    - hlen
    - hmset
    - hset
    - hvals


## How to use this
Since this is Cargo-compatible, you just run:
```sh
cargo r
```

## Testing
There are a number of tests testing EACH command, run those by doing:
```sh
cargo t
```

## Dependencies
Just take a look at [Cargo.toml](./Cargo.toml), section "dependencies".

## Developers
- [Petar (Shannarra) Angelov](https://www.github.com/Shannarra)
