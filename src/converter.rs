use std::{fs::File, error::Error, io::{Read, Write, BufWriter}};

pub fn to_json(path: &str) -> Result<(), Box<dyn Error>> {
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
                    ':' => {
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

pub fn to_lang(path: &str) -> Result<(), Box<dyn Error>> {
    let mut data: String = String::new();
    let mut prev = File::open(path)?;
    let new = File::create({
        let mut data = path.chars().collect::<Vec<char>>();
        data.pop(); data.pop(); data.pop(); data.pop();
        format!("{}lang", String::from_iter(data))
    })?;
    let mut stream = BufWriter::new(new);
    prev.read_to_string(&mut data)?;
    let mut key = String::new();
    let mut value = String::new();
    let mut flag;
    let mut iter = data.chars();
    'outer: loop {
        flag = false;
        loop {
            if let Some(c) = iter.next() {
                match c {
                    ',' => {
                        break;
                    }
                    ':' => {
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
                    let key = key.trim().trim_start_matches("{").trim().trim_matches('"');
                    let value = value.trim_end_matches("}").trim().trim_matches('"');
                    write!(stream, "\n{}={}", key, value)?;
                }
                break 'outer;
            }
        }
        if flag {
            let key = key.trim().trim_matches('"');
            let value = value.trim().trim_matches('"');
            write!(stream, "\n{}={}", key, value)?;
        }
        key.clear();
        value.clear();
    }
    return Ok(());
}
