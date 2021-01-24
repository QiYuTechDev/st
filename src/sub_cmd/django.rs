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
    /// 导出数据库数据
    ///
    /// 注意: 这个命令只应该在 开发环境 运行, 不允许在线上运行
    /// 防止线上数据量过大，影响业务
    DumpData,
    /// 把数据导入到数据库中
    ///
    /// 注意: 这个命令只应该在 开发环境 运行, 不允许在线上运行
    /// 防止导入的数据 损坏 实际的数据
    LoadData,
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
        Django::poetry_django_admin_run(vec!["collectstatic".to_string()]);
    }

    fn do_dump_data(&self) {
        if !Django::check_django_project() {
            eprintln!("当前不是 Django 项目, 无法执行");
            return;
        }
        Django::poetry_django_admin_run(vec![
            "dumpdata".to_string(),
            "--output".to_string(),
            "dump.json".to_string(),
        ]);
    }

    fn do_load_data(&self) {
        if !Django::check_django_project() {
            eprintln!("当前不是 Django 项目, 无法执行");
            return;
        }
        Django::poetry_django_admin_run(vec!["loaddata".to_string(), "dump.json".to_string()]);
    }
}

impl RunTrait for DjangoSubCmd {
    fn run(&self) {
        match self {
            Self::CollectStatic => self.do_collect_static(),
            Self::DumpData => self.do_dump_data(),
            Self::LoadData => self.do_load_data(),
        }
    }
}
