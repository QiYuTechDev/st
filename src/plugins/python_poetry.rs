use std::fs;

use crate::public::StTrait;
use crate::utils;

/// Python Poetry Build Runner
#[derive(Default)]
pub struct Poetry {}

impl Poetry {
    #[inline]
    pub fn check_poetry_project() -> bool {
        let f = "pyproject.toml";
        if !utils::check_current_dir_file_exists(f) {
            return false;
        }

        if Self::get_poetry_config().is_none() {
            return false;
        }

        if !utils::check_exe_exists("poetry") {
            return false;
        }

        true
    }

    /// 保证获取到 代码 目录
    /// 否则直接退出
    ///
    /// 返回的是目录名
    pub fn ensure_get_src_dir() -> String {
        Self::get_src_dir().expect("获取代码目录失败")
    }

    /// 获取源代码的目录
    ///
    /// 返回的是目录名
    /// 例如:
    ///     django_div_node
    ///
    pub fn get_src_dir() -> Option<String> {
        let name = Self::get_poetry_config().map(|v| -> Option<String> {
            if let Some(name) = v.get("name") {
                if let Some(s) = name.as_str() {
                    return Some(String::from(s));
                }
            }
            None
        });

        if let Some(Some(dir)) = name {
            // Python 包 不允许使用 '-' 需要用 '_' 替换
            return Some(dir.replace("-", "_"));
        }
        None
    }

    /// 获取 poetry 的配置
    pub fn get_poetry_config() -> Option<toml::Value> {
        let f = "pyproject.toml";

        // extract to utils
        let s = fs::read_to_string(f).expect("读取 pyproject.toml 失败!");
        let v = toml::from_str::<toml::Value>(s.as_str()).expect("解析 pyproject.toml 失败");
        if !v.is_table() {
            return None;
        }

        // tool.poetry 存在 才证明是 poetry 项目
        if let Some(tool) = v.get("tool") {
            if !tool.is_table() {
                return None;
            }
            if let Some(poetry) = tool.get("poetry") {
                return Some(poetry.clone());
            }
        }

        None
    }

    #[inline]
    pub fn poetry_run(args: Vec<String>) -> bool {
        let poetry = utils::get_exec_path("poetry");
        utils::run_with_args(poetry, args)
    }

    /// 检测 poetry 中是否已经安装了相应的工具
    pub fn check_poetry_tools_exists(name: &str) -> bool {
        if !Self::poetry_run(vec![
            "run".to_string(),
            "test".to_string(),
            name.to_string(),
        ]) {
            println!("{} 不存在, 请先安装", name);
            return false;
        }
        true
    }
}

impl StTrait for Poetry {
    fn name(&self) -> String {
        String::from("poetry")
    }

    fn support_build(&self) -> bool {
        Self::check_poetry_project()
    }

    fn do_build(&self) {
        Self::poetry_run(vec!["build".to_string()]);
    }

    fn support_clean(&self) -> bool {
        false
    }

    fn do_clean(&self) {}

    fn support_format(&self) -> bool {
        if !Self::check_poetry_project() {
            return false;
        }
        Self::check_poetry_tools_exists("black")
    }

    fn do_format(&self) {
        Self::poetry_run(vec![
            "run".to_string(),
            "black".to_string(),
            Self::ensure_get_src_dir(),
        ]);
    }

    fn support_outdated(&self) -> bool {
        Self::check_poetry_project()
    }

    fn do_outdated(&self) {
        Self::poetry_run(vec!["show".to_string(), "-o".to_string()]);
    }

    fn support_run(&self) -> bool {
        false
    }

    fn do_run(&self) {}

    fn support_update(&self) -> bool {
        Self::check_poetry_project()
    }

    fn do_update(&self) {
        Self::poetry_run(vec!["update".to_string()]);
    }

    fn support_lint(&self) -> bool {
        if !Self::check_poetry_project() {
            return false;
        }
        Self::check_poetry_tools_exists("pylama")
    }

    fn do_lint(&self) {
        Self::poetry_run(vec![
            "run".to_string(),
            "pylama".to_string(),
            Self::ensure_get_src_dir(),
        ]);
    }

    fn support_test(&self) -> bool {
        if !Self::check_poetry_project() {
            return false;
        }

        Self::check_poetry_tools_exists("pytest")
    }

    fn do_test(&self) {
        Self::poetry_run(vec![
            "run".to_string(),
            "pytest".to_string(),
            Self::ensure_get_src_dir(),
        ]);
    }

    fn support_lock(&self) -> bool {
        Self::check_poetry_project()
    }

    fn do_lock(&self) {
        Self::poetry_run(vec![
            "export".to_string(),
            "--without-hashes".to_string(),
            "-f".to_string(),
            "requirements.txt".to_string(),
            "-o".to_string(),
            "requirements.txt".to_string(),
        ]);
    }
}
