use std::collections::btree_map::Values;

/// Операчия над двумя выражениями.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// Выражение в форме узла дерева.
#[derive(Debug)]
enum Expression {
    /// Операция над двумя дочерними выражениями.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// Значение
    Value(i64),
}

fn eval(e: Expression) -> i64 {
    match e {
        Expression::Op { op, left, right } => match op {
            Operation::Add =>  eval(*left) + eval(*right),
            Operation::Sub =>  eval(*left) - eval(*right),
            Operation::Mul =>  eval(*left) * eval(*right),
            Operation::Div =>  eval(*left) / eval(*right),
        },
        Expression::Value(result) => result,
    }
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), 19);
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        30
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        85
    );
}

#[test]
fn test_zeros() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
}

pub fn demonstrate() {
    println!("--- Task 6 ---");

    println!("Результат 19 = {}", eval(Expression::Value(19)));

    println!("Результат 10 + 20 = {}", eval(Expression::Op {
        op: Operation::Add,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(20)),
    }));

    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    println!("Результат (10 * 9) + ((3 - 4) * 5) = {}", eval(Expression::Op {
        op: Operation::Add,
        left: Box::new(term1),
        right: Box::new(term2),
    }));

    println!("Результат 0 + 0 = {}", eval(Expression::Op {
        op: Operation::Add,
        left: Box::new(Expression::Value(0)),
        right: Box::new(Expression::Value(0))
    }));

    println!("Результат 0 - 0 = {}", eval(Expression::Op {
        op: Operation::Sub,
        left: Box::new(Expression::Value(0)),
        right: Box::new(Expression::Value(0))
    }));

    println!("Результат 0 * 0 = {}", eval(Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(0)),
        right: Box::new(Expression::Value(0))
    }));
}