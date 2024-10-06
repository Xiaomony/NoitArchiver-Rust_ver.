@echo off
setlocal enabledelayedexpansion
chcp 65001>nul
set NoitaExe=noitarchiver_cmd.exe

::判断Noita存档器程序是否存在
if not exist %NoitaExe% (
    echo 本脚本只用于实现定时存档功能，需与Noita存档器本体配合使用

    echo 请将本脚本和Noita存档器放在同一文件夹下

    echo 若未下载存档器本体，请前往https://github.com/Xiaomony/NoitArchiver-Rust_ver./releases

    exit
)

if not "%1" == "" (goto time_saver)

:cls
%NoitaExe% cls
echo 设置定时存档使用"st + 时间间隔(分钟)"的形式(Noita的自动存档时间间隔为3min,建议设置的时间间隔大于3min)

echo "st 0"可用于关闭定时存档

echo ============================此条输出由单独的定时存档脚本创建===========================

::start command loop
:LOOP

set /p command=^>^>^>

if "!command!" == "q" (exit)
if "!command!" == "quit" (exit)
if "!command!" == "cls" (goto cls)

if "!command:~0,2!" == "st" (
    echo !command:~3!
    goto LOOP
)

%NoitaExe% %command%
goto LOOP
exit

:time_saver
echo %1
