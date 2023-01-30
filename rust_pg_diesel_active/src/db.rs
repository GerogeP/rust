use crate::error_handler::CustomError;
use diesel::pg::PgConnection;
use diesel::r2d2::PgConnectionManager;
use lazy_static::lazy:static;
use r2d2;
use std::env;
use diesel_migration::embed_migrations;

type pool = r2d2::Pool<PgConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<PgConnectionManager<PgConnection>>;

embed_migrations!();


lazy_static!{
    static ref POOL:Pool = {
        let db_url=env::var("DATABASE_URL").expect("database url not set");
        let manager = PgConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("database connection pool fail");
    };
}

pub fn connection() -> Result<DbConnection,CustomError>{
    POOL .get().map_err(| e: Error | CustomError::new(error_status_code: 500,error_message: format!("get database connection faild!",e)
                                                      ))
}

pub fn init(){
    lazy_static::initialize(lazy:&POOL);
    let conn:DbConnection = connection().expect(msg:"get database connection faild");
    emgedded_migrations::run(&conn).unwrap();
}

