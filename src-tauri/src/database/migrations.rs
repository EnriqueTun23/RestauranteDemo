use rusqlite::Connection;

const INITIAL_MIGRATION: &str =
    include_str!("../../migrations/001_initial.sql");

pub fn run_migrations(
    connection: &Connection,
) -> Result<(), rusqlite::Error> {
    connection.execute_batch(INITIAL_MIGRATION)?;
    Ok(())
}