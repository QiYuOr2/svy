/// 配置文件
pub const SVYRC: &str = ".svyrc";

/// check过滤的文件后缀
pub const IGNORE_EXT: [&str; 4] = [".png", ".jpg", ".gif", ".ico"];

/// 初始化配置
pub const BASIC_CONFIG: &str = r#"{
  "registry": {
      "npm":"https://registry.npmjs.org/",
      "taobao":"https://registry.npmmirror.com/"
  },
  "git": {}
}"#;

pub const NPM: &str = if cfg!(target_os = "window") {
    "npm"
} else {
    "npm.cmd"
};
// pub const NPM: &str = "npm";

pub const GIT: &str = "git";