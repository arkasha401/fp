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
        self.flags.push(flag);
        self
    }

    pub fn parse(&mut self) {
        let us_arguments:Vec<String> = std::env::args().collect();
        for arg in us_arguments{
            if self.flags.contains(&arg){
                println!("flag provided {}",arg);
                if arg == "-a"{
                    self.provided_flags.insert(arg, false);
                    println!("how it's going?")
                }
                
                
                     
            } else if self.flags.contains(&arg) == false{
                println!("Error")
            }
        }

    }

}


    

// }
// #[derive(Debug)]
// struct Circle {
//     x: u64,
//     y: u64,
//     radius: u64
// }
// impl Circle {
//     fn new(oneside: u64, secondside: u64, r: u64) -> Circle {
//         Circle {
//             x:oneside,
//             y:secondside,
//             radius: r
//         }
//     }
//     fn area(&self) ->u64 {
//         3 * (self.radius * self.radius)
//     }
    
//     fn set_rad(&mut self, r: u64) {
//         self.radius = r;
//     }
// }
// pub fn proj(){
    
    // let kva = Circle::new(2,3,4);
    
    // println!("{}", kva.x)
    
    // }
