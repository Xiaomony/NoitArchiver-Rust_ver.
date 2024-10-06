#[macro_use(out, outln_err, outln_warn)]
//out_err, out_log, out_suc, out_warn, outln, outln_log, outln_suc
extern crate noitarchiver_core;

use noitarchiver_core::utils::io_manager::IOManager;
use noitarchiver_core::Manager;

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


    match Manager::new(&logger) {
        Ok(m) => manager = m,
        Err(e) => {
            outln_warn!(logger, "{}", e);
            puase();
            return;
        }
    }

    let args:Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let mut combined_args:Vec<String> = args.into_iter().skip(1).collect();
        // combined_args = combined_args.into_iter().map(|mut item| {
        //     if let Some(_) = item.find(' ') {
        //         item=format!("\"{}\"", item);
        //     }
        //     item
        // }).collect();
        combined_args.iter_mut().for_each(|item|{
            if let Some(_) = item.find(' ') {
                item.insert(0, '"');
                item.push('"');
            }
        });
        let combined_str = combined_args.join(" ");
        if let Err(e) = manager.run_command(&combined_str) {
            outln_err!(logger, "{}", e);
            puase();
        }
        return;
    }

    if let Err(msg) = manager.run_command("cls") {
        outln_warn!(logger, "初始化错误\n\t{}", msg);
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
