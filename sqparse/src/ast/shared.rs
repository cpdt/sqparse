use crate::ast::{Expression, Statement, TableExpression, Type};
use crate::Token;

#[derive(Debug, Clone)]
pub struct SeparatedList1<'s, T> {
    pub items: Vec<(T, &'s Token<'s>)>,
    pub last_item: Box<T>,
}

pub type SeparatedList0<'s, T> = Option<SeparatedList1<'s, T>>;

impl<'s, T> SeparatedList1<'s, T> {
    pub fn push(mut self, last_separator: &'s Token<'s>, item: T) -> Self {
        self.items.push((
            std::mem::replace(self.last_item.as_mut(), item),
            last_separator,
        ));
        self
    }
}

#[derive(Debug, Clone)]
pub struct Identifier<'s> {
    pub value: &'s str,
    pub token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct FunctionDeclaration<'s> {
    pub environment: Option<FunctionEnvironment<'s>>,
    pub args: FunctionArgs<'s>,
    pub captures: Option<FunctionCaptures<'s>>,
    pub body: Box<Statement<'s>>,

    pub open_args_token: &'s Token<'s>,
    pub close_args_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct ClassDeclaration<'s> {
    pub extends: Option<ClassExtends<'s>>,
    pub members: Vec<ClassMember<'s>>,

    pub open_members_token: &'s Token<'s>,
    pub close_members_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct StructDeclaration<'s> {
    pub properties: Vec<StructProperty<'s>>,

    pub open_properties_token: &'s Token<'s>,
    pub close_properties_token: &'s Token<'s>,
}

//

#[derive(Debug, Clone)]
pub struct FunctionEnvironment<'s> {
    pub environment: Box<Expression<'s>>,

    pub open_token: &'s Token<'s>,
    pub close_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub enum FunctionArgs<'s> {
    List {
        arg_list: SeparatedList1<'s, FunctionArg<'s>>,
        trailing: Option<FunctionArgsTrailing<'s>>,
    },
    Empty {
        vararg_token: Option<&'s Token<'s>>,
    },
}

#[derive(Debug, Clone)]
pub struct FunctionCaptures<'s> {
    pub capture_names: SeparatedList0<'s, Identifier<'s>>,

    pub separator_token: &'s Token<'s>,
    pub open_token: &'s Token<'s>,
    pub trailing_separator_token: Option<&'s Token<'s>>,
    pub close_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct FunctionArg<'s> {
    pub arg_type: Option<Type<'s>>,
    pub arg_name: Identifier<'s>,
    pub initializer: Option<VarInitializer<'s>>,
}

#[derive(Debug, Clone)]
pub enum FunctionArgsTrailing<'s> {
    VarArg {
        separator_token: &'s Token<'s>,
        vararg_token: &'s Token<'s>,
    },
    Separator {
        token: &'s Token<'s>,
    },
}

#[derive(Debug, Clone)]
pub struct ClassExtends<'s> {
    pub name: Box<Expression<'s>>,

    pub extends_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub struct ClassMember<'s> {
    pub attributes: Option<Box<TableExpression<'s>>>,
    pub ty: ClassMemberType<'s>,

    pub separator_token: Option<&'s Token<'s>>,
}

#[derive(Debug, Clone)]
pub struct StructProperty<'s> {
    pub property_type: Type<'s>,
    pub property_name: Identifier<'s>,
    pub initializer: Option<StructInitializer<'s>>,

    pub separator_token: Option<&'s Token<'s>>,
}

//

#[derive(Debug, Clone)]
pub struct VarInitializer<'s> {
    pub value: Box<Expression<'s>>,

    pub separator_token: &'s Token<'s>,
}

#[derive(Debug, Clone)]
pub enum ClassMemberType<'s> {
    Property {
        name: Identifier<'s>,
        value: Box<Expression<'s>>,

        static_token: Option<&'s Token<'s>>,
        separator_token: &'s Token<'s>,
        end_token: Option<&'s Token<'s>>,
    },
    ComputedProperty {
        name: Box<Expression<'s>>,
        value: Box<Expression<'s>>,

        static_token: Option<&'s Token<'s>>,
        open_name_token: &'s Token<'s>,
        close_name_token: &'s Token<'s>,
        separator_token: &'s Token<'s>,
        end_token: Option<&'s Token<'s>>,
    },
    Constructor {
        function: FunctionDeclaration<'s>,

        constructor_token: &'s Token<'s>,
    },
    Function {
        return_type: Option<Type<'s>>,
        name: Identifier<'s>,
        function: FunctionDeclaration<'s>,

        function_token: &'s Token<'s>,
    },
}

#[derive(Debug, Clone)]
pub struct StructInitializer<'s> {
    pub value: Box<Expression<'s>>,

    pub separator_token: &'s Token<'s>,
}
