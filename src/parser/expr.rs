use super::identifier::identifier;
use super::literal::literal;
use ast::BinaryExpression;
use ast::Expression;
use ast::Operator;

named!(
    simple_expression<Expression>,
    ws!(alt_complete!(
        map!(literal, |l| Expression::Literal(l)) | map!(identifier, |i| Expression::Identifier(i))
    ))
);

named!(
  pub expression<Expression>,
  do_parse!(
    expr: alt_complete!(
      map!(single_binary_expression, |e| Expression::BinaryExpression(Box::new(e))) |
      // TODO recursive binary expression
     // map!(binary_expression, |e| Expression::BinaryExpression(Box::new(e))) |
     simple_expression
    ) >> tag!(";") >> (expr)
  )
);

named!(
    operator<Operator>,
    alt_complete!(
        map!(tag!("+"), |_| Operator::Plus)
            | map!(tag!("-"), |_| Operator::Minus)
            | map!(tag!("=="), |_| Operator::Equal)
            | map!(tag!("!="), |_| Operator::NotEqual)
            | map!(tag!(">"), |_| Operator::GreaterThan)
            | map!(tag!("<"), |_| Operator::LessThan)
    )
);

named!(
    binary_expression<BinaryExpression>,
    do_parse!(
        v1: expression
            >> op: operator
            >> v2: expression
            >> (BinaryExpression {
                left: v1,
                right: v2,
                operator: op,
            })
    )
);

named!(
    single_binary_expression<BinaryExpression>,
    do_parse!(
        v1: simple_expression
            >> op: operator
            >> v2: simple_expression
            >> (BinaryExpression {
                left: v1,
                right: v2,
                operator: op,
            })
    )
);

#[cfg(test)]
mod test {
    use super::*;
    use ast::Identifier;
    use ast::Literal;

    #[test]
    fn op() {
        assert_eq!(operator(&b"+"[..]), Ok((&b""[..], Operator::Plus)));
        assert_eq!(operator(&b"-"[..]), Ok((&b""[..], Operator::Minus)));
        assert_eq!(operator(&b"=="[..]), Ok((&b""[..], Operator::Equal)));
        assert_eq!(operator(&b"!="[..]), Ok((&b""[..], Operator::NotEqual)));
        assert_eq!(operator(&b">"[..]), Ok((&b""[..], Operator::GreaterThan)));
        assert_eq!(operator(&b"<"[..]), Ok((&b""[..], Operator::LessThan)));
    }

    #[test]

    fn binary_expression_literals() {
        assert_eq!(
            single_binary_expression(&b"\"kosong\" != kosong;"[..]),
            Ok((
                &b";"[..],
                BinaryExpression {
                    left: Expression::Literal(Literal::String(String::from("kosong"))),
                    right: Expression::Literal(Literal::Null),
                    operator: Operator::NotEqual
                }
            ))
        )
    }

    #[test]
    fn binary_expression_literal_id() {
        assert_eq!(
            single_binary_expression(&b"x > 5;"[..]),
            Ok((
                &b";"[..],
                BinaryExpression {
                    left: Expression::Identifier(Identifier {
                        name: String::from("x")
                    }),
                    right: Expression::Literal(Literal::Number(5)),
                    operator: Operator::GreaterThan
                }
            ))
        )
    }

    #[test]
    fn binary_expression_as_expression() {
        assert_eq!(
            expression(&b"x > 5;"[..]),
            Ok((
                &b""[..],
                Expression::BinaryExpression(Box::new(BinaryExpression {
                    left: Expression::Identifier(Identifier {
                        name: String::from("x")
                    }),
                    right: Expression::Literal(Literal::Number(5)),
                    operator: Operator::GreaterThan
                }))
            ))
        )
    }
}
