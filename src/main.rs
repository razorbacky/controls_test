// Rust Programming 25.01.20
// 화씨 온도를 섭씨 온도로 바꾸기

use std::io;

fn main() {
    println!("[Rust] 화씨 온도를 섭씨 온도로 변환");

    println!("화씨 온도를 입력하십시오. :");

    // temperture_f 새로운 가변형 변수 만들기
    let mut temperture_f = String::new();

    // 사용자의 입력을 받아서 temperture_f 에 넣어주기
    io::stdin().read_line(&mut temperture_f).expect("");

    // temperture_f를 float로 바꾸기
    let _temperture_f: f64 = temperture_f.trim().parse().expect("");

    // _temperture_f 를 섭씨로 계산
    let result = (_temperture_f - 32.0) / 1.8;

    // 결괏값 출력
    println!("섭씨 {result}도 입니다.");
}
