fn main() {
    match automaton_runner::main() {
        Err(err) => eprintln!("{}", err),
        _ => (),
    }
}
