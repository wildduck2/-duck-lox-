use diagnostic::DiagnosticEngine;
use lexer::token::Token;

use crate::{ast::Expr, Parser};

impl Parser {
  /// Parses the `true`/`false` keywords into a boolean literal expression.
  pub(crate) fn parser_bool(
    &mut self,
    token: &mut Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let value = self.get_token_lexeme(token).parse::<bool>().unwrap();
    self.advance(engine); // consume the identifier
    token.span.merge(self.current_token().span);

    Ok(Expr::Bool {
      value,
      span: token.span,
    })
  }
}
