//! # cdwin
//! 在 `WSL2` 环境中通过 windows 风格的目录输入来切换目录。
use std::env;
use std::process::Command;
#[cfg(target_os = "windows")]
fn main() {
    let mut args = env::args();
    assert_eq!(
        args.len(),
        2,
        "\ncdwin 只接受一个参数，即路径参数。
        \n帮助提示：你的路径是不是有空格啊？可以用双引号包起来再传参哦。
        \n例如 {{cd D:\\a \\b }} 应当改成 {{cd \"D:\\a \\b\"}} \n"
    );
    let path = args.nth(1).unwrap();
    let path_str = &path;
    // println!("{}", path);
    assert!(env::set_current_dir(path_str).is_ok(), "\n路径{{ {} }}不存在！
    \n帮助提示：用ls或者dir命令查看一下目录是否存在！
    \n帮助提示：输入的路径与接收到的不一致？试试用双引号括住路径。", path_str);
    let mut cmd = Command::new("wsl");
    let wsl_existence_message = "wsl启动失败！\n帮助提示：请确保wsl已安装且发行版设置正确！";
    assert_eq!(cmd.get_program(), "wsl", "{}", wsl_existence_message);
    cmd.status().expect(wsl_existence_message);
}
