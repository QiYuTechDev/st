use structopt::StructOpt;

pub(crate) mod plugins;
pub(crate) mod public;
pub(crate) mod run_cmd;
pub(crate) mod utils;

#[derive(Debug, StructOpt)]
#[structopt(name = "st")]
pub enum StCli {
    /// 编译
    Build,
    /// 回滚到干净的环境
    Clean,
    /// 格式化代码
    ///
    /// 当前支持 Python poetry 的项目 [需要安装 black]
    ///
    ///
    Format,
    /// 检测依赖是否有新版
    Outdated,
    /// 运行
    Run,
    /// 升级依赖版本
    Update,
}

impl StCli {
    pub fn run(&self) {
        match self {
            StCli::Build => run_cmd::run_build_cmd(),
            StCli::Clean => run_cmd::run_clean_cmd(),
            StCli::Format => run_cmd::run_format_cmd(),
            StCli::Outdated => run_cmd::run_outdated_cmd(),
            StCli::Run => run_cmd::run_run_cmd(),
            StCli::Update => run_cmd::run_update_cmd(),
        }
    }
}
