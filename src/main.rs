use my_redis_server::redis_engine;
extern crate job_scheduler;
use job_scheduler::{JobScheduler, Job};

/*
{
  "key_value_pairs": {
    "name": "Petar",
    "age": 22
  },
  "lists": {
    "list1": [
      "value1",
      "value2"
    ],
    "list2": [
      "newvalue"
    ]
  },
  "hashes": {
    "hash1": {
      "name": "Petar",
      "age": 22
    }
  }
}

 */

#[tokio::main]
async fn main() {
    let commands = vec![
        // // --START KVP COMMANDS --
        "echo hi",
        "ping",
        "ping asd",
        "set kekw hi",
        "get kekw",
        "key kekw",
        "type kekw",
        "set dummy zero",
        "expire dummy 1",
        "get dummy",
        "set nametest 123",
        "rename nametest henlo",
        "get henlo",
        "set testvar this_is_a_test_string",
        // // --END KVP COMMANDS --
        // // --START LIST COMMANDS --
        "llen list1",
        "llen list_that_doesnt_exist",
        "lpush new_list 1 2 3 2 2",
        "llen new_list",
        "lrem new_list -2 2",
        "llen new_list",
        // // --END LIST COMMANDS --
        // // --START HASH COMMANDS --
        "hget hash1 name",
        "hset hash2 newfield 69 field_no_3 420"
        // // --END HASH COMMANDS --

    ];

    let mut sched = JobScheduler::new();
    let executor = redis_engine::setup_executor(false);

    if executor.setup_properly {
        println!("Executor setup :)");

        for command in &commands {
            println!("{:?}", &executor.exec(command.to_string()).await);
        }
    }

    let clone = executor.clone();
    sched.add(Job::new("1/3 * * * * *".parse().unwrap(), move || {
        clone.save();
    }));

    loop {
        sched.tick();

        let mut st = String::new();
        let stdin = std::io::stdin();

        stdin.read_line(&mut st).unwrap();
        println!("{:?}", executor.exec(st.trim().to_string()).await);
    }
}
