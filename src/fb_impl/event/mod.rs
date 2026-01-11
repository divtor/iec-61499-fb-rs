pub mod ctu;
pub mod sr;
pub mod switch;

const DEBUG: bool = true;

pub fn dbg_state_print(name: &'static str, state: &'static str) {
    if DEBUG {
        println!("{name}: {state}");
    }
}
