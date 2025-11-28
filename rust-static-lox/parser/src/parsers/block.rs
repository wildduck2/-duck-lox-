use crate::{ast::Expr, match_and_consume, parser_utils::ExprContext, Parser};
use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine,
};
use lexer::token::TokenKind;

impl Parser {
  pub(crate) fn parse_block_expression(
    &mut self,
    label: Option<String>,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let mut token = self.current_token();
    let (is_unsafe, is_async, is_try) = self.parse_block_flavors(ExprContext::Default, engine)?;
    self.advance(engine); // consume the "{"

    let mut stmts = vec![];
    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::CloseBrace) {
      stmts.push(self.parse_stmt(ExprContext::Default, engine)?);
      match_and_consume!(self, engine, TokenKind::Semi)?;
    }
    self.expect(TokenKind::CloseBrace, engine)?;

    token.span.merge(self.current_token().span);
    Ok(Expr::Block {
      stmts,
      label,
      is_unsafe,
      is_async,
      is_try,
      span: token.span,
    })
  }

  pub(crate) fn parse_block_flavors(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<(bool, bool, bool), ()> {
    // A block flavor can only appear in normal expression positions
    if !matches!(context, ExprContext::Default) {
      let token = self.current_token();

      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::InvalidBlockFlavorContext),
        "invalid block flavor here".to_string(),
        self.source_file.path.clone(),
      )
      .with_label(
        token.span,
        Some("this token cannot start a block flavor in this position".to_string()),
        LabelStyle::Primary,
      )
      .with_help(
        "block flavors (unsafe, async, try) are only allowed before a block expression. \
             Example: unsafe { ... } or async { ... }"
          .to_string(),
      );

      engine.add(diagnostic);
      return Err(());
    }

    let token_before = self.current_token();

    let is_unsafe = if matches!(self.current_token().kind, TokenKind::KwUnsafe) {
      self.advance(engine);
      true
    } else {
      false
    };

    let is_async = if matches!(self.current_token().kind, TokenKind::KwAsync) {
      self.advance(engine);
      true
    } else {
      false
    };

    let is_try = if matches!(self.current_token().kind, TokenKind::KwTry) {
      self.advance(engine);
      true
    } else {
      false
    };

    // Additional error: if user wrote a flavor but forgot to follow with a block
    if (is_unsafe || is_async || is_try)
      && !matches!(self.current_token().kind, TokenKind::OpenBrace)
    {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::ExpectedBlockAfterFlavor),
        "expected a block after flavor keyword".to_string(),
        self.source_file.path.clone(),
      )
      .with_label(
        token_before.span,
        Some("a block must follow here".to_string()),
        LabelStyle::Primary,
      )
      .with_label(
        self.current_token().span,
        Some("this token is not a valid block start".to_string()),
        LabelStyle::Secondary,
      )
      .with_help("write it as for example: unsafe { ... } or async { ... }".to_string());

      engine.add(diagnostic);
      return Err(());
    }

    Ok((is_unsafe, is_async, is_try))
  }
}
