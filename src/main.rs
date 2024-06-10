#[macro_use(out, outln_err, outln_warn)]
//out_err, out_log, out_suc, out_warn, outln, outln_log, outln_suc
extern crate noita_archive_manager;
use noita_archive_manager::utils::io_manager::IOManager;
use noita_archive_manager::Manager;

mod io_commandline;
use io_commandline::IOCommandLine;

fn main() {
    let logger = IOCommandLine::new();
    let mut manager;

    let puase = || {
        outln_warn!(logger, "\n\nPress Enter to exit ......");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("fail to wait a press");
    };

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
