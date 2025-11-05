use crate::{token::TokenKind, Lexer};

impl Lexer {
  pub fn lex_keywords(&mut self) -> Option<TokenKind> {
    let next_char = match self.peek() {
      Some(ch) => ch.to_string(),
      None => "".to_string(),
    };

    // Consume valid identifier characters
    while let Some(ch) = self.peek() {
      if !ch.is_ascii_alphanumeric() && ch != '_' {
        break;
      }
      self.advance();
    }

    match self.get_current_lexeme() {
      // Control Flow Keywords
      "if" => Some(TokenKind::KwIf),
      "else" => Some(TokenKind::KwElse),
      "match" => Some(TokenKind::KwMatch),
      "loop" => Some(TokenKind::KwLoop),
      "while" => Some(TokenKind::KwWhile),
      "for" => Some(TokenKind::KwFor),
      "break" => Some(TokenKind::KwBreak),
      "continue" => Some(TokenKind::KwContinue),
      "return" => Some(TokenKind::KwReturn),

      // Declaration Keywords
      "let" => Some(TokenKind::KwLet),
      "fn" => Some(TokenKind::KwFn),
      "struct" => Some(TokenKind::KwStruct),
      "enum" => Some(TokenKind::KwEnum),
      "union" => Some(TokenKind::KwUnion),
      "trait" => Some(TokenKind::KwTrait),
      "impl" => Some(TokenKind::KwImpl),
      "type" => Some(TokenKind::KwType),
      "mod" => Some(TokenKind::KwMod),
      "use" => Some(TokenKind::KwUse),
      "const" => Some(TokenKind::KwConst),
      "static" => Some(TokenKind::KwStatic),
      "extern" => Some(TokenKind::KwExtern),
      "macro" => Some(TokenKind::KwMacro),
      "auto" => Some(TokenKind::KwAuto),
      "default" => Some(TokenKind::KwDefault),

      // Modifier Keywords
      "pub" => Some(TokenKind::KwPub),
      "mut" => Some(TokenKind::KwMut),
      "ref" => Some(TokenKind::KwRef),
      "move" => Some(TokenKind::KwMove),
      "unsafe" => Some(TokenKind::KwUnsafe),
      "async" => Some(TokenKind::KwAsync),
      "await" => Some(TokenKind::KwAwait),
      "dyn" => Some(TokenKind::KwDyn),

      // Special Identifiers
      "self" => Some(TokenKind::KwSelfValue),
      "Self" => Some(TokenKind::KwSelfType),
      "super" => Some(TokenKind::KwSuper),
      "crate" => Some(TokenKind::KwCrate),

      // Literal Keywords
      "true" => Some(TokenKind::KwTrue),
      "false" => Some(TokenKind::KwFalse),

      // Other Keywords
      "as" => Some(TokenKind::KwAs),
      "in" => Some(TokenKind::KwIn),
      "where" => Some(TokenKind::KwWhere),

      // Reserved Keywords (not yet used, but reserved for future use)
      "abstract" => Some(TokenKind::KwAbstract),
      "become" => Some(TokenKind::KwBecome),
      "box" => Some(TokenKind::KwBox),
      "do" => Some(TokenKind::KwDo),
      "final" => Some(TokenKind::KwFinal),
      "override" => Some(TokenKind::KwOverride),
      "priv" => Some(TokenKind::KwPriv),
      "try" => Some(TokenKind::KwTry),
      "typeof" => Some(TokenKind::KwTypeof),
      "unsized" => Some(TokenKind::KwUnsized),
      "virtual" => Some(TokenKind::KwVirtual),
      "yield" => Some(TokenKind::KwYield),

      _ => None,
    }
  }
}
