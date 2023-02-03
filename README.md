# Redis-Rs

A Redis clone created in Rust.
This is a university project and is NOT a full re-implementation of Redis.

## Features
This is a basic engine implementation, there will be a server that is wrapping the engine up and making it accessible via tcp.

There are 4 types of commands being run:
1. ### Redis Engine features

1.1. #### Single-run commands
    - [echo](https://redis.io/commands/echo/)
    - [ping](https://redis.io/commands/ping/)
    - [flushall](https://redis.io/commands/flushall/)

1.2 #### Key-Value operations
    - [set](https://redis.io/commands/set/)
    - [get](https://redis.io/commands/get/)
    - [key](https://redis.io/commands/key/)
    - [type](https://redis.io/commands/type/)
    - [del](https://redis.io/commands/del/)
    - [unlink](https://redis.io/commands/unlink/)
    - [expire](https://redis.io/commands/expire/)
    - [rename](https://redis.io/commands/rename/)

1.3. #### Lists
    - [llen](https://redis.io/commands/llen/)
    - [lrem](https://redis.io/commands/lrem/)
    - [lindex](https://redis.io/commands/lindex/)
    - [lpop](https://redis.io/commands/lpop/)
    - [rpop](https://redis.io/commands/rpop/)
    - [lpush](https://redis.io/commands/lpush/)
    - [rpush](https://redis.io/commands/rpush/)
    - [lset](https://redis.io/commands/lset/)

1.4. #### Hashes
    - [hget](https://redis.io/commands/hget/)
    - [hexists](https://redis.io/commands/hexists/)
    - [hdel](https://redis.io/commands/hdel/)
    - [hgetall](https://redis.io/commands/hgetall/)
    - [hkeys](https://redis.io/commands/hkeys/)
    - [hlen](https://redis.io/commands/hlen/)
    - [hmset](https://redis.io/commands/hmset/)
    - [hset](https://redis.io/commands/hset/)
    - [hvals](https://redis.io/commands/hvals/)

> You can click each link to read what each command is doing.

1.5  #### Autosave
Every 300 seconds (5 minutes) all of the data will be automatically persisted to the respective dump file (`dump.my_rdb` or `debug_dump.my_rdb` depending on the mode you spin the executor in.

Saving is being done in the background so you can modify your redis console or data as much as you'd want.

## How to use this
Since this is Cargo-compatible, you just run:
```sh
cargo r
```

This will run the server on the default port, which is `6379`.
If you want to run the server on a specific port you can provide the `--port` flag with an integer value.
```sh
cargo b && ./target/debug/my_redis_server --port 1234
```

If the port provided is already in use you will be greeted with the following message:
```
[ERROR]: Address already in use (os error 98) (address: localhost:1234)
```

To free this port under Linux you can just do:
```sh
lsof -ti tcp:1234 | xargs kill -9
```
This will kill all processes on the given port and free it up, so you can run the server on the port you'd like. :)

> Pressing CTRL+C or CTRL+Z will terminate the server, saving the data beforehand.

## Testing
There are a number of tests testing EACH command, run those by doing:
```sh
cargo t
```

## Dependencies
Just take a look at [Cargo.toml](./Cargo.toml), section "dependencies".

## Developers
- [Petar (Shannarra) Angelov](https://www.github.com/Shannarra)
