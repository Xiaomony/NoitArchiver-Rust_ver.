function run_command(command) {
    const event = window.__TAURI__.event;
    event.emit("run_command", command);
}

function on_modArch() {
    let list = get_checked();
    if (list.length!=1) {
        opr_addlog_warn("需要且仅能选中一个存档进行修改");
        return;
    }
    window.get_modarch_info.style.display = "block";
    window.in_modarch_name.value = "";
    window.in_modarch_note.value = "";
}
function confirm_modArch() {
    let list = get_checked();
    var command = "ma " + list[0].toString() + ` "` + window.in_modarch_name.value + `" "` + window.in_modarch_note.value + `"`;
    run_command(command);
    hide_infogetter();
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
    refresh_arch_container();
}

function on_rsave() {
    let list = get_checked();
    if (list.length!=1) {
        opr_addlog_warn("需要且仅能选中一个存档进行覆盖");
        return;
    }
    run_command("rsave " + list[0].toString());
    refresh_arch_container();
}

function confirm_save() {
    var command = `save "` + window.in_save_name.value + `" "` + window.in_save_note.value + `"`;
    run_command(command);
    hide_infogetter();
}
//-----------------load------------------------
function on_load() {
    let list = get_checked();
    if (list.length!=1) {
        opr_addlog_warn("需要且仅能选中一个存档读取");
        return;
    }
    run_command("load " + list[0].toString());
    refresh_arch_container();
}

function on_qload() {
    run_command("qload");
}
//-----------------del------------------------
function on_del() {
    let list = get_checked();
    if (list.length<=0) {
        opr_addlog_warn("未选中存档");
        return;
    }
    run_command("del " + checked_tostr(list));
    refresh_arch_container();
}

function on_qdel() {
    run_command("qdel");
    refresh_arch_container();
}
//-----------------favor------------------------
function on_favor() {
    let list = get_checked();
    if (list.length<=0) {
        opr_addlog_warn("未选中存档");
        return;
    }
    list.forEach((item) => {
        run_command("favor " + item.toString());
    });
    refresh_arch_container();
}

function on_unfavor() {
    let list = get_checked();
    if (list.length<=0) {
        opr_addlog_warn("未选中存档");
        return;
    }
    list.forEach((item) => {
        run_command("unfavor " + item.toString());
    })
    refresh_arch_container();
}


function get_checked() {
    var list = document.getElementsByClassName("info_checkbox");
    let checked = [];
    for (let i=0;i<list.length;i++) {
        if (list[i].checked) {
            checked.push(i + 1);
        }
    }
    return checked;
}

function checked_tostr(list) {
    return list.join(' ');
}