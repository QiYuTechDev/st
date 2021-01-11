use structopt::StructOpt;

/// docker 构造命令
#[derive(Debug, StructOpt)]
pub enum Docker {
    /// 构造 开发 环境的版本
    Dev,
    /// 构造 测试 环境的版本
    Test,
    /// 构造 线上 环境的版本
    Prod,
}
