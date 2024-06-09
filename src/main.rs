#[macro_use(
    out, out_err, out_log, out_suc, out_warn, outln, outln_err, outln_log, outln_suc, outln_warn
)]
extern crate noita_archive_manager;
use noita_archive_manager::utils::io_manager::IOManager;
use noita_archive_manager::Manager;

mod io_commandline;
use io_commandline::IOCommandLine;

fn main() {
    let logger = IOCommandLine::new();
    let mut manager = Manager::new(&logger);
    while manager.is_running() {
        out!(logger, ">>>");
        let input = logger.io_getline();
        manager.run_command(&input);
    }
}
