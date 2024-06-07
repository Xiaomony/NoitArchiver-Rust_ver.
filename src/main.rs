extern crate noita_archive_manager;
use noita_archive_manager::Manager;

fn main() {
    let a = Manager::new();
    a.run_command("del 1 bbbb");
}
