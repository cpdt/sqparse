use crate::lexer::error::{LexerError, LexerErrorType};
use crate::lexer::parse_str::ParseStr;
use crate::token::{LiteralBase, LiteralToken, StringToken};
use lexical::NumberFormatBuilder;

pub fn try_literal(val: ParseStr) -> Result<Option<(LiteralToken, ParseStr)>, LexerError> {
    if let Some((num_val, remaining)) = try_number(val) {
        return Ok(Some((num_val, remaining)));
    }
    if let Some((char_val, remaining)) = try_string_val(val, '\'', false)? {
        return Ok(Some((LiteralToken::Char(char_val), remaining)));
    }
    if let Some((str_val, remaining)) = try_string(val)? {
        return Ok(Some((LiteralToken::String(str_val), remaining)));
    }
    Ok(None)
}

fn starts_with_octal(val: &str) -> bool {
    let mut chars = val.chars();
    match chars.next() {
        Some('0') => {}
        Some(_) | None => return false,
    };
    chars
        .next()
        .map(|digit| digit.is_ascii_digit())
        .unwrap_or(false)
}

fn try_number(val: ParseStr) -> Option<(LiteralToken, ParseStr)> {
    let starts_with_digit = val.as_str().starts_with(|c: char| c.is_ascii_digit());

    if starts_with_digit {
        let (base, remaining) = if val.as_str().starts_with("0x") {
            (LiteralBase::Hexadecimal, val.from(2))
        } else if starts_with_octal(val.as_str()) {
            (LiteralBase::Octal, val.from(1))
        } else {
            (LiteralBase::Decimal, val)
        };

        const DECIMAL_FORMAT: u128 = NumberFormatBuilder::decimal();
        const OCTAL_FORMAT: u128 = NumberFormatBuilder::octal();
        const HEXADECIMAL_FORMAT: u128 = NumberFormatBuilder::hexadecimal();

        let parse_result = match base {
            LiteralBase::Decimal => lexical::parse_partial_with_options::<i64, _, DECIMAL_FORMAT>(
                remaining.as_str(),
                &lexical::ParseIntegerOptions::new(),
            ),
            LiteralBase::Octal => lexical::parse_partial_with_options::<i64, _, OCTAL_FORMAT>(
                remaining.as_str(),
                &lexical::ParseIntegerOptions::new(),
            ),
            LiteralBase::Hexadecimal => {
                lexical::parse_partial_with_options::<i64, _, HEXADECIMAL_FORMAT>(
                    remaining.as_str(),
                    &lexical::ParseIntegerOptions::new(),
                )
            }
        };

        let (int_val, remaining) = match parse_result {
            Err(_) | Ok((_, 0)) => return None,
            Ok((parsed_val, len)) => (LiteralToken::Int(parsed_val, base), remaining.from(len)),
        };

        if !remaining.as_str().starts_with('.') {
            return Some((int_val, remaining));
        }
    }

    let starts_with_dot = val.as_str().starts_with('.');
    if !starts_with_digit && !starts_with_dot {
        return None;
    }

    match lexical::parse_partial::<f64, _>(val.as_str()) {
        Err(_) => None,
        Ok((_, 1)) if starts_with_dot => None,
        Ok((parsed_val, len)) => {
            // The Squirrel lexer is a bit cursed and parses but ignores any [.0-9] chars after
            // a floating point number, so we need to skip those here
            let val = val.from(len);
            let float_end = val
                .as_str()
                .find(|c: char| c != '.' && !c.is_ascii_digit())
                .unwrap_or_else(|| val.len());
            Some((LiteralToken::Float(parsed_val), val.from(float_end)))
        }
    }
}

fn try_string_val(
    val: ParseStr,
    delimiter: char,
    is_verbatim: bool,
) -> Result<Option<(&str, ParseStr)>, LexerError> {
    if !val.as_str().starts_with(delimiter) {
        return Ok(None);
    }

    let val = val.from(1);

    // Scan the string looking for an escape sequence or end of string
    let mut char_indices = val.as_str().char_indices().peekable();
    loop {
        let (next_index, next_char) = match char_indices.next() {
            Some(val) => val,
            None => {
                return Err(LexerError::new(
                    val.end_offset()..val.end_offset(),
                    LexerErrorType::EndOfInputInsideString,
                ));
            }
        };

        // Strings are not allowed to span multiple lines, unless they are verbatim.
        if !is_verbatim && next_char == '\n' {
            let newline_offset = val.start_offset() + next_index;
            return Err(LexerError::new(
                newline_offset..newline_offset,
                LexerErrorType::EndOfLineInsideString,
            ));
        }

        // Non-verbatim strings can use escape sequences to include newlines or delimiter chars.
        if !is_verbatim && next_char == '\\' {
            char_indices.next();
            continue;
        }

        // Verbatim strings can include a double delimiter
        if is_verbatim && next_char == delimiter {
            if let Some((_, skip_char)) = char_indices.peek() {
                if *skip_char == delimiter {
                    char_indices.next();
                    continue;
                }
            }
        }

        if next_char == delimiter {
            return Ok(Some((
                &val.as_str()[..next_index],
                val.from(next_index + 1),
            )));
        }
    }
}

fn try_string(val: ParseStr) -> Result<Option<(StringToken, ParseStr)>, LexerError> {
    if let Some(remaining) = val.strip_prefix("@") {
        Ok(try_string_val(remaining, '"', true)?
            .map(|(val, remaining)| (StringToken::Verbatim(val), remaining)))
    } else if let Some(remaining) = val.strip_prefix("$") {
        Ok(try_string_val(remaining, '"', false)?
            .map(|(val, remaining)| (StringToken::Asset(val), remaining)))
    } else {
        Ok(try_string_val(val, '"', false)?
            .map(|(val, remaining)| (StringToken::Literal(val), remaining)))
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::literal::try_string;
    use crate::lexer::parse_str::ParseStr;
    use crate::token::StringToken;

    #[test]
    fn single_line_literal_string() {
        let (val, _) = try_string(ParseStr::new(r#""this is a literal string""#))
            .unwrap()
            .unwrap();

        assert_eq!(val, StringToken::Literal("this is a literal string"));
    }

    #[test]
    fn single_line_escaped_literal_string() {
        let (val, _) = try_string(ParseStr::new(
            r#""this literal string includes \" a delimiter""#,
        ))
        .unwrap()
        .unwrap();

        assert_eq!(
            val,
            StringToken::Literal(r#"this literal string includes \" a delimiter"#)
        );
    }

    #[test]
    fn escaped_linebreak_literal_string() {
        let (val, _) = try_string(ParseStr::new(
            r#""this literal string includes \
a newline \
or two""#,
        ))
        .unwrap()
        .unwrap();

        assert_eq!(
            val,
            StringToken::Literal(
                r#"this literal string includes \
a newline \
or two"#
            )
        );
    }

    #[test]
    fn unescaped_linebreak_literal_string() {
        let result = try_string(ParseStr::new(
            r#""this literal string includes
a newline""#,
        ));

        assert!(result.is_err());
    }

    #[test]
    fn single_line_verbatim_string() {
        let (val, _) = try_string(ParseStr::new(r#"@"this is a verbatim string""#))
            .unwrap()
            .unwrap();

        assert_eq!(val, StringToken::Verbatim("this is a verbatim string"));
    }

    #[test]
    fn single_line_delimiter_verbatim_string() {
        let (val, _) = try_string(ParseStr::new(
            r#"@"this verbatim string includes a "" delimiter""#,
        ))
        .unwrap()
        .unwrap();

        assert_eq!(
            val,
            StringToken::Verbatim(r#"this verbatim string includes a "" delimiter"#)
        );
    }

    #[test]
    fn linebreak_verbatim_string() {
        let (val, _) = try_string(ParseStr::new(
            r#"@"this verbatim string includes
a newline
or two""#,
        ))
        .unwrap()
        .unwrap();

        assert_eq!(
            val,
            StringToken::Verbatim(
                r#"this verbatim string includes
a newline
or two"#
            )
        );
    }
}
