// Rust Programming 25.01.20
// 화씨 온도를 섭씨 온도로 바꾸기

use std::io;

fn main() {
    println!("[Rust] 화씨 온도를 섭씨 온도로 변환");

    loop {
        println!("화씨 온도를 입력하십시오. :");

        // temperture_f 새로운 가변형 변수 만들기
        let mut temperture_f = String::new();

        // 사용자의 입력을 받아서 temperture_f 에 넣어주기
        io::stdin().read_line(&mut temperture_f).expect("");

        // temperture_f를 float로 바꾸기
        // 잘못된 입력 오류 처리
        let _temperture_f: f64 = match temperture_f.trim().parse() {
            Ok(c_float) => c_float,
            Err(_) => {
                // 오류 발생 시 출력
                println!("오류 : 화씨 온도를 입력하십시오.");
                // 계속 진행
            continue
            }
        };

        // _temperture_f 를 섭씨로 계산
        let result = (_temperture_f - 32.0) / 1.8;

        // 결괏값 출력
        println!("섭씨 {result}도 입니다.");

        break;
    };
}
