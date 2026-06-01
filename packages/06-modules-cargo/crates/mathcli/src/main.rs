//! CLI binary that uses the mathlib library crate.

use std::process::ExitCode;

fn run(args: &[String]) -> Result<String, String> {
    match args {
        [cmd, a, b] if cmd == "add" => {
            let a: f64 = a.parse().map_err(|e| format!("invalid number: {e}"))?;
            let b: f64 = b.parse().map_err(|e| format!("invalid number: {e}"))?;
            Ok(format!("{}", mathlib::add(a, b)))
        }
        [cmd, a, b] if cmd == "divide" => {
            let a: f64 = a.parse().map_err(|e| format!("invalid number: {e}"))?;
            let b: f64 = b.parse().map_err(|e| format!("invalid number: {e}"))?;
            mathlib::divide(a, b)
                .map(|v| format!("{v}"))
                .map_err(|e| e.to_string())
        }
        [cmd, deg] if cmd == "sin" => {
            let deg: f64 = deg.parse().map_err(|e| format!("invalid number: {e}"))?;
            Ok(format!("{:.6}", mathlib::sin(deg)))
        }
        [cmd, vals @ ..] if cmd == "mean" && !vals.is_empty() => {
            let numbers: Result<Vec<f64>, _> = vals.iter().map(|s| s.parse::<f64>()).collect();
            let numbers = numbers.map_err(|e| format!("invalid number: {e}"))?;
            match mathlib::mean(&numbers) {
                Some(m) => Ok(format!("{m:.4}")),
                None => Err("no values provided".into()),
            }
        }
        _ => Err(String::from(
            "usage:\n  mathcli add <a> <b>\n  mathcli divide <a> <b>\n  mathcli sin <degrees>\n  mathcli mean <values...>",
        )),
    }
}

fn main() -> ExitCode {
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
