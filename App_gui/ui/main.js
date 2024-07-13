//import { shell } from '@tauri-apps/api';

function jumpTo(link) {
    var shell = window.__TAURI__.shell;
    shell.open(link);
}

function loadComList() {
    try {
        const { invoke } = window.__TAURI__.tauri;
        invoke("get_comlist").then( (comlist) => displayComInfos(comlist) )
    } catch (error) {
        alert("加载命令信息失败", error);
    }
}

function displayComInfos(comlist) {
    window.bt_cls.innerHTML = comlist[0].breif_info;
    window.bt_help.innerHTML = comlist[1].breif_info;

    window.bt_save.innerHTML = comlist[3].breif_info;
    window.bt_qsave.innerHTML = comlist[4].breif_info;
    window.bt_rsave.innerHTML = comlist[5].breif_info;

    window.bt_load.innerHTML = comlist[6].breif_info;
    window.bt_qload.innerHTML = comlist[7].breif_info;

    window.bt_ma.innerHTML = comlist[10].breif_info;
    
    window.bt_del.innerHTML = comlist[11].breif_info;
    window.bt_qdel.innerHTML = comlist[12].breif_info;
    
    window.bt_favor.innerHTML = comlist[13].breif_info;
    window.bt_unfavor.innerHTML = comlist[14].breif_info;

    window.bt_usage.innerHTML = comlist[15].breif_info;
}

function loadArchiveInfo() {
    try {
        const { invoke } = window.__TAURI__.tauri;
        invoke("get_archinfos").then( (infos) => {
            infos.forEach(item => {
                add_archinfo(item);
            });
        })

    } catch (error) {
        console.error("加载存档信息失败" , error);
    }
}

function add_archinfo(item) {
    var favored_str = "";
    if (item.is_favored) {
        favored_str = " favored";
    }
    var info_str = "";
    var formatted_time = item.time.map(unit => unit.toString().padStart(2, '0')).join(':');
    info_str += "<tr>";
    info_str += "<td class='infotab_checkbox'><input type='checkbox' name='select'/></td>";
    info_str += `<td class="infotab_data${favored_str}">${item.date.join('-')}</td>`;
    info_str += `<td class="infotab_time${favored_str}">${formatted_time}</td>`;
    info_str += `<td class="infotab_name${favored_str}">${item.name}</td>`;
    info_str += `<td class="infotab_note${favored_str}">${item.note}</td>`;
    info_str += "</tr>";
    window.Archinfo_tbody.innerHTML += info_str;
}

function init_helpage() {
    try {
        const { invoke } = window.__TAURI__.tauri;
        invoke("get_help_str").then( (help_str) => {
            window.helpage_content.innerHTML = help_str;
        })
        window.onclick = function(event) {
            var helpage = document.getElementById("help_page");
            var helpage_content = document.getElementById("helpage_content");
            var bt_github_link = document.getElementById("bt_github_link");
            var bt_bilibili_link = document.getElementById("bt_bilibili_link");
            console.log(window.help_page.style.display, event.target);
            var target=event.target;
            if (target != helpage && target != helpage_content && target != window.bt_help
                && target != bt_github_link && target != bt_bilibili_link
                && helpage.style.display != "none") {
                window.help_page.style.display = "none";
            }
        }
    } catch (error) {
        console.error("加载存档信息失败" , error);
    }
}

function opr_cls() {
    window.OutputLogs.innerHTML = "";
}

function opr_help() {
    window.help_page.style.display = "block";
}

function opr_addlog_common(log) {
    formattedString = "<div class='output common'>"+log+"</div>";
    const log_container = document.getElementById("OutputLogs");
    log_container.innerHTML += formattedString;
    log_container.scrollTop = log_container.scrollHeight;
}
function opr_addlog_err(log) {
    formattedString = "<div class='output err'>"+log+"</div>";
    const log_container = document.getElementById("OutputLogs");
    log_container.innerHTML += formattedString;
    log_container.scrollTop = log_container.scrollHeight;
}
function opr_addlog_warn(log) {
    formattedString = "<div class='output warn'>"+log+"</div>";
    const log_container = document.getElementById("OutputLogs");
    log_container.innerHTML += formattedString;
    log_container.scrollTop = log_container.scrollHeight;
}
function opr_addlog_log(log) {
    formattedString = "<div class='output log'>"+log+"</div>";
    const log_container = document.getElementById("OutputLogs");
    log_container.innerHTML += formattedString;
    log_container.scrollTop = log_container.scrollHeight;
}
function opr_addlog_suc(log) {
    formattedString = "<div class='output suc'>"+log+"</div>";
    const log_container = document.getElementById("OutputLogs");
    log_container.innerHTML += formattedString;
    log_container.scrollTop = log_container.scrollHeight;
}

function doc_loaded() {
    loadComList();
    loadArchiveInfo();
    init_helpage();
    add_listen();
}

addEventListener("DOMContentLoaded", () => { doc_loaded() });


function add_listen() {
    const event = window.__TAURI__.event;
    event.listen("out_common", (event) => { opr_addlog_common(event.payload); })
    event.listen("out_err", (event) => { opr_addlog_err(event.payload); })
    event.listen("out_warn", (event) => { opr_addlog_warn(event.payload); })
    event.listen("out_log", (event) => { opr_addlog_log(event.payload); })
    event.listen("out_suc", (event) => { opr_addlog_suc(event.payload); })
}