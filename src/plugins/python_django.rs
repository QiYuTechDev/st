use std::env;

use super::Poetry;
use crate::public::StTrait;
use crate::utils;

/// Python Django Build Runner
#[derive(Default)]
pub struct Django {}

impl Django {
    fn set_django_env() {
        let src = Poetry::ensure_get_src_dir();
        utils::set_env("DJANGO_SETTINGS_MODULE", format!("{}.settings", src));
        utils::set_env("DJANGO_LOCAL", "1")
    }

    /// 实际执行的命令为:
    ///
    /// poetry run python manage.py ...args
    fn poetry_django_admin_run(args: Vec<String>) {
        Self::set_django_env(); // 设置必要的环境变量

        let cur_dir = env::current_dir().expect("获取当前目录失败");

        let full_args = {
            let mut t = vec![
                "run".to_string(),
                "python".to_string(),
                "manage.py".to_string(),
            ];
            args.into_iter().for_each(|s| t.push(s));
            t
        };

        let django_dir = cur_dir.join(Poetry::ensure_get_src_dir());
        env::set_current_dir(django_dir).expect("设置工作目录失败");
        Poetry::poetry_run(full_args);
        env::set_current_dir(cur_dir).expect("还原工作目录失败");
    }

    /// 检测是否为 django 的项目
    fn check_django_project() -> bool {
        if !Poetry::check_poetry_project() {
            return false;
        }

        // 检测 django-admin 是否存在
        if !Poetry::check_poetry_tools_exists("django-admin") {
            return false;
        }

        // check if `repo_name`/`repo_name`/wsgi.py
        // wsgi.py 是否存在
        let wsgi_file = {
            let src_dir = Poetry::ensure_get_src_dir();
            let mut cur_dir = env::current_dir().expect("获取当前工作目录失败");
            cur_dir.push(src_dir.clone());
            cur_dir.push(src_dir);
            cur_dir.push("wsgi.py");
            cur_dir
        };
        wsgi_file.exists()
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
