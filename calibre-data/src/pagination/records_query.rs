use sea_orm::{EntityTrait, Select};

pub trait RecordsQuery<E>
where
    E: EntityTrait,
{
    fn records_query(&self, page: u64) -> Select<E>;
}
