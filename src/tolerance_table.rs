use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

const DUMP_SQL: &str = include_str!("../data/dump.sql");

pub struct ToleranceTable {
    pub pool: Pool<SqliteConnectionManager>,
}

impl ToleranceTable {
    pub fn new() -> Self {
        ToleranceTable {
            pool: Self::init_pool(),
        }
    }
}

trait ToleranceTablePool {
    fn init_pool() -> Pool<SqliteConnectionManager>;
}

impl ToleranceTablePool for ToleranceTable {
    fn init_pool() -> Pool<SqliteConnectionManager> {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::new(manager).expect("Не удалось создать пул соединений");
        {
            let conn = pool.get().expect("Не удалось получить соединение из пула");
            conn.execute_batch(DUMP_SQL)
                .expect("Не удалось развернуть БД из dump.sql");
        }

        pool
    }
}
