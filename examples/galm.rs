use galm;

// Cargo.toml のバージョン
const VERSIONSTR: &'static str = env!("CARGO_PKG_VERSION");

// --help で表示する文言
const HELPSTR: &'static str = r"Usage:
    galm [-f | --file (<file_path> | <string>)]
         [-h | --help] [-v | --version]
Options:
    -f --file           file path.
    -h --help           Show this message.
    -v --version        Show version number.";

// 似ているパラメータ名を表示する
fn get_similar_param(input_param: &str) -> &str {
    use galm::search::Iterator;
    return [
        "-f", "--file",
        "-h", "--help",
        "-v", "--version"
    ].iter().get_similar_word(input_param);
}

// main
fn main() {

    // 翻訳対象を読み込み
    let args: Vec<String> = std::env::args().collect();

    // --version もしくは --help が指定されていた場合は、他のオプションに関係なく出力
    // また、両方指定されている場合は、先に指定されていた方を優先して出力
    for arg in args.iter() {
        match &*arg.to_string() {

            // print version
            "-v" | "--version" => {
                println!("galm version {}", VERSIONSTR);
                return ()
            },

            // print help
            "-h" | "--help" => {
                println!("{}", HELPSTR);
                return ()
            },
            _ => {}
        }
    }

    match args.len() {
        1 => {
            // 引数が指定されていない場合は --help を出力
            println!("{}", HELPSTR);
            return ();
        },
        2 => {

            use std::io::{self, Read};
            let mut buffer = String::new();
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            let _ = handle.read_to_string(&mut buffer);

            // initialize galm
            let galm = galm::new();

            // sort
            let sort_key = &*args[1];
            let mut vec = buffer.lines().collect::<Vec<&str>>();
            vec.sort_by_cached_key( |candidate| galm.get_word_distance(sort_key, candidate) );
            
            // print
            for word in vec {
                println!("{}", word);
            }
            return ();
        },
        _ => {}
    }

    // 各処理
    match &*args[2] {
        "-f" | "--file" => {
            if args.len() > 3 {

                use std::fs::File;
                use std::io::prelude::*;
                let mut file = File::open(&*args[3]).unwrap();
                let mut buffer = String::new();
                let _ = file.read_to_string(&mut buffer);

                // initialize galm
                let galm = galm::new();

                // sort
                let sort_key = &*args[1];
                let mut vec = buffer.lines().collect::<Vec<&str>>();
                vec.sort_by_cached_key( |candidate| galm.get_word_distance(sort_key, candidate) );

                // print
                use std::io::{stdout, Write, BufWriter};
                let out = stdout();
                let mut out = BufWriter::new(out.lock());
                for word in vec {
                    out.write(word.as_bytes()).unwrap();
                    out.write(b"\n").unwrap();
                }
            } else {
                println!(r"Error:
    filepath is not specified.");
            }
        },
        arg => {
            println!(r"Error:
    not exists option: `{}`
    most similar param name: `{}`
    try 'galm --help' for more information",
                arg,
                get_similar_param(arg));
        }
    }
}
