use std::env;

use super::Poetry;
use crate::public::bump::{Bump, VerNewOld, Version};
use crate::public::docker::{DockerCmd, DockerEnv, DockerTrait};
use crate::public::StTrait;
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
    pub fn poetry_django_admin_run(args: Vec<String>) {
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

    fn load_version() -> Version {
        let s = std::fs::read_to_string("version.json").expect("读取 version 版本失败");
        serde_json::from_str(s.as_str()).expect("解析 version.json 失败")
    }

    fn get_docker_tag(&self, env: &DockerEnv, old: bool) -> String {
        let version = Self::load_version();
        let project_name = Poetry::ensure_get_src_dir();
        let tag = if old {
            match env {
                DockerEnv::Dev => version.dev.old,
                DockerEnv::Test => version.test.old,
                DockerEnv::Prod => version.prod.old,
            }
        } else {
            match env {
                DockerEnv::Dev => version.dev.new,
                DockerEnv::Test => version.test.new,
                DockerEnv::Prod => version.prod.new,
            }
        };
        format!("{}_{}:{}", project_name, env.to_string(), tag)
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

    fn support_docker(&self) -> bool {
        if !Self::check_django_project() {
            return false;
        }
        // 检查 docker 文件夹是否存在
        let docker_dir = std::env::current_dir().unwrap().join("docker");
        if !docker_dir.exists() {
            eprintln!("docker directory not exists");
            return false;
        }

        let buf = docker_dir;
        let dev = buf.join("dev.Dockerfile");
        let test = buf.join("test.Dockerfile");
        let prod = buf.join("prod.Dockerfile");
        if !dev.exists() || !test.exists() || !prod.exists() {
            eprintln!("dev test prod docker file not all exists");
            return false;
        }

        true
    }

    fn do_docker(&self, env: &DockerCmd) {
        let ptr = self as &dyn DockerTrait;
        match env {
            DockerCmd::Build(env) => {
                if ptr.can_build(env) {
                    ptr.do_build(env)
                }
            }
            DockerCmd::Run(env) => {
                if ptr.can_run(env) {
                    ptr.do_run(env)
                }
            }
            DockerCmd::Stop(env) => {
                if ptr.can_stop(env) {
                    ptr.do_stop(env)
                }
            }
            DockerCmd::Restart(env) => {
                if ptr.can_restart(env) {
                    ptr.do_restart(env)
                }
            }
            DockerCmd::Upgrade(env) => {
                if ptr.can_upgrade(env) {
                    ptr.do_upgrade(env)
                }
            }
        }
    }
}

impl DockerTrait for Django {
    fn can_build(&self, env: &DockerEnv) -> bool {
        let docker_file = {
            let cur_dir = std::env::current_dir().expect("获取当前目录失败");
            let docker_file = self.get_docker_file(env);
            cur_dir.join(docker_file)
        };

        docker_file.exists()
    }

    fn do_build(&self, env: &DockerEnv) {
        let docker_file = self.get_docker_file(env);
        let args = vec![
            "build".to_string(),
            "-f".to_string(),
            docker_file,
            "-t".to_string(),
            self.get_new_tag(env),
            ".".to_string(),
        ];
        utils::docker::run(args);
    }

    fn can_run(&self, _env: &DockerEnv) -> bool {
        true
    }

    fn do_run(&self, env: &DockerEnv) {
        let tag = self.get_new_tag(env);
        let name = self.get_new_name(env);

        let cur_dir = std::env::current_dir().expect("获取当前目录失败");
        let static_dir = cur_dir.join("static");
        let media_dir = cur_dir.join("media");
        let run_dir = cur_dir.join("run");
        let logs_dir = cur_dir.join("logs");

        let args = vec![
            "run".to_string(),
            "-d".to_string(),
            "--network=host".to_string(),
            "--restart=always".to_string(),
            "-v".to_string(),
            format!("{}:/app/static", static_dir.to_str().unwrap()),
            "-v".to_string(),
            format!("{}:/app/media", media_dir.to_str().unwrap()),
            "-v".to_string(),
            format!("{}:/app/run", run_dir.to_str().unwrap()),
            "-v".to_string(),
            format!("{}:/app/logs", logs_dir.to_str().unwrap()),
            format!("--name={}", name),
            tag,
        ];
        utils::docker::run(args);
    }

    fn can_stop(&self, _env: &DockerEnv) -> bool {
        true
    }

    fn do_stop(&self, env: &DockerEnv) {
        let tag = self.get_new_name(env);
        let args = vec!["stop".to_string(), tag];
        utils::docker::run(args);
    }

    fn get_new_tag(&self, env: &DockerEnv) -> String {
        self.get_docker_tag(env, false)
    }

    fn get_old_tag(&self, env: &DockerEnv) -> String {
        self.get_docker_tag(env, true)
    }

    fn get_docker_file(&self, env: &DockerEnv) -> String {
        let s = env.to_string();
        format!("docker/{}.Dockerfile", s)
    }
}
