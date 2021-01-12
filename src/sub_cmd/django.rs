use structopt::StructOpt;

use crate::plugins::Django;
use crate::public::RunTrait;

/// Django 子命令
///
/// 基于 Django 的软件
#[derive(Debug, StructOpt)]
#[structopt(name = "django")]
pub enum DjangoSubCmd {
    /// 收集静态文件 为部署做准备
    CollectStatic,
}

impl DjangoSubCmd {
    /// 收集静态文件
    fn do_collect_static(&self) {
        if !Django::check_django_project() {
            eprintln!("当前不是 Django 项目, 无法执行");
            return;
        }

        // attention:
        // it will auto switch working directory
        //
        //     poetry run python manage.py collectstatic
        //
        let args = vec!["collectstatic".to_string()];
        Django::poetry_django_admin_run(args);
    }
}

impl RunTrait for DjangoSubCmd {
    fn run(&self) {
        match self {
            Self::CollectStatic => self.do_collect_static(),
        }
    }
}
