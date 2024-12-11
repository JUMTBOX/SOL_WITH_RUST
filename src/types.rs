// Primitive types 
/**
 * integers
 * i8, i16, i32, i64, i128 and isize  == signed Integer
 * u8, u16, u32, u64, u128 and usize  == unsigned Integer
 * 
 * 음수가 될 수 있는 수는 i 사용
 * 음수가 될 수 있는 수라면 u 사용
 * 
 * bits = 8bit = 1byte -> 메모리 사이즈가 클 수록 큰 수를 가질 수 있음
 * 
 * isize & usize ?
 * [ computer architecture 에 따라달라짐 ]
 * isize -> 32비트 운영체제 -> i32
 * usize -> 32비트 운영체제 -> u32 
 *  
 * isize -> 64비트 운영체제 -> i64
 * usize -> 64비트 운영체제 -> u64  
 */

pub(crate) fn main () {
    let my_number1:u8 = 100; // i32 - 가장 활용도가 높은 타입
    let my_number2 = 100;
    
    // type inference
    /* 같은 정수형이어도 메모리타입이 틀리면 같이 쓸 수 없다.. */
    let third_num = my_number1 + my_number2;

    println!("THIRD_NUMBER >>> {}", third_num);
}