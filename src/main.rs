mod cli;
use cli::CliCfg;

fn main() {
    let mut cli_cfg = CliCfg::new()
        .new_flag("-a".to_string())
        .new_flag("-f".to_string());
    let cli_opt = cli_cfg.parse(); 
        // .new_flag("v")
    
        // if cli_opt.is_flag("a") {
            //     println!("you provided flag '-a'");
            // }
            
    
}
