use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

/// コンテンツモデル
#[derive(Deserialize, Serialize)]
pub struct Content {
    pub text: String,
    pub selects: HashMap<String, String>,
    pub answer: String,
}
impl std::fmt::Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "
        {}
        {:?}
        {}
        ---
        ",
            self.text, self.selects, self.answer
        )
    }
}

/// データファイルを読み込みContentにマッピングする。
/// データファイル名は環境変数「DATA_FILE」から取得する。
/// 設定されていない場合はデフォルトで「data.json」を使用する。
pub fn input_content() -> Option<Vec<Content>> {
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
#[allow(dead_code)]
fn output_content(c: Vec<Content>, file_name: &str) -> Result<(), ()> {
    let s = serde_json::to_string(&c).expect("cannot create json data.");
    let mut f = File::create(file_name).expect("failed to create file.");
    f.write_all(s.as_bytes()).expect("cannot write to file.");

    Ok(())
}

/// コンソール上で入力を受け付ける。引数で受け取った文字列を入力時のメッセージとして表示する。
pub fn input(message: &str) -> String {
    use std::io::{stdin, stdout};
    let mut s = String::new();
    print!("{message}");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("failed input.");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_content() {
        let contents: Vec<Content> = {
            let mut results: Vec<Content> = Vec::new();
            for i in 0..100 {
                let text = format!("text{}", i);
                let mut selects: HashMap<String, String> = HashMap::new();
                let answer = format!("ans{}", i);

                for j in 1..=4 {
                    selects.insert(format! {"{}",j}, format! {"selection{}",j});
                }

                results.push(Content {
                    text,
                    selects,
                    answer,
                });
            }
            results
        };

        assert_eq!(output_content(contents, "data_test.json"), Ok(()));
    }
}
