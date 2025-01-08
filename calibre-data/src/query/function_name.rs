/// Wrapper type for arbitrary SQL functions
pub struct FunctionName<'a>(pub &'a str);

impl sea_orm::Iden for FunctionName<'_> {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        s.write_str(self.0).unwrap()
    }
}

impl From<FunctionName<'static>> for sea_orm::sea_query::FunctionCall {
    fn from(value: FunctionName<'static>) -> Self {
        sea_orm::sea_query::Func::cust(value)
    }
}

impl FunctionName<'static> {
    /// Convert into a sea-query::FunctionCall with provided arguments
    pub fn into_func_with_args<I>(self, args: I) -> sea_orm::sea_query::SimpleExpr
    where
        I: IntoIterator<Item = sea_orm::sea_query::SimpleExpr>,
    {
        sea_orm::sea_query::FunctionCall::from(self)
            .args(args)
            .into()
    }
}
