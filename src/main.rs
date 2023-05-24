use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

// model
#[derive(Deserialize, Serialize)]
struct Content {
    text: String,
    selects: HashMap<String,String>,
//    selects: Vec<(String,String)>,
    answer: String,
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

// main
fn main(){
    // 入力結果保持用変数
    let mut received_answer = HashMap::new();
    // データ取得
    let data = input_content();
    //データ数分ループ
    for (i,d) in data.iter().enumerate() {
        let no = i+1;
        //インデックスから問題番号表示
        println!("No.{}",no);
        //問題文表示
        println!("{}",d.text);
        //選択肢表示
        let mut selects_vec: Vec<(&String,&String)> = d.selects.iter().collect();
        selects_vec.sort();
        for (k,v) in selects_vec{
            println!("{}: {}", k, v);
        }
        //入力受付
        let answer = input(">>").to_uppercase();
        if answer == "Q" {break;}
        received_answer.insert(i, answer);
        println!("------------------------------------------------------------------");
    }

    let mut correct_count = 0;
    let mut sorted_recieved_answer: Vec<(&usize, &String)> = received_answer.iter().collect();
    sorted_recieved_answer.sort();
    for (k, v) in sorted_recieved_answer {
        //正誤判定(contentのanswerの文字列と入力結果の文字列を比較するだけ)
        let result = if data.get(*k).unwrap().answer == v.clone() {
            correct_count +=1;
            "o"
        } else {
            "x"
        };
        //問題番号:入力値 -> 正解 o/x の形式で各問題結果出力
        println!("No.{}: {} -> {} [{}]", k+1, v, data.get(*k).unwrap().answer, result);
    }
    //正誤集計出力
    println!("{}/{}", correct_count, received_answer.len());
    input("終了するには何かキーを入力してください....");
}

/// データファイルを読み込みContentにマッピングする。
/// データファイル名は環境変数「DATA_FILE」から取得する。
/// 設定されていない場合はデフォルトで「data.json」を使用する。
fn input_content() -> Vec<Content>{
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

/// Content構造体からdata.jsonファイルを生成する。
/// テスト用メソッド。
#[allow(dead_code)]
fn output_content() {
    let c = Content {
        text: String::from("text1"),
        selects: HashMap::from([
            (String::from("A"), String::from("questionA")),
            (String::from("B"), String::from("questionB")),
            (String::from("C"), String::from("questionC")),
            (String::from("D"), String::from("questionD")),
            ]),
        answer: String::from("A"),
    };
    let s = serde_json::to_string(&c).expect("cannot create json data.");
    println!("{}",s);
    let mut f = File::create("data.json").expect("failed to create file.");
    f.write_all(
        serde_json::to_string(&c)
            .unwrap_or_default()
            .as_bytes(),
    ).expect("cannot write to file.");
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