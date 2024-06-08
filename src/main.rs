extern crate noita_archive_manager;
use noita_archive_manager::Manager;

mod io_commandline;
use io_commandline::IOCommandLine;

fn main() {
    let logger = IOCommandLine::new();
    let a = Manager::new(&logger);
    a.run_command("cls");
    a.run_command("save  爽种😊😊😊😊  \"aaa    aaa😍😍😍 阿斯蒂芬\"");
}
