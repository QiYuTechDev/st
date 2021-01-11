/// 运行 docker 命令
pub fn run(args: Vec<String>) -> bool {
    let docker = super::get_exec_path("docker");
    super::run_with_args(docker, args)
}
