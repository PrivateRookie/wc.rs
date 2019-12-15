use prettytable::{color, format, Attr, Cell, Row, Table};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

fn pure_format() -> format::TableFormat {
    format::FormatBuilder::new()
        .column_separator(' ')
        .borders(' ')
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Bottom],
            format::LineSeparator::new(' ', ' ', ' ', ' '),
        )
        .padding(1, 1)
        .build()
}

fn no_sep_format() -> format::TableFormat {
    *format::consts::FORMAT_NO_COLSEP
}

#[derive(StructOpt, Debug, Clone)]
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

    /// Disable sep in output
    #[structopt(long)]
    no_sep: bool,

    /// Use pure mode
    #[structopt(long)]
    pure: bool,

    /// Do not show headers
    #[structopt(long)]
    no_headers: bool,

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

impl Opts {
    fn turn_over_output_opts(&mut self) {
        if !(self.lines || self.words || self.chars) {
            self.lines = true;
            self.words = true;
            self.chars = true;
        }
    }
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
    opts: Opts,
}

impl WcStats {
    fn new(opts: &Opts) -> WcStats {
        let mut new_opts = opts.clone();
        new_opts.turn_over_output_opts();
        WcStats {
            stats: Vec::new(),
            number_of_files: 0,
            total: FileStats {
                name: PathBuf::new(),
                lines: 0,
                words: 0,
                characters: 0,
            },
            opts: new_opts,
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

    fn format_table(&self) -> Table {
        let mut tb = Table::new();
        if self.opts.pure {
            tb.set_format(pure_format())
        } else if self.opts.no_sep {
            tb.set_format(no_sep_format())
        }
        if !self.opts.no_headers {
            let mut headers = vec![];
            if self.opts.lines {
                headers.push("lines");
            }
            if self.opts.words {
                headers.push("words");
            }
            if self.opts.chars {
                headers.push("charactors");
            }
            headers.push("file");
            if self.opts.no_color {
                let headers: Vec<Cell> = headers.iter().map(|h| Cell::new(h)).collect();
                tb.set_titles(Row::new(headers));
            } else {
                let headers: Vec<Cell> = headers
                    .iter()
                    .map(|h| {
                        Cell::new(h)
                            .with_style(Attr::Bold)
                            .with_style(Attr::ForegroundColor(color::BLUE))
                    })
                    .collect();
                tb.set_titles(Row::new(headers));
            }
        };
        tb
    }

    fn print_to_console(self) {
        let mut tb = self.format_table();

        for stat in self.stats {
            let mut r = vec![];
            if self.opts.lines {
                r.push(Cell::new(&stat.lines.to_string()));
            }
            if self.opts.words {
                r.push(Cell::new(&stat.words.to_string()))
            }
            if self.opts.chars {
                r.push(Cell::new(&stat.characters.to_string()))
            }
            if self.opts.no_color {
                r.push(Cell::new(stat.name.to_str().unwrap_or("")));
            } else {
                r.push(
                    Cell::new(stat.name.to_str().unwrap_or(""))
                        .with_style(Attr::ForegroundColor(color::GREEN)),
                );
            }
            tb.add_row(Row::new(r));
        }

        if self.number_of_files > 1 {
            let mut r = vec![];
            if self.opts.lines {
                r.push(Cell::new(&self.total.lines.to_string()));
            }
            if self.opts.words {
                r.push(Cell::new(&self.total.words.to_string()));
            }
            if self.opts.chars {
                r.push(Cell::new(&self.total.characters.to_string()));
            }
            if self.opts.no_color {
                r.push(Cell::new("total"));
            } else {
                r.push(Cell::new("total").with_style(Attr::ForegroundColor(color::GREEN)));
            }
            tb.add_row(Row::new(r));
        }
        tb.printstd();
    }
}

fn main() {
    let opts = Opts::from_args();
    let mut wc = WcStats::new(&opts);

    for path in opts.files {
        match wc.get_stats(path) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        }
    }
    wc.number_of_files = wc.stats.len();
    wc.print_to_console();
}
