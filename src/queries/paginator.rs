use sea_orm::{
    sea_query::{Expr, Func, SimpleExpr},
    ColumnTrait, EntityTrait, QueryOrder, QuerySelect, SelectGetableTuple, Selector,
    TryGetableMany,
};

use crate::queries::{FunctionName, SelectAlias};

pub fn substring_buckets<E, C, T>(
    bucket_column: C,
    substring_length: usize,
) -> Selector<SelectGetableTuple<T>>
where
    E: EntityTrait,
    C: ColumnTrait,
    T: TryGetableMany,
{
    let bucket_select_alias = SelectAlias("__bucket_select_alias");
    E::find()
        .select_only()
        .column_as(
            SimpleExpr::FunctionCall(Func::cust(FunctionName("substring")).args([
                bucket_column.into_expr().into(),
                1.into(),
                (substring_length as u64).into(),
            ])),
            bucket_select_alias,
        )
        .group_by(Expr::col(bucket_select_alias))
        .order_by_asc(Expr::col(bucket_select_alias))
        .into_tuple::<T>()
}
