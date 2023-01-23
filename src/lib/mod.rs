use egg::*;

define_language! {
    pub enum BooleanAlgebra {
        Boolean(bool),
        "+" = Dis([Id; 2]),
        "*" = Con([Id; 2]),
        "~" = Neg(Id),
        Symbol(Symbol),
    }
}

fn boolean_algebra_rules() -> Vec<Rewrite<BooleanAlgebra, ()>> {
    vec![
        rewrite!("commute-disjunction"; "(+ ?p ?q)" => "(+ ?q ?p)"),
        rewrite!("commute-conjunction"; "(* ?p ?q)" => "(* ?q ?p)"),
        rewrite!("double-negation"; "(~ (~ ?p))" => "?p"),
    ]
}

pub fn simplify(expr: &RecExpr<BooleanAlgebra>) -> RecExpr<BooleanAlgebra> {
    // parse the expression, the type annotation tells it which Language to use

    // simplify the expression using a Runner, which creates an e-graph with
    // the given expression and runs the given rules over it
    let runner = Runner::default()
        .with_expr(expr)
        .run(&boolean_algebra_rules());

    // the Runner knows which e-class the expression given with `with_expr` is in
    let root = runner.roots[0];

    // use an Extractor to pick the best element of the root eclass
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best) = extractor.find_best(root);
    println!("Simplified {expr} to {best} with cost {best_cost}");
    best
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_negation() {
        let expr = "(~ (~ p))".parse().unwrap();
        let simplified = simplify(&expr);
        let expected = "p".parse().unwrap();
        assert_eq!(simplified, expected);
    }
}
