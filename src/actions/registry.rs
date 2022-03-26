use clap::Args;

#[derive(Args)]
pub struct Registry {
    /// 查询所有源
    #[clap(short, long)]
    pub list: bool,

    /// 切换对应源
    pub name: Option<String>,
}

impl Registry {
    pub fn exec(&self) {
        // 含有`--list`时，查询所有源
        if self.list == true {
            println!("registry list");
            return;
        }
        // 切换源
        if let Some(name) = &self.name {
            println!("registry use {}", name)
        }
    }
}
