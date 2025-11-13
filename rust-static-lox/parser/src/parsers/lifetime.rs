use crate::{DiagnosticEngine, Parser};
use lexer::token::TokenKind;

impl Parser {
  pub(crate) fn parse_lifetime_bounds(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<Vec<String>, ()> {
    let mut bounds = vec![];
    while !self.is_eof()
      && !matches!(
        self.current_token().kind,
        TokenKind::OpenBrace | TokenKind::Comma | TokenKind::Gt
      )
    {
      let lifetime = self.get_token_lexeme(&self.current_token());
      self.advance(engine); // consume the lifetime

      if matches!(self.current_token().kind, TokenKind::Plus) {
        self.advance(engine); // consume the plus
      }

      bounds.push(lifetime);
    }

    Ok(bounds)
  }
}
