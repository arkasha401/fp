use std::env;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CliCfg {
    flags:Vec<String>,
    provided_flags: HashMap<String, bool>
}

impl CliCfg{
    pub fn new() -> CliCfg{
        CliCfg{
            flags:Vec::new(),
            provided_flags:HashMap::new()
        }
    }


    pub fn new_flag(mut self, flag: String) -> CliCfg{
        self.flags.push(flag.clone());
        self.provided_flags.insert(flag, false);
        self
    }

    pub fn parse(&mut self) {
        let us_arguments:Vec<String> = std::env::args().collect();
        for arg in us_arguments{
            if self.flags.contains(&arg){
                println!("flag provided {}",arg);
                if self.provided_flags.contains_key(&arg){
                    *self.provided_flags.get_mut(&arg).unwrap() = true;
                }
                
                
            }
        }
    }
}

