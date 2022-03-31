# SVY 自用 CLI 工具

使用 Rust 实现

## 功能列表

| Command              | Options                                                                                     | Description                                                                                                                                                              | TODO                                          |
| -------------------- | ------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------- |
| `registry [NAME]`    | `--list, -l`<br /> `--set, -s`                                                              | 切换/查询 NPM 源<br /> `svy registry -l`,<br /> `svy registry taobao`,<br /> `svy registry taobao --set https://registry.npmmirror.com/`                                 |                                               |
| `check [keyword]...` |                                                                                             | 检查当前目录下的文件中是否含有关键词，会自动忽略.git, .gitignore 和.gitignore 中文件                                                                                     | `Regex::new("*.log")`会报错，目前做了忽略处理 |
| git                  | `-e, --set-email <NEW_EMAIL>`, <br />`-l, --list`, <br />`-n, --set-name <NEW_NAME>` <br /> | 切换 git config user.name 和 git config user.email<br />`svy git -l`<br />`svy git github -e mengyu6498@icloud.com`<br/>`svy git github -n qiyuor2`<br/>`svy git github` |                                               |
| create               |                                                                                             | todo                                                                                                                                                                     |                                               |

## 使用

```bash
git clone https://github.com/QiYuOr2/svy.git
cargo build --release
cargo install --path .

svy registry
```
