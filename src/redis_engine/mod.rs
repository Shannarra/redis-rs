use serde_json::{Result, Value};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod util;
use util::*;

mod command_execution;
use command_execution::*;


pub const DUMP_FILE_NAME: &str = "dump.my_rdb";

pub struct ExecutionContext {
    pub key_value_pairs: Arc<Mutex<HashMap<String, RedisValue>>>,
    lists: Arc<Mutex<HashMap<String, Vec<RedisValue>>>>,
    //std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, std::collections::HashMap<String, RedisValue>>>>
    hashes: Arc<Mutex<HashMap<String, HashMap<String, RedisValue>>>>
}

impl ExecutionContext {
    fn new() -> Self {
        Self {
            key_value_pairs: Arc::new(Mutex::new(HashMap::new())),
            lists: Arc::new(Mutex::new(HashMap::new())),
            hashes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn from_file_contents(text: String) -> Self {
        let mut context = ExecutionContext::new();
        let val: serde_json::Value = serde_json::from_str(&text).unwrap();

        let key_value_pairs = get_kvps_from_json("key_value_pairs".to_string(), &val);
        let lists = get_lists_from_json(&val);
        let hashes = get_hashes_from_json(&val);

        context.key_value_pairs = Arc::new(Mutex::new(key_value_pairs));
        context.lists = Arc::new(Mutex::new(lists));
        context.hashes = Arc::new(Mutex::new(hashes));

        context
    }
}

pub struct Executor {
    pub context: ExecutionContext,
    pub setup_properly: bool,
}


pub fn setup_executor() -> Executor {
    use std::{path, fs};

    let mut setup_properly = false;

    let path = path::Path::new(&DUMP_FILE_NAME);
    if !path.exists() {
        if let Err(_) = fs::File::create(&DUMP_FILE_NAME) {
            eprintln!("[ERROR]: Cannot create a dump file!");
        } else {
            let setup_data = r#"{
    "key_value_pairs": {

    },
    "lists": {

    },
    "hashes": {

    }
}"#;
            fs::write(&DUMP_FILE_NAME, setup_data);
        }
    } else {
        if let Ok(text) = fs::read_to_string(&DUMP_FILE_NAME) {
            let context = ExecutionContext::from_file_contents(text);
            setup_properly = true;

            if setup_properly {
                return Executor { context, setup_properly }
            }
        } else {
            eprintln!("[ERROR]: Cannot read dump file!");
        }
    }

    Executor::error_default()
}

impl Executor {
    fn error_default() -> Self {
        Self {
            context: ExecutionContext::new(),
            setup_properly: false
        }
    }

    fn exec_one_off(&self, command: &str, args: Vec<&str>) -> command_execution::Result {
        match command {
            "echo" => { one_off_command::echo(args) },
            "ping" => { one_off_command::ping(args) },
            "flushall" => {
                /*
                Delete all the keys of all the existing databases, not just the currently selected one.
                This command never fails.
                 */
                Ok("(nil)".to_string())
            },
            _ => { panic!("This will never be reached"); }
        }
    }

    async fn exec_kvp_command(&mut self, command: &str, args: Vec<&str>) -> command_execution::Result {
        //let mut kvp = &mut self.context.key_value_pairs.lock().unwrap();

        match command {
            "set"    => { kvp_command::set(&mut self.context.key_value_pairs.lock().unwrap()    , args) },
            "get"    => { kvp_command::get(&mut self.context.key_value_pairs.lock().unwrap()    , args) },
            "key"    => { kvp_command::key(&mut self.context.key_value_pairs.lock().unwrap()    , args) },
            "type"   => { kvp_command::r#type(&mut self.context.key_value_pairs.lock().unwrap() , args) },
            "del"    => { kvp_command::del(&mut self.context.key_value_pairs.lock().unwrap()    , args) },
            "unlink" => { kvp_command::unlink(&mut self.context.key_value_pairs                 , args.iter().map(|x| x.to_string()).collect()) },
            "expire" => { kvp_command::expire(&mut self.context.key_value_pairs, args.iter().map(|x| x.to_string()).collect()).await },
            "rename" => { kvp_command::rename(&mut self.context.key_value_pairs.lock().unwrap() , args) },
            _ => { panic!("This will never be reached"); }
        }
    }

    pub async fn exec(&mut self, command: String) -> command_execution::Result {

        let mut command_words = command.split(" ");
        let cmd_name = command_words.nth(0).unwrap();
        let cmd_args = command_words.collect::<Vec<_>>();

        let mut execution_value = match cmd_name {
            "echo" | "ping" | "flushall" => self.exec_one_off(cmd_name, cmd_args),
            "set" | "get" | "key" | "type" | "del" | "unlink" | "expire" | "rename"  => self.exec_kvp_command(cmd_name, cmd_args).await,

            _ => todo!()
        };

        execution_value
    }
}
