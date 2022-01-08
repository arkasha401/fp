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
        let arguments:Vec<String> = std::env::args().collect();
        let our_flag = arguments.last().unwrap();
        our_flag.to_string();
        println!("{:?}", our_flag);
        if self.flags.contains(&our_flag){
            println!("flag provided {}",our_flag);
            *self.provided_flags.get_mut(&*our_flag).unwrap() = true;
                
        } else {
            println!("Error!")
        }
    }

    pub fn used_flags(self, a_flag: String) -> bool{
        if self.provided_flags.contains_key(&a_flag){
            true
        } else {
            false
        }
    }

}

