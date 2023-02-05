use crate::lexer::error::LexerError;
use crate::lexer::parse_str::ParseStr;
use crate::token::Comment;

pub fn try_comment(val: ParseStr) -> Result<Option<(Comment, ParseStr)>, LexerError> {
    if let Some(val) = val.strip_prefix("/*") {
        return match val.as_str().find("*/") {
            Some(end_index) => Ok(Some((
                Comment::MultiLine(&val.as_str()[..end_index]),
                val.from(end_index + 2),
            ))),
            None => Ok(Some((Comment::MultiLine(val.as_str()), val.end()))),
        };
    }

    if let Some(remaining) = val.strip_prefix("#") {
        let (comment_val, remaining) = get_rest_of_line(remaining);
        return Ok(Some((Comment::ScriptStyle(comment_val), remaining)));
    }

    if let Some(remaining) = val.strip_prefix("//") {
        let (comment_val, remaining) = get_rest_of_line(remaining);
        return Ok(Some((Comment::SingleLine(comment_val), remaining)));
    }

    Ok(None)
}

fn get_rest_of_line(val: ParseStr) -> (&str, ParseStr) {
    val.split_at(val.as_str().find('\n'))
}
