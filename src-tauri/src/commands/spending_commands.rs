use serde::Serialize;
use tauri::State;

use crate::db::queries;
use crate::db::DbState;
use crate::error::AppError;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpendingRate {
    id: i64,
    tool_name: String,
    rate_type: String,
    rate_value: f64,
    billing_period: Option<String>,
}

#[tauri::command]
pub fn get_spending_rates(db: State<DbState>) -> Result<Vec<SpendingRate>, AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;
    let rows = queries::get_spending_rates(&conn)?;
    Ok(rows
        .into_iter()
        .map(
            |(id, tool_name, rate_type, rate_value, billing_period)| SpendingRate {
                id,
                tool_name,
                rate_type,
                rate_value,
                billing_period,
            },
        )
        .collect())
}

#[tauri::command]
pub fn upsert_spending_rate(
    db: State<DbState>,
    tool_name: String,
    rate_type: String,
    rate_value: f64,
    billing_period: String,
) -> Result<(), AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;
    queries::upsert_spending_rate(&conn, &tool_name, &rate_type, rate_value, &billing_period)
}

#[tauri::command]
pub fn delete_spending_rate(db: State<DbState>, id: i64) -> Result<(), AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;
    queries::delete_spending_rate(&conn, id)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BudgetConfig {
    id: i64,
    period: String,
    limit_amount: f64,
    enabled: bool,
}

#[tauri::command]
pub fn get_budget_configs(db: State<DbState>) -> Result<Vec<BudgetConfig>, AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;
    let rows = queries::get_budget_configs(&conn)?;
    Ok(rows
        .into_iter()
        .map(|(id, period, limit_amount, enabled)| BudgetConfig {
            id,
            period,
            limit_amount,
            enabled,
        })
        .collect())
}

#[tauri::command]
pub fn set_budget(db: State<DbState>, period: String, limit_amount: f64) -> Result<(), AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;
    queries::upsert_budget(&conn, &period, limit_amount)
}
