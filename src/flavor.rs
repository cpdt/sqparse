/// A flavor of Squirrel to parse.
///
/// This allows the lexer and parser to adapt to incompatibilities in the variants.
///
/// For example, Respawn's Squirrel variant adds some tokens which would be valid identifiers in
/// Squirrel 3.
///
/// In general however, if the addition of a construct in one variant does not make valid code in
/// another variant unparsable, it will not be gated by the flavor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flavor {
    /// Parse tokens in Respawn's Squirrel variant.
    SquirrelRespawn,

    /// Parse tokens in Squirrel 3.
    Squirrel3,
}
