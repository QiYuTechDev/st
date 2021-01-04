use std::fs;

use toml;

use crate::public::StTrait;
use crate::utils;

/// Python Poetry Build Runner
#[derive(Default)]
pub struct Poetry {}

impl Poetry {
    #[inline]
    fn check_py_project(&self) -> bool {
        let f = "pyproject.toml";
        if !utils::check_current_dir_file_exists(f) {
            return false;
        }

        if let Some(_) = self.get_poetry_config() {
            return true;
        }
        return false;
    }

    /// 获取源代码的目录
    fn get_src_dir(&self) -> Option<String> {
        let name = self.get_poetry_config().map(|v| -> Option<String> {
            if let Some(name) = v.get("name") {
                if let Some(s) = name.as_str() {
                    return Some(String::from(s));
                }
            }
            None
        });

        if let Some(Some(dir)) = name {
            return Some(dir.replace("-", "_"));
        }
        return None;
    }

    /// 获取 poetry 的配置
    fn get_poetry_config(&self) -> Option<toml::Value> {
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

        return None;
    }

    #[inline]
    fn poetry_run(&self, args: Vec<String>) -> bool {
        let poetry = utils::get_exec_path("poetry");
        utils::run_with_args(poetry, args)
    }

    /// 检测 poetry 中是否已经安装了相应的工具
    fn check_poetry_tools_exists(&self, name: &str) -> bool {
        println!("检测 {} 是否存在:", name);
        if !self.poetry_run(vec![
            "run".to_string(),
            "which".to_string(),
            name.to_string(),
        ]) {
            println!("{} 不存在, 请在 pyproject.tmol 中添加相应的依赖.", name);
            return false;
        }
        return true;
    }
}

impl StTrait for Poetry {
    fn name(&self) -> String {
        String::from("poetry")
    }

    fn support_build(&self) -> bool {
        self.check_py_project()
    }

    fn do_build(&self) {
        self.poetry_run(vec!["build".to_string()]);
    }

    fn support_clean(&self) -> bool {
        false
    }

    fn do_clean(&self) {}

    fn support_format(&self) -> bool {
        if !self.check_py_project() {
            return false;
        }

        return self.check_poetry_tools_exists("black");
    }

    fn do_format(&self) {
        self.poetry_run(vec![
            "run".to_string(),
            "black".to_string(),
            self.get_src_dir().expect("获取源码目录失败"),
        ]);
    }

    fn support_outdated(&self) -> bool {
        self.check_py_project()
    }

    fn do_outdated(&self) {
        self.poetry_run(vec!["show".to_string(), "-o".to_string()]);
    }

    fn support_run(&self) -> bool {
        false
    }

    fn do_run(&self) {}

    fn support_update(&self) -> bool {
        self.check_py_project()
    }

    fn do_update(&self) {
        self.poetry_run(vec!["update".to_string()]);
    }

    fn support_test(&self) -> bool {
        if !self.check_py_project() {
            return false;
        }

        return self.check_poetry_tools_exists("pytest");
    }

    fn do_test(&self) {
        self.poetry_run(vec![
            "run".to_string(),
            "pytest".to_string(),
            self.get_src_dir().expect("获取源码目录失败"),
        ]);
    }
}
