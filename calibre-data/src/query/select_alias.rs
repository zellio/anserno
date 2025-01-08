/// Wrapper type for a bare string in an SQL query.
#[derive(::core::marker::Copy, ::std::clone::Clone, ::std::fmt::Debug)]
pub struct SelectAlias<'a>(pub &'a str);

impl ::std::fmt::Display for SelectAlias<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl sea_orm::Iden for SelectAlias<'_> {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        s.write_str(self.0).unwrap()
    }
}

impl sea_orm::IdenStatic for SelectAlias<'static> {
    fn as_str(&self) -> &str {
        self.0
    }
}
