/* This file is generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};


type Connection = create_rust_app::Connection;

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name=tsp_set, primary_key(id))]
pub struct TspSet {
    pub id: i32,
    pub label: String,
    pub created_at: chrono::NaiveDateTime,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=tsp_set)]
pub struct TspSetForm {
    pub label: String,
}

#[tsync::tsync]
#[derive(Serialize)]
pub struct PaginationResult<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    /// 0-based index
    pub page: i64,
    pub page_size: i64,
    pub num_pages: i64,
}

impl TspSet {
    pub fn create(db: &mut Connection, item: &TspSetForm) -> QueryResult<TspSet> {
        use crate::schema::tsp_set::dsl::*;

        insert_into(tsp_set).values(item).get_result::<TspSet>(db)
    }

    pub fn read(db: &mut Connection, param_id: i32) -> QueryResult<TspSet> {
        use crate::schema::tsp_set::dsl::*;

        tsp_set.filter(id.eq(param_id)).first::<TspSet>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<TspSet>> {
        use crate::schema::tsp_set::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = tsp_set.count().get_result(db)?;
        let items = tsp_set.limit(page_size).offset(page * page_size).load::<TspSet>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    pub fn update(db: &mut Connection, param_id: i32, item: &TspSetForm) -> QueryResult<TspSet> {
        use crate::schema::tsp_set::dsl::*;

        diesel::update(tsp_set.filter(id.eq(param_id))).set(item).get_result(db)
    }

    pub fn delete(db: &mut Connection, param_id: i32) -> QueryResult<usize> {
        use crate::schema::tsp_set::dsl::*;

        diesel::delete(tsp_set.filter(id.eq(param_id))).execute(db)
    }
}
