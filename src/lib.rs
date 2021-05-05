//! # GalM
//!
//! [![Actions Status](https://github.com/awrznc/GalM/workflows/Build/badge.svg)](https://github.com/awrznc/GalM/actions)
//! [![Crate](https://img.shields.io/crates/v/galm.svg)](https://crates.io/crates/galm)
//!
//! GalM is matching library.<br>
//! Inspired by Galmoji.
//!
//! <img src="https://awrznc.github.io/GalM/assets/image/galm_20210504.png" width="600">
//!
//! ## Quick start
//!
//! Put the following in your project's Cargo.toml file:
//!
//! ```toml
//! [dependencies]
//! galm = "0.1.3"
//! ```
//!
//! And overwrite in your project's main.rs file:
//!
//! ```rust
//! // Get the matching rate.
//! fn main() {
//!
//!     // Initialize GalM Database instance.
//!     let galm: galm::Database = galm::new();
//!
//!     // Get the matching rate of character.
//!     let distance: u8 = galm.get_distance("王", "玉");
//!
//!     assert_eq!(distance, 30);
//! }
//! ```
//!
//! GalM's matcher supports the following characters.
//!
//! ```text
//! 一右雨円王音下火花貝学気九休玉金空月犬見五口校左三山子四糸字耳七車手十出女小上森人水正生青夕石赤千川
//! 先早草足村大男竹中虫町天田土二日入年白八百文木本名目立力林六引羽雲園遠何科夏家歌画回会海絵外角楽活間
//! 丸岩顔汽記帰弓牛魚京強教近兄形計元言原戸古午後語工公広交光考行高黄合谷国黒今才細作算止市矢姉思紙寺自
//! 時室社弱首秋週春書少場色食心新親図数西声星晴切雪船線前組走多太体台地池知茶昼長鳥朝直通弟店点電刀冬当
//! 東答頭同道読内南肉馬売買麦半番父風分聞米歩母方北毎妹万明鳴毛門夜野友用曜来里理話悪安暗医委意育員院飲
//! 運泳駅央横屋温化荷界開階寒感漢館岸起期客究急級宮球去橋業曲局銀区苦具君係軽血決研県庫湖向幸港号根祭皿
//! 仕死使始指歯詩次事持式実写者主守取酒受州拾終習集住重宿所暑助昭消商章勝乗植申身神真深進世整昔全相送想
//! 息速族他打対待代第題炭短談着注柱丁帳調追定庭笛鉄転都度投豆島湯登等動童農波配倍箱畑発反坂板皮悲美鼻筆
//! 氷表秒病品負部服福物平返勉放味命面問役薬由油有遊予羊洋葉陽様落流旅両緑礼列練路和愛案以衣位囲胃印英栄
//! 塩億加果貨課芽改械害街各覚完官管関観願希季紀喜旗器機議求泣救給挙漁共協鏡競極訓軍郡径型景芸欠結建健験
//! 固功好候航康告差菜最材昨札刷殺察参産散残士氏史司試児治辞失借種周祝順初松笑唱焼象照賞臣信成省清静席積
//! 折節説浅戦選然争倉巣束側続卒孫帯隊達単置仲貯兆腸低底停的典伝徒努灯堂働特得毒熱念敗梅博飯飛費必票標不
//! 夫付府副粉兵別辺変便包法望牧末満未脈民無約勇要養浴利陸良料量輪類令冷例歴連老労録圧移因永営衛易益液演
//! 応往桜恩可仮価河過賀快解格確額刊幹慣眼基寄規技義逆久旧居許境均禁句群経潔件券険検限現減故個護効厚耕鉱
//! 構興講混査再災妻採際在財罪雑酸賛支志枝師資飼示似識質舎謝授修述術準序招承証条状常情織職制性政勢精製税
//! 責績接設舌絶銭祖素総造像増則測属率損退貸態団断築張提程適敵統銅導徳独任燃能破犯判版比肥非備俵評貧布婦
//! 富武復複仏編弁保墓報豊防貿暴務夢迷綿輸余預容略留領異遺域宇映延沿我灰拡革閣割株干巻看簡危机揮貴疑吸供
//! 胸郷勤筋系敬警劇激穴絹権憲源厳己呼誤后孝皇紅降鋼刻穀骨困砂座済裁策冊蚕至私姿視詞誌磁射捨尺若樹収宗就
//! 衆従縦縮熟純処署諸除将傷障城蒸針仁垂推寸盛聖誠宣専泉洗染善奏窓創装層操蔵臓存尊宅担探誕段暖値宙忠著庁
//! 頂潮賃痛展討党糖届難乳認納脳派拝背肺俳班晩否批秘腹奮並陛閉片補暮宝訪亡忘棒枚幕密盟模訳郵優幼欲翌乱卵
//! 覧裏律臨朗論
//! ```
//!
//! ## Example
//!
//! Print the most similar string from the strings separated by newline.
//!
//! ```bash
//! # build
//! cargo build --example galm --release
//!
//! # use galm
//! echo -e "皇様\n玉様\n大様" | ./target/release/examples/galm "王様"
//! # => 玉様
//! # => 大様
//! # => 皇様
//! ```
//!
//! ## GalM Command
//!
//! ### Install
//!
//! ```bash
//! # install galm
//! cargo install galm --example galm
//!
//! # using galm
//! echo -e "皇様\n玉様\n大様" | galm "王様"
//! # => 玉様
//! # => 大様
//! # => 皇様
//! ```
//!
//! ### Uninstall
//!
//! ```bash
//! # uninstall galm
//! cargo uninstall galm
//! ```
//!


// #[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[derive(Debug)]
pub struct Characters {
    pub names: Vec<&'static str>,
    pub costs: Vec<usize>,
}


macro_rules! load_csv {
    ($path:tt) => {
        {
            let mut characters: Vec<&str> = Vec::new();
            let costs: Vec<usize> = include_str!(
                $path
            ).lines().enumerate().filter_map( |(line_index, line_value)| {
                if line_index != 0 {
                    Some(
                        line_value.split(',').map( |comma_value| {
                            comma_value.parse::<usize>().unwrap()
                        }).collect::<Vec<usize>>()
                    )
                } else {
                    characters = line_value.split(',').collect();
                    None
                }
            }).into_iter().flatten().collect();

            Characters { names: characters, costs: costs }
        }
    };
}


/// Database
#[derive(Debug)]
pub struct Database {
    pub characters: Characters,
    pub max_distance_size: usize
}

/// Database
impl Database {

    /// Initialize GalM Database instance.
    ///
    /// ```rust
    /// let galm: galm::Database = galm::Database::new();
    /// ```
    pub fn new() -> Database {
        return Database {
            characters: load_csv!("./../docs/assets/csv/characters.csv"),
            max_distance_size: 255
        };
    }

    /// Get the matching rate of character.
    /// Range is 0..[`galm::Database.max_distance_size`](#structfield.max_distance_size).
    /// The more similar the two arguments, the smaller the return value.
    ///
    /// ```rust
    /// // Initialize GalM Database instance.
    /// let galm: galm::Database = galm::Database::new();
    ///
    /// // Get the matching rate of character.
    /// let distance: u8 = galm.get_distance("王", "玉");
    ///
    /// assert_eq!(distance, 30);
    /// ```
    pub fn get_distance(&self, from: &str, to: &str) -> u8 {
        let index_x = match self.characters.names.iter().position(|&r| r == from) {
            Some(i) => i,
            None => return self.max_distance_size  as u8
        };
        let index_y = match self.characters.names.iter().position(|&r| r == to) {
            Some(i) => i,
            None => return self.max_distance_size  as u8
        };
        return match self.characters.costs[index_y + ( index_x * self.characters.names.len() )] {
            i if self.max_distance_size < i => { self.max_distance_size as u8 },
            i => i as u8,
        };
    }

    /// Get the matching rate of word.
    /// Range is 0..[`std::usize::MAX`](https://doc.rust-lang.org/std/usize/constant.MAX.html).
    /// The more similar the two arguments, the smaller the return value.
    ///
    /// ```rust
    /// // Initialize GalM Database instance.
    /// let galm: galm::Database = galm::Database::new();
    ///
    /// let sort_key = "王様";
    /// let mut vec = ["皇様", "玉様", "大様"];
    ///
    /// // Sort Example
    /// vec.sort_by_key( |candidate| galm.get_word_distance(sort_key, candidate) );
    ///
    /// assert_eq!(vec, ["玉様", "大様", "皇様"]);
    /// ```
    pub fn get_word_distance(&self, str1: &str, str2: &str) -> usize {

        // initialize table
        let table_x_size = str1.chars().count() + 1;
        let table_y_size = str2.chars().count() + 1;
        let mut table = vec![0; table_x_size * table_y_size];
        for i in 0..table_x_size {
            table[i] = i * self.max_distance_size;
        }
        for i in 0..table_y_size {
            table[i*(table_x_size)] = i * self.max_distance_size;
        }

        // テーブルを埋める
        // Extend of Levenshtein distance
        for i in 1..table_y_size {
            for j in 1..table_x_size {

                // 比較値の用意
                let up          = table[j+((i-1)*table_x_size)  ] + self.max_distance_size;
                let left        = table[j+(  i  *table_x_size)-1] + self.max_distance_size;
                let upper_left  = {
                    let char1 = str1.chars().nth(j-1).unwrap();
                    let char2 = str2.chars().nth(i-1).unwrap();
                    let c = if char1 == char2 { 0 } else { self.get_distance( &char1.to_string(), &char2.to_string() ) as usize };
                    table[j+((i-1)*table_x_size)-1] + c
                };

                // 最小値を求める
                table[j+(i*table_x_size)] = std::cmp::min(std::cmp::min(up, left), upper_left);
            }
        }

        // テーブルの右下（配列の最後）の値を返す
        return table[(table_x_size*table_y_size)-1 as usize];
    }
}


/// Initialize GalM Database instance.
///
/// ```rust
/// let galm: galm::Database = galm::new();
/// ```
pub fn new() -> Database {
    return Database::new();
}

pub mod search;
