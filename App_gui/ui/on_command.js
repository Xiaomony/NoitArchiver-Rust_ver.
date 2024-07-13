function on_modArch() {

}

function on_usage() {
    const { invoke } = window.__TAURI__.tauri;
    invoke("run_command", { command:"usage" });
}