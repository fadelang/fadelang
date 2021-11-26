pub trait Token: std::fmt::Debug {}

impl<Rhs: ?Sized + 'static> PartialEq<Rhs> for dyn Token {
  fn eq(&self, _: &Rhs) -> bool {
    std::any::TypeId::of::<Self>() == std::any::TypeId::of::<Rhs>()
  }
}

impl Eq for dyn Token {}

///////////////////////////////////////////////////////////////////////
/// EndOfFile, NewLine, Whitespace
///////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct EndOfFile;

impl Default for EndOfFile {
  fn default() -> Self {
    Self {}
  }
}

impl Token for EndOfFile {}

#[derive(Debug)]
pub struct NewLine;

impl Default for NewLine {
  fn default() -> Self {
    Self {}
  }
}

impl Token for NewLine {}

#[derive(Debug)]
pub struct Whitespace;

impl Default for Whitespace {
  fn default() -> Self {
    Self {}
  }
}

impl Token for Whitespace {}

///////////////////////////////////////////////////////////////////////
/// Keyword
///////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Keyword {
  keyword: String,
}

impl Keyword {
  pub fn is_valid_char(character: &char) -> bool {
    ('a'..='z').contains(character)
  }
}

impl Keyword {
  pub fn keyword(&self) -> String {
    self.keyword.clone()
  }
}

impl Token for Keyword {}

impl From<&str> for Keyword {
  fn from(string: &str) -> Self {
    Self::from(String::from(string))
  }
}

impl From<String> for Keyword {
  fn from(string: String) -> Self {
    Self { keyword: string }
  }
}

pub fn is_keyword(string: &str) -> bool {
  matches!(string, "u8" | "return")
}

///////////////////////////////////////////////////////////////////////
/// Identifier
///////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Identifier {
  identifier: String,
}

impl Identifier {
  pub fn is_valid_char(character: &char, beginning: bool) -> bool {
    ('a'..='z').contains(character)
      || ('A'..='Z').contains(character)
      || character == &'_'
      || if beginning {
        false
      } else {
        ('0'..='9').contains(character)
      }
  }
}

impl Identifier {
  pub fn identifier(&self) -> String {
    self.identifier.clone()
  }
}

impl Token for Identifier {}

impl From<&str> for Identifier {
  fn from(string: &str) -> Self {
    Self::from(String::from(string))
  }
}

impl From<String> for Identifier {
  fn from(string: String) -> Self {
    Self { identifier: string }
  }
}

///////////////////////////////////////////////////////////////////////
/// Paranthesis, Bracket, Brace
///////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub enum BracketType {
  Opening,
  Closing,
}

#[derive(Debug)]
pub struct Parenthesis {
  bracket_type: BracketType,
}

impl Token for Parenthesis {}

impl Parenthesis {
  pub fn bracket_type(&self) -> BracketType {
    self.bracket_type
  }
}

impl Parenthesis {
  pub fn open() -> Self {
    Self::from(BracketType::Opening)
  }

  pub fn close() -> Self {
    Self::from(BracketType::Closing)
  }
}

impl From<BracketType> for Parenthesis {
  fn from(bracket_type: BracketType) -> Self {
    Self { bracket_type }
  }
}

#[derive(Debug)]
pub struct Bracket {
  bracket_type: BracketType,
}

impl Token for Bracket {}

impl Bracket {
  pub fn bracket_type(&self) -> BracketType {
    self.bracket_type
  }
}

impl Bracket {
  pub fn open() -> Self {
    Self::from(BracketType::Opening)
  }

  pub fn close() -> Self {
    Self::from(BracketType::Closing)
  }
}

impl From<BracketType> for Bracket {
  fn from(bracket_type: BracketType) -> Self {
    Self { bracket_type }
  }
}

#[derive(Debug)]
pub struct Brace {
  bracket_type: BracketType,
}

impl Token for Brace {}

impl Brace {
  pub fn bracket_type(&self) -> BracketType {
    self.bracket_type
  }
}

impl Brace {
  pub fn open() -> Self {
    Self::from(BracketType::Opening)
  }

  pub fn close() -> Self {
    Self::from(BracketType::Closing)
  }
}

impl From<BracketType> for Brace {
  fn from(bracket_type: BracketType) -> Self {
    Self { bracket_type }
  }
}

///////////////////////////////////////////////////////////////////////
/// Operator
///////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub enum OperatorType {
  // Scoping, Accessing
  ScopeAccessor,  // ::
  MemberAccessor, // ->

  // Generics
  GenericBlockBegin, // <
  GenericBlockEnd,   // >

  // Types
  TypeSpecifier,       // :
  ReturnType,          // ->
  CommaSeparator,      // ,
  StatementTerminator, // ;

  // Arithmetic
  Addition,       // +
  Subtraction,    // -
  Multiplication, // *
  Division,       // /
  Modulo,         // %

  // Comparison
  Equals,             // ==
  NotEquals,          // !=
  LessThan,           // <
  LessThanOrEqual,    // <=
  GreaterThan,        // >
  GreaterThanOrEqual, // >=

  // Logic
  LogicalAnd, // &&
  LogicalOr,  // ||
  LogicalNot, // !

  // Bitwise
  BitwiseAnd,        // &
  BitwiseXOr,        // ^
  BitwiseOr,         // |
  BitwiseNot,        // ~
  BitwiseRightShift, // >>
  BitwiseLeftShift,  // <<

  //Assignment
  ValueAssignment,          // =
  AdditionAssignment,       // +=
  SubtractionAssignment,    // -=
  MultiplicationAssignment, // *=
  DivisionAssignment,       // /=
  ModuloAssignment,         // %=
  Increment,                // ++
  Decrement,                // --

  BitwiseRightShiftAssignment, // >>=
  BitwiseLeftShiftAssignment,  // <<=
  BitwiseAndAssignment,        // &=
  BitwiseXOrAssignment,        // ^=
  BitwiseOrAssignment,         // |=
}

#[derive(Debug)]
pub struct Operator {
  operator_type: OperatorType,
}

impl Token for Operator {}

impl Operator {
  pub fn operator_type(&self) -> OperatorType {
    self.operator_type
  }
}

impl From<OperatorType> for Operator {
  fn from(operator_type: OperatorType) -> Self {
    Self { operator_type }
  }
}
