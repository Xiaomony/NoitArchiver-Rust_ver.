function run_command(command) {
    const event = window.__TAURI__.event;
    event.emit("run_command", command);
}

function on_modArch() {

}

function on_usage() {
    run_command("usage");
}
//-----------------save------------------------
function on_save() {
    window.get_save_info.style.display = "block";
    window.in_save_name.value = "";
    window.in_save_note.value = "";
}

function on_qsave() {
    run_command("qsave");
}

function on_rsave() {

}

function confirm_save() {
    var command = "save \"" + window.in_save_name.value + "\" \"" + window.in_save_note.value;
    run_command(command);
    hide_infogetter();
}
//-----------------load------------------------
function on_load() {

}

function on_qload() {

}
//-----------------del------------------------
function on_del() {

}

function on_qdel() {
    run_command("qdel");
}
//-----------------favor------------------------
function on_favor() {

}

function on_unfavor() {

}


function get_checked() {
    var list = document.getElementsByClassName("info_checkbox");
    let checked = [];
    for (let i=0;i<list.length;i++) {
        if (list[i].checked) {
            checked.push(i-1);
        }
    }
    return checked;
}