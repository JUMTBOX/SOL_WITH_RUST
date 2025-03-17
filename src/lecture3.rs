
pub fn lec03 () {
    // println!("remainder >>> {}", 13 % 6);
    // println!("divison >>> {}", 13 / 6);
    // println!("{}", 23 - 6 % 5);

    // 실수 -> 부동소수
    println!("합  : {}", 80.3 + 34.8);
    println!("합  : {}", 80.3 + 34.9);
    
    println!("{}", 115.2);
    // 4바이트 실수  => 115.1999 
    // 8바이트 실수  => 115.19999999999999  
    // 소수 x(곱하기) 10 의 몇승이냐로 실수가 기억된다.

    str();
}

// 더하기 
pub fn add01 (a:i32, b:i32) -> i32 {
    return a + b;
}

fn str () {
    println!("\n안녕하세요 \n\
        러스트에 오신것을 \n\
        환영합니다.
    ");


    // 쿼리 문 쓸때 좋다..!!!
    println!("\n\
        SELECT \n\
        MAX(created_at) as latest \n\
        FROM coupon.cpn_usr_pin_st\n\
        WHERE cpn_id = :cpnId
    ");
}