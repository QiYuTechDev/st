use structopt::StructOpt;

use crate::plugins::Django;
use crate::public::RunTrait;
use crate::utils;

/// Django 子命令
///
/// 基于 Django 的软件
#[derive(Debug, StructOpt)]
#[structopt(name = "django")]
pub enum DjangoSubCmd {
    /// 收集静态文件 为部署做准备
    ///
    /// 会设置环境变量
    /// * DJANGO_COLLECT_STATIC 为 1
    /// * DJANGO_PROD 为 1
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

        utils::set_env("DJANGO_COLLECT_STATIC", "1");

        // attention:
        // it will auto switch working directory
        //
        //     poetry run python manage.py collectstatic
        //
        Django::poetry_django_admin_prod_run(vec!["collectstatic".to_string()]);
    }

    fn do_dump_data(&self) {
        if !Django::check_django_project() {
            eprintln!("当前不是 Django 项目, 无法执行");
            return;
        }
        // we do should export to json lines
        // https://stackoverflow.com/questions/853796/problems-with-contenttypes-when-loading-a-fixture-in-django
        Django::poetry_django_admin_dev_run(vec![
            "dumpdata".to_string(),
            "--natural-foreign".to_string(),
            "--natural-primary".to_string(),
            "-e".to_string(),
            "contenttypes".to_string(),
            "-e".to_string(),
            "auth.Permission".to_string(),
            "-e".to_string(),
            "sessions".to_string(),
            "-e".to_string(),
            "admin".to_string(),
            // wagtail admin table is indeed not exists
            // just ignore on export data
            "-e".to_string(),
            "wagtailadmin.admin".to_string(),
            "--output".to_string(),
            Self::get_export_filename(),
        ]);
    }

    fn do_load_data(&self) {
        if !Django::check_django_project() {
            eprintln!("当前不是 Django 项目, 无法执行");
            return;
        }
        Django::poetry_django_admin_dev_run(vec!["loaddata".to_string(), Self::get_export_filename()]);
    }

    /// 导出、导入文件名称
    /// 使用 JSON lines 格式
    /// https://docs.djangoproject.com/en/dev/topics/serialization/#serialization-formats
    /// todo Django 3.2 才支持 JSON Lines
    fn get_export_filename() -> String {
        String::from("dump.json")
        // String::from("dump.jsonl")
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
