use std::collections::HashMap;
use crate::common::{ Content,input };

pub fn run(data: Vec<Content>) -> Option<i32>{
	
    // 入力結果保持用変数
    let mut received_answer = HashMap::new();
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

	Some(0)

}