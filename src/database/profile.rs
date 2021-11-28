use rusqlite::{Connection, Error, Result, params};

// The SQL for the queries
const SETUP_SQL: &str = include_str!("queries/setup.sql");
const SET_PROFILE_SQL: &str = include_str!("queries/set_profile.sql");
const GET_PROFILE_SQL: &str = include_str!("queries/get_profile.sql");

pub struct UserProfile {
    id: u64,
    catchphrase: Option<String>
}

impl UserProfile {
    pub fn new(id: u64, catchphrase: Option<String>) -> Self {
        Self { id, catchphrase }
    }
}

pub fn setup(conn: &Connection) -> Result<()> {
    conn.execute(SETUP_SQL, [])?;
    Ok(())
}

fn user_exists(id: u64) -> bool {
    let conn = Connection::open("db.db3").expect("Error getting a connection to the database");
    let res: Result<u64> = conn.query_row(
        GET_PROFILE_SQL,
        [id],
        |row| row.get(0)
    );
    match res {
        Ok(_) => true,
        Err(Error::QueryReturnedNoRows) => false,
        Err(why) => { panic!("Error executing statement: {}", why) }
    }
}

pub fn set_profile(profile: UserProfile) -> Result<usize> {
    let conn = Connection::open("db.db3").expect("Error getting a connection to the database");
    conn.execute(SET_PROFILE_SQL, params![profile.id, profile.catchphrase])
}

pub fn register(id: u64) {
    let profile = UserProfile::new(id, None);
    set_profile(profile).expect("Failed to set profile");
}

pub fn get_profile(id: u64) -> Option<String> {
    let conn = Connection::open("db.db3").expect("Error getting a connection to the database");
    if !user_exists(id) { register(id) };

    let res: Option<String> = conn.query_row(
        GET_PROFILE_SQL,
        [id.to_string()],
        |row| row.get(1)
    ).expect("Failed to get profile");

    res
}