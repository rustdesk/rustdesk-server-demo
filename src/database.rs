use async_trait::async_trait;
use hbb_common::{log, ResultType};
use sqlx::{
    sqlite::SqliteConnectOptions, ConnectOptions, Connection, Error as SqlxError, SqliteConnection,
};
use std::{ops::DerefMut, str::FromStr, sync::atomic::AtomicUsize};
//use sqlx::postgres::PgPoolOptions;
//use sqlx::mysql::MySqlPoolOptions;

pub type Pool = deadpool::managed::Pool<DbPool>;

pub struct DbPool {
    url: String,
    recycle_count: AtomicUsize,
}
unsafe impl Sync for DbPool {}
unsafe impl Send for DbPool {}

#[async_trait]
impl deadpool::managed::Manager for DbPool {
    type Type = SqliteConnection;
    type Error = SqlxError;

    fn create(&self) -> impl std::future::Future<Output = Result<Self::Type, Self::Error>> + Send {
        SqliteConnection::connect(&self.url)
    }
    
    fn detach(&self, _obj: &mut Self::Type) {}
    
    fn recycle(
        &self,
        conn: &mut Self::Type,
        metrics: &deadpool_sqlite::Metrics,
    ) -> impl std::future::Future<Output = deadpool::managed::RecycleResult<Self::Error>> + Send {
        async move {
            match conn.ping().await {
                Ok(a) => { Ok(()) }
                Err(e) => { Err(deadpool::managed::RecycleError::from(e)) }
            }
        }
    }
}

#[derive(Clone)]
pub struct Database {
    pool: Pool
}

#[derive(Default, Debug)]
pub struct DbPeer {
    pub guid: Vec<u8>,
    pub id: String,
    pub last_reg_time: hbb_common::chrono::NaiveDateTime,
    pub uuid: Vec<u8>,
    pub pk: Vec<u8>,
    pub user: Option<Vec<u8>>,
    pub info: String,
    pub status: Option<i64>,
}

impl Database {
    pub async fn new(url: &str) -> ResultType<Database> {
        if !std::path::Path::new(url).exists() {
            std::fs::File::create(url).ok();
        }
        let n: usize = std::env::var("MAX_DATABASE_CONNECTIONS")
            .unwrap_or_else(|_| "1".to_owned())
            .parse()
            .unwrap_or(1);
        log::debug!("MAX_DATABASE_CONNECTIONS={}", n);
        // let cfg = deadpool_sqlite::Config::new(url.to_owned());
        // let pool = cfg.create_pool(deadpool::Runtime::Tokio1).unwrap();
        let mgr = DbPool {
            url: url.clone().to_string(),
            recycle_count: AtomicUsize::new(0),
        };
        let pool = Pool::builder(mgr).max_size(n).build().unwrap();;
        let _ = pool.get().await?; // test
        let db = Database { pool };
        db.create_tables().await?;
        Ok(db)
    }

    async fn create_tables(&self) -> ResultType<()> {
        // sqlx::query!(
        //     "
        //     create table if not exists peer (
        //         guid blob primary key not null,
        //         id varchar(100) not null,
        //         uuid blob not null,
        //         pk blob not null,
        //         created_at datetime not null default(current_timestamp),
        //         last_reg_time datetime not null,
        //         user blob,
        //         status tinyint,
        //         note varchar(300),
        //         info text not null
        //     ) without rowid;
        //     create unique index if not exists index_peer_id on peer (id);
        //     create index if not exists index_peer_user on peer (user);
        //     create index if not exists index_peer_created_at on peer (created_at);
        //     create index if not exists index_peer_status on peer (status);
        // "
        // )
        // .execute(self.pool.get().await?.deref_mut())
        // .await?;
        sqlx::raw_sql("create table if not exists peer (
                guid blob primary key not null,
                id varchar(100) not null,
                uuid blob not null,
                pk blob not null,
                created_at datetime not null default(current_timestamp),
                last_reg_time datetime not null,
                user blob,
                status tinyint,
                note varchar(300),
                info text not null
            ) without rowid;
            create unique index if not exists index_peer_id on peer (id);
            create index if not exists index_peer_user on peer (user);
            create index if not exists index_peer_created_at on peer (created_at);
            create index if not exists index_peer_status on peer (status);")
        .execute(self.pool.get().await?.deref_mut())
        .await?;
        Ok(())
    }

    pub async fn get_peer(&self, id: &str) -> ResultType<Option<DbPeer>> {
        Ok(sqlx::query_as!(
            DbPeer,
            "select guid, id, uuid, pk, last_reg_time, user, status, info from peer where id = ?",
            id
        )
        .fetch_optional(self.pool.get().await?.deref_mut())
        .await?)
        // Ok(None)
    }

    pub async fn insert_peer(
        &self,
        id: &str,
        last_reg_time: hbb_common::chrono::NaiveDateTime,
        uuid: &[u8],
        pk: &[u8],
        info: &str,
    ) -> ResultType<Vec<u8>> {
        let guid = uuid::Uuid::new_v4().as_bytes().to_vec();
        sqlx::query!(
            "insert into peer(guid, id, last_reg_time, uuid, pk, info) values(?, ?, ?, ?, ?, ?)",
            guid,
            id,
            last_reg_time,
            uuid,
            pk,
            info
        )
        .execute(self.pool.get().await?.deref_mut())
        .await?;
        Ok(guid)
    }

    pub async fn update_last_reg_time(
        &self,
        guid: Vec<u8>,
    ) -> ResultType<()> {
        let naive_datetime = hbb_common::chrono::Local::now().naive_local();
        sqlx::query!(
            "update peer set last_reg_time=? where guid=?",
            naive_datetime,
            guid
        )
        .execute(self.pool.get().await?.deref_mut())
        .await?;
        Ok(())
    }

    pub async fn sync_mem_to_db(
        &self,
        guid: Vec<u8>,
        last_reg_time: hbb_common::chrono::NaiveDateTime,
    ) -> ResultType<()> {
        sqlx::query!(
            "update peer set last_reg_time=? where guid=?",
            last_reg_time,
            guid
        )
        .execute(self.pool.get().await?.deref_mut())
        .await?;
        Ok(())
    }

    pub async fn update_pk(
        &self,
        guid: &Vec<u8>,
        id: &str,
        last_reg_time: hbb_common::chrono::NaiveDateTime,
        pk: &[u8],
        info: &str,
    ) -> ResultType<()> {
        sqlx::query!(
            "update peer set id=?, last_reg_time=?, pk=?, info=? where guid=?",
            id,
            last_reg_time,
            pk,
            info,
            guid
        )
        .execute(self.pool.get().await?.deref_mut())
        .await?;
        Ok(())
    }

    pub async fn delete_table(&self) -> ResultType<()> {
        sqlx::query!(
            "DROP TABLE IF EXISTS peer"
        )
        .execute(self.pool.get().await?.deref_mut())
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use hbb_common::tokio;
    #[test]
    fn test_insert() {
        insert();
    }
    #[tokio::main(flavor = "multi_thread")]
    async fn insert() {
        let db = super::Database::new("test.sqlite3").await.unwrap();
        let mut jobs = vec![];
        for i in 0..10000 {
            let cloned = db.clone();
            let id = i.to_string();
            let a = tokio::spawn(async move {
                let empty_vec = Vec::new();
                let naive_datetime = chrono::Local::now().naive_local();
                cloned
                    .insert_peer(&id, naive_datetime, &empty_vec, &empty_vec, "")
                    .await
                    .unwrap();
            });
            jobs.push(a);
        }
        for i in 0..10000 {
            let cloned = db.clone();
            let id = i.to_string();
            let a = tokio::spawn(async move {
                cloned.get_peer(&id).await.unwrap();
            });
            jobs.push(a);
        }
        hbb_common::futures::future::join_all(jobs).await;
    }

    #[test]
    fn test_delete() {
        delete();
    }
    #[tokio::main(flavor = "multi_thread")]
    async fn delete() {
        let db = super::Database::new("test.sqlite3").await.unwrap();
        let ret = db.delete_table().await;
    }

    #[test]
    fn test_create() {
        create();
    }
    #[tokio::main(flavor = "multi_thread")]
    async fn create() {
        let db = super::Database::new("db_v2.sqlite3").await.unwrap();
    }
}
