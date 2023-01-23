use std::iter::FromIterator;
use std::time::Duration;

use egg::*;

define_language! {
    pub enum BooleanLanguage {
        Boolean(bool),
        "+" = Dis([Id; 2]),
        "*" = Con([Id; 2]),
        "~" = Neg(Id),
        "=>" = Impl([Id; 2]),
        "<=>" = BiImpl([Id; 2]),
        Symbol(Symbol),
    }
}

fn boolean_algebra_rules() -> Vec<Rewrite<BooleanLanguage, ()>> {
    let biderectional = vec![
        rewrite!("double-negation"; "(~ (~ ?p))" <=> "?p"),
        rewrite!("ident-disjunction"; "(+ ?p false)" <=> "?p"),
        rewrite!("ident-conjunction"; "(* ?p true)" <=> "?p"),
        rewrite!("idemp-disjunction"; "(+ ?p ?p)" <=> "?p"),
        rewrite!("idemp-conjunction"; "(* ?p ?p)" <=> "?p"),
        rewrite!("distrib-over-disjunction"; "(* ?p (+ ?q ?r))" <=> "(+ (* ?p ?q) (* ?p ?r))"),
        rewrite!("distrib-over-conjunction"; "(+ ?p (* ?q ?r))" <=> "(* (+ ?p ?q) (+ ?p ?r))"),
        rewrite!("demorgan-1"; "(~ (+ ?p ?q))" <=> "(* (~ ?p) (~ ?q))"),
        rewrite!("demorgan-2"; "(~ (* ?p ?q))" <=> "(+ (~ ?p) (~ ?q))"),
        rewrite!("implication"; "(=> ?p ?q)" <=> "(+ (~ ?p) ?q)"),
        rewrite!("biimplication"; "(<=> ?p ?q)" <=> "(* (=> ?p ?q) (=> ?q ?p))"),
    ]
    .concat();
    let directional = vec![
        rewrite!("commute-disjunction"; "(+ ?p ?q)" => "(+ ?q ?p)"),
        rewrite!("commute-conjunction"; "(* ?p ?q)" => "(* ?q ?p)"),
        rewrite!("assoc-disjunction"; "(+ ?p (+ ?q ?r))" => "(+ (+ ?p ?q) ?r)"),
        rewrite!("assoc-conjunction"; "(* ?p (* ?q ?r))" => "(* (* ?p ?q) ?r)"),
        rewrite!("annihil-disjunction"; "(+ ?p true)" => "true"),
        rewrite!("annihil-conjunction"; "(* ?p false)" => "false"),
        rewrite!("absorption-1"; "(* ?p (+ ?p ?q))" => "?p"),
        rewrite!("absorption-2"; "(+ ?p (* ?p ?q))" => "?p"),
        rewrite!("complement-1"; "(+ ?p (~ ?p))" => "true"),
        rewrite!("complement-2"; "(* ?p (~ ?p))" => "false"),
    ];

    Vec::from_iter(biderectional.into_iter().chain(directional))
}

pub fn simplify(expr: &RecExpr<BooleanLanguage>) -> RecExpr<BooleanLanguage> {
    // parse the expression, the type annotation tells it which Language to use

    // simplify the expression using a Runner, which creates an e-graph with
    // the given expression and runs the given rules over it
    let runner = Runner::default()
        .with_iter_limit(100)
        .with_time_limit(Duration::from_secs(120))
        .with_node_limit(30_000)
        .with_expr(expr)
        .run(&boolean_algebra_rules());

    // the Runner knows which e-class the expression given with `with_expr` is in
    let root = runner.roots[0];

    // use an Extractor to pick the best element of the root eclass
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best) = extractor.find_best(root);
    println!("Simplified {expr} \nto {best} \nwith cost {best_cost}");
    best
}

pub fn check_equivalence(
    expr_a: &RecExpr<BooleanLanguage>,
    expr_b: &RecExpr<BooleanLanguage>,
) -> bool {
    let runner = Runner::default()
        .with_iter_limit(100)
        .with_time_limit(Duration::from_secs(120))
        .with_node_limit(30_000)
        .with_expr(expr_a)
        .with_expr(expr_b)
        .run(&boolean_algebra_rules());

    !runner.egraph.equivs(expr_a, expr_b).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expr(s: &str) -> RecExpr<BooleanLanguage> {
        s.parse().unwrap()
    }

    #[test]
    fn double_negation() {
        let e = expr("(~ (~ p))");
        let simplified = simplify(&e);
        let expected = expr("p");
        assert_eq!(simplified, expected);
    }

    #[test]
    fn check_contains() {
        let complex = expr("(* (~ (=> p q)) (=> p r))");
        let simplified = expr("(* p (* (~ q) r))");
        assert!(check_equivalence(&complex, &simplified))
    }
}
