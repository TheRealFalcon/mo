use std::collections::HashMap;
use std::env;
use std::fs::{create_dir, File};
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::process::Command;

use clap::{App, Arg};
use termion::{color, style};
use tracing::debug;

fn dir() -> String {
    env::var("HOME").unwrap() + "/.cache"
}

fn filename() -> String {
    dir() + "/mo"
}

fn search(insensitive: bool, args: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    // Setup our cache file
    // let dir = env::var("HOME")? + "/.cache";
    debug!("doing search");
    debug!("Args are: {:?}", &args);
    match create_dir(dir()) {
        Ok(_) => (),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => return Err(e.into()),
        },
    }
    let file = File::create(filename().as_str())?;
    let mut file = LineWriter::new(file);

    let mut my_collection: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut cmd = Command::new("rg");
    if insensitive {
        cmd.arg("-i");
    }
    cmd.arg("-H").arg("-n").args(&args);
    debug!("Running: {:?}", &cmd);
    let output = cmd.output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    if !stderr.is_empty() {
        println!("{}", stderr);
    }
    debug!("rg stdout is: {}", &stdout);
    debug!("rg stderr is: {}", &stderr);
    stdout.lines().into_iter().for_each(|line| {
        debug!(line);
        let (filename, line_match) = line.split_once(':').unwrap();
        my_collection
            .entry(filename)
            .and_modify(|v| v.push(line_match))
            .or_insert_with(|| vec![line_match]);
    });

    let search_term = if args.len() == 1 {
        args[0]
    } else {
        args.iter().nth_back(1).unwrap()
    };
    debug!("found search term: {}", search_term);

    let mut current_num = 1;
    for (filename, contents) in &my_collection {
        println!("{}{}{}:", color::Fg(color::Green), filename, style::Reset);
        for line in contents {
            let split_line: Vec<_> = line.split(search_term).collect();
            let line_with_color = split_line.join(
                format!(
                    "{}{}{}{}",
                    color::Fg(color::Red),
                    color::Fg(color::Red),
                    // style::Bold,
                    search_term,
                    style::Reset
                )
                .as_str(),
            );
            println!(
                "[{}{}{}]: {}",
                color::Fg(color::Green),
                &current_num,
                style::Reset,
                &line_with_color
            );
            let (line_num, _) = line.split_once(':').unwrap();
            file.write_all(format!("{}:{}\n", filename, line_num).as_bytes())?;
            current_num += 1;
        }
        println!();
    }
    file.flush()?;

    Ok(())
}

fn open_match(line_num: usize) -> Result<(), Box<dyn std::error::Error>> {
    debug!("in open");
    let reader = BufReader::new(File::open(filename().as_str())?);
    let matching_line = reader.lines().take(line_num).last().unwrap()?;
    // let (filename, line) = matching_line.rsplit_once(':').unwrap();
    let mut cmd = Command::new("code");
    cmd.arg("-g").arg(matching_line);
    debug!("Running: {:?}", cmd);
    cmd.spawn()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let matches = App::new("Mo")
        .version("0.1")
        .author("James Falcon <therealfalcon@gmail.com>")
        .about("Yay search")
        .arg(Arg::with_name("insensitive").short("i"))
        .arg(Arg::with_name("args").required(true).min_values(1))
        .get_matches();
    let args: Vec<_> = matches.values_of("args").unwrap().collect();
    if args.len() == 1 {
        match args[0].parse::<usize>() {
            Ok(v) => open_match(v),
            Err(_) => search(matches.is_present("insensitive"), args),
        }
    } else {
        search(matches.is_present("insensitive"), args)
    }
}
