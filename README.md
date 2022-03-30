# SVY 自用 CLI 工具

使用 Rust 实现

## 功能列表

| Command              | Options                   | Description                                                                                                            | TODO                                          |
| -------------------- | ------------------------- | ---------------------------------------------------------------------------------------------------------------------- | --------------------------------------------- |
| `registry [NAME]`    | `--list, -l`, `--add, -a` | 切换/查询 NPM 源 `svy registry -l`, `svy registry taobao`, `svy registry taobao --add https://registry.npmmirror.com/` |                                               |
| `check [keyword]...` |                           | 检查当前目录下的文件中是否含有关键词，会自动忽略.git, .gitignore 和.gitignore 中文件                                   | `Regex::new("*.log")`会报错，目前做了忽略处理 |
| git                  |                           | 切换 git config user.name 和 git config user.email                                                                     |                                               |
| create               |                           | todo                                                                                                                   |                                               |


## 使用

```bash
git clone https://github.com/QiYuOr2/svy.git
cargo build --release
cargo install --path .

svy registry
```