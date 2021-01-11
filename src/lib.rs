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
    Format,
    /// 代码检测
    ///
    /// Rust 使用 Cargo clippy
    ///
    /// Python 使用 pylama
    Lint,
    /// 检测依赖是否有新版
    ///
    /// Rust 使用 Cargo
    ///
    /// Python 使用 Poetry
    Outdated,
    /// 运行
    ///
    /// Rust 使用 cargo run
    ///
    /// Python Django 项目使用 django-admin runserver
    Run,
    /// 升级依赖版本
    ///
    /// Rust 使用 cargo
    ///
    /// Python 使用 Poetry
    Update,
    /// 测试
    ///
    /// Rust 语言使用 cargo test
    ///
    /// Python 使用 pytest
    Test,
    /// 同步依赖
    Sync,
    /// 锁定依赖
    ///
    /// 锁定当前的依赖
    ///
    /// Python 使用 Poetry
    Lock,
    /// 本地安装
    ///
    /// 本地安装当前的软件
    /// Rust 使用 cargo install --path .
    Install,
    /// 发布
    ///
    /// Python 使用 Poetry 发布到 Pypi
    ///
    /// todo Rust 使用 cargo 发布到 Crates
    Publish,
    /// 提升版本
    Bump(public::bump::Bump),
    /// docker 命令
    Docker(public::docker::DockerCmd),
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
            StCli::Bump(bump) => run_cmd::run_bump_cmd(bump),
            StCli::Docker(d) => run_cmd::run_docker_cmd(d),
        }
    }
}
