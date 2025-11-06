use rusqlite::{ params, Connection, Result };

const DB_NAME: &str = "score.db";
const LEADER_BOARD_TABLE_NAME: &str = "leaderboard";
const ID_COLUMN: &str = "id";
const NAME_COLUMN: &str = "name";
const SCORE_COLUMN: &str = "score";

#[derive(Debug)]
pub struct LeaderboardRecord {
    name: String,
    score: u32,
}

/// Create Leaderboard Table if it doesn't exist, will only happen once
pub fn create_db_if_not_exists() -> Result<()> {
    let connection: Connection = Connection::open(DB_NAME)?;
    
    let create_table_query: String = format!(
        "create table if not exists {} ({} integer primary key autoincrement, {} text not null, {} integer not null)",
        LEADER_BOARD_TABLE_NAME,
        ID_COLUMN,
        NAME_COLUMN,
        SCORE_COLUMN
    );
    connection.execute(&create_table_query, []).unwrap();
    
    return Ok(());
}

/// Takes a player name and their score to add to the leaderboard table
pub fn insert_score(player_name: &str, score: u32) -> Result<()> {
    let connection: Connection = Connection::open(DB_NAME)?;
    
    let insert_sql: String = format!(
        "insert into {} ([{}], [{}]) values (?1, ?2)",
        LEADER_BOARD_TABLE_NAME,
        NAME_COLUMN,
        SCORE_COLUMN
    );
    connection.execute(&insert_sql, params![player_name, score]).unwrap();
    
    return Ok(());
}

/// Get all the records stored in the Leaderboard table
pub fn get_leaderboard() -> Result<Vec<LeaderboardRecord>, rusqlite::Error> {
    let connection: Connection = Connection::open(DB_NAME)?;
    
    let select_sql: String = format!(
        "select {NAME_COLUMN}, {SCORE_COLUMN} from {LEADER_BOARD_TABLE_NAME} order by {SCORE_COLUMN} desc"
    );
    let mut leaderboard_records = connection.prepare(&select_sql)?;
    let database_records = leaderboard_records.query_map([], |row| {
        Ok(LeaderboardRecord {
            name: row.get(0)?,
            score: row.get(1)?,
        })
    })?;
    
    let mut leaderboard_results: Vec<LeaderboardRecord> = Vec::new();
    for row in database_records {
        leaderboard_results.push(match row {
            row => row?,
        });
    }
    
    return Ok(leaderboard_results);
}

/// Clears the leaderboard table
pub fn clear_leaderboard() -> Result<()> {
    let connection: Connection = Connection::open(DB_NAME)?;
    
    let clear_sql: String = format!(
        "delete from {LEADER_BOARD_TABLE_NAME}"
    );    
    connection.execute(&clear_sql, []).unwrap();
    
    return Ok(());
}
