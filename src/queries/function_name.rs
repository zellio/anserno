use sea_orm::{
    sea_query::{Func, FunctionCall, SimpleExpr},
    Iden,
};
use std::fmt::Write;

#[derive(Clone)]
pub struct FunctionName(pub &'static str);

impl FunctionName {
    pub fn as_function_with_args<I>(self, args: I) -> SimpleExpr
    where
        I: IntoIterator<Item = SimpleExpr>,
    {
        FunctionCall::from(self).args(args).into()
    }
}

impl Iden for FunctionName {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "{}", self.0).unwrap();
    }
}

impl From<FunctionName> for FunctionCall {
    fn from(value: FunctionName) -> Self {
        Func::cust(value)
    }
}
