use crate::common::{input, Content};
use std::collections::HashMap;

/// コンソールアプリのメイン処理を実行する
pub fn run(data: Vec<Content>) -> Option<i32> {
    let mut received_answer: HashMap<usize, String> = HashMap::new();
    for (i, d) in data.iter().enumerate() {
        let no: usize = i + 1;

        println!("No.{}", no);
        println!("{}", d.text);

        let mut selects_vec: Vec<(&String, &String)> = d.selects.iter().collect();
        selects_vec.sort();
        for (k, v) in selects_vec {
            println!("{}: {}", k, v);
        }
        let answer = input("input(Q: quit)>>").to_uppercase();
        if answer == "Q" {
            break;
        }
        received_answer.insert(i, answer);
        println!("------------------------------------------------------------------");
    }

    let mut correct_count = 0;
    let mut sorted_recieved_answer: Vec<(&usize, &String)> = received_answer.iter().collect();
    sorted_recieved_answer.sort();
    for (k, v) in sorted_recieved_answer {
        let result = if data.get(*k).unwrap().answer == v.clone() {
            correct_count += 1;
            "o"
        } else {
            "x"
        };
        println!(
            "No.{}: {} -> {} [{}]",
            k + 1,
            v,
            data.get(*k).unwrap().answer,
            result
        );
    }
    println!("{}/{}", correct_count, received_answer.len());
    input("終了するには何かキーを入力してください....");

    Some(0)
}
