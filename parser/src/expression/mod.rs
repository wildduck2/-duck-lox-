/*
*
*  The expressions
*
*  expression -> literal
*              | unary
*              | binary
*              | grouping;
*
*  literal    -> NUMBER
*              | STRING
*              | "true"
*              | "false"
*              | "nil";
*
*  grouping   -> "(" expression ")" ;
*
*  unary      -> ("-" | "!") expression
*
*  binary     -> expression operator expression
*
*  operator   -> "+" | "-" | "*" | "/"
*                "!=" | "==" | "<=" | ">=" | "<" | ">"
*
*/

use scanner::token::Token;

#[derive(Debug)]
pub enum Expr {
  Literal(Token),
  Unary {
    operator: Token,
    right: Box<Expr>,
  },
  Binary {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
  },
  Grouping(Box<Expr>),
}
