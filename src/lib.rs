use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
  files: Vec<String>,
  number_lines: bool,
  number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
  for filename in config.files {
    match open(&filename) {
      Err(err) => eprintln!("Failed to open {}: {}", filename, err),
      Ok(file) => {
        let mut empty_line_counter = 0;
        for (count, line_result) in file.lines().enumerate() {
          let line = line_result?;
          // let numbered_line = if config.number_lines {
          //   format!("{} {}", count+1, line);
          // } else {
          //   line;
          // };
          // println!("{}", numbered_line);
          if config.number_nonblank_lines {
            
            if line.is_empty() {
              println!("{}", line);
              empty_line_counter = empty_line_counter + 1;
            } else {
              println!("{}    {}", count+1-empty_line_counter, line);
            }
          } else if config.number_lines {
            println!("{}    {}", count+1, line);
          } else {
            println!("{}", line);
          }
          
        }
      }
    };
  }
  Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
  match filename {
    "-" => Ok(Box::new(BufReader::new(io::stdin()))),
    _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
  }
}

pub fn get_args() -> MyResult<Config> {
  let matches = App::new("catr")
    .version("0.1.0")
    .author("Siva Karasala <siva.karasala@gmail.com>")
    .about("Rust cat")
    .arg(
      Arg::with_name("files")
          .value_name("FILE")
          .help("Input file(s) [default: -]")
          .multiple(true)
          .default_value("-"),
    )
    .arg(
      Arg::with_name("number")
          .short("n")
          .long("number")
          .help("Number lines")
          .takes_value(false)
          .conflicts_with("number_nonblank"),
    )
    .arg(
      Arg::with_name("number_nonblank")
        .short("b")
        .long("number-nonblank")
        .help("Number non-blank lines")
        .takes_value(false)
    )
    .get_matches();

    Ok(Config {
      files: matches.values_of_lossy("files").unwrap(),
      number_lines: matches.is_present("number"),
      number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}
