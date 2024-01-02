use sea_orm::{
    sea_query::{IdenStatic, SeaRc},
    Iden, Identity, IntoIdentity,
};
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct SelectAlias(pub &'static str);

impl Display for SelectAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl IdenStatic for SelectAlias {
    fn as_str(&self) -> &'static str {
        self.0
    }
}

impl Iden for SelectAlias {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}", self.as_str()).unwrap();
    }
}

impl IntoIdentity for SelectAlias {
    fn into_identity(self) -> Identity {
        Identity::Unary(SeaRc::new(self))
    }
}
