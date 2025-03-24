pub (crate) fn var () {
    /* 변수 */
    let _x = 6;
    // x = 5;   => compile error / 기본적으로 변수는 불변성을 지닌다.

    let mut _y = 5; // => mut 키워드를 통해 컴파일러에게 가변적인 변수 선언임을 알린다.
    _y = 10; 

    /* 상수 */
    const _THREE_HOURS_IN_SECONDS:u32 = 3 * 60 * 60; 
    // => mut 키워드와 같이 사용될 수 없으며, 값의 타입이 "반드시" 명시되어야 한다.
    // => "반드시" 표현식으로만 설정될 수 있고, 런타임에만 계산될 수 있는 결괏값으로는 안된다. 

    /* 섀도잉 */
    let xy =5;

    let xy = xy + 1;

    {
        let xy = xy * 2;
        println!("The value of xy in the inner scope : {xy}");
    }

    println!("value of xy in outer : {xy}");
    // => mut 와의 차이점은 섀도잉은 다른 타입으로의 전환이 가능하지만 
    // mut 키워드를 통한 재할당은 다른 타입의 값을 할당하면 컴파일러 에러가 발생한다. 

}