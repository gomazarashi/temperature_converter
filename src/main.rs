use std::io; //io(入出力)ライブラリ

fn main() {
    loop {
        println!("変換前の温度の形式を入力して下さい(C/F) (終了するには 'Q' を入力):");
        let degree_type = read_input().trim().to_uppercase();

        match degree_type.as_str() {
            "Q" => break,
            "C" | "F" => {
                let degree = loop {
                    println!("変換前の温度の数値を入力して下さい:");
                    match read_input().trim().parse::<f64>() {
                        Ok(num) => break num,
                        Err(_) => println!("数値を正しく入力して下さい。"),
                    }
                };

                let result = temperature_converter(&degree_type, degree);
                println!("変換結果: {}", result);
            }
            _ => println!("無効な形式です。'C' または 'F' を入力して下さい。"),
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("行の読み込みに失敗しました");
    return input;
}

fn temperature_converter(degree_type: &str, degree: f64) -> f64 {
    if degree_type == "C" {
        degree * (9.0 / 5.0) + 32.0
    } else {
        (degree - 32.0) * (5.0 / 9.0)
    }
}
