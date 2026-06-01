//! 极简 CLI。
//!
//! 用法:
//!   calc add <a> <b>
//!   calc divide <a> <b>
//!   calc even <n>
//!
//! 故意不引第三方 CLI 库 — 第 11 章会换成 clap。

use std::process::ExitCode;

use rustlab01::calc;

fn parse(arg: &str) -> Result<i64, String> {
    arg.parse::<i64>()
        .map_err(|e| format!("invalid number {arg:?}: {e}"))
}

fn run(args: &[String]) -> Result<String, String> {
    match args {
        [cmd, a, b] if cmd == "add" => {
            let a = parse(a)?;
            let b = parse(b)?;
            Ok(calc::add(a, b).to_string())
        }
        [cmd, a, b] if cmd == "divide" => {
            let a = parse(a)?;
            let b = parse(b)?;
            calc::divide(a, b)
                .map(|v| v.to_string())
                .map_err(|e| e.to_string())
        }
        [cmd, n] if cmd == "even" => {
            let n = parse(n)?;
            Ok(if calc::is_even(n) {
                "true".into()
            } else {
                "false".into()
            })
        }
        _ => Err(String::from(
            "usage:\n  calc add <a> <b>\n  calc divide <a> <b>\n  calc even <n>",
        )),
    }
}

fn main() -> ExitCode {
    // argv[0] 是程序名,跳过
    let args: Vec<String> = std::env::args().skip(1).collect();

    match run(&args) {
        Ok(out) => {
            println!("{out}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
