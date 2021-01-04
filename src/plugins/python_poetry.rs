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
        // extract to utils
        let s = fs::read_to_string(f).expect("读取 pyproject.toml 失败!");
        let v = toml::from_str::<toml::Value>(s.as_str()).expect("解析 pyproject.toml 失败");
        if !v.is_table() {
            return false;
        }

        // tool.poetry 存在 才证明是 poetry 项目
        if let Some(tool) = v.get("tool") {
            if !tool.is_table() {
                return false;
            }
            if let Some(_) = tool.get("poetry") {
                return true;
            }
        }

        return false;
    }

    #[inline]
    fn poetry_run(&self, args: Vec<String>) -> bool {
        let poetry = utils::get_exec_path("poetry");
        utils::run_with_args(poetry, args)
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

        if !self.poetry_run(vec![
            "run".to_string(),
            "which".to_string(),
            "black".to_string(),
        ]) {
            return false;
        }
        return true;
    }

    fn do_format(&self) {
        // check black exists
        self.poetry_run(vec!["run".to_string(), "black".to_string()]);
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
}
