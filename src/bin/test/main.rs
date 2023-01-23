use boolean_lang::simplify;

fn main() {
    let expr = "(~ (~ p))".parse().unwrap();
    println!("{}", simplify(&expr));
}
