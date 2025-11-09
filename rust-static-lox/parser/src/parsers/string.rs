use diagnostic::DiagnosticEngine;
use lexer::token::Token;

use crate::{
  ast::{Expr, StrKind},
  Parser,
};

impl Parser {
  pub(crate) fn parser_string(
    &mut self,
    _engine: &mut DiagnosticEngine,
    token: &Token,
  ) -> Result<Expr, ()> {
    let value = self
      .source_file
      .src
      .get(token.span.start..token.span.end)
      .unwrap();

    Ok(Expr::String {
      kind: StrKind::Normal,
      value: value.to_string(),
      span: token.span,
    })
  }

  pub(crate) fn parser_byte_string(
    &mut self,
    _engine: &mut DiagnosticEngine,
    token: &Token,
  ) -> Result<Expr, ()> {
    let value = self
      .source_file
      .src
      .get(token.span.start..token.span.end)
      .unwrap();

    Ok(Expr::String {
      kind: StrKind::Byte,
      value: value.to_string(),
      span: token.span,
    })
  }

  pub(crate) fn parser_c_string(
    &mut self,
    _engine: &mut DiagnosticEngine,
    token: &Token,
  ) -> Result<Expr, ()> {
    let value = self
      .source_file
      .src
      .get(token.span.start..token.span.end)
      .unwrap();

    Ok(Expr::String {
      kind: StrKind::C,
      value: value.to_string(),
      span: token.span,
    })
  }

  pub(crate) fn parser_raw_string(
    &mut self,
    _engine: &mut DiagnosticEngine,
    token: &Token,
    n_hashes: u16,
  ) -> Result<Expr, ()> {
    let value = self
      .source_file
      .src
      .get(token.span.start..token.span.end)
      .unwrap();

    Ok(Expr::String {
      kind: StrKind::Raw(n_hashes.into()),
      value: value.to_string(),
      span: token.span,
    })
  }

  pub(crate) fn parser_raw_byte_string(
    &mut self,
    _engine: &mut DiagnosticEngine,
    token: &Token,
    n_hashes: u16,
  ) -> Result<Expr, ()> {
    let value = self
      .source_file
      .src
      .get(token.span.start..token.span.end)
      .unwrap();

    Ok(Expr::String {
      kind: StrKind::RawByte(n_hashes.into()),
      value: value.to_string(),
      span: token.span,
    })
  }

  pub(crate) fn parser_raw_c_string(
    &mut self,
    _engine: &mut DiagnosticEngine,
    token: &Token,
    n_hashes: u16,
  ) -> Result<Expr, ()> {
    let value = self
      .source_file
      .src
      .get(token.span.start..token.span.end)
      .unwrap();

    Ok(Expr::String {
      kind: StrKind::RawC(n_hashes.into()),
      value: value.to_string(),
      span: token.span,
    })
  }
}
