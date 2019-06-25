use std::fs::File;
use std::{env, process};
use std::io::{BufReader, BufRead, Write};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let filename = get_filename(env::args()).unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1);
    });

    let tmp_dest_file_name = Path::new(&filename).file_stem().unwrap().to_str().unwrap();
    let dest_file_name = format!("{}.json", tmp_dest_file_name);

    let file = File::open(filename).unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1);
    });

    let mut header: Vec<String> = Vec::new();
    let mut data: Vec<HashMap<String, String>> = Vec::new();

    for (i, item) in BufReader::new(file).lines().enumerate() {
        if i == 0 {
            header = item.unwrap().clone().split(',').map(|e| e.to_string()).collect();
        } else {
            let mut tmp: Vec<String> = Vec::new();
            tmp = item.unwrap().split(',').map(|e| e.to_string()).collect();

            let mut foo: HashMap<String, String> = HashMap::new();

            for (i, it) in tmp.iter().enumerate() {
                foo.insert(header[i].clone(), it.clone());
            }

            data.push(foo);
        }
    }

    let dest_file_path = Path::new(&dest_file_name);
    let mut dest_file = match File::create(dest_file_path) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    dest_file.write("[\n".as_bytes());
    let mut content = String::from("");

    for elem in data.iter() {
        let mut json = String::from("");

        for head in header.iter().rev() {
            json = format!("\"{}\":\"{}\",\n\t\t{}", head, elem[head], json);
        }

        json.pop();
        json.pop();
        json.pop();
        json.pop();

        let tmp_str = format!("\t{{\n\t\t{}\n\t}},\n", json);
        content = format!("{}{}", content, tmp_str);
    }

    content.pop();
    content.pop();

    dest_file.write(content.as_bytes());
    dest_file.write("\n]".as_bytes());
}

fn get_filename(mut args: env::Args) -> Result<String, &'static str> {
    // Consume the binary name
    args.next();

    let filename = match args.next() {
        Some(v) => v,
        None => return Err("File name not found")
    };

    Ok(filename)
}
