extern crate noita_archive_manager;
use noita_archive_manager::Manager;

mod io_commandline;
use io_commandline::IOCommandLine;

fn main() {
    let logger = IOCommandLine::new();
    let mut a = Manager::new(&logger);
    //a.run_command("cls");
    //a.run_command("save  1  \"aaa aaa\"");
    //a.run_command("save 2 bb");
    //a.run_command("save 3 cc");
    a.run_command("del 3");
    a.run_command("ma 2 xx yy");
    a.run_command("log");
}
