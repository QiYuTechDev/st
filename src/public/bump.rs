use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Default, Serialize, Deserialize)]
pub struct VerNewOld {
    pub old: String,
    pub new: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Version {
    pub dev: VerNewOld,
    pub test: VerNewOld,
    pub prod: VerNewOld,
}

/// 提升 版本 命令
#[derive(Debug, StructOpt)]
pub enum Bump {
    /// 提升 开发 环境的版本
    Dev,
    /// 提升 测试 环境的版本
    Test,
    /// 提升 线上 环境的版本
    Prod,
}
