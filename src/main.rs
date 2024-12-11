mod types;

// 타입 - Documentation
// struct Book;
fn main() {
    // unused variable 있으면 compiler 가 뭐라함 
    let x/* : i16*/ = 10;
    // _ (underscore 붙이면 무시함) @ts-ignore 같은 것
    let _y : i16 = 9; 

    let guess = "hello world";
    
    println!("print hello world >>> {}",guess);
    types::main();
}

