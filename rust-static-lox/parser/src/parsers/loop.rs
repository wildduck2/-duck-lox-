use crate::{
  ast::{Attribute, Expr},
  Parser,
};
use diagnostic::DiagnosticEngine;

impl Parser {
  pub(crate) fn parse_loop_expression(
    &mut self,
    label: Option<String>,
    outer_attributes: Vec<Attribute>,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let mut token = self.current_token();
    self.advance(engine); // consume the "loop"
    let body = self.parse_block(None, outer_attributes, engine)?;

    token.span.merge(self.current_token().span);
    Ok(Expr::Loop {
      label,
      body: Box::new(body),
      span: token.span,
    })
  }
}
