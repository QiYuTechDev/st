use structopt::StructOpt;

pub(crate) mod plugins;
pub(crate) mod public;
pub(crate) mod run_cmd;
pub(crate) mod utils;

#[derive(Debug, StructOpt)]
#[structopt(name = "st")]
pub enum StCli {
    /// 编译
    ///
    /// Rust 项目 默认使用: cargo build
    /// npm 项目 不支持
    /// poetry 项目 默认使用: poetry build 打包
    Build,
    /// 清理开发环境
    ///
    /// Rust 使用 cargo clean
    ///
    /// npm 清理缓存 npm cache clean
    ///
    /// poetry 不支持
    ///
    Clean,
    /// 格式化代码
    ///
    /// 当前支持
    ///
    /// Python poetry 的项目 [需要安装 black]
    ///
    /// Rust 项目 cargo fmt
    ///
    /// npm 项目使用 prettier
    Format,
    /// 代码检测
    ///
    /// Rust 使用 Cargo clippy
    Lint,
    /// 检测依赖是否有新版
    Outdated,
    /// 运行
    Run,
    /// 升级依赖版本
    Update,
    /// 测试
    ///
    /// Rust 语言使用 cargo test
    Test,
    /// 同步依赖
    Sync,
    /// 锁定依赖
    ///
    /// 锁定当前的依赖
    Lock,
    /// 本地安装
    ///
    /// 本地安装当前的软件
    /// 当前支持: Rust
    Install,
    /// 发布
    ///
    /// 发布到中心仓库
    Publish,
}

impl StCli {
    pub fn run(&self) {
        match self {
            StCli::Build => run_cmd::run_build_cmd(),
            StCli::Clean => run_cmd::run_clean_cmd(),
            StCli::Format => run_cmd::run_format_cmd(),
            StCli::Lint => run_cmd::run_lint_cmd(),
            StCli::Outdated => run_cmd::run_outdated_cmd(),
            StCli::Run => run_cmd::run_run_cmd(),
            StCli::Update => run_cmd::run_update_cmd(),
            StCli::Test => run_cmd::run_test_cmd(),
            StCli::Sync => run_cmd::run_sync_cmd(),
            StCli::Lock => run_cmd::run_lock_cmd(),
            StCli::Install => run_cmd::run_install_cmd(),
            StCli::Publish => run_cmd::run_publish_cmd(),
        }
    }
}
