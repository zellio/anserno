#[derive(Debug, serde::Serialize, Clone)]
pub struct Page {
    pub previous: Option<usize>,
    pub page: usize,
    pub next: Option<usize>,
}

impl<T> From<&T> for Page
where
    T: super::Paginator,
{
    fn from(value: &T) -> Self {
        Page {
            previous: if value.page() > 1 {
                Some(value.page() - 1)
            } else {
                None
            },
            page: value.page(),
            next: if value.page() < value.pages() {
                Some(value.page() + 1)
            } else {
                None
            },
        }
    }
}
