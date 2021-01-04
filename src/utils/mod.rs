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
pub fn run_with_args(bin: PathBuf, args: Vec<String>) -> bool {
    let bin_s = bin.to_str().map(String::from).expect("bin is unknown");

    let args_s = args.join(" ");

    let success_msg = format!("执行命令: {} {} 成功!", bin_s, args_s);
    let failure_msg = format!("执行命令: {} {} 失败!", bin_s, args_s);

    println!("开始执行命令: {} {}", bin_s, args_s);
    let o = process::Command::new(bin)
        .args(&args)
        .status()
        .expect(failure_msg.as_str());

    if o.success() {
        println!("{}", success_msg);
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
