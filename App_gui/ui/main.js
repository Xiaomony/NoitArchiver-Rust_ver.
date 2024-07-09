
function loadComList() {
    try {
        const { invoke } = window.__TAURI__.tauri;
        invoke("get_comlist").then( (comlist) => displayComInfos(comlist) )
    } catch (error) {
        console.error("加载命令信息失败", error);
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

addEventListener("DOMContentLoaded", () => { loadComList() });