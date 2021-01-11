pub mod bump;
pub mod docker;

/// st tools 支持
pub trait StTrait {
    /// 名称
    fn name(&self) -> String;

    /// 支持 `build` 命令
    fn support_build(&self) -> bool {
        false
    }
    fn do_build(&self) {}

    /// 支持 `clean` 命令
    fn support_clean(&self) -> bool {
        false
    }
    fn do_clean(&self) {}

    /// 支持 `format` 命令
    fn support_format(&self) -> bool {
        false
    }
    fn do_format(&self) {}

    /// 支持 `outdated` 命令
    fn support_outdated(&self) -> bool {
        false
    }
    fn do_outdated(&self) {}

    /// 支持 `run` 命令
    fn support_run(&self) -> bool {
        false
    }
    fn do_run(&self) {}

    /// 支持 `update` 命令
    fn support_update(&self) -> bool {
        false
    }
    fn do_update(&self) {}

    /// 支持 `lint` 命令
    /// C 程序员应该知道这是啥意思 :)
    fn support_lint(&self) -> bool {
        false
    }
    fn do_lint(&self) {}

    /// 是否支持一键测试
    fn support_test(&self) -> bool {
        false
    }
    fn do_test(&self) {}

    /// 同步依赖
    fn support_sync(&self) -> bool {
        false
    }
    fn do_sync(&self) {}

    /// 锁定 依赖文件
    fn support_lock(&self) -> bool {
        false
    }
    fn do_lock(&self) {}

    /// 发布到中心仓库
    fn support_publish(&self) -> bool {
        false
    }
    fn do_publish(&self) {}

    /// 本地安装
    fn support_install(&self) -> bool {
        false
    }
    fn do_install(&self) {}

    /// 是否支持提升版本
    fn support_bump(&self) -> bool {
        false
    }

    fn do_bump(&self, _: &bump::Bump) {}

    /// 是否支持 构造 docker 镜像
    fn support_docker(&self) -> bool {
        false
    }

    fn do_docker(&self, _: &docker::DockerCmd) {}
}
