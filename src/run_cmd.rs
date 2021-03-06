use crate::plugins::{Cargo, Django, Npm, Poetry};
use crate::public::{self, StTrait};

/// 运行 `build` 命令
pub fn run_build_cmd() {
    do_run_all_cmd(|p| p.support_build(), |p| p.do_build())
}

pub fn run_clean_cmd() {
    do_run_all_cmd(|p| p.support_clean(), |p| p.do_clean())
}

pub fn run_format_cmd() {
    do_run_all_cmd(|p| p.support_format(), |p| p.do_format())
}

pub fn run_lint_cmd() {
    do_run_all_cmd(|p| p.support_lint(), |p| p.do_lint())
}

pub fn run_outdated_cmd() {
    do_run_all_cmd(|p| p.support_outdated(), |p| p.do_outdated())
}

pub fn run_run_cmd() {
    do_run_all_cmd(|p| p.support_run(), |p| p.do_run())
}

pub fn run_update_cmd() {
    do_run_all_cmd(|p| p.support_update(), |p| p.do_update())
}

pub fn run_test_cmd() {
    do_run_all_cmd(|p| p.support_test(), |p| p.do_test())
}

pub fn run_sync_cmd() {
    do_run_all_cmd(|p| p.support_sync(), |p| p.do_sync())
}

pub fn run_lock_cmd() {
    do_run_all_cmd(|p| p.support_lock(), |p| p.do_lock())
}

pub fn run_install_cmd() {
    do_run_all_cmd(|p| p.support_install(), |p| p.do_install())
}

pub fn run_publish_cmd() {
    do_run_all_cmd(|p| p.support_publish(), |p| p.do_publish())
}

pub fn run_bump_cmd(bump: &public::bump::Bump) {
    do_run_all_cmd(|p| p.support_bump(), |p| p.do_bump(bump))
}

fn do_run_all_cmd<P, R>(check_fn: P, do_fn: R)
where
    P: Fn(&Box<dyn StTrait>) -> bool,
    R: Fn(&Box<dyn StTrait>),
{
    let mut hint = false;
    get_all_cmd().into_iter().for_each(|v| {
        if check_fn(&v) {
            hint = true;
            do_fn(&v)
        }
    });
    if !hint {
        println!("没有找到处理工具");
        std::process::exit(1);
    }
}

fn get_all_cmd() -> Vec<Box<dyn StTrait>> {
    vec![
        Box::new(Cargo::default()),
        Box::new(Npm::default()),
        Box::new(Poetry::default()),
        Box::new(Django::default()),
    ]
}
