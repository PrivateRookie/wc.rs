use prettytable::{color, format, Attr, Cell, Row, Table};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "wc.rs",
    version = "0.1.0",
    author = "PrivateRookie <996514515@qq.com>",
    about = "wc impl with Rust"
)]
struct Opts {
    /// Disable colorful output
    #[structopt(long)]
    no_color: bool,

    /// Disable colorful output
    #[structopt(long)]
    no_sep: bool,

    /// Prints the new line counts
    #[structopt(short, long)]
    lines: bool,

    /// Prints the words counts
    #[structopt(short, long)]
    words: bool,

    /// Prints the character counts
    #[structopt(short = "m", long)]
    chars: bool,

    /// File path(s) to run wc
    #[structopt(name = "FILE", parse(from_os_str), required = true)]
    files: Vec<PathBuf>,
}

#[derive(Debug)]
struct FileStats {
    name: PathBuf,
    lines: usize,
    words: usize,
    characters: usize,
}

#[derive(Debug)]
struct WcStats {
    stats: Vec<FileStats>,
    number_of_files: usize,
    total: FileStats,
    color_flag: bool,
    sep_flat: bool,
    line_flag: bool,
    word_flag: bool,
    char_flag: bool,
}

impl WcStats {
    fn new() -> WcStats {
        WcStats {
            stats: Vec::new(),
            number_of_files: 0,
            total: FileStats {
                name: PathBuf::new(),
                lines: 0,
                words: 0,
                characters: 0,
            },
            color_flag: true,
            sep_flat: true,
            line_flag: false,
            word_flag: false,
            char_flag: false,
        }
    }

    fn get_flag_from_opts(&mut self, opts: &Opts) {
        self.color_flag = !opts.no_color;
        self.sep_flat = !opts.no_sep;
        if !(opts.words || opts.lines || opts.chars) {
            self.line_flag = true;
            self.word_flag = true;
            self.char_flag = true;
        } else {
            self.line_flag = opts.lines;
            self.word_flag = opts.words;
            self.char_flag = opts.chars;
        }
    }

    fn get_stats(&mut self, file: PathBuf) -> Result<(), String> {
        match File::open(file.clone()) {
            Ok(mut fd) => {
                let mut contents = String::new();
                match fd.read_to_string(&mut contents) {
                    Ok(_) => {
                        let lines: Vec<&str> = contents.lines().collect();
                        let words: Vec<&str> = contents.split_ascii_whitespace().collect();

                        self.total.lines += lines.len();
                        self.total.words += words.len();
                        self.total.characters += contents.len();
                        self.stats.push(FileStats {
                            name: file,
                            lines: lines.len(),
                            words: words.len(),
                            characters: contents.len(),
                        });
                        Ok(())
                    }
                    Err(e) => Err(format!("wc: {}: {}", file.to_str().unwrap_or(""), e)),
                }
            }
            Err(_) => Err(format!(
                "wc: {}: No such file or direcotry",
                file.to_str().unwrap_or("")
            )),
        }
    }

    fn print_to_console(self) {
        let mut tb = Table::new();
        if !self.sep_flat {
            tb.set_format(*format::consts::FORMAT_NO_COLSEP);
        }
        let mut headers = vec![];
        if self.line_flag {
            headers.push("lines");
        }
        if self.word_flag {
            headers.push("words");
        }
        if self.char_flag {
            headers.push("charactors");
        }
        headers.push("file");
        if self.color_flag {
            let headers: Vec<Cell> = headers
                .iter()
                .map(|h| {
                    Cell::new(h)
                        .with_style(Attr::Bold)
                        .with_style(Attr::ForegroundColor(color::BLUE))
                })
                .collect();
            tb.set_titles(Row::new(headers));
        } else {
            let headers: Vec<Cell> = headers.iter().map(|h| Cell::new(h)).collect();
            tb.set_titles(Row::new(headers));
        }

        for stat in self.stats {
            let mut r = vec![];
            if self.line_flag {
                r.push(Cell::new(&stat.lines.to_string()));
            }
            if self.word_flag {
                r.push(Cell::new(&stat.words.to_string()))
            }
            if self.char_flag {
                r.push(Cell::new(&stat.characters.to_string()))
            }
            if self.color_flag {
                r.push(
                    Cell::new(stat.name.to_str().unwrap_or(""))
                        .with_style(Attr::ForegroundColor(color::GREEN)),
                );
            } else {
                r.push(Cell::new(stat.name.to_str().unwrap_or("")));
            }
            tb.add_row(Row::new(r));
        }

        if self.number_of_files > 1 {
            let mut r = vec![];
            if self.line_flag {
                r.push(Cell::new(&self.total.lines.to_string()));
            }
            if self.word_flag {
                r.push(Cell::new(&self.total.words.to_string()));
            }
            if self.char_flag {
                r.push(Cell::new(&self.total.characters.to_string()));
            }
            if self.color_flag {
                r.push(Cell::new("total").with_style(Attr::ForegroundColor(color::GREEN)));
            } else {
                r.push(Cell::new("total"));
            }
            tb.add_row(Row::new(r));
        }
        tb.printstd();
    }
}

fn main() {
    let opts = Opts::from_args();
    let mut wc = WcStats::new();
    wc.get_flag_from_opts(&opts);

    for path in opts.files {
        match wc.get_stats(path) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        }
    }
    wc.number_of_files = wc.stats.len();
    wc.print_to_console();
}
