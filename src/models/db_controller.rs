use sqlx::PgPool;
#[derive(Debug)]
pub struct DbController{
    pub connection: Option<PgPool>
}