fn get_from_file() -> Result<i32, String> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    num_str // 最初 num_str は"&str"型
        .trim() // 文字列前後の空白文字を削除
        .parse::<i32>() // '&str' を 'i32' に変換する．結果は Result<i32, ParseIntError>
        .map(|t| t * 2) // parseの結果が Ok(t)の場合のみここを実行
        .map_err(|e| e.to_string()) // parseの結果が Err(e)の場合のみeの文字列表現
}

fn main() {
    match get_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}
