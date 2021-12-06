fn main() {
    if let Err(e) = fatalii::run() {
        eprintln!("{}", e);
    }
}
