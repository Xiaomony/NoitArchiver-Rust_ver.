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
            window.ArchiveInfos.innerHTML = "";
            infos.forEach(item => {
                let formattedString = `<pre>${item.date.join('-')} ${item.time.join(':')}\t${item.name}\t\t${item.note}\n</pre>`;
                window.ArchiveInfos.innerHTML += formattedString;
            });
        })

    } catch (error) {
        console.error("加载存档信息失败" , error);
    }
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
    window.OutputLogs.innerHTML += formattedString;
}
function opr_addlog_err(log) {
    formattedString = "<div class='output err'>"+log+"</div>";
    window.OutputLogs.innerHTML += formattedString;
}
function opr_addlog_warn(log) {
    formattedString = "<div class='output warn'>"+log+"</div>";
    window.OutputLogs.innerHTML += formattedString;
}
function opr_addlog_log(log) {
    formattedString = "<div class='output log'>"+log+"</div>";
    window.OutputLogs.innerHTML += formattedString;
}
function opr_addlog_suc(log) {
    formattedString = "<div class='output suc'>"+log+"</div>";
    window.OutputLogs.innerHTML += formattedString;
}

function doc_loaded() {
    loadComList();
    loadArchiveInfo();
    init_helpage();

    opr_addlog_common("common");
    opr_addlog_err("error");
    opr_addlog_warn("warn");
    opr_addlog_log("log");
    opr_addlog_suc("success");
}

addEventListener("DOMContentLoaded", () => { doc_loaded() });