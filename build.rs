//use std::env;

extern crate winres;

fn main()
{

    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();

        let crate_name = env!("CARGO_PKG_NAME");
        let mut icon_path = "./src/Giga Holy Bomb.ico";
        match crate_name {
            "noitarchiver_cmd" => icon_path = "./src/Giga Holy Bomb.ico",
            _ => {}
        }
            
        res.set_icon(icon_path);
        res.compile().unwrap();
    }
}