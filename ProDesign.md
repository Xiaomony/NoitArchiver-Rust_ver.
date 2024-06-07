# Noita Archive Manager(Rust ver.) Projcet Design Docs

## FileSystem Structor:
  - ```
    --./
      |---NoitaArchiveManager.exe
      |
      |---Archives
          |---infos.json
          |
          |---Archive1
          |---Archive2
          |---goodArchive
          |---...
    ```
  - Json格式:
    ```json
    {
      {
        "name": "%ArchiveName%",
        "note": "%ArchiveNote%",
        "date": [%year%, %month%, %day%],
        "time": [%hour%, %minute%, %second%]
      },
      ...
    }
    ```

## 代码结构:
  - `./src/main.rs`
    - ### **_读取键盘输入、命令行参数等并调用lib中内容_**
  - `./src/lib.rs`
    - ### **_Manager(对传入的命令调用ComAnalyzer、处理其返回的enum并调用FileManager中各个方法)_**
  - `./src/bin`
    - `OutLog.rs`
      - **_OutLog(trait 用于日志的输出)_**
    - `ComAnalyzer.rs`
      - **_Analyzer(将字符串分析、切割，转为枚举类型)_**
      - _CommandID(enum 存储分析后的命令的信息)_
    - `FileManager.rs`
      - **_FileManager(拷贝存档、删除文件、调用JsonSaver中各种方法……)_**
      - **_JsonSaver(只负责存储、读取infos.json)_**

      - **_ArchiveInfos(存储infos.json中加载出的信息)_**