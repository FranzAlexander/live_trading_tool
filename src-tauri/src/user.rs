use crate::model::AppState;

#[tauri::command]
pub async fn get_symbols(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    let res = state
        .client
        .get("http://127.0.0.1:3080/get_symbols")
        .send()
        .await
        .unwrap()
        .json::<Vec<String>>()
        .await
        .unwrap();

    Ok(res)
}
