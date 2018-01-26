//! The connection module that handles databases via the diesel Connections
//! 

use connectors::DbConnection;

use diesel::connection::*;
use diesel::prelude::*;


pub struct DieselPGSQL {
    conn: PgConnection,
}

impl DbConnection for DieselPGSQL {

    fn new(route: &str) -> DieselPGSQL {
        let c: PgConnection = Connection::establish(route).unwrap();
        return DieselPGSQL {
            conn: c
        };
    }

    fn batch_exec(&mut self, sql: &str) {
        self.conn.batch_execute(sql).unwrap();
    }
}