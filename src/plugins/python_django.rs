use super::Poetry;
use crate::public::StTrait;


/// Python Django Build Runner
#[derive(Default)]
pub struct Django {}

impl Django {
    fn set_django_env() {
        let src = Poetry::get_src_dir().expect("获取源代码目录失败");
        std::env::set_var("DJANGO_SETTINGS_MODULE", format!("{}.wsgi", src));
    }

    fn poetry_django_admin_run(args: Vec<String>) {
        Self::set_django_env();

        let cur_dir = std::env::current_dir().expect("获取当前目录失败");

        let full_args = {
            let mut t = vec!["run".to_string(), "django-admin".to_string()];
            args.into_iter().for_each(|s| t.push(s));
            t
        };

        let django_dir = {
            let src_dir = Poetry::get_src_dir().expect("获取源代码目录失败");
            cur_dir.join(src_dir)
        };
        std::env::set_current_dir(django_dir).expect("设置工作目录失败");
        Poetry::poetry_run(full_args);
        std::env::set_current_dir(cur_dir).expect("还原工作目录失败");
    }

    fn check_django_project() -> bool {
        if !Poetry::check_poetry_project() {
            return false;
        }

        Poetry::check_poetry_tools_exists("django-admin")
    }
}

impl StTrait for Django {
    fn name(&self) -> String {
        String::from("django")
    }

    fn support_run(&self) -> bool {
        Self::check_django_project()
    }

    fn do_run(&self) {
        Self::poetry_django_admin_run(vec!["runserver".to_string()])
    }

    fn support_lint(&self) -> bool {
        Self::check_django_project()
    }

    fn do_lint(&self) {
        Self::poetry_django_admin_run(vec!["check".to_string()])
    }
}
