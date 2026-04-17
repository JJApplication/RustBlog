use axum::{
    extract::{Multipart, Query, State},
    response::IntoResponse,
    Json,
};
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter,
};
use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    error::AppError,
    handlers::common::ok,
    models::post,
    state::AppState,
    utils::content_to_abstract::content_to_abs,
};

/// 获取文章查询参数
#[derive(Debug, Deserialize)]
pub struct GetPostQuery {
    /// 文章名，可选
    pub name: Option<String>,
    /// 页码，从1开始
    pub page: Option<usize>,
    /// 每页数量
    #[serde(rename = "pageSize")]
    pub page_size: Option<usize>,
}

/// 更新文章查询参数
#[derive(Debug, Deserialize)]
pub struct UpdatePostQuery {
    /// 更新类型：file/args/editor
    pub r#type: Option<String>,
    /// type=file 时的文章名
    pub name: Option<String>,
}

/// 导出文章查询参数
#[derive(Debug, Deserialize)]
pub struct ExportQuery {
    /// 文章名，可选
    pub name: Option<String>,
}

/// 删除文章请求体
#[derive(Debug, Deserialize)]
pub struct DeletePostBody {
    /// 文章名
    pub name: String,
}

/// 新增空白文章请求体
#[derive(Debug, Deserialize)]
pub struct AddPostBody {
    /// 文章名
    pub name: String,
    /// 标题
    pub title: Option<String>,
    /// 标签，空格分隔
    pub tags: Option<String>,
    /// 日期时间
    pub date: Option<String>,
}

/// type=args 请求体
#[derive(Debug, Deserialize)]
pub struct UpdateArgsBody {
    /// 原文章名
    pub name: String,
    /// 新文章名
    pub newname: Option<String>,
    /// 标题
    pub title: Option<String>,
    /// 日期
    pub date: Option<String>,
    /// 标签（空格分隔）
    pub tags: Option<String>,
    /// 置顶
    pub pin: Option<i32>,
    /// 锁定
    pub lock: Option<i32>,
}

/// type=editor 请求体
#[derive(Debug, Deserialize)]
pub struct UpdateEditorBody {
    /// 文章名
    pub name: String,
    /// 标题
    pub title: Option<String>,
    /// 标签
    pub tags: Option<String>,
    /// 内容
    pub content: String,
}

/// dashboard 获取文章（GET /api/dashboard/post）
pub async fn get(
    State(state): State<AppState>,
    Query(q): Query<GetPostQuery>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(name) = q.name {
        let row = post::Entity::find()
            .filter(post::Column::Name.eq(name))
            .one(&state.db)
            .await?;
        return Ok(Json(serde_json::json!({
            "code": 200,
            "msg": "success",
            "data": row
        })));
    }
    let page = q.page.unwrap_or(1).max(1);
    let page_size = q.page_size.unwrap_or(5).max(1);

    let rows = post::Entity::find().all(&state.db).await?;
    let total_len = rows.len();
    let start = (page - 1).saturating_mul(page_size);
    let list = rows
        .into_iter()
        .skip(start)
        .take(page_size)
        .collect::<Vec<_>>();

    Ok(Json(serde_json::json!({
        "code": 200,
        "msg": "success",
        "data": list,
        "len": total_len
    })))
}

/// 文章导出（POST /api/dashboard/post/export）
pub async fn export(
    State(state): State<AppState>,
    Query(q): Query<ExportQuery>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(name) = q.name {
        let row = post::Entity::find()
            .filter(post::Column::Name.eq(name))
            .one(&state.db)
            .await?;
        return Ok(ok(serde_json::json!(row)));
    }
    let rows = post::Entity::find().all(&state.db).await?;
    Ok(ok(serde_json::json!(rows)))
}

/// 新增空白文章（POST /api/dashboard/post/add）
pub async fn add(
    State(state): State<AppState>,
    Json(body): Json<AddPostBody>,
) -> Result<impl IntoResponse, AppError> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let date = body
        .date
        .clone()
        .unwrap_or_else(|| now.clone())
        .split(' ')
        .next()
        .unwrap_or("")
        .to_string();
    let title = body.title.unwrap_or_else(|| body.name.clone());
    let tags = body.tags.unwrap_or_default();

    let model = post::ActiveModel {
        name: Set(body.name),
        title: Set(title),
        date: Set(date),
        date_plus: Set(now.clone()),
        update_date: Set(now),
        abstract_text: Set(String::new()),
        content: Set(String::new()),
        tags: Set(tags),
        categories: Set(String::new()),
        pin: Set(0),
        lock: Set(Some(0)),
        ..Default::default()
    };
    model.insert(&state.db).await?;
    Ok(ok("success"))
}

/// 上传并写入文章（POST /api/dashboard/post/upload）
pub async fn upload(
    State(state): State<AppState>,
    multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let (file_name, content) = read_upload_md(multipart).await?;
    if !file_name.ends_with(".md") || content.len() >= 1024 * 100 {
        return Ok(ok("fail"));
    }
    if !check_article_ok(&content) {
        return Ok(ok("fail"));
    }

    let meta = parse_md_meta(&content);
    let name = meta.get("name").cloned().unwrap_or_else(|| "untitled".to_string());
    let title = meta.get("title").cloned().unwrap_or_else(|| name.clone());
    let tags = meta.get("tags").cloned().unwrap_or_default();
    let date = meta
        .get("date")
        .cloned()
        .unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());
    let date_plus = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let abs = meta.get("abstract").cloned().unwrap_or_default();
    let abstract_text = content_to_abs(&abs, &content, &state.config.app);

    if let Some(old) = post::Entity::find()
        .filter(post::Column::Name.eq(name.clone()))
        .one(&state.db)
        .await?
    {
        let mut active: post::ActiveModel = old.into();
        active.title = Set(title);
        active.date = Set(date.clone());
        active.date_plus = Set(date_plus.clone());
        active.update_date = Set(date_plus.clone());
        active.tags = Set(tags);
        active.abstract_text = Set(abstract_text);
        active.content = Set(content);
        active.update(&state.db).await?;
    } else {
        post::ActiveModel {
            name: Set(name),
            title: Set(title),
            date: Set(date),
            date_plus: Set(date_plus.clone()),
            update_date: Set(date_plus),
            abstract_text: Set(abstract_text),
            content: Set(content),
            tags: Set(tags),
            categories: Set(String::new()),
            pin: Set(0),
            lock: Set(Some(0)),
            ..Default::default()
        }
        .insert(&state.db)
        .await?;
    }
    Ok(ok("success"))
}

/// 上传解析回调（POST /api/dashboard/post/parse）
pub async fn parse(multipart: Multipart) -> Result<impl IntoResponse, AppError> {
    let (_, content) = read_upload_md(multipart).await?;
    let meta = parse_md_meta(&content);
    Ok(ok(meta))
}

/// 上传文件校验（POST /api/dashboard/post/check）
pub async fn check(multipart: Multipart) -> Result<impl IntoResponse, AppError> {
    let (_, content) = read_upload_md(multipart).await?;
    if check_article_ok(&content) {
        Ok(ok("success"))
    } else {
        Ok(ok("fail"))
    }
}

/// 文章更新（POST/PUT /api/dashboard/post）
pub async fn post(
    State(state): State<AppState>,
    Query(q): Query<UpdatePostQuery>,
    body: String,
) -> Result<impl IntoResponse, AppError> {
    update_post_impl(state, q, body).await
}

/// 文章更新（POST/PUT /api/dashboard/post）
pub async fn put(
    State(state): State<AppState>,
    Query(q): Query<UpdatePostQuery>,
    body: String,
) -> Result<impl IntoResponse, AppError> {
    update_post_impl(state, q, body).await
}

/// 删除文章（DELETE /api/dashboard/post）
pub async fn delete(
    State(state): State<AppState>,
    Json(body): Json<DeletePostBody>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(v) = post::Entity::find()
        .filter(post::Column::Name.eq(body.name))
        .one(&state.db)
        .await?
    {
        let active: post::ActiveModel = v.into();
        active.delete(&state.db).await?;
        return Ok(ok("success"));
    }
    Ok(ok("fail"))
}

async fn update_post_impl(
    state: AppState,
    q: UpdatePostQuery,
    body: String,
) -> Result<impl IntoResponse, AppError> {
    let Some(update_type) = q.r#type else {
        return Ok(ok("fail"));
    };

    if update_type == "args" {
        let d: UpdateArgsBody = serde_json::from_str(&body)
            .map_err(|_| AppError::BadRequest("parse body failed".to_string()))?;
        let Some(v) = post::Entity::find()
            .filter(post::Column::Name.eq(d.name))
            .one(&state.db)
            .await?
        else {
            return Ok(ok("fail"));
        };
        let mut active: post::ActiveModel = v.into();
        if let Some(n) = d.newname { active.name = Set(n); }
        if let Some(t) = d.title { active.title = Set(t); }
        if let Some(date) = d.date { active.date = Set(date); }
        if let Some(tags) = d.tags { active.tags = Set(tags); }
        if let Some(pin) = d.pin { active.pin = Set(pin); }
        if let Some(lock) = d.lock { active.lock = Set(Some(lock)); }
        active.update_date = Set(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        active.update(&state.db).await?;
        return Ok(ok("success"));
    }

    if update_type == "editor" {
        let d: UpdateEditorBody = serde_json::from_str(&body)
            .map_err(|_| AppError::BadRequest("parse body failed".to_string()))?;
        let Some(v) = post::Entity::find()
            .filter(post::Column::Name.eq(d.name))
            .one(&state.db)
            .await?
        else {
            return Ok(ok("fail"));
        };
        let mut active: post::ActiveModel = v.into();
        if let Some(t) = d.title { active.title = Set(t); }
        if let Some(tags) = d.tags { active.tags = Set(tags); }
        active.content = Set(d.content.clone());
        active.abstract_text = Set(content_to_abs("", &d.content, &state.config.app));
        active.update_date = Set(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        active.update(&state.db).await?;
        return Ok(ok("success"));
    }

    if update_type == "file" {
        if q.name.clone().unwrap_or_default().is_empty() {
            return Ok(ok("fail"));
        }
        // file 模式需要 multipart，当前端走该模式请使用 /post/upload，更符合旧逻辑。
        return Ok(ok("fail"));
    }

    Ok(ok("fail"))
}

async fn read_upload_md(mut multipart: Multipart) -> Result<(String, String), AppError> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("file parse failed: {e}")))?
    {
        if field.name() != Some("uploadmd") {
            continue;
        }
        let file_name = field.file_name().unwrap_or("upload.md").to_string();
        let data = field
            .bytes()
            .await
            .map_err(|e| AppError::BadRequest(format!("file parse failed: {e}")))?;
        let content = String::from_utf8_lossy(&data).to_string();
        return Ok((file_name, content));
    }
    Err(AppError::BadRequest("file upload failed".to_string()))
}

fn check_article_ok(content: &str) -> bool {
    let has_name = content.contains("name:");
    let has_title = content.contains("title:");
    let has_body = !content.trim().is_empty();
    has_body && (has_name || has_title)
}

fn parse_md_meta(content: &str) -> HashMap<String, String> {
    let mut m = HashMap::new();
    let lines: Vec<&str> = content.lines().collect();
    if lines.first().copied() != Some("---") {
        return m;
    }
    for line in lines.iter().skip(1) {
        if *line == "---" {
            break;
        }
        if let Some((k, v)) = line.split_once(':') {
            m.insert(k.trim().to_lowercase(), v.trim().to_string());
        }
    }
    m
}
