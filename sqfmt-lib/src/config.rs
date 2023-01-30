pub enum IndentStyle {
    Tab { columns: u32 },
    Spaces { count: u32 },
}

pub struct Config {
    pub column_limit: u32,
    pub indent_style: IndentStyle,
    pub omit_semicolons: bool,
    pub spaces_inside_arg_decls: bool,
    pub spaces_inside_generics: bool,
    pub spaces_inside_control_statements: bool,
    pub spaces_inside_arrays: bool,
    pub spaces_inside_tables: bool,
    pub spaces_inside_expr_brackets: bool,
    pub spaces_inside_calls: bool,
    pub spaces_inside_array_getters: bool,
}

pub struct Format {
    pub column_limit: usize,

    pub indent: String,
    pub indent_columns: usize,

    pub spaces_in_expr_brackets: bool,

    pub array_spaces: bool,
    pub array_multiline_commas: bool,
    pub array_multiline_trailing_commas: bool,
    pub array_singleline_trailing_commas: bool,
}
