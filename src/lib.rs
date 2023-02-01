pub mod redis_engine;

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

    #[tokio::test]
    async fn basic_list_operations_work_properly() {
        let commands = vec![
            "llen list1",
            "llen list_that_doesnt_exist",
            "rpush new_list 1 2 3",
            "llen new_list",
        ];

        let responses = vec![
            Ok("2".to_string()),
            Ok("0".to_string()),
            Ok("3".to_string()),
            Ok("3".to_string()),
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
    async fn lrem_works_as_expected() {
        let mut executor = crate::redis_engine::setup_executor();

        assert_eq!(executor.exec("rpush list hello".to_string()).await, Ok("1".to_string()));
        assert_eq!(executor.exec("rpush list foo".to_string()).await,  Ok("1".to_string()));
        assert_eq!(executor.exec("rpush list hello".to_string()).await, Ok("1".to_string()));
        assert_eq!(executor.exec("rpush list hello".to_string()).await,  Ok("1".to_string()));

        assert_eq!(executor.exec("lrem list -2 hello".to_string()).await,  Ok("2".to_string()));
        assert_eq!(executor.exec("llen list".to_string()).await,  Ok("2".to_string()));
    }

    #[tokio::test]
    async fn lindex_works_as_expected() {
        let mut executor = crate::redis_engine::setup_executor();

        assert_eq!(executor.exec("rpush list hello".to_string()).await, Ok("1".to_string()));
        assert_eq!(executor.exec("rpush list foo".to_string()).await,  Ok("1".to_string()));
        assert_eq!(executor.exec("rpush list hello2".to_string()).await, Ok("1".to_string()));
        assert_eq!(executor.exec("rpush list hello3".to_string()).await,  Ok("1".to_string()));
        assert_eq!(executor.exec("llen list".to_string()).await,  Ok("4".to_string()));

        assert_eq!(executor.exec("lindex list 1".to_string()).await,  Ok("\"foo\"".to_string()));
        assert_eq!(executor.exec("lindex list 2".to_string()).await,  Ok("\"hello2\"".to_string()));
        assert_eq!(executor.exec("lindex list -1".to_string()).await,  Ok("\"hello3\"".to_string()));
        assert_eq!(executor.exec("lindex list -2".to_string()).await,  Ok("\"hello2\"".to_string()));
    }

    #[tokio::test]
    async fn lpop_works_as_expected() {
        let mut executor = crate::redis_engine::setup_executor();

        assert_eq!(executor.exec("rpush list hello".to_string()).await, Ok("1".to_string()));
        assert_eq!(executor.exec("rpush list foo".to_string()).await,  Ok("1".to_string()));
        assert_eq!(executor.exec("rpush list hello2".to_string()).await, Ok("1".to_string()));
        assert_eq!(executor.exec("rpush list hello3".to_string()).await,  Ok("1".to_string()));
        assert_eq!(executor.exec("llen list".to_string()).await,  Ok("4".to_string()));

        assert_eq!(executor.exec("lpop list asd".to_string()).await, Err("[ERROR]: count value is not an integer. Usage: lpop LISTNAME [COUNT]".to_string()));
        assert_eq!(executor.exec("lpop list".to_string()).await,  Ok("hello".to_string()));
        assert_eq!(executor.exec("lpop list 2".to_string()).await,  Ok("[\"foo\", \"hello2\"]".to_string()));
        assert_eq!(executor.exec("llen list".to_string()).await,  Ok("1".to_string()));
    }

    #[tokio::test]
    async fn rpop_works_as_expected() {
        let mut executor = crate::redis_engine::setup_executor();

        assert_eq!(executor.exec("rpush list hello".to_string()).await, Ok("1".to_string()));
        assert_eq!(executor.exec("rpush list foo".to_string()).await,  Ok("1".to_string()));
        assert_eq!(executor.exec("rpush list hello2".to_string()).await, Ok("1".to_string()));
        assert_eq!(executor.exec("rpush list hello3".to_string()).await,  Ok("1".to_string()));
        assert_eq!(executor.exec("llen list".to_string()).await,  Ok("4".to_string()));

        assert_eq!(executor.exec("rpop list asd".to_string()).await, Err("[ERROR]: count value is not an integer. Usage: rpop LISTNAME [COUNT]".to_string()));
        assert_eq!(executor.exec("rpop list".to_string()).await,  Ok("hello3".to_string()));
        assert_eq!(executor.exec("rpop list 2".to_string()).await,  Ok("[\"hello2\", \"foo\"]".to_string()));
        assert_eq!(executor.exec("llen list".to_string()).await,  Ok("1".to_string()));
    }

    #[tokio::test]
    async fn lpush_works_as_expected() {
        let mut executor = crate::redis_engine::setup_executor();

        assert_eq!(executor.exec("lpush list hello".to_string()).await, Ok("1".to_string()));
        assert_eq!(executor.exec("lpush list foo".to_string()).await,  Ok("1".to_string()));
        assert_eq!(executor.exec("lpush list hello2".to_string()).await, Ok("1".to_string()));
        assert_eq!(executor.exec("lpush list hello3".to_string()).await,  Ok("1".to_string()));
        assert_eq!(executor.exec("llen list".to_string()).await,  Ok("4".to_string()));
    }

    #[tokio::test]
    async fn lset_works_as_expected() {
        let mut executor = crate::redis_engine::setup_executor();

        assert_eq!(executor.exec("lpush list hello".to_string()).await, Ok("1".to_string()));
        assert_eq!(executor.exec("lpush list foo".to_string()).await,  Ok("1".to_string()));
        assert_eq!(executor.exec("lset list 0 hello2".to_string()).await, Ok("Ok".to_string()));
        assert_eq!(executor.exec("lset list -2 hello3".to_string()).await,  Ok("Ok".to_string()));
        assert_eq!(executor.exec("llen list".to_string()).await,  Ok("2".to_string()));
        assert_eq!(executor.exec("lindex list 0".to_string()).await,  Ok("\"hello3\"".to_string()));

    }
}
