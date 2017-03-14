use core::{BuiltinType, Expr, X};
use lexer::Builtin;

pub type ParsedExpr<'i> = Expr<'i, X, X>; // FIXME Parse paths and replace the second X with Path
pub type BoxExpr<'i> = Box<ParsedExpr<'i>>;
pub type ExprOpFn<'i> = fn(BoxExpr<'i>, BoxExpr<'i>) -> ParsedExpr<'i>;

pub fn builtin_expr<'i, S, A>(b: Builtin) -> Expr<'i, S, A> {
    match b {
        Builtin::Type(t)  => Expr::BuiltinType(t),
        Builtin::Value(v) => Expr::BuiltinValue(v),
    }
}

pub fn annot<'i, S: Eq + Clone, A: Eq + Clone>(e: Box<Expr<'i, S, A>>,
                                               t: Box<Expr<'i, S, A>>)
                                               -> Expr<'i, S, A> {
    match e {
        box Expr::ListLit(None, ref xs) if *t == Expr::BuiltinType(BuiltinType::List) => {
            Expr::ListLit(Some(t), (*xs).clone())
        }
        box Expr::ListLit(None, ref xs) if *t == Expr::BuiltinType(BuiltinType::Optional) => {
            Expr::OptionalLit(t, (*xs).clone())
        }
        _ => Expr::Annot(e, t),
    }
}
