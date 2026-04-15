use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Schema, Statement};

use crate::{config::AppConfig, error::AppError, models};

/// 初始化数据库连接
pub async fn init_db(cfg: &AppConfig) -> Result<DatabaseConnection, AppError> {
    let mut options = ConnectOptions::new(cfg.database.url.clone());
    options.sqlx_logging(cfg.debug);
    let db = Database::connect(options).await?;
    migrate(&db).await?;
    Ok(db)
}

/// 执行基础表迁移
pub async fn migrate(db: &DatabaseConnection) -> Result<(), AppError> {
    // 先做旧库兼容（重命名旧表、补齐缺失列），再走标准建表。
    migrate_compat(db).await?;

    let backend = db.get_database_backend();
    let schema = Schema::new(backend);
    let stmts = vec![
        backend.build(&schema.create_table_from_entity(models::post::Entity).if_not_exists().to_owned()),
        backend.build(&schema.create_table_from_entity(models::admin::Entity).if_not_exists().to_owned()),
        backend.build(&schema.create_table_from_entity(models::message::Entity).if_not_exists().to_owned()),
        backend.build(&schema.create_table_from_entity(models::tag::Entity).if_not_exists().to_owned()),
        backend.build(&schema.create_table_from_entity(models::comment::Entity).if_not_exists().to_owned()),
        backend.build(&schema.create_table_from_entity(models::view::Entity).if_not_exists().to_owned()),
        backend.build(&schema.create_table_from_entity(models::like::Entity).if_not_exists().to_owned()),
        backend.build(&schema.create_table_from_entity(models::share::Entity).if_not_exists().to_owned()),
    ];

    for stmt in stmts {
        db.execute(stmt).await?;
    }
    Ok(())
}

/// 兼容旧数据库结构（表名与字段）
async fn migrate_compat(db: &DatabaseConnection) -> Result<(), AppError> {
    // 兼容历史表名（含 db_ 前缀、复数形式）
    rename_table_if_needed(db, "db_blog_post", "blog_post").await?;
    rename_table_if_needed(db, "blog_posts", "blog_post").await?;
    rename_table_if_needed(db, "db_blog_share", "blog_share").await?;
    rename_table_if_needed(db, "blog_shares", "blog_share").await?;
    rename_table_if_needed(db, "db_blog_admin", "blog_admin").await?;
    rename_table_if_needed(db, "db_blog_comments", "blog_comments").await?;
    rename_table_if_needed(db, "db_blog_likes", "blog_likes").await?;
    rename_table_if_needed(db, "db_blog_messages", "blog_messages").await?;
    rename_table_if_needed(db, "db_blog_tags", "blog_tags").await?;
    rename_table_if_needed(db, "db_blog_views", "blog_views").await?;

    // 为所有业务表补齐通用审计字段
    let common_tables = [
        "blog_post",
        "blog_admin",
        "blog_comments",
        "blog_likes",
        "blog_messages",
        "blog_share",
        "blog_tags",
        "blog_views",
    ];
    for table in common_tables {
        // 主键列兼容：旧库可能只有 id，没有 primary_id。
        if table_exists(db, table).await?
            && !column_exists(db, table, "primary_id").await?
            && column_exists(db, table, "id").await?
        {
            add_column_if_missing(db, table, "primary_id", "INTEGER NOT NULL DEFAULT 0").await?;
            exec_sql(
                db,
                &format!(
                    "UPDATE \"{}\" SET primary_id = id WHERE primary_id = 0 OR primary_id IS NULL;",
                    table
                ),
            )
            .await?;
        }

        add_column_if_missing(
            db,
            table,
            "create_at",
            "TEXT NOT NULL DEFAULT ''",
        )
        .await?;
        add_column_if_missing(
            db,
            table,
            "update_at",
            "TEXT NOT NULL DEFAULT ''",
        )
        .await?;
        add_column_if_missing(db, table, "create_by", "INTEGER NOT NULL DEFAULT 0").await?;
        add_column_if_missing(db, table, "update_by", "INTEGER NOT NULL DEFAULT 0").await?;

        // SQLite ALTER TABLE ADD COLUMN 不支持表达式默认值，补列后再回填当前时间。
        exec_sql(
            db,
            &format!(
                "UPDATE \"{}\" SET create_at = datetime('now') WHERE create_at IS NULL OR create_at = '';",
                table
            ),
        )
        .await?;
        exec_sql(
            db,
            &format!(
                "UPDATE \"{}\" SET update_at = datetime('now') WHERE update_at IS NULL OR update_at = '';",
                table
            ),
        )
        .await?;
    }

    // blog_post 特有字段兼容
    add_column_if_missing(db, "blog_post", "date_plus", "TEXT NOT NULL DEFAULT ''").await?;
    add_column_if_missing(db, "blog_post", "update_date", "TEXT NOT NULL DEFAULT ''").await?;
    add_column_if_missing(db, "blog_post", "pin", "INTEGER NOT NULL DEFAULT 0").await?;
    add_column_if_missing(db, "blog_post", "lock", "INTEGER NOT NULL DEFAULT 0").await?;
    add_column_if_missing(db, "blog_post", "categories", "TEXT NOT NULL DEFAULT ''").await?;

    // 旧库列名 abstract 兼容新模型列 abstract_text（若 abstract_text 不存在则补齐并回填）
    if table_exists(db, "blog_post").await?
        && !column_exists(db, "blog_post", "abstract_text").await?
        && column_exists(db, "blog_post", "abstract").await?
    {
        add_column_if_missing(db, "blog_post", "abstract_text", "TEXT NOT NULL DEFAULT ''").await?;
        exec_sql(
            db,
            "UPDATE blog_post SET abstract_text = abstract WHERE abstract_text = '';",
        )
        .await?;
    }

    Ok(())
}

/// 目标表不存在且源表存在时，执行重命名
async fn rename_table_if_needed(
    db: &DatabaseConnection,
    from: &str,
    to: &str,
) -> Result<(), AppError> {
    if !table_exists(db, to).await? && table_exists(db, from).await? {
        exec_sql(
            db,
            &format!("ALTER TABLE \"{}\" RENAME TO \"{}\";", from, to),
        )
        .await?;
    }
    Ok(())
}

/// 表存在时补齐缺失列
async fn add_column_if_missing(
    db: &DatabaseConnection,
    table: &str,
    column: &str,
    definition: &str,
) -> Result<(), AppError> {
    if !table_exists(db, table).await? {
        return Ok(());
    }
    if !column_exists(db, table, column).await? {
        exec_sql(
            db,
            &format!(
                "ALTER TABLE \"{}\" ADD COLUMN \"{}\" {};",
                table, column, definition
            ),
        )
        .await?;
    }
    Ok(())
}

/// 判断表是否存在
async fn table_exists(db: &DatabaseConnection, table: &str) -> Result<bool, AppError> {
    let sql = format!(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='{}' LIMIT 1;",
        table
    );
    let row = db
        .query_one(Statement::from_string(DbBackend::Sqlite, sql))
        .await?;
    Ok(row.is_some())
}

/// 判断列是否存在
async fn column_exists(
    db: &DatabaseConnection,
    table: &str,
    column: &str,
) -> Result<bool, AppError> {
    let sql = format!("PRAGMA table_info('{}');", table);
    let rows = db
        .query_all(Statement::from_string(DbBackend::Sqlite, sql))
        .await?;
    for row in rows {
        let name: String = row.try_get("", "name").unwrap_or_default();
        if name == column {
            return Ok(true);
        }
    }
    Ok(false)
}

/// 执行原生 SQL
async fn exec_sql(db: &DatabaseConnection, sql: &str) -> Result<(), AppError> {
    db.execute(Statement::from_string(DbBackend::Sqlite, sql.to_string()))
        .await?;
    Ok(())
}
