use std::io; // io(入出力)ライブラリ

fn main() {
    loop {
        let degree_type = loop {
            println!("変換前の温度の形式を入力して下さい(C/F/K) (終了するには 'Q' を入力):");
            let input = read_input().trim().to_uppercase();
            if is_valid_type(&input) {
                break input;
            } else if input == "Q" {
                return;
            } else {
                println!("無効な形式です。'C'、'F'、または 'K' を入力して下さい。");
            }
        };

        let degree = loop {
            println!("変換前の温度の数値を入力して下さい:");
            match read_input().trim().parse::<f64>() {
                Ok(num) => break num,
                Err(_) => println!("数値を正しく入力して下さい。"),
            }
        };

        let target_type = loop {
            println!("変換後の温度の形式を入力して下さい(C/F/K):");
            let input = read_input().trim().to_uppercase();
            if is_valid_type(&input) {
                break input;
            } else {
                println!("無効な形式です。'C'、'F'、または 'K' を入力して下さい。");
            }
        };

        let result = convert_temperature(&degree_type, &target_type, degree);
        println!("変換結果: {} {}", result, target_type);
    }
}

// 標準入力を読み込む関数
fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("行の読み込みに失敗しました");
    input
}

// 温度の形式が有効かどうかを判定する関数
fn is_valid_type(degree_type: &str) -> bool {
    matches!(degree_type, "C" | "F" | "K")
}

// 温度を変換する関数
fn convert_temperature(from: &str, to: &str, degree: f64) -> f64 {
    match (from, to) {
        ("C", "F") => degree * (9.0 / 5.0) + 32.0,
        ("C", "K") => degree + 273.15,
        ("F", "C") => (degree - 32.0) * (5.0 / 9.0),
        ("F", "K") => (degree - 32.0) * (5.0 / 9.0) + 273.15,
        ("K", "C") => degree - 273.15,
        ("K", "F") => (degree - 273.15) * (9.0 / 5.0) + 32.0,
        _ => degree, // 同じ形式への変換はそのまま返す
    }
}
