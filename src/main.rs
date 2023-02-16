use std::{fs::File, error::Error, io::{Read, Write, BufWriter}, path::Path, fmt::format};

fn main() {
    let data = std::env::args().collect::<Vec<String>>();
    match data.get(1).map(|a| a.as_str()) {
        Some(a) => {
            if a.ends_with(".lang") {
                match to_json(a) {
                    Ok(_) => {
                        println!("Finished");
                    },
                    Err(e) => {
                        println!("{}", e);
                    },
                }
            } else if a.ends_with(".json") {
                todo!()
            } else {
                let path = Path::new(a);
                for a in path {
                    println!("{:?}", a);
                }
                todo!("フォルダ内のすべてのファイルを変換する処理")
            }
        },
        None => show_help()
    }
}

fn show_help() {
    println!(".lang ファイルまたは .json ファイルを指定してください。");
}

fn to_json(path: &str) -> Result<(), Box<dyn Error>> {
    let mut data: String = String::new();
    let mut prev = File::open(path)?;
    let new = File::create({
        let mut data = path.chars().collect::<Vec<char>>();
        data.pop(); data.pop(); data.pop(); data.pop();
        format!("{}json", String::from_iter(data))
    })?;
    let mut stream = BufWriter::new(new);
    prev.read_to_string(&mut data)?;
    let mut key = String::new();
    let mut value = String::new();
    let mut flag;
    let mut t = true;
    let mut iter = data.chars();
    write!(stream, "{{")?;
    'outer: loop {
        flag = false;
        loop {
            if let Some(c) = iter.next() {
                match c {
                    '\n' | '\r' => {
                        break;
                    }
                    '=' => {
                        flag = true;
                    }
                    _ => {
                        if flag {
                            value.push(c);
                        } else {
                            key.push(c);
                        }
                    }
                }
            } else {
                if flag {
                    write!(stream, "{}\n    {:?}:{:?}", if t {""} else {","}, key, value)?;
                }
                break 'outer;
            }
        }
        if flag {
            write!(stream, "{}\n    {:?}:{:?}", if t {t = false; ""} else {","}, key, value)?;
        }
        key.clear();
        value.clear();
    }
    write!(stream, "\n}}")?;
    return Ok(());
}