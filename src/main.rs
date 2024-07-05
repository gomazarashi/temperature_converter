use std::io; //io(入出力)ライブラリ

fn main() {
    loop {
        let mut degree_type = String::new();

        println!("変換前の温度の形式を入力して下さい(C/F) (終了するには 'Q' を入力):");
        io::stdin()
            .read_line(&mut degree_type)
            .expect("行の読み込みに失敗しました");
        let degree_type = degree_type.trim().to_uppercase();

        // ユーザーが 'Q' を入力した場合、ループを終了
        if degree_type == "Q" {
            break;
        } else if degree_type != "C" && degree_type != "F" {
            println!("無効な形式です。'C' または 'F' を入力して下さい。");
            continue;
        }

        let degree = loop {
            let mut degree_str = String::new();
            println!("変換前の温度の数値を入力して下さい:");
            io::stdin()
                .read_line(&mut degree_str)
                .expect("行の読み込みに失敗しました");

            // 入力された温度をString型からf64型へ変換
            match degree_str.trim().parse::<f64>() {
                Ok(num) => break num,
                Err(_) => println!("数値を正しく入力して下さい。"),
            }
        };

        let result = temperature_converter(&degree_type, degree);
        println!("変換結果: {}", result);
    }
}

fn temperature_converter(degree_type: &str, degree: f64) -> f64 {
    if degree_type == "C" {
        degree * (9.0 / 5.0) + 32.0
    } else {
        (degree - 32.0) * (5.0 / 9.0)
    }
}
