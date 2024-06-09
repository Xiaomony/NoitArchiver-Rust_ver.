#[macro_use(out, outln_err)]
//out_err, out_log, out_suc, out_warn, outln, outln_log, outln_suc, outln_warn
extern crate noita_archive_manager;
use noita_archive_manager::utils::io_manager::IOManager;
use noita_archive_manager::Manager;

mod io_commandline;
use io_commandline::IOCommandLine;

fn puase() {
    println!("Press Enter to exit ......");
}

fn main() {
    let logger = IOCommandLine::new();
    let mut manager;
    if let Ok(m) = Manager::new(&logger) {
        manager = m;
    } else {
        puase();
        return;
    }
    
    while manager.is_running() {
        out!(logger, ">>>");
        let input = logger.io_getline();
        if let Err(e) = manager.run_command(&input) {
            outln_err!(logger, "{}", e);
            puase();
            return;
        }
    }
}
