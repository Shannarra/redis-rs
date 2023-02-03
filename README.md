# Redis-Rs

A Redis clone created in Rust.
This is a university project and is NOT a full re-implementation of Redis.

## Features
This is a basic engine implementation, there will be a server that is wrapping the engine up and making it accessible via tcp.


There are 4 types of commands being run:
1. ### Redis Engine features
    1.1. __Single-run commands__
    <ul>
      <li><a href="https://redis.io/commands/echo/">echo</a></li>
      <li><a href="https://redis.io/commands/ping/">ping</a></li>
      <li><a href="https://redis.io/commands/flushall/">flushall</a></li>
    </ul>

    1.2 __Key-Value operations__
    <ul>
      <li><a href="https://redis.io/commands/set/">set</a></li>
      <li><a href="https://redis.io/commands/get/">get</a></li>
      <li><a href="https://redis.io/commands/key/">key</a></li>
      <li><a href="https://redis.io/commands/type/">type</a></li>
      <li><a href="https://redis.io/commands/del/">del</a></li>
      <li><a href="https://redis.io/commands/unlink/">unlink</a></li>
      <li><a href="https://redis.io/commands/expire/">expire</a></li>
      <li><a href="https://redis.io/commands/rename/">rename</a></li>
    </ul>

    1.3. __Lists__
    <ul>
      <li><a href="https://redis.io/commands/llen/">llen</a></li>
      <li><a href="https://redis.io/commands/lrem/">lrem</a></li>
      <li><a href="https://redis.io/commands/lindex/">lindex</a></li>
      <li><a href="https://redis.io/commands/lpop/">lpop</a></li>
      <li><a href="https://redis.io/commands/rpop/">rpop</a></li>
      <li><a href="https://redis.io/commands/lpush/">lpush</a></li>
      <li><a href="https://redis.io/commands/rpush/">rpush</a></li>
      <li><a href="https://redis.io/commands/lset/">lset</a></li>
    </ul>

    1.4. __Hashes__
    <ul>
      <li><a href="https://redis.io/commands/hget/">hget</li>
      <li><a href="https://redis.io/commands/hexists/">hexists</li>
      <li><a href="https://redis.io/commands/hdel/">hdel</li>
      <li><a href="https://redis.io/commands/hgetall/">hgetall</li>
      <li><a href="https://redis.io/commands/hkeys/">hkeys</li>
      <li><a href="https://redis.io/commands/hlen/">hlen</li>
      <li><a href="https://redis.io/commands/hmset/">hmset</li>
      <li><a href="https://redis.io/commands/hset/">hset</li>
      <li><a href="https://redis.io/commands/hvals/">hvals</li>
    </ul>

    1.5  __Autosave__
        Every 300 seconds (5 minutes) all of the data will be automatically persisted to the respective dump file `dump.my_rdb` or `debug_dump.my_rdb` depending on the mode you spin the executor in.

        Saving is being done in the background so you can modify your redis console or data as much as you'd want.

> You can click each link to read what each command is doing.

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
