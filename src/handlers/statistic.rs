use axum::{extract::State, response::IntoResponse, Json};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::{error::AppError, models::view, state::AppState};

#[derive(Debug, Serialize)]
struct MemInfo {
    /// 当前进程ID
    pid: u32,
    /// 进程名
    process: String,
    /// 当前内存占用（字节）
    memory_bytes: u64,
}

/// 获取运行线程数近似值（对齐Go的 NumGoroutine 能力）
pub async fn routines() -> impl IntoResponse {
    let routines = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    Json(serde_json::json!(routines))
}

/// 获取内存占用信息
pub async fn mem() -> impl IntoResponse {
    let pid = std::process::id();
    let process = std::env::current_exe()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_else(|| "rustblog".to_string());

    // 尝试通过Windows API / procfs 统一能力前，先返回稳定可用结构。
    let info = MemInfo {
        pid,
        process,
        memory_bytes: 0,
    };
    Json(serde_json::json!(info))
}

/// 获取全局访问量（name=all）
pub async fn views(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let row = view::Entity::find()
        .filter(view::Column::Name.eq("all"))
        .one(&state.db)
        .await?;
    Ok(Json(serde_json::json!(row.map(|v| v.view).unwrap_or(0))))
}

/// 获取当日访问量（Go版本当前固定返回0）
pub async fn daily() -> impl IntoResponse {
    Json(serde_json::json!(0))
}
