use std::fs;
use std::io::Read;
use std::path::Path;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

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
            let path = Path::new("data/dump.sql");
            let mut file = fs::File::open(&path).expect("Не удалось открыть файл БД");
            let mut dump = String::new();
            file.read_to_string(&mut dump)
                .expect("Не удалось прочитать файл БД");
            conn.execute_batch(&dump)
                .expect("Не удалось развернуть БД из dump.sql");
        }

        pool
    }
}
