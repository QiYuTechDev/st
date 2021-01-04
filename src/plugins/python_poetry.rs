use crate::public::StTrait;
use crate::utils;

/// Python Poetry Build Runner
#[derive(Default)]
pub struct Poetry {}

impl Poetry {
    #[inline]
    fn check_py_project(&self) -> bool {
        utils::check_current_dir_file_exists("pyproject.toml")
    }

    #[inline]
    fn poetry_run(&self, args: Vec<String>) {
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
        self.poetry_run(vec!["build".to_string()])
    }

    fn support_clean(&self) -> bool {
        false
    }

    fn do_clean(&self) {}

    fn support_format(&self) -> bool {
        self.check_py_project()
    }

    fn do_format(&self) {
        // check black exists
        self.poetry_run(vec!["run".to_string(), "black".to_string()])
    }

    fn support_outdated(&self) -> bool {
        self.check_py_project()
    }

    fn do_outdated(&self) {
        self.poetry_run(vec!["show".to_string(), "-o".to_string()])
    }

    fn support_run(&self) -> bool {
        false
    }

    fn do_run(&self) {}

    fn support_update(&self) -> bool {
        self.check_py_project()
    }

    fn do_update(&self) {
        self.poetry_run(vec!["update".to_string()])
    }
}
