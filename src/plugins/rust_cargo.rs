use crate::public::*;
use crate::utils;

/// Cargo Runner
#[derive(Default)]
pub struct Cargo {}

impl Cargo {
    #[inline]
    fn check_cargo_project(&self) -> bool {
        utils::check_current_dir_file_exists("Cargo.toml")
    }

    #[inline]
    fn cargo_run(&self, args: Vec<String>) {
        let cargo = utils::get_exec_path("cargo");
        utils::run_with_args(cargo, args);
    }
}

impl StTrait for Cargo {
    fn name(&self) -> String {
        String::from("cargo")
    }

    fn support_build(&self) -> bool {
        self.check_cargo_project()
    }

    fn do_build(&self) {
        self.cargo_run(vec!["build".to_string()])
    }

    fn support_clean(&self) -> bool {
        self.check_cargo_project()
    }

    fn do_clean(&self) {
        self.cargo_run(vec!["clean".to_string()])
    }

    fn support_format(&self) -> bool {
        self.check_cargo_project()
    }

    fn do_format(&self) {
        self.cargo_run(vec!["fmt".to_string()])
    }

    fn support_outdated(&self) -> bool {
        self.check_cargo_project()
    }

    fn do_outdated(&self) {
        self.cargo_run(vec!["outdated".to_string()])
    }

    fn support_run(&self) -> bool {
        self.check_cargo_project()
    }

    fn do_run(&self) {
        self.cargo_run(vec!["run".to_string()])
    }

    fn support_update(&self) -> bool {
        self.check_cargo_project()
    }

    fn do_update(&self) {
        self.cargo_run(vec!["update".to_string()])
    }

    fn support_lint(&self) -> bool {
        self.check_cargo_project()
    }

    fn do_lint(&self) {
        self.cargo_run(vec!["clippy".to_string()])
    }

    fn support_test(&self) -> bool {
        self.check_cargo_project()
    }

    fn do_test(&self) {
        self.cargo_run(vec!["test".to_string()])
    }

    fn support_install(&self) -> bool {
        self.check_cargo_project()
    }

    fn do_install(&self) {
        self.cargo_run(vec![
            "install".to_string(),
            "--path".to_string(),
            ".".to_string(),
        ])
    }
}
