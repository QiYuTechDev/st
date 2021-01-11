use std::env;

use super::Poetry;
use crate::public::{Bump, StTrait, VerNewOld, Version};
use crate::utils;

/// Python Django Build Runner
#[derive(Default)]
pub struct Django {}

impl Django {
    fn set_django_env() {
        let src = Poetry::ensure_get_src_dir();
        utils::set_env("DJANGO_SETTINGS_MODULE", format!("{}.settings", src));
        utils::set_env("DJANGO_DEV", "1")
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

        utils::switch_dir_exec(django_dir, move || Poetry::poetry_run(full_args.clone()));
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
        println!("django start make migrations ...");
        Self::poetry_django_admin_run(vec!["makemigrations".to_string()]);
        println!("django start migrate ...");
        Self::poetry_django_admin_run(vec!["migrate".to_string()]);
        println!("django start run server ...");
        Self::poetry_django_admin_run(vec!["runserver".to_string()]);
    }

    fn support_lint(&self) -> bool {
        Self::check_django_project()
    }

    fn do_lint(&self) {
        Self::poetry_django_admin_run(vec!["check".to_string()])
    }

    fn support_bump(&self) -> bool {
        Self::check_django_project()
    }

    fn do_bump(&self, bump: &Bump) {
        let version_file = "version.json";

        let v = std::fs::read_to_string(version_file).expect("version.json 文件不存在");

        let old = match serde_json::from_str(v.as_str()) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("解析 {} 失败: {}, 使用默认值", version_file, e);
                Version::default()
            }
        };

        let new_version =
            Poetry::get_poetry_project_version().expect("从 pyproject.toml 获取 新版本失败");

        let new = match bump {
            Bump::Dev => Version {
                dev: VerNewOld {
                    old: old.dev.new.clone(),
                    new: new_version,
                },
                ..old
            },
            Bump::Test => Version {
                test: VerNewOld {
                    old: old.test.new.clone(),
                    new: new_version,
                },
                ..old
            },
            Bump::Prod => Version {
                prod: VerNewOld {
                    old: old.prod.new.clone(),
                    new: new_version,
                },
                ..old
            },
        };
        let s = serde_json::to_string(&new).expect("序列化新版本信息失败");
        std::fs::write(version_file, s).expect("写入新版本失败");
    }
}
