use boolean_lang::simplify;

fn main() {
    // let expr = "(* (~ (+ (~ p) q)) (+ (~ p) r))".parse().unwrap();
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    println!("{}", simplify(&buf.parse().unwrap()));
}
