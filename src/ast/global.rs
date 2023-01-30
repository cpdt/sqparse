use crate::ast::{
    ClassDeclarationStatement, ConstStatement, EnumStatement, Identifier, SeparatedList1,
    StructDeclarationStatement, TypedefStatement, VarDeclarationStatement, VarInitializer,
};
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum GlobalDeclaration<'s> {
    // `function` SeparatedList1<Identifier `::`>
    Function {
        function: &'s Token<'s>,
        name: SeparatedList1<'s, Identifier<'s>>,
    },
    UntypedVar {
        name: Identifier<'s>,
        initializer: VarInitializer<'s>,
    },

    TypedVar(VarDeclarationStatement<'s>),
    Const(ConstStatement<'s>),
    Enum(EnumStatement<'s>),
    Class(ClassDeclarationStatement<'s>),
    Struct(StructDeclarationStatement<'s>),
    Type(TypedefStatement<'s>),
}
