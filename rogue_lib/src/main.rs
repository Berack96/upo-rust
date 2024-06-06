fn main() {
    let seed = rand::random();
    rogue_lib::run_console("Player".to_string(), seed);
}
