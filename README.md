# Redis-Rs

A Redis clone created in Rust.
This is a university project and is NOT a full re-implementation of Redis.

## Features
This is a basic engine implementation, there will be a server that is wrapping the engine up and making it accessible via tcp.

There are 4 types of commands being run:

1. ### Single-run commands
    - [echo](https://redis.io/commands/echo/)
    - [ping](https://redis.io/commands/ping/)
    - [flushall](https://redis.io/commands/flushall/)

2. ### Key-Value operations
    - [set](https://redis.io/commands/set/)
    - [get](https://redis.io/commands/get/)
    - [key](https://redis.io/commands/key/)
    - [type](https://redis.io/commands/type/)
    - [del](https://redis.io/commands/del/)
    - [unlink](https://redis.io/commands/unlink/)
    - [expire](https://redis.io/commands/expire/)
    - [rename](https://redis.io/commands/rename/)

3. ### Lists
    - [llen](https://redis.io/commands/llen/)
    - [lrem](https://redis.io/commands/lrem/)
    - [lindex](https://redis.io/commands/lindex/)
    - [lpop](https://redis.io/commands/lpop/)
    - [rpop](https://redis.io/commands/rpop/)
    - [lpush](https://redis.io/commands/lpush/)
    - [rpush](https://redis.io/commands/rpush/)
    - [lset](https://redis.io/commands/lset/)

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

> You can click each link to read what each command is doing.

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
