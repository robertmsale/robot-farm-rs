use crate::db::{self, DbResult};
use openapi::models::{Feed, FeedLevel, FeedOrderField};
use sqlx::{QueryBuilder, Row, Sqlite};

pub struct FeedFilters {
    pub source: Option<String>,
    pub target: Option<String>,
    pub level: Option<FeedLevel>,
    pub order_by: Option<FeedOrderField>,
}

pub async fn list_feed(filters: FeedFilters) -> DbResult<Vec<Feed>> {
    let mut builder = QueryBuilder::<Sqlite>::new(
        "SELECT id, source, target, ts, level, text, raw, category FROM feed",
    );
    let mut has_clause = false;

    if let Some(source) = filters.source {
        builder
            .push(if has_clause { " AND " } else { " WHERE " })
            .push("source = ")
            .push_bind(source);
        has_clause = true;
    }

    if let Some(target) = filters.target {
        builder
            .push(if has_clause { " AND " } else { " WHERE " })
            .push("target = ")
            .push_bind(target);
        has_clause = true;
    }

    if let Some(level) = filters.level {
        builder
            .push(if has_clause { " AND " } else { " WHERE " })
            .push("level = ")
            .push_bind(level_to_str(level));
    }

    let order_field = filters.order_by.unwrap_or(FeedOrderField::Ts);
    builder
        .push(" ORDER BY ")
        .push(order_field_name(order_field));

    if matches!(order_field, FeedOrderField::Ts | FeedOrderField::Id) {
        builder.push(" DESC");
    }

    let rows = builder.build().fetch_all(db::pool()).await?;
    let feed = rows.into_iter().filter_map(row_to_feed).collect();
    Ok(feed)
}

pub async fn delete_feed() -> DbResult<()> {
    sqlx::query("DELETE FROM feed").execute(db::pool()).await?;
    Ok(())
}

fn level_to_str(level: FeedLevel) -> &'static str {
    match level {
        FeedLevel::Info => "info",
        FeedLevel::Warning => "warning",
        FeedLevel::Error => "error",
    }
}

fn parse_level(value: &str) -> Option<FeedLevel> {
    match value {
        "info" => Some(FeedLevel::Info),
        "warning" => Some(FeedLevel::Warning),
        "error" => Some(FeedLevel::Error),
        _ => None,
    }
}

fn order_field_name(field: FeedOrderField) -> &'static str {
    match field {
        FeedOrderField::Id => "id",
        FeedOrderField::Source => "source",
        FeedOrderField::Target => "target",
        FeedOrderField::Ts => "ts",
        FeedOrderField::Level => "level",
        FeedOrderField::Text => "text",
        FeedOrderField::Raw => "raw",
        FeedOrderField::Category => "category",
    }
}

fn row_to_feed(row: sqlx::sqlite::SqliteRow) -> Option<Feed> {
    let level: String = row.get("level");
    let level = parse_level(&level)?;
    Some(Feed {
        id: row.get("id"),
        source: row.get("source"),
        target: row.get("target"),
        ts: row.get("ts"),
        level,
        text: row.get("text"),
        raw: row.get("raw"),
        category: row.get("category"),
    })
}
