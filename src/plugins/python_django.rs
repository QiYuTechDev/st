use std::env;

use super::Poetry;
use crate::public::bump::{Bump, VerNewOld, Version};
use crate::public::StTrait;
use crate::utils;

/// Python Django Build Runner
#[derive(Default)]
pub struct Django {}

impl Django {
    fn set_django_env(prod: bool) {
        let src = Poetry::ensure_get_src_dir();
        utils::set_env("DJANGO_SETTINGS_MODULE", format!("{}.settings", src));
        if prod {
            utils::set_env("DJANGO_PROD", "1");
        } else {
            utils::set_env("DJANGO_DEV", "1");
        }
    }

    pub fn poetry_django_admin_prod_run(args: Vec<String>) {
        Self::set_django_env(true); // 设置必要的环境变量
        Self::do_poetry_django_admin_run(args);
    }

    /// 实际执行的命令为:
    ///
    /// poetry run python manage.py ...args
    pub fn poetry_django_admin_dev_run(args: Vec<String>) {
        Self::set_django_env(false); // 设置必要的环境变量
        Self::do_poetry_django_admin_run(args);
    }

    fn do_poetry_django_admin_run(args: Vec<String>) {
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
    pub fn check_django_project() -> bool {
        if !Poetry::check_poetry_project() {
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
        Self::poetry_django_admin_dev_run(vec!["makemigrations".to_string()]);
        println!("django start migrate ...");
        Self::poetry_django_admin_dev_run(vec!["migrate".to_string()]);
        println!("django start run server ...");
        Self::poetry_django_admin_dev_run(vec!["runserver".to_string()]);
    }

    fn support_lint(&self) -> bool {
        Self::check_django_project()
    }

    fn do_lint(&self) {
        Self::poetry_django_admin_dev_run(vec!["check".to_string()])
    }

    fn support_bump(&self) -> bool {
        Self::check_django_project()
    }

    /// 版本升级 dev,test and prod
    fn do_bump(&self, bump: &Bump) {
        let version_file = "version.json";

        let old = {
            // read old version
            let v = std::fs::read_to_string(version_file).expect("version.json 文件不存在");
            match serde_json::from_str(v.as_str()) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("解析 {} 失败: {}, 使用默认值", version_file, e);
                    Version::default()
                }
            }
        };

        // parse new version
        let new_version =
            Poetry::get_poetry_project_version().expect("从 pyproject.toml 获取 新版本失败");

        // set new version
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
        // write to disk
        {
            let s = serde_json::to_string_pretty(&new).expect("序列化新版本信息失败");
            std::fs::write(version_file, s).expect("写入新版本失败");
        };
    }
}
