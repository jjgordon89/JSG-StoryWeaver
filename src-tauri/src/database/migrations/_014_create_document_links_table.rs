pub async fn up(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS document_links (
            id TEXT PRIMARY KEY NOT NULL,
            from_document_id TEXT NOT NULL,
            to_document_id TEXT NOT NULL,
            link_order INTEGER NOT NULL,
            created_at TEXT NOT NULL
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn down(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("DROP TABLE IF EXISTS document_links;")
        .execute(pool)
        .await?;
    Ok(())
}