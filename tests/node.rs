//! The tests in this suite use the default Node.js.


#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
extern crate wasm_game_of_life;
use wasm_game_of_life::Universe;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}


#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(1,2), (2,3), (3,1), (3,2), (3,3)]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(2,1), (2,3), (3,2), (3,3), (4,2)]);
    universe
}

#[wasm_bindgen_test]
pub fn test_tick() {
    // 让我们创建一个小点 的宇宙，带着我们微飞船测试吧!
    let mut input_universe = input_spaceship();

    // 我们宇宙的一次滴答后，我们的飞船应该变成了这样
    let expected_universe = expected_spaceship();

    // 调用 `tick` ，然后看看 `Universe`的单元是否一致
    input_universe.tick();
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}