// modules 의 약어 mod? (== import) 
mod types;

// 타입 - Documentation
// struct Book;
fn main() {
    // unused variable 있으면 compiler 가 뭐라함 
    let x/* : i16*/ = 10;
    // _ (underscore 붙이면 무시함) @ts-ignore 같은 것
    let _y : i16 = 9; 

    let guess = "hello world";
    
    // java 의 slf4j 의 log.info 같음...
    println!("{}", x);
    
    // ! : 매크로 ->> ! 앞에 있는 함수명(x) , 매크로명(o)
    // 함수 : 컴파일 -> 실행시 제어가 넘어간다.
    // 매크로 : 치환 -> 컴파일 단계에서 다른 코드로 바꿔치기 된다.
    
    //  println 의 실제 구현
    //  
    //   () => {
    //       $crate::print!("\n")
    //   };
    //   ($($arg:tt)*) => {{
    //       $crate::io::_print($crate::format_args_nl!($($arg)*));
    //   }};

    println!("print hello world >>> {}",guess);
    
    types::main2();

    println!("Addition  >>> {} + {} = {}", 5, 10 , add01(5, 10));
}

// 더하기 
fn add01 (a:i32, b:i32) -> i32 {
    return a + b;
}