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
        let mut our_flags:Vec<String> = Vec::new();
        for i in arguments.iter().skip(1){;
            our_flags.push(i.to_string());
        }
        println!("{:?}", our_flags);
        our_flag.to_string();
        println!("{:?}", our_flags);
        for i in our_flags{
            if self.flags.contains(&i){
                println!("flag provided {}",i);
                *self.provided_flags.get_mut(&*i).unwrap() = true;

            } else {
                println!("Error!")
        }
        }
    }

    pub fn is_flag_provided(self, a_flag: String) -> bool {
        self.provided_flags.get(&a_flag).unwrap() == &true
    }

}

