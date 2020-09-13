use crate::command::Command;
use crate::store_info::read_store_info;

pub struct Get {
    key: String,
}

impl Get {
    pub fn new(args: Vec<&str>) -> Get {
        if args.len() != 2 {
            return Get {
                key: "".to_string(),
            };
        }
        Get {
            key: args.get(1).unwrap().to_string(),
        }
    }
}

impl Command for Get {
    fn exec(&self) {
        // JSONファイルから既存分を取得
        let saved_si = read_store_info();

        if let Some(v) = saved_si.kvs.get(&self.key) {
            println!("{}", v);
        };
    }
}
