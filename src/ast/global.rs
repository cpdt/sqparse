use crate::ast::{
    ClassDeclarationStatement, ConstStatement, EnumStatement, Identifier, SeparatedList1,
    StructDeclarationStatement, TypedefStatement, VarDeclarationStatement, VarInitializer,
};
use crate::token::Token;

/// Right-hand-side of a [`GlobalStatement`].
///
/// [`GlobalStatement`]: crate::ast::GlobalStatement
#[derive(Debug, Clone)]
pub enum GlobalDeclaration<'s> {
    /// Global function.
    ///
    /// Grammar: `function` [SeparatedList1]<[Identifier] `::`>
    Function {
        function: &'s Token<'s>,
        name: SeparatedList1<'s, Identifier<'s>>,
    },

    /// Global untyped variable.
    ///
    /// Grammar: [Identifier] [VarInitializer]
    UntypedVar {
        name: Identifier<'s>,
        initializer: VarInitializer<'s>,
    },

    /// Global typed variable.
    ///
    /// Grammar: [VarDeclarationStatement]
    TypedVar(VarDeclarationStatement<'s>),

    /// Global constant.
    ///
    /// Grammar: [ConstStatement]
    Const(ConstStatement<'s>),

    /// Global enum.
    ///
    /// Grammar: [EnumStatement]
    Enum(EnumStatement<'s>),

    /// Global class declaration.
    ///
    /// Grammar: [ClassDeclarationStatement]
    Class(ClassDeclarationStatement<'s>),

    /// Global struct declaration.
    ///
    /// Grammar: [StructDeclarationStatement]
    Struct(StructDeclarationStatement<'s>),

    /// Global type definition.
    ///
    /// Grammar: [TypedefStatement]
    Type(TypedefStatement<'s>),
}
