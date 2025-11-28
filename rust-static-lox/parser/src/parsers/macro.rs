use crate::{
  ast::{path::Path, Delimiter, Expr, MacroInvocation, RepeatKind, TokenTree, Type},
  match_and_consume,
  parser_utils::ExprContext,
  Parser,
};

use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine,
};
use lexer::token::TokenKind;

impl Parser {
  // TODO: implement full macro expansion
  pub(crate) fn parse_macro_invocation(
    &mut self,
    path: Path,
    qself: Option<Box<Type>>,
    engine: &mut DiagnosticEngine,
  ) -> Result<MacroInvocation, ()> {
    let mut token = self.current_token();

    match token.kind {
      TokenKind::OpenParen | TokenKind::OpenBracket | TokenKind::OpenBrace => {
        self.advance(engine); // consume the '('
        let tokens = self.parse_macro_tokens(engine)?;

        let delimiter = match token.kind {
          TokenKind::OpenParen => {
            self.expect(TokenKind::CloseParen, engine)?;
            Delimiter::Paren
          }
          TokenKind::OpenBracket => {
            self.expect(TokenKind::CloseBracket, engine)?;
            Delimiter::Bracket
          }
          TokenKind::OpenBrace => {
            self.expect(TokenKind::CloseBrace, engine)?;
            Delimiter::Brace
          }
          _ => unreachable!(),
        };

        token.span.merge(self.current_token().span);
        Ok(MacroInvocation {
          qself,
          path,
          delimiter,
          tokens,
          span: token.span,
        })
      }

      _ => {
        let lexeme = self.get_token_lexeme(&token);
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          format!("Unexpected token `{lexeme}` in macro invocation"),
          self.source_file.path.clone(),
        )
        .with_label(
          token.span,
          Some("Expected a macro invocation, found a primary expression".to_string()),
          LabelStyle::Primary,
        )
        .with_help("Macro invocations must be surrounded by parentheses or braces.".to_string());
        engine.add(diagnostic);
        Err(())
      }
    }
  }

  fn parse_macro_tokens(&mut self, engine: &mut DiagnosticEngine) -> Result<Vec<TokenTree>, ()> {
    let mut tokens = vec![];

    while !self.is_eof()
      && !matches!(
        self.current_token().kind,
        TokenKind::CloseParen | TokenKind::CloseBracket | TokenKind::CloseBrace
      )
    {
      tokens.push(self.parse_token_tree(engine)?);
      match_and_consume!(self, engine, TokenKind::Comma)?;
    }

    Ok(tokens)
  }

  fn parse_token_tree(&mut self, engine: &mut DiagnosticEngine) -> Result<TokenTree, ()> {
    let mut token = self.current_token();
    match token.kind {
      TokenKind::Ident | TokenKind::Literal { .. } | TokenKind::KwTrue | TokenKind::KwFalse => {
        self.advance(engine);
        Ok(TokenTree::Token(self.get_token_lexeme(&token)))
      }

      // FIX: this swhen you get to the macro full parsing
      TokenKind::OpenParen | TokenKind::OpenBracket | TokenKind::OpenBrace => {
        let tokens = self.parse_macro_tokens(engine)?;
        self.expect(TokenKind::CloseParen, engine)?;
        Ok(TokenTree::Delimited {
          delimiter: match token.kind {
            TokenKind::OpenParen => Delimiter::Paren,
            TokenKind::OpenBracket => Delimiter::Bracket,
            TokenKind::OpenBrace => Delimiter::Brace,
            _ => unreachable!(),
          },
          tokens,
        })
      }

      // FIX: this swhen you get to the macro full parsing
      TokenKind::DotDot => {
        self.advance(engine);

        let kind = match self.current_token().kind {
          TokenKind::DotDot => RepeatKind::ZeroOrMore,
          TokenKind::DotDotEq => RepeatKind::OneOrMore,
          TokenKind::Eq => RepeatKind::ZeroOrOne,
          _ => unreachable!(),
        };

        Ok(TokenTree::Repeat {
          tokens: vec![],
          separator: None,
          kind,
        })
      }

      // FIX: this swhen you get to the macro full parsing
      TokenKind::OpenBrace => {
        self.expect(TokenKind::CloseBrace, engine)?;
        Ok(TokenTree::MetaVar {
          name: self.get_token_lexeme(&token),
          kind: self.get_token_lexeme(&token),
        })
      }

      _ => {
        let lexeme = self.get_token_lexeme(&token);
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          format!("Unexpected token `{lexeme}` in macro invocation"),
          self.source_file.path.clone(),
        )
        .with_label(
          token.span,
          Some("Expected a macro invocation, found a primary expression".to_string()),
          LabelStyle::Primary,
        )
        .with_help("Macro invocations must be surrounded by parentheses or braces.".to_string());
        engine.add(diagnostic);
        Err(())
      }
    }
  }
}
