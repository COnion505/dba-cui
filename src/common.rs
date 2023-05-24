use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};

// model
#[derive(Deserialize, Serialize)]
pub struct Content {
    pub text: String,
    pub selects: HashMap<String,String>,
    pub answer: String,
}
impl std::fmt::Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "
        {}
        {:?}
        {}
        ---
        ", self.text,self.selects,self.answer)
    }
}

/// データファイルを読み込みContentにマッピングする。
/// データファイル名は環境変数「DATA_FILE」から取得する。
/// 設定されていない場合はデフォルトで「data.json」を使用する。
pub fn input_content() -> Option<Vec<Content>>{
    extern crate dotenv;
    use dotenv::dotenv;
    use std::env;
    dotenv().ok();
    let data_path: String = env::var("DATA_FILE").unwrap_or_else(|_| String::from("data.json"));
    let mut f = File::open(data_path).expect("cannot open file.");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("cannot read file.");

    serde_json::from_str(s.as_str()).expect("cannot serialize to json.")
}

/// コンソール上で入力を受け付ける。引数で受け取った文字列を入力時のメッセージとして表示する。
pub fn input(message: &str)-> String {
    use std::io::{stdin, stdout};
    let mut s = String::new();
    print!("{message}");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("failed input.");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    s
}