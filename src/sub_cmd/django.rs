use structopt::StructOpt;

use crate::public::RunTrait;

/// Django 子命令
#[derive(Debug, StructOpt)]
#[structopt(name = "django")]
pub enum DjangoSubCmd {
    /// 收集静态文件
    CollectStatic,
}

impl DjangoSubCmd {
    fn do_collect_static(&self) {}
}

impl RunTrait for DjangoSubCmd {
    fn run(&self) {
        match self {
            Self::CollectStatic => self.do_collect_static(),
        }
    }
}
