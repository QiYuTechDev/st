use std::env;
use std::path::PathBuf;
use std::process;

/// 获取 执行 代码的绝对路径
pub fn get_exec_path(name: &str) -> PathBuf {
    match which::which(name) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("获取可执行文件: {} 失败, 错误原因: {:?}", name, e);
            process::exit(1)
        }
    }
}

/// 使用 args 运行 bin
///
/// fix: 使用这个运行有可能暴露 token 出来
/// 因此只允许交互式的 publish 不允许
pub fn run_with_args(bin: PathBuf, args: Vec<String>) -> bool {
    let bin_s = bin.to_str().map(String::from).expect("bin is unknown");

    let failure_msg = format!("执行命令: {} {} 失败!", bin_s, args.join(" "));

    let o = process::Command::new(bin)
        .args(&args)
        .status()
        .unwrap_or_else(|e| panic!("msg: {}, error: {}", failure_msg, e));

    if o.success() {
        true
    } else {
        eprintln!("{}", failure_msg);
        false
    }
}

/// 检测当前目录是否存在指定的文件
pub fn check_current_dir_file_exists(file: &str) -> bool {
    return match env::current_dir() {
        Ok(d) => {
            let dir_path = d.as_path();
            let file_path = dir_path.join(file);
            file_path.exists()
        }
        Err(e) => {
            eprintln!("获取当前路径失败: {:?} ", e);
            false
        }
    };
}
