use std::collections::HashMap;

//第1章: 準備運動

//00. 文字列の逆順
//文字列"stressed"の文字を逆に（末尾から先頭に向かって）並べた文字列を得よ．
fn knock00() {
    let input = "stressed";
    let target = "desserts";
    let answer: String = input.chars().rev().collect::<String>();

    assert_eq!(answer, target);
}

//01. 「パタトクカシーー」
//「パタトクカシーー」という文字列の1,3,5,7文字目を取り出して連結した文字列を得よ．
fn knock01() {
    let input = "パタトクカシーー";
    let target = "パトカー";
    let answer = input.chars().step_by(2).collect::<String>();

    assert_eq!(answer, target);
}

//02. 「パトカー」＋「タクシー」＝「パタトクカシーー」
//「パトカー」＋「タクシー」の文字を先頭から交互に連結して文字列「パタトクカシーー」を得よ．
fn knock02() {
    let input1 = "パトカー";
    let input2 = "タクシー";
    let target = "パタトクカシーー";
    let answer = input1.chars().zip(input2.chars()).map(|(c1, c2)| format!("{}{}", c1, c2)).collect::<String>();

    assert_eq!(answer, target);
}

//03. 円周率
//"Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics."という文を単語に分解し，各単語の（アルファベットの）文字数を先頭から出現順に並べたリストを作成せよ．
fn knock03() {
    let input = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    let answer: Vec<i32> = vec![3,1,4,1,5,9,2,6,5,3,5,8,9,7,9];
    let target = input
                 .replace(",", "")
                 .replace(".", "")
                 .split(" ")
                 .map(|w| w.len() as i32)
                 .collect::<Vec<i32>>();

    assert_eq!(answer, target);
}

//04. 元素記号
//"Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can."という文を単語に分解し，1, 5, 6, 7, 8, 9, 15, 16, 19番目の単語は先頭の1文字，それ以外の単語は先頭に2文字を取り出し，取り出した文字列から単語の位置（先頭から何番目の単語か）への連想配列（辞書型もしくはマップ型）を作成せよ．
fn knock04() {
    let input = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let indexes = vec![0, 4, 5, 6, 7, 8, 14, 15, 18];
    let target = vec![
            ("H".to_owned(),  1),
            ("He".to_owned(), 2),
            ("Li".to_owned(), 3),
            ("Be".to_owned(), 4),
            ("B".to_owned(),  5),
            ("C".to_owned(),  6),
            ("N".to_owned(),  7),
            ("O".to_owned(),  8),
            ("F".to_owned(),  9),
            ("Ne".to_owned(), 10),
            ("Na".to_owned(), 11),
            ("Mi".to_owned(), 12),
            ("Al".to_owned(), 13),
            ("Si".to_owned(), 14),
            ("P".to_owned(),  15),
            ("S".to_owned(),  16),
            ("Cl".to_owned(), 17),
            ("Ar".to_owned(), 18),
            ("K".to_owned(),  19),
            ("Ca".to_owned(), 20),
        ]
        .into_iter()
        .collect::<HashMap<String, i32>>();

    let answer = input
                 .replace(",", "")
                 .replace(".", "")
                 .split(" ")
                 .enumerate()
                 .map(|(i, w)| {
                     if indexes.contains(&i) {
                        (format!("{}", &w[..1]), (i + 1) as i32)

                     } else {
                        (format!("{}", &w[..2]), (i + 1) as i32)

                     }
                 })
                 .collect::<HashMap<String, i32>>();

    for (k, v) in &target {
        assert_eq!(v, &answer[k]);
    }
}

//05. n-gram
//与えられたシーケンス（文字列やリストなど）からn-gramを作る関数を作成せよ．この関数を用い，"I am an NLPer"という文から単語bi-gram，文字bi-gramを得よ．
fn knock05() {

}

//06. 集合
//"paraparaparadise"と"paragraph"に含まれる文字bi-gramの集合を，それぞれ, XとYとして求め，XとYの和集合，積集合，差集合を求めよ．さらに，'se'というbi-gramがXおよびYに含まれるかどうかを調べよ．
fn knock06() {

}

//07. テンプレートによる文生成
//引数x, y, zを受け取り「x時のyはz」という文字列を返す関数を実装せよ．さらに，x=12, y="気温", z=22.4として，実行結果を確認せよ．
fn knock07() {

}

//08. 暗号文
//与えられた文字列の各文字を，以下の仕様で変換する関数cipherを実装せよ．
//英小文字ならば(219 - 文字コード)の文字に置換
//その他の文字はそのまま出力
//この関数を用い，英語のメッセージを暗号化・復号化せよ．
fn knock08() {

}

//09. Typoglycemia
//スペースで区切られた単語列に対して，各単語の先頭と末尾の文字は残し，それ以外の文字の順序をランダムに並び替えるプログラムを作成せよ．ただし，長さが４以下の単語は並び替えないこととする．適当な英語の文（例えば"I couldn't believe that I could actually understand what I was reading : the phenomenal power of the human mind ."）を与え，その実行結果を確認せよ．
fn knock09() {

}

fn main() {
    knock00();
    knock01();
    knock02();
    knock03();
    knock04();
    knock05();
    knock06();
    knock07();
    knock08();
    knock09();

}
