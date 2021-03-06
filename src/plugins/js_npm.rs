use crate::public::StTrait;
use crate::utils;

#[derive(Default)]
pub struct Npm {}

impl Npm {
    #[inline]
    fn check_npm_project(&self) -> bool {
        // package.json 文件是 npm 项目的标准配置文件
        utils::check_current_dir_file_exists("package.json")
    }

    #[inline]
    fn npm_run(&self, args: Vec<String>) {
        let npm = utils::get_exec_path("npm");
        utils::run_with_args(npm, args);
    }
}

impl StTrait for Npm {
    fn name(&self) -> String {
        String::from("npm")
    }

    /// 支持 `clean`
    /// 删除 npm 缓存的 文件
    fn support_clean(&self) -> bool {
        self.check_npm_project()
    }

    fn do_clean(&self) {
        self.npm_run(vec!["cache".to_string(), "clean".to_string()])
    }

    fn support_outdated(&self) -> bool {
        self.check_npm_project()
    }

    /// 检查依赖是否已经过时
    fn do_outdated(&self) {
        self.npm_run(vec!["outdated".to_string()])
    }

    fn support_run(&self) -> bool {
        false
    }

    fn do_run(&self) {}

    fn support_update(&self) -> bool {
        self.check_npm_project()
    }

    fn do_update(&self) {
        self.npm_run(vec!["update".to_string()])
    }
}
