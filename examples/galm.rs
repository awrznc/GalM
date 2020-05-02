use galm;

// Cargo.toml のバージョン
const VERSIONSTR: &'static str = env!("CARGO_PKG_VERSION");

// --help で表示する文言
const HELPSTR: &'static str = r"Usage:
    galm [-d | --dictionary (<file_path> | <string>)]
         [-h | --help] [-v | --version]
Options:
    -d --dictionary     dictionary file_path or string.
    -h --help           Show this message.
    -v --version        Show version number.";

// Extend of Levenshtein distance
fn get_levenshtein_distance(str1: &str, str2: &str) -> usize {

    // initialize galm
    let galm = galm::new();

    // initialize table
    let table_x_size = str1.chars().count() + 1;
    let table_y_size = str2.chars().count() + 1;
    let mut table = vec![0; table_x_size * table_y_size];
    for i in 0..table_x_size {
        table[i] = i * galm.max_distance_size;
    }
    for i in 0..table_y_size {
        table[i*(table_x_size)] = i * galm.max_distance_size;
    }

    // テーブルを埋める
    for i in 1..table_y_size {
        for j in 1..table_x_size {

            // 比較値の用意
            let up          = table[j+((i-1)*table_x_size)  ] + galm.max_distance_size;
            let left        = table[j+(  i  *table_x_size)-1] + galm.max_distance_size;
            let upper_left  = {
                let char1 = str1.chars().nth(j-1).unwrap();
                let char2 = str2.chars().nth(i-1).unwrap();
                let c = if char1 == char2 { 0 } else { galm.get_distance( &char1.to_string(), &char2.to_string() ) as usize };
                table[j+((i-1)*table_x_size)-1] + c
            };

            // 最小値を求める
            table[j+(i*table_x_size)] = std::cmp::min(std::cmp::min(up, left), upper_left);
        }
    }

    // テーブルの右下（配列の最後）の値を返す
    return table[(table_x_size*table_y_size)-1 as usize];
}


// 似ているパラメータ名を表示する
fn get_similar_param(input_param: &str) -> &str {
    use galm::search::Iterator;
    return [
        "-d", "--dictionary",
        "-h", "--help",
        "-v", "--version"
    ].iter().get_similar_word(input_param);
}

// main
fn main() {

    // 翻訳対象を読み込み
    let args: Vec<String> = std::env::args().collect();

    // 引数が指定されていない場合は --help を出力
    if args.len() <= 1 {
        println!("{}", HELPSTR);
        return ();
    }

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

    // 各処理
    match &*args[2] {
        "-d" | "--dictionary" => {
            if args.len() > 2 {
                let max_distance = usize::max_value();
                let input_param = &*args[1];
                let result = &*args[3].split(",").fold(("", max_distance), |most_similar_param, param| {
                        let distance = get_levenshtein_distance(input_param, param);
                        if distance < most_similar_param.1 { (param, distance) } else { most_similar_param }
                }).0;
                println!("{}", result);
            } else {
                println!("Error");
            }
        },
        _ => {
            println!(r"Error:
    not exists option: `{}`
    most similar param name: `{}`
    try 'galm --help' for more information",
                &*args[1],
                get_similar_param(&*args[1]));
        }
    }
}
