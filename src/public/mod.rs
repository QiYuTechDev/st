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
}
