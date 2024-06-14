
extern crate winres;

fn main()
{
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("./src/Giga Holy Bomb.ico"); // 替换为你的图标文件路径
        res.compile().unwrap();
    }
}