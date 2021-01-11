use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum DockerEnv {
    /// 开发 环境的版本
    Dev,
    /// 测试 环境的版本
    Test,
    /// 线上 环境的版本
    Prod,
}

/// docker 构造命令
#[derive(Debug, StructOpt)]
#[structopt(name = "docker")]
pub enum DockerCmd {
    /// 构造
    Build(DockerEnv),
    /// 运行
    Run(DockerEnv),
    /// 暂停当前的升级
    Stop(DockerEnv),
    /// 重启 当前的 镜像
    Restart(DockerEnv),
    /// 升级 docker 镜像
    /// from old to new
    Upgrade(DockerEnv),
}

pub trait DockerTrait {
    fn can_build(&self, _env: &DockerEnv) -> bool {
        false
    }

    fn do_build(&self, _env: &DockerEnv) {}

    fn can_run(&self, _env: &DockerEnv) -> bool {
        false
    }

    fn do_run(&self, _env: &DockerEnv) {}

    fn can_stop(&self, _env: &DockerEnv) -> bool {
        false
    }

    fn do_stop(&self, _env: &DockerEnv) {}

    fn can_restart(&self, env: &DockerEnv) -> bool {
        self.can_stop(env) && self.can_run(env)
    }

    fn do_restart(&self, env: &DockerEnv) {
        assert!(self.can_restart(env));

        self.do_stop(env);
        self.do_run(env);
    }

    fn can_upgrade(&self, _env: &DockerEnv) -> bool {
        false
    }

    /// 执行升级计划
    fn do_upgrade(&self, _env: &DockerEnv) {}

    /// 获取 docker 新版本(当前)的 tag
    fn get_new_tag(&self, env: &DockerEnv) -> String;

    /// 获取 docker 老版本的 tag
    fn get_old_tag(&self, env: &DockerEnv) -> String;

    /// 获取 docker 文件名称 docker build 的时候需要
    fn get_docker_file(&self, env: &DockerEnv) -> String;

    /// 获取 docker 新版运行的名字
    fn get_new_name(&self, env: &DockerEnv) -> String {
        let tag = self.get_new_tag(env);
        tag.replace(":", "_")
    }

    /// 获取 docker 老版运行的名字
    fn get_old_name(&self, env: &DockerEnv) -> String {
        let tag = self.get_old_tag(env);
        tag.replace(":", "_")
    }
}

impl ToString for DockerEnv {
    fn to_string(&self) -> String {
        let v = match self {
            DockerEnv::Dev => "dev",
            DockerEnv::Test => "test",
            DockerEnv::Prod => "prod",
        };
        String::from(v)
    }
}
