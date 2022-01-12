mod cli;
use cli::CliCfg;

fn main() {
    let mut cli_cfg = CliCfg::new()
        .new_flag("-a".to_string())
        .new_flag("-f".to_string());
    cli_cfg.parse(); 
    if cli_cfg.is_flag_provided("-a".to_string()) {
        println!("WAS PROVIDED FLAG A");
    } else if cli_cfg.is_flag_provided("-f".to_string()) {
        println!("WAS PROVIDED FLAG F");
    }
    
        // .new_flag("v")
    
        // if cli_opt.is_flag("a") {
            //     println!("you provided flag '-a'");
            // }
            
    
}
