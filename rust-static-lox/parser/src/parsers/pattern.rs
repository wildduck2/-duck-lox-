use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::{
  ast::{pattern::*, Mutability},
  match_and_consume, Parser,
};

impl Parser {
  pub(crate) fn parse_pattern(
    &mut self,
    reference: bool,
    mutability: Mutability,
    engine: &mut DiagnosticEngine,
  ) -> Result<Pattern, ()> {
    let mut token = self.current_token();

    match token.kind {
      TokenKind::Ident => {
        token.span.merge(self.current_token().span);
        self.advance(engine);

        let subpattern = if match_and_consume!(self, engine, TokenKind::At)? {
          Some(self.parse_pattern(reference, mutability.clone(), engine)?)
        } else {
          None
        };

        Ok(Pattern::Ident {
          reference,
          mutability,
          name: self.get_token_lexeme(&token),
          subpattern: subpattern.map(Box::new),
          span: token.span,
        })
      },
      _ => {
        self.advance(engine);
        Ok(Pattern::Wildcard { span: token.span })
      },
    }
  }
}
