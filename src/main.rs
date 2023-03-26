mod converter;

use std::{fs::ReadDir, error::Error};
use std::fs;

fn main() {
    let data = std::env::args().collect::<Vec<String>>();
    match data.get(1).map(|a| a.as_str()) {
        Some(a) => {
            if a.ends_with(".lang") {
                finish_or_errorlog!(converter::to_json(a));
            } else if a.ends_with(".json") {
                finish_or_errorlog!(converter::to_lang(a));
            } else {
                finish_or_errorlog!(conv_all(a));
            }
        },
        None => show_help()
    }
}

#[macro_export]
macro_rules! finish_or_errorlog {
    ($func: expr) => {
        match $func {
            Ok(_) => {
                println!("変換が終了しました。");
            },
            Err(e) => {
                println!("{}", e);
            },
        }
    };
}

fn show_help() {
    println!(".lang ファイルまたは .json ファイル, またはそれらを含むフォルダを指定してください。");
}

fn conv_all(path: &str) -> Result<(), Box<dyn Error>> {
    let tmp = fs::read_dir(path)?;
    let mut file_stack1 = Vec::<String>::new();
    let mut file_stack2 = Vec::<String>::new();
    let mut dir_stack = Vec::<ReadDir>::new();
    dir_stack.push(tmp);
    while !dir_stack.is_empty() {
        for a in dir_stack.pop().unwrap() {
            match a {
                Ok(d) => {
                    let ft = d.file_type()?;
                    let name = d.path().to_string_lossy().into_owned();
                    if ft.is_file() {
                        if name.ends_with(".lang") {
                            file_stack1.push(name);
                        } else if name.ends_with(".json") {
                            file_stack2.push(name);
                        }
                    } else if ft.is_dir() {
                        dir_stack.push(fs::read_dir(name)?);
                    }
                },
                Err(e) => return Err(Box::new(e)),
            }
        }
    }
    for a in file_stack1 {
        println!("変換中: {}", a);
        converter::to_json(a.as_str())?;
    }
    for a in file_stack2 {
        println!("変換中: {}", a);
        converter::to_lang(a.as_str())?;
    }
    return Ok(());
}
