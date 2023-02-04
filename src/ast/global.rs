use crate::ast::{
    ClassDefinitionStatement, ConstDefinitionStatement, EnumDefinitionStatement, Identifier,
    StructDefinitionStatement, TypeDefinitionStatement, VarDefinitionStatement, VarInitializer,
};
use crate::token::Token;

/// Right-hand-side of a [`GlobalStatement`].
///
/// [`GlobalStatement`]: crate::ast::GlobalStatement
#[derive(Debug, Clone)]
pub enum GlobalDefinition<'s> {
    /// Global function.
    ///
    /// Grammar: `function` [Identifier]
    Function {
        function: &'s Token<'s>,
        name: Identifier<'s>,
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
    /// Grammar: [VarDefinitionStatement]
    TypedVar(VarDefinitionStatement<'s>),

    /// Global constant.
    ///
    /// Grammar: [ConstDefinitionStatement]
    Const(ConstDefinitionStatement<'s>),

    /// Global enum.
    ///
    /// Grammar: [EnumDefinitionStatement]
    Enum(EnumDefinitionStatement<'s>),

    /// Global class definition.
    ///
    /// Grammar: [ClassDefinitionStatement]
    Class(ClassDefinitionStatement<'s>),

    /// Global struct definition.
    ///
    /// Grammar: [StructDefinitionStatement]
    Struct(StructDefinitionStatement<'s>),

    /// Global type definition.
    ///
    /// Grammar: [TypeDefinitionStatement]
    Type(TypeDefinitionStatement<'s>),
}
