pub mod list {
    use crate::redis_engine::RedisValue;
    type Result = super::super::Result;
    type List = std::collections::HashMap<String, Vec<RedisValue>>;

    pub fn to_be_deleted(list: &mut List, args: Vec<&str>) -> Result {
        println!("List contents: {:?}", list);

        Err("DELETE ME".to_string())
    }

    pub fn llen(list: &mut List, args: Vec<&str>) -> Result {
        // https://redis.io/commands/llen/

        if args.len() != 1 {
            return Err("[ERROR]: \"llen\" accepts only one argument".to_string());
        }

        if !list.contains_key(args[0]) {
            // from docs:
            // If key does not exist, it is interpreted as an empty list and 0 is returned.
            Ok("0".to_string())
        } else {
            Ok(list[args[0]].len().to_string())
        }
    }

    pub fn lpush(list: &mut List, args: Vec<&str>) -> Result {
        // https://redis.io/commands/lpush/

        if args.len() < 2 {
            return Err("[ERROR]: At least two arguments required for \"llen\"!".to_string());
        }

        let mut vec = vec![];
        for v in &args[1..] {
            vec.push(RedisValue::from_str(v));
        }

        if !list.contains_key(args[0]) {
            list.insert(args[0].to_string(), vec);
        } else {
            for v in &args[1..] {
                list.get_mut(args[0]).unwrap().push(RedisValue::from_str(v));
            }
        }

        Ok(format!("{}", args.len() - 1))
    }

    pub fn lrem(list: &mut List, args: Vec<&str>) -> Result {
        // https://redis.io/commands/lpush/

        if args.len() < 3 {
            return Err("[ERROR]: At least three arguments required for \"llen\"!".to_string());
        }

        let name = args[0];
        let mut count = 0;
        let key = args[2];

        if let Err(_) = args[1].parse::<i32>() {
            return Err(format!("Second argument of \"lrem\" MUST be a whole number! Got \"{}\"", args[1]));
        } else {
            count = args[1].parse::<i32>().unwrap();
        }

        if !list.contains_key(name) {
            // Note that non-existing keys are treated like empty lists, so when key does not exist, the command will always return 0.
            return Ok("0".to_string());
        }

        let old_len = list[name].len();

        let mutable_list = list.get_mut(name).unwrap();
        let mut vec_cpy = vec![];

        if count == 0 {
            mutable_list
                .retain(|x| {
                    x.to_string() != key
                });
        } else if count < 0{
            let mut to_remove = -count;

            for item in mutable_list.iter().rev() {
                if to_remove > 0  && item.to_string() == key {
                    to_remove-=1;
                    continue;
                } else {
                    vec_cpy.push(item.clone());
                }
            }

            mutable_list.clear();
            // doing this:
            *mutable_list = vec_cpy.clone();
            // will save the list in reverse, so we need to reverse it back:
            mutable_list.reverse();
        } else {
            for item in mutable_list.iter() {
                if count > 0  && item.to_string() == key {
                    count-=1;
                    continue;
                } else {
                    vec_cpy.push(item.clone());
                }
            }

            mutable_list.clear();
            *mutable_list = vec_cpy.clone();
        }

        println!("Vec: {:?}", list[name]);

        Ok(format!("{}", old_len - list[name].len()))
    }
}
