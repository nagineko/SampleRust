fn main() {
    hello(); // 基本的な型の扱い方、実行時のエラー文への理解が深まる
    array();
    tuple(); // タプル型の扱い方について理解が深まる
    if_loop();
    class();
}

fn hello() {
    println!("########## hello関数の実行を開始します。 ##########");

    println!("Hello, world!");
    
    // 変数の宣言 (値の更新は不可)
    // 値の更新ができないので、事実上定数の宣言と同じ
    let bool_test: bool = true;
    let float_test: f64 = 1.0;
    let int_test: i32 = 5;
    let int_test_2: i32 = int_test;
    let string_test: &str = "配列のスライスに文字を割り当てました";                    // UTF-8配列のスライス
    let string_test_2: &'static str = "プログラムの実行中は常に有効な値です";         // UTF-8配列のスライス
    let string_test_3: &str = string_test_2;
    let string_test_4 = String::from("String::fromによって文字が割り当てられました"); // UTF-8配列であることを保証された Vec<u8>

    // 型を指定しない場合、デフォルトの型が採用される
    let float_default_test = 1.0;
    let int_default_test = 5;

    println!("bool_test: {}", bool_test);
    println!("float_test: {}", float_test);
    println!("int_test: {}", int_test);
    println!("int_test_2: {}", int_test_2);
    println!("float_default_test: {}", float_default_test);
    println!("int_default_test: {}", int_default_test);
    println!("string_test: {}", string_test);
    println!("string_test_2: {}", string_test_2);
    println!("string_test_3: {}", string_test_3);
    println!("string_test_4: {}", string_test_4);

    // 変数の宣言（値の更新可）
    let mut bool_test_2 = true;
    println!("bool_test_2(値更新前): {}", bool_test_2);

    bool_test_2 = false;
    println!("bool_test_2(値更新後): {}", bool_test_2);

    // 文字列を数値に変換
    let stri: String = String::from("31");
    let num: i32 = stri.parse().unwrap();
    println!("{}", num * 3);  // 5

    println!("########## hello関数の実行を終了します。 ##########");
}

fn array() {
    println!("########## array関数の実行を開始します。 ##########");

    // 固定長の配列
    let array_test: [i32; 5] = [1, 2, 3, 4, 5];
    let array_test_2 = [998, 30, 22];
    println!("{}", array_test[0]);
    println!("{}", array_test_2[0]);

    // 可変長の配列
    let mut array_length_test = vec![1, 2, 3];
    println!("配列の長さは {}", array_length_test.len());

    array_length_test.push(4);

    println!("配列の長さは {}", array_length_test.len());

    println!("########## array関数の実行を終了します。 ##########");
}

fn tuple() {
    println!("########## tuple関数の実行を開始します。 ##########");

    // タプルの中身を各変数にコピー
    let tuple = ([1, 2], "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{}, {}, {}, {}", a[1], b, c, d);

    println!("########## tuple関数の実行を終了します。 ##########");
}

fn if_loop() {
    println!("########## if_loop関数の実行を開始します。 ##########");

    let number: i32 = -1;

    if number == 1 {
        println!("numberの値は1です\nnumber: {}", number);
    } else if number > 0 {
        println!("numberの値は1以上\n: number {}", number);
    } else {
        println!("numberは1、または1以上のどちらでもありません\nnumber: {}", number);
    }

    for value in 1..5 {
        println!("for文が実行されています。\nvalue: {}", value);
    }

    let mut countries = vec!["日本", "フィリピン", "アメリカ"];

    countries.push("トルコ");

    for country in countries{
        println!("for文が実行されています {}", country);
    }

    println!("########## if_loop関数の実行を終了します。 ##########");
}

fn class() {
    println!("########## class関数の実行を開始します。 ##########");

    let human = Human { id: 222 };
    let id = human.get_id();
    println!("humanオブジェクトが持つidプロパティの値: {}", id);

    println!("########## class関数の実行を開始します。 ##########");
}

#[cfg(test)]
mod sample_test {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        assert_eq!(bad_add(1, 2), 3);
    }
}

/// # mardown形式でdocを書くことができます
/// `テスト`です
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

/// 人間オブジぇクト
struct Human {
    id: i32
}
impl Human {
    fn get_id(&self) -> i32 {
         self.id
    }
}
