mod guess_num_game;
mod variable;

pub fn official_main () {
    println!("============================  OFFICIAL_MAIN_EXECUTION  ============================");
    /* 2. 추리 게임 예제 */
    guess_num_game::guess_game();

    /* 3.1 변수와 가변성 */
    variable::var();

    println!("============================  OFFICIAL_MAIN_DONE  ============================");
}