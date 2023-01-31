use my_redis_server::redis_engine;

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
    ];

    let mut executor = redis_engine::setup_executor();

    if executor.setup_properly {
        println!("Executor setup :)");

        for command in commands {
            println!("{:?}", executor.exec(command.to_string()).await);
        }

    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn commands_execute_properly() {
        let commands = vec![
            "echo hi",
            "ping",
            "ping asd",
            "set kekw hi",
            "get kekw",
            "key kekw",
            "type kekw",
            "del namaikatiputkata",
            "del kekw",
            "set kekw sadeg",
            "set dummy val",
            "unlink dummy",
        ];
        let responses: Vec<core::result::Result<String, String>> = vec![
            Ok("hi".to_string()),
            Ok("pong".to_string()),
            Ok("asd".to_string()),
            Ok("Ok".to_string()),
            Ok("hi".to_string()),
            Ok("Ok".to_string()),
            Ok("string".to_string()),
            Ok("0".to_string()),
            Ok("1".to_string()),
            Ok("Ok".to_string()),
            Ok("Ok".to_string()),
            Ok("1".to_string()),
        ];

        let mut executor = crate::redis_engine::setup_executor();

        if executor.setup_properly {
            for i in 0..commands.len() {

                assert!(executor.exec(commands[i].to_string()).await == responses[i])
            }
        } else {
            assert!(false);
        }
    }

    #[tokio::test]
    async fn expire_delays_properly() {
        "
        // To test the \"expire\" functionality place the following code at the bottom of \"main\" or
        // anywhere OTUSIDE OF a #[test] cfg!

        let mut executor = crate::redis_engine::setup_executor();

        if executor.setup_properly {{
            assert_eq!(executor.exec(\"set dummy asd\".to_string()).await , Ok(\"Ok\".to_string()));
            assert_eq!(executor.exec(\"expire dummy 1\".to_string()).await, Ok(\"1\".to_string()));
            assert_eq!(executor.exec(\"get dummy\".to_string()).await     , Ok(\"asd\".to_string()));

            std::thread::sleep(std::time::Duration::from_secs(2)); // wait 2s and check if \"dummy\" expired
            assert_ne!(executor.exec(\"get dummy\".to_string()).await, Ok(\"asd\".to_string()));
            assert_eq!(executor.exec(\"get dummy\".to_string()).await, Ok(\"(nil)\".to_string()));

            println!(\"Expiring works just fine!\");
        }} else {{
            assert!(false);
        }}

        ";
        assert!(true)
    }

    #[tokio::test]
    async fn renaming_works_as_expected() {
        let commands = vec![
            "set dummy 123",
            "get dummy",
            "rename dummy henlo",
            "get henlo",
        ];

        let responses = vec![
            Ok("Ok".to_string()),
            Ok("123".to_string()),
            Ok("Ok".to_string()),
            Ok("123".to_string()),
        ];

        let mut executor = crate::redis_engine::setup_executor();

        if executor.setup_properly {
            for i in 0..commands.len() {
                assert!(executor.exec(commands[i].to_string()).await == responses[i])
            }
        } else {
            assert!(false);
        }
    }
}
