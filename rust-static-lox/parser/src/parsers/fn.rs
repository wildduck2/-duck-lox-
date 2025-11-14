use crate::{ast::*, parser_utils::ExprContext, DiagnosticEngine, Parser};
use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
};
use lexer::token::{Token, TokenKind};

impl Parser {
  pub(crate) fn parse_fn_decl(
    &mut self,
    attributes: Vec<Attribute>,
    visibility: Visibility,
    engine: &mut DiagnosticEngine,
  ) -> Result<Item, ()> {
    Err(())
  }
}
