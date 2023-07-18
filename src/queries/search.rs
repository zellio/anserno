use crate::{
    entities::{search_index},
    queries::{SelectAlias},
};
use sea_orm::{
    sea_query::{Expr, SimpleExpr}, EntityTrait, QueryFilter, Select,
};

fn search_expr(query: &str) -> SimpleExpr {
    Expr::col(SelectAlias("anserno_search_index")).eq(query)
}

pub fn search_query(query: &str) -> Select<search_index::Entity> {
    search_index::Entity::find().filter(search_expr(query))
}

// pub fn search_flat_book_query(query: &str) -> Select<books::Entity> {
//     search_index::Entity::find()
//         .filter(search_expr(query))
//         .join(
//             JoinType::LeftJoin,

//     flat_books_query()
//         .left_join(search_index::Entity)
//         .filter(search_expr(query))
// }

// .join_as(
//            JoinType::LeftJoin,
//            cake::Relation::Fruit
//                .def()
//                .on_condition(|_left, right| {
//                    Expr::tbl(right, fruit::Column::Name)
//                        .like("%tropical%")
//                        .into_condition()
//                }),
//            Alias::new("fruit_alias")
//        )
