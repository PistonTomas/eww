use eww_shared_util::Span;
use itertools::Itertools;

use crate::{ast::BinOp, dynval::DynVal, SimplExpr};

use super::lexer::{LexicalError, Sp, StrLitSegment, Token};

pub fn b<T>(x: T) -> Box<T> {
    Box::new(x)
}

pub fn parse_stringlit(
    span: Span,
    segs: Vec<Sp<StrLitSegment>>,
) -> Result<SimplExpr, lalrpop_util::ParseError<usize, Token, LexicalError>> {
    let file_id = span.2;
    let parser = crate::simplexpr_parser::ExprParser::new();

    segs.into_iter()
        .filter_map(|(lo, segment, hi)| {
            let span = Span(lo, hi, file_id);
            match segment {
                StrLitSegment::Literal(lit) if lit.is_empty() => None,
                StrLitSegment::Literal(lit) => Some(Ok(SimplExpr::Literal(DynVal(lit, span)))),
                StrLitSegment::Interp(toks) => {
                    let token_stream = toks.into_iter().map(|x| Ok(x));
                    Some(parser.parse(file_id, token_stream))
                }
            }
        })
        .fold_ok(None, |acc, cur| match acc {
            Some(ast) => Some(SimplExpr::BinOp(span, Box::new(ast), BinOp::Plus, Box::new(cur))),
            None => Some(cur),
        })
        .map(|ast| ast.unwrap_or_else(|| SimplExpr::Literal(DynVal(String::new(), span))))
}