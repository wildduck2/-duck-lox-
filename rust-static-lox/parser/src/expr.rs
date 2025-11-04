// ==================== EXPRESSIONS ====================

use diagnostic::{SourceFile, SourceMap, Span};

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Expr {
  // Literals
  Number {
    span: Span,
  },
  String {
    span: Span,
  },
  Template {
    parts: Vec<String>,
    expressions: Vec<Box<Expr>>,
    span: Span,
  },
  Bool {
    span: Span,
  },
  Null {
    span: Span,
  },
  Undefined {
    span: Span,
  },

  // Identifiers and special
  Identifier {
    span: Span,
  },
  This {
    span: Span,
  },
  Super {
    span: Span,
  },

  // Operations
  Binary {
    left: Box<Expr>,
    op: BinaryOp,
    right: Box<Expr>,
    span: Span,
  },
  Unary {
    op: UnaryOp,
    expr: Box<Expr>,
    span: Span,
  },
  Postfix {
    expr: Box<Expr>,
    op: PostfixOp,
    span: Span,
  },

  // Access
  Call {
    callee: Box<Expr>,
    args: Vec<Argument>,
    span: Span,
  },
  Member {
    object: Box<Expr>,
    property: String,
    optional: bool, // for ?.
    span: Span,
  },
  Index {
    object: Box<Expr>,
    index: Box<Expr>,
    span: Span,
  },

  // Assignment
  Assign {
    target: Box<Expr>,
    op: AssignOp,
    value: Box<Expr>,
    span: Span,
  },

  // Literals
  Array {
    elements: Vec<ArrayElement>,
    span: Span,
  },
  Object {
    properties: Vec<ObjectProperty>,
    span: Span,
  },

  // Functions
  Arrow {
    params: Vec<Param>,
    return_type: Option<Type>,
    body: ArrowBody,
    is_async: bool,
    span: Span,
  },

  // Special expressions
  Ternary {
    condition: Box<Expr>,
    then_expr: Box<Expr>,
    else_expr: Box<Expr>,
    span: Span,
  },
  Sequence {
    expressions: Vec<Expr>,
    span: Span,
  },
  New {
    constructor: Box<Expr>,
    args: Vec<Argument>,
    span: Span,
  },
  TypeAssertion {
    expr: Box<Expr>,
    type_annotation: Type,
    span: Span,
  },
  NonNull {
    expr: Box<Expr>,
    span: Span,
  },
  Grouping {
    expr: Box<Expr>,
    span: Span,
  },
}

#[derive(Debug, Clone)]
pub enum ArrowBody {
  Expression(Box<Expr>),
  Block(Vec<Stmt>),
}

#[derive(Debug, Clone)]
pub struct Argument {
  pub spread: bool,
  pub expr: Expr,
}

#[derive(Debug, Clone)]
pub enum ArrayElement {
  Expression(Expr),
  Spread(Expr),
}

#[derive(Debug, Clone)]
pub enum ObjectProperty {
  Property {
    key: PropertyKey,
    value: Expr,
  },
  Shorthand {
    name: String,
  },
  Spread {
    expr: Expr,
  },
  Method {
    key: PropertyKey,
    params: Vec<Param>,
    body: Vec<Stmt>,
    is_async: bool,
  },
}

#[derive(Debug, Clone)]
pub enum PropertyKey {
  Identifier(String),
  String(String),
  Number(f64),
  Computed(Box<Expr>),
}

// ==================== OPERATORS ====================

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum BinaryOp {
  // Arithmetic
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Exp,

  // Bitwise
  BitAnd,
  BitOr,
  BitXor,
  LeftShift,
  RightShift,
  UnsignedRightShift,

  // Comparison
  Eq,
  NotEq,
  StrictEq,
  StrictNotEq,
  Less,
  LessEq,
  Greater,
  GreaterEq,

  // Logical
  And,
  Or,
  NullishCoalescing,

  // Special
  In,
  InstanceOf,
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum UnaryOp {
  Not,
  BitwiseNot,
  Neg,
  Plus,
  TypeOf,
  Void,
  Delete,
  Await,
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum PostfixOp {
  Increment, // ++
  Decrement, // --
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum AssignOp {
  Assign,        // =
  AddAssign,     // +=
  SubAssign,     // -=
  MulAssign,     // *=
  DivAssign,     // /=
  ModAssign,     // %=
  AndAssign,     // &&=
  OrAssign,      // ||=
  NullishAssign, // ??=
}

// ==================== STATEMENTS ====================

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Stmt {
  Expr(Expr),

  VarDecl {
    kind: VarKind,
    name: String,
    type_annotation: Option<Type>,
    initializer: Option<Expr>,
    span: Span,
  },

  FunctionDecl {
    name: String,
    type_params: Vec<TypeParam>,
    params: Vec<Param>,
    return_type: Option<Type>,
    body: Vec<Stmt>,
    is_async: bool,
    span: Span,
  },

  ClassDecl {
    name: String,
    type_params: Vec<TypeParam>,
    extends: Option<Type>,
    implements: Vec<Type>,
    members: Vec<ClassMember>,
    is_abstract: bool,
    span: Span,
  },

  InterfaceDecl {
    name: String,
    type_params: Vec<TypeParam>,
    extends: Vec<Type>,
    members: Vec<InterfaceMember>,
    span: Span,
  },

  TypeAliasDecl {
    name: String,
    type_params: Vec<TypeParam>,
    type_annotation: Type,
    span: Span,
  },

  EnumDecl {
    name: String,
    members: Vec<EnumMember>,
    span: Span,
  },

  NamespaceDecl {
    name: String,
    body: Vec<Stmt>,
    span: Span,
  },

  If {
    condition: Box<Expr>,
    then_branch: Box<Stmt>,
    else_branch: Option<Box<Stmt>>,
    span: Span,
  },

  While {
    condition: Box<Expr>,
    body: Box<Stmt>,
    span: Span,
  },

  DoWhile {
    body: Box<Stmt>,
    condition: Box<Expr>,
    span: Span,
  },

  For {
    init: Option<Box<Stmt>>,
    condition: Option<Box<Expr>>,
    update: Option<Box<Expr>>,
    body: Box<Stmt>,
    span: Span,
  },

  ForIn {
    kind: VarKind,
    variable: String,
    iterable: Box<Expr>,
    body: Box<Stmt>,
    span: Span,
  },

  ForOf {
    kind: VarKind,
    variable: String,
    iterable: Box<Expr>,
    body: Box<Stmt>,
    span: Span,
  },

  Switch {
    discriminant: Box<Expr>,
    cases: Vec<SwitchCase>,
    span: Span,
  },

  Try {
    block: Vec<Stmt>,
    catch_clause: Option<CatchClause>,
    finally_block: Option<Vec<Stmt>>,
    span: Span,
  },

  Throw {
    expr: Box<Expr>,
    span: Span,
  },

  Return {
    value: Option<Box<Expr>>,
    span: Span,
  },

  Break {
    span: Span,
  },

  Continue {
    span: Span,
  },

  Block(Vec<Stmt>),

  Export {
    declaration: Box<Stmt>,
    span: Span,
  },

  Import {
    specifiers: ImportSpecifier,
    source: String,
    span: Span,
  },
}

// ==================== DECLARATIONS ====================

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VarKind {
  Var,
  Let,
  Const,
}

#[derive(Debug, Clone)]
pub struct Param {
  pub name: String,
  pub type_annotation: Option<Type>,
  pub optional: bool,
  pub rest: bool, // for ...args
  pub default_value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct ClassMember {
  pub visibility: Visibility,
  pub is_static: bool,
  pub is_readonly: bool,
  pub kind: ClassMemberKind,
}

#[derive(Debug, Clone)]
pub enum ClassMemberKind {
  Property {
    name: String,
    type_annotation: Option<Type>,
    initializer: Option<Expr>,
    optional: bool,
  },
  Method {
    name: String,
    type_params: Vec<TypeParam>,
    params: Vec<Param>,
    return_type: Option<Type>,
    body: Vec<Stmt>,
    is_async: bool,
  },
  Constructor {
    params: Vec<Param>,
    body: Vec<Stmt>,
  },
}

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Visibility {
  Public,
  Private,
  Protected,
}

#[derive(Debug, Clone)]
pub struct InterfaceMember {
  pub name: String,
  pub kind: InterfaceMemberKind,
}

#[derive(Debug, Clone)]
pub enum InterfaceMemberKind {
  Property {
    type_annotation: Type,
    optional: bool,
  },
  Method {
    type_params: Vec<TypeParam>,
    params: Vec<Param>,
    return_type: Type,
  },
  IndexSignature {
    key_name: String,
    key_type: Type,
    value_type: Type,
  },
}

#[derive(Debug, Clone)]
pub struct EnumMember {
  pub name: String,
  pub value: Option<EnumValue>,
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  Number(f64),
  String(String),
}

#[derive(Debug, Clone)]
pub struct SwitchCase {
  pub test: Option<Expr>, // None for default case
  pub consequent: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct CatchClause {
  pub param: Option<String>,
  pub type_annotation: Option<Type>,
  pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum ImportSpecifier {
  Default(String),
  Named(Vec<String>),
  Namespace(String),
}

// ==================== TYPES ====================

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Type {
  // Primitives
  Number,
  String,
  Boolean,
  Void,
  Any,
  Unknown,
  Never,
  Null,
  Undefined,
  Symbol,
  BigInt,

  // Literal types
  NumberLiteral(f64),
  StringLiteral(String),
  BooleanLiteral(bool),

  // Composite
  Array(Box<Type>),
  Tuple(Vec<Type>),

  Function {
    type_params: Vec<TypeParam>,
    params: Vec<FunctionParam>,
    return_type: Box<Type>,
  },

  Object {
    members: Vec<TypeMember>,
  },

  // Reference
  Reference {
    name: String,
    type_args: Vec<Type>,
  },

  // Operators
  Union(Vec<Type>),
  Intersection(Vec<Type>),
  Optional(Box<Type>),

  // Advanced
  TypeQuery(String), // typeof x
  Conditional {
    check_type: Box<Type>,
    extends_type: Box<Type>,
    true_type: Box<Type>,
    false_type: Box<Type>,
  },

  IndexedAccess {
    object_type: Box<Type>,
    index_type: Box<Type>,
  },
}

#[derive(Debug, Clone)]
pub struct TypeParam {
  pub name: String,
  pub constraint: Option<Type>, // extends Type
  pub default: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct FunctionParam {
  pub name: String,
  pub type_annotation: Type,
  pub optional: bool,
}

#[derive(Debug, Clone)]
pub struct TypeMember {
  pub name: String,
  pub type_annotation: Type,
  pub optional: bool,
}

//NOTE: Helper macro for tree printing
macro_rules! print_node {
  ($prefix:expr, $connector:expr, $label:expr) => {
    println!("{}{}{}", $prefix, $connector, $label)
  };
  ($prefix:expr, $connector:expr, $label:expr, $value:expr) => {
    println!("{}{}{}({})", $prefix, $connector, $label, $value)
  };
}

impl Expr {
  pub fn build_tree(&self, prefix: &str, is_last: bool, source_file: &SourceFile) {
    let (connector, extension) = if is_last {
      ("└── ", "    ")
    } else {
      ("├── ", "│   ")
    };
    let new_prefix = format!("{}{}", prefix, extension);

    match self {
      // ========== LITERALS ==========
      Expr::Number { span } => {
        let lexeme = source_file.src.get(span.start..span.end).unwrap();

        print_node!(prefix, connector, "Number", lexeme);
      },

      Expr::String { span } => {
        let lexeme = source_file.src.get(span.start..span.end).unwrap();
        print_node!(prefix, connector, "String", lexeme);
      },

      Expr::Template {
        parts, expressions, ..
      } => {
        print_node!(prefix, connector, "Template");
        println!("{}├── parts: {:?}", new_prefix, parts);
        if !expressions.is_empty() {
          println!("{}└── expressions:", new_prefix);
          let expr_prefix = format!("{}    ", new_prefix);
          for (i, expr) in expressions.iter().enumerate() {
            expr.build_tree(&expr_prefix, i == expressions.len() - 1, source_file);
          }
        }
      },

      Expr::Bool { span, .. } => {
        let lexeme = source_file.src.get(span.start..span.end).unwrap();

        print_node!(prefix, connector, "Bool", lexeme);
      },

      Expr::Null { .. } => {
        print_node!(prefix, connector, "Null");
      },

      Expr::Undefined { .. } => {
        print_node!(prefix, connector, "Undefined");
      },

      // ========== IDENTIFIERS & SPECIAL ==========
      Expr::Identifier { span } => {
        let lexeme = source_file.src.get(span.start..span.end).unwrap();

        print_node!(prefix, connector, "Identifier", lexeme);
      },

      Expr::This { .. } => {
        print_node!(prefix, connector, "This");
      },

      Expr::Super { .. } => {
        print_node!(prefix, connector, "Super");
      },

      // ========== OPERATIONS ==========
      Expr::Binary {
        left, op, right, ..
      } => {
        print_node!(prefix, connector, "Binary", format!("{:?}", op));
        left.build_tree(&new_prefix, false, source_file);
        right.build_tree(&new_prefix, true, source_file);
      },

      Expr::Unary { op, expr, .. } => {
        print_node!(prefix, connector, "Unary", format!("{:?}", op));
        expr.build_tree(&new_prefix, true, source_file);
      },

      Expr::Postfix { expr, op, .. } => {
        print_node!(prefix, connector, "Postfix", format!("{:?}", op));
        expr.build_tree(&new_prefix, true, source_file);
      },

      // ========== ACCESS ==========
      Expr::Call { callee, args, .. } => {
        if args.is_empty() {
          print_node!(prefix, connector, "Call()");
          callee.build_tree(&new_prefix, true, source_file);
        } else {
          print_node!(prefix, connector, "Call");
          callee.build_tree(&new_prefix, false, source_file);
          println!("{}└── args:", new_prefix);
          let args_prefix = format!("{}    ", new_prefix);
          for (i, arg) in args.iter().enumerate() {
            let label = if arg.spread { "...spread" } else { "arg" };
            println!(
              "{}{}{}",
              args_prefix,
              if i == args.len() - 1 {
                "└── "
              } else {
                "├── "
              },
              label
            );
            let arg_prefix = format!(
              "{}{}",
              args_prefix,
              if i == args.len() - 1 {
                "    "
              } else {
                "│   "
              }
            );
            arg.expr.build_tree(&arg_prefix, true, source_file);
          }
        }
      },

      Expr::Member {
        object,
        property,
        optional,
        ..
      } => {
        let op = if *optional { "?." } else { "." };
        print_node!(prefix, connector, "Member", format!("{}{}", op, property));
        object.build_tree(&new_prefix, true, source_file);
      },

      Expr::Index { object, index, .. } => {
        print_node!(prefix, connector, "Index");
        object.build_tree(&new_prefix, false, source_file);
        index.build_tree(&new_prefix, true, source_file);
      },

      // ========== ASSIGNMENT ==========
      Expr::Assign {
        target, op, value, ..
      } => {
        print_node!(prefix, connector, "Assign", format!("{:?}", op));
        target.build_tree(&new_prefix, false, source_file);
        value.build_tree(&new_prefix, true, source_file);
      },

      // ========== LITERALS (COMPOSITE) ==========
      Expr::Array { elements, .. } => {
        print_node!(prefix, connector, "Array");
        for (i, elem) in elements.iter().enumerate() {
          match elem {
            ArrayElement::Expression(expr) => {
              expr.build_tree(&new_prefix, i == elements.len() - 1, source_file);
            },
            ArrayElement::Spread(expr) => {
              println!(
                "{}{}...spread:",
                new_prefix,
                if i == elements.len() - 1 {
                  "└── "
                } else {
                  "├── "
                }
              );
              let spread_prefix = format!(
                "{}{}",
                new_prefix,
                if i == elements.len() - 1 {
                  "    "
                } else {
                  "│   "
                }
              );
              expr.build_tree(&spread_prefix, true, source_file);
            },
          }
        }
      },

      Expr::Object { properties, .. } => {
        print_node!(prefix, connector, "Object");
        for (i, prop) in properties.iter().enumerate() {
          let is_last = i == properties.len() - 1;
          let prop_connector = if is_last { "└── " } else { "├── " };
          let prop_prefix = format!("{}{}", new_prefix, if is_last { "    " } else { "│   " });

          match prop {
            ObjectProperty::Property { key, value } => {
              let key_str = match key {
                PropertyKey::Identifier(s) => s.clone(),
                PropertyKey::String(s) => format!("\"{}\"", s),
                PropertyKey::Computed(_) => "[computed]".to_string(),
                PropertyKey::Number(n) => format!("{}", n),
              };
              println!("{}{}{}: ", new_prefix, prop_connector, key_str);
              value.build_tree(&prop_prefix, true, source_file);
            },
            ObjectProperty::Shorthand { name } => {
              println!("{}{}{} (shorthand)", new_prefix, prop_connector, name);
            },
            ObjectProperty::Spread { expr } => {
              println!("{}{}...spread:", new_prefix, prop_connector);
              expr.build_tree(&prop_prefix, true, source_file);
            },
            ObjectProperty::Method {
              key,
              params,
              body,
              is_async,
            } => {
              let key_str = match key {
                PropertyKey::Identifier(s) => s.clone(),
                PropertyKey::String(s) => format!("\"{}\"", s),
                PropertyKey::Computed(_) => "[computed]".to_string(),
                PropertyKey::Number(n) => format!("{}", n),
              };
              let async_str = if *is_async { "async " } else { "" };
              println!(
                "{}{}{}method {}({} params, {} stmts)",
                new_prefix,
                prop_connector,
                async_str,
                key_str,
                params.len(),
                body.len()
              );
            },
          }
        }
      },

      // ========== FUNCTIONS ==========
      Expr::Arrow {
        params,
        return_type,
        body,
        is_async,
        ..
      } => {
        let async_str = if *is_async { "async " } else { "" };
        let return_str = if let Some(rt) = return_type {
          format!(": {:?}", rt)
        } else {
          String::new()
        };
        print_node!(
          prefix,
          connector,
          format!("{}Arrow{}", async_str, return_str)
        );

        // Print params
        if !params.is_empty() {
          let has_body = match body {
            ArrowBody::Expression(_) => true,
            ArrowBody::Block(stmts) => !stmts.is_empty(),
          };

          println!(
            "{}{}params:",
            new_prefix,
            if has_body { "├── " } else { "└── " }
          );
          let param_prefix = format!("{}{}", new_prefix, if has_body { "│   " } else { "    " });

          for (i, param) in params.iter().enumerate() {
            let mut param_str = param.name.clone();
            if param.rest {
              param_str = format!("...{}", param_str);
            }
            if param.optional {
              param_str = format!("{}?", param_str);
            }
            if let Some(ref t) = param.type_annotation {
              param_str = format!("{}: {:?}", param_str, t);
            }
            if let Some(ref default) = param.default_value {
              param_str = format!("{} = {:?}", param_str, default);
            }
            println!(
              "{}{}{}",
              param_prefix,
              if i == params.len() - 1 {
                "└── "
              } else {
                "├── "
              },
              param_str
            );
          }
        }

        // Print body
        match body {
          ArrowBody::Expression(expr) => {
            println!("{}└── expr:", new_prefix);
            let body_prefix = format!("{}    ", new_prefix);
            expr.build_tree(&body_prefix, true, source_file);
          },
          ArrowBody::Block(stmts) => {
            if !stmts.is_empty() {
              println!("{}└── body:", new_prefix);
              let body_prefix = format!("{}    ", new_prefix);
              for (i, stmt) in stmts.iter().enumerate() {
                stmt.build_tree(&body_prefix, i == stmts.len() - 1, source_file);
              }
            }
          },
        }
      },

      // ========== SPECIAL EXPRESSIONS ==========
      Expr::Ternary {
        condition,
        then_expr,
        else_expr,
        ..
      } => {
        print_node!(prefix, connector, "Ternary");
        println!("{}├── condition:", new_prefix);
        condition.build_tree(&format!("{}│   ", new_prefix), true, source_file);
        println!("{}├── then:", new_prefix);
        then_expr.build_tree(&format!("{}│   ", new_prefix), true, source_file);
        println!("{}└── else:", new_prefix);
        else_expr.build_tree(&format!("{}    ", new_prefix), true, source_file);
      },

      Expr::Sequence { expressions, .. } => {
        print_node!(prefix, connector, "Sequence");
        for (i, expr) in expressions.iter().enumerate() {
          expr.build_tree(&new_prefix, i == expressions.len() - 1, source_file);
        }
      },

      Expr::New {
        constructor, args, ..
      } => {
        print_node!(prefix, connector, "New");
        constructor.build_tree(&new_prefix, args.is_empty(), source_file);
        if !args.is_empty() {
          println!("{}└── args:", new_prefix);
          let args_prefix = format!("{}    ", new_prefix);
          for (i, arg) in args.iter().enumerate() {
            arg
              .expr
              .build_tree(&args_prefix, i == args.len() - 1, source_file);
          }
        }
      },

      Expr::TypeAssertion {
        expr,
        type_annotation,
        ..
      } => {
        print_node!(
          prefix,
          connector,
          "TypeAssertion",
          format!("as {:?}", type_annotation)
        );
        expr.build_tree(&new_prefix, true, source_file);
      },

      Expr::NonNull { expr, .. } => {
        print_node!(prefix, connector, "NonNull", "!");
        expr.build_tree(&new_prefix, true, source_file);
      },

      Expr::Grouping { expr, .. } => {
        print_node!(prefix, connector, "Grouping");
        expr.build_tree(&new_prefix, true, source_file);
      },
    }
  }
}

impl Stmt {
  pub fn print_tree(&self, source_file: &SourceFile) {
    self.build_tree("", true, source_file);
  }

  pub fn build_tree(&self, prefix: &str, is_last: bool, source_file: &SourceFile) {
    let (connector, extension) = if is_last {
      ("└── ", "    ")
    } else {
      ("├── ", "│   ")
    };
    let new_prefix = format!("{}{}", prefix, extension);

    match self {
      Stmt::Expr(expr) => {
        print_node!(prefix, connector, "ExprStmt");
        expr.build_tree(&new_prefix, true, source_file);
      },

      Stmt::VarDecl {
        kind,
        name,
        type_annotation,
        initializer,
        ..
      } => {
        let type_str = if let Some(t) = type_annotation {
          format!(": {:?}", t)
        } else {
          String::new()
        };
        print_node!(
          prefix,
          connector,
          format!("{:?} {}{}", kind, name, type_str)
        );
        if let Some(init) = initializer {
          init.build_tree(&new_prefix, true, source_file);
        }
      },

      Stmt::FunctionDecl {
        name,
        type_params,
        params,
        return_type,
        body,
        is_async,
        ..
      } => {
        let async_str = if *is_async { "async " } else { "" };
        let generics = if !type_params.is_empty() {
          format!("<{}>", type_params.len())
        } else {
          String::new()
        };
        let return_str = if let Some(rt) = return_type {
          format!(": {:?}", rt)
        } else {
          String::new()
        };
        print_node!(
          prefix,
          connector,
          format!(
            "{}function {}{}({} params){}",
            async_str,
            name,
            generics,
            params.len(),
            return_str
          )
        );

        if !body.is_empty() {
          println!("{}└── body:", new_prefix);
          let body_prefix = format!("{}    ", new_prefix);
          for (i, stmt) in body.iter().enumerate() {
            stmt.build_tree(&body_prefix, i == body.len() - 1, source_file);
          }
        }
      },

      Stmt::ClassDecl {
        name,
        type_params,
        extends,
        implements,
        members,
        is_abstract,
        ..
      } => {
        let abstract_str = if *is_abstract { "abstract " } else { "" };
        let generics = if !type_params.is_empty() {
          format!("<{}>", type_params.len())
        } else {
          String::new()
        };
        let extends_str = if let Some(e) = extends {
          format!(" extends {:?}", e)
        } else {
          String::new()
        };
        let implements_str = if !implements.is_empty() {
          format!(" implements {} types", implements.len())
        } else {
          String::new()
        };
        print_node!(
          prefix,
          connector,
          format!(
            "{}class {}{}{}{}",
            abstract_str, name, generics, extends_str, implements_str
          )
        );

        for (i, member) in members.iter().enumerate() {
          let is_last_member = i == members.len() - 1;
          let vis = match member.visibility {
            Visibility::Public => "public",
            Visibility::Private => "private",
            Visibility::Protected => "protected",
          };
          let static_str = if member.is_static { " static" } else { "" };
          let readonly_str = if member.is_readonly { " readonly" } else { "" };

          match &member.kind {
            ClassMemberKind::Property {
              name,
              type_annotation,
              optional,
              ..
            } => {
              let opt = if *optional { "?" } else { "" };
              let type_str = if let Some(t) = type_annotation {
                format!(": {:?}", t)
              } else {
                String::new()
              };
              println!(
                "{}{}{}{}{} {}{}{}",
                new_prefix,
                if is_last_member {
                  "└── "
                } else {
                  "├── "
                },
                vis,
                static_str,
                readonly_str,
                name,
                opt,
                type_str
              );
            },
            ClassMemberKind::Method {
              name,
              params,
              return_type,
              is_async,
              ..
            } => {
              let async_str = if *is_async { "async " } else { "" };
              let return_str = if let Some(rt) = return_type {
                format!(": {:?}", rt)
              } else {
                String::new()
              };
              println!(
                "{}{}{}{}{} {}{}({} params){}",
                new_prefix,
                if is_last_member {
                  "└── "
                } else {
                  "├── "
                },
                vis,
                static_str,
                readonly_str,
                async_str,
                name,
                params.len(),
                return_str
              );
            },
            ClassMemberKind::Constructor { params, .. } => {
              println!(
                "{}{}{}constructor({} params)",
                new_prefix,
                if is_last_member {
                  "└── "
                } else {
                  "├── "
                },
                vis,
                params.len()
              );
            },
          }
        }
      },

      Stmt::InterfaceDecl {
        name,
        type_params,
        extends,
        members,
        ..
      } => {
        let generics = if !type_params.is_empty() {
          format!("<{}>", type_params.len())
        } else {
          String::new()
        };
        let extends_str = if !extends.is_empty() {
          format!(" extends {} types", extends.len())
        } else {
          String::new()
        };
        print_node!(
          prefix,
          connector,
          format!("interface {}{}{}", name, generics, extends_str)
        );

        for (i, member) in members.iter().enumerate() {
          let is_last_member = i == members.len() - 1;
          match &member.kind {
            InterfaceMemberKind::Property {
              type_annotation,
              optional,
            } => {
              let opt = if *optional { "?" } else { "" };
              println!(
                "{}{}{}{}: {:?}",
                new_prefix,
                if is_last_member {
                  "└── "
                } else {
                  "├── "
                },
                member.name,
                opt,
                type_annotation
              );
            },
            InterfaceMemberKind::Method {
              params,
              return_type,
              ..
            } => {
              println!(
                "{}{}{}({} params): {:?}",
                new_prefix,
                if is_last_member {
                  "└── "
                } else {
                  "├── "
                },
                member.name,
                params.len(),
                return_type
              );
            },
            InterfaceMemberKind::IndexSignature {
              key_name,
              key_type,
              value_type,
            } => {
              println!(
                "{}{}[{}: {:?}]: {:?}",
                new_prefix,
                if is_last_member {
                  "└── "
                } else {
                  "├── "
                },
                key_name,
                key_type,
                value_type
              );
            },
          }
        }
      },

      Stmt::TypeAliasDecl {
        name,
        type_params,
        type_annotation,
        ..
      } => {
        let generics = if !type_params.is_empty() {
          format!("<{}>", type_params.len())
        } else {
          String::new()
        };
        print_node!(
          prefix,
          connector,
          format!("type {}{} = {:?}", name, generics, type_annotation)
        );
      },

      Stmt::EnumDecl { name, members, .. } => {
        print_node!(prefix, connector, format!("enum {}", name));
        for (i, member) in members.iter().enumerate() {
          let value_str = if let Some(ref val) = member.value {
            match val {
              EnumValue::Number(n) => format!(" = {}", n),
              EnumValue::String(s) => format!(" = \"{}\"", s),
            }
          } else {
            String::new()
          };
          println!(
            "{}{}{}{}",
            new_prefix,
            if i == members.len() - 1 {
              "└── "
            } else {
              "├── "
            },
            member.name,
            value_str
          );
        }
      },

      Stmt::NamespaceDecl { name, body, .. } => {
        print_node!(prefix, connector, format!("namespace {}", name));
        for (i, stmt) in body.iter().enumerate() {
          stmt.build_tree(&new_prefix, i == body.len() - 1, source_file);
        }
      },

      Stmt::If {
        condition,
        then_branch,
        else_branch,
        ..
      } => {
        print_node!(prefix, connector, "If");
        println!("{}├── condition:", new_prefix);
        condition.build_tree(&format!("{}│   ", new_prefix), true, source_file);
        println!(
          "{}{}then:",
          new_prefix,
          if else_branch.is_some() {
            "├── "
          } else {
            "└── "
          }
        );
        let then_prefix = format!(
          "{}{}",
          new_prefix,
          if else_branch.is_some() {
            "│   "
          } else {
            "    "
          }
        );
        then_branch.build_tree(&then_prefix, true, source_file);
        if let Some(else_stmt) = else_branch {
          println!("{}└── else:", new_prefix);
          else_stmt.build_tree(&format!("{}    ", new_prefix), true, source_file);
        }
      },

      Stmt::While {
        condition, body, ..
      } => {
        print_node!(prefix, connector, "While");
        condition.build_tree(&new_prefix, false, source_file);
        body.build_tree(&new_prefix, true, source_file);
      },

      Stmt::DoWhile {
        body, condition, ..
      } => {
        print_node!(prefix, connector, "DoWhile");
        body.build_tree(&new_prefix, false, source_file);
        condition.build_tree(&new_prefix, true, source_file);
      },

      Stmt::For {
        init,
        condition,
        update,
        body,
        ..
      } => {
        print_node!(prefix, connector, "For");
        if let Some(i) = init {
          println!("{}├── init:", new_prefix);
          i.build_tree(&format!("{}│   ", new_prefix), true, source_file);
        }
        if let Some(c) = condition {
          println!("{}├── condition:", new_prefix);
          c.build_tree(&format!("{}│   ", new_prefix), true, source_file);
        }
        if let Some(u) = update {
          println!("{}├── update:", new_prefix);
          u.build_tree(&format!("{}│   ", new_prefix), true, source_file);
        }
        println!("{}└── body:", new_prefix);
        body.build_tree(&format!("{}    ", new_prefix), true, source_file);
      },

      Stmt::ForIn {
        kind,
        variable,
        iterable,
        body,
        ..
      } => {
        print_node!(
          prefix,
          connector,
          format!("ForIn({:?} {} in)", kind, variable)
        );
        iterable.build_tree(&new_prefix, false, source_file);
        body.build_tree(&new_prefix, true, source_file);
      },

      Stmt::ForOf {
        kind,
        variable,
        iterable,
        body,
        ..
      } => {
        print_node!(
          prefix,
          connector,
          format!("ForOf({:?} {} of)", kind, variable)
        );
        iterable.build_tree(&new_prefix, false, source_file);
        body.build_tree(&new_prefix, true, source_file);
      },

      Stmt::Switch {
        discriminant,
        cases,
        ..
      } => {
        print_node!(prefix, connector, "Switch");
        discriminant.build_tree(&new_prefix, false, source_file);
        println!("{}└── cases:", new_prefix);
        let cases_prefix = format!("{}    ", new_prefix);
        for (i, case) in cases.iter().enumerate() {
          if let Some(test) = &case.test {
            println!(
              "{}{}case:",
              cases_prefix,
              if i == cases.len() - 1 {
                "└── "
              } else {
                "├── "
              }
            );
            let case_prefix = format!(
              "{}{}",
              cases_prefix,
              if i == cases.len() - 1 {
                "    "
              } else {
                "│   "
              }
            );
            test.build_tree(&case_prefix, case.consequent.is_empty(), source_file);
            for (j, stmt) in case.consequent.iter().enumerate() {
              stmt.build_tree(&case_prefix, j == case.consequent.len() - 1, source_file);
            }
          } else {
            println!(
              "{}{}default:",
              cases_prefix,
              if i == cases.len() - 1 {
                "└── "
              } else {
                "├── "
              }
            );
            let case_prefix = format!(
              "{}{}",
              cases_prefix,
              if i == cases.len() - 1 {
                "    "
              } else {
                "│   "
              }
            );
            for (j, stmt) in case.consequent.iter().enumerate() {
              stmt.build_tree(&case_prefix, j == case.consequent.len() - 1, source_file);
            }
          }
        }
      },

      Stmt::Try {
        block,
        catch_clause,
        finally_block,
        ..
      } => {
        print_node!(prefix, connector, "Try");
        let has_catch = catch_clause.is_some();
        let has_finally = finally_block.is_some();

        println!(
          "{}{}try:",
          new_prefix,
          if has_catch || has_finally {
            "├── "
          } else {
            "└── "
          }
        );
        let try_prefix = format!(
          "{}{}",
          new_prefix,
          if has_catch || has_finally {
            "│   "
          } else {
            "    "
          }
        );
        for (i, stmt) in block.iter().enumerate() {
          stmt.build_tree(&try_prefix, i == block.len() - 1, source_file);
        }

        if let Some(catch) = catch_clause {
          let catch_label = if let Some(ref param) = catch.param {
            format!("catch({})", param)
          } else {
            "catch".to_string()
          };
          println!(
            "{}{}{}:",
            new_prefix,
            if has_finally {
              "├── "
            } else {
              "└── "
            },
            catch_label
          );
          let catch_prefix = format!(
            "{}{}",
            new_prefix,
            if has_finally { "│   " } else { "    " }
          );
          for (i, stmt) in catch.body.iter().enumerate() {
            stmt.build_tree(&catch_prefix, i == catch.body.len() - 1, source_file);
          }
        }

        if let Some(finally) = finally_block {
          println!("{}└── finally:", new_prefix);
          let finally_prefix = format!("{}    ", new_prefix);
          for (i, stmt) in finally.iter().enumerate() {
            stmt.build_tree(&finally_prefix, i == finally.len() - 1, source_file);
          }
        }
      },

      Stmt::Throw { expr, .. } => {
        print_node!(prefix, connector, "Throw");
        expr.build_tree(&new_prefix, true, source_file);
      },

      Stmt::Return { value, .. } => {
        print_node!(prefix, connector, "Return");
        if let Some(val) = value {
          val.build_tree(&new_prefix, true, source_file);
        }
      },

      Stmt::Break { .. } => {
        print_node!(prefix, connector, "Break");
      },

      Stmt::Continue { .. } => {
        print_node!(prefix, connector, "Continue");
      },

      Stmt::Block(stmts) => {
        print_node!(prefix, connector, "Block");
        for (i, stmt) in stmts.iter().enumerate() {
          stmt.build_tree(&new_prefix, i == stmts.len() - 1, source_file);
        }
      },

      Stmt::Export { declaration, .. } => {
        print_node!(prefix, connector, "Export");
        declaration.build_tree(&new_prefix, true, source_file);
      },

      Stmt::Import {
        specifiers, source, ..
      } => {
        let spec_str = match specifiers {
          ImportSpecifier::Default(name) => format!("{}", name),
          ImportSpecifier::Named(names) => format!("{{ {} }}", names.join(", ")),
          ImportSpecifier::Namespace(name) => format!("* as {}", name),
        };
        print_node!(
          prefix,
          connector,
          format!("Import {} from \"{}\"", spec_str, source)
        );
      },
    }
  }
}
