mod array;
mod class;
mod control;
mod enum_;
mod expression;
mod function;
mod global;
mod identifier;
mod list;
mod operator;
mod precedence;
mod statement;
mod struct_;
mod table;
mod type_;
mod variable;

pub use self::array::*;
pub use self::class::*;
pub use self::control::*;
pub use self::enum_::*;
pub use self::expression::*;
pub use self::function::*;
pub use self::global::*;
pub use self::identifier::*;
pub use self::list::*;
pub use self::operator::*;
pub use self::precedence::*;
pub use self::statement::*;
pub use self::struct_::*;
pub use self::table::*;
pub use self::type_::*;
pub use self::variable::*;

#[derive(Debug, Clone)]
pub struct Program<'s> {
    pub statements: Vec<Statement<'s>>,
}
