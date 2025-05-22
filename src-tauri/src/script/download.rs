use std::path::Path;
use reqwest;
use tokio::io::AsyncWriteExt;
use futures::stream::StreamExt;
use tauri::Emitter;

#[tauri::command]
pub async fn download_file(id: u32, url: String, save_path: String, app: tauri::AppHandle) -> Result<(), String> {
    let client = reqwest::Client::new();
    let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
    let total_size = resp.content_length().ok_or("无法获取内容长度")?;
    let mut downloaded: u64 = 0;

    // 创建目录（如果不存在）
    if let Some(parent_dir) = Path::new(&save_path).parent() {
        tokio::fs::create_dir_all(parent_dir).await.map_err(|e| e.to_string())?;
    }

    // 创建文件
    let mut file = tokio::fs::File::create(&save_path).await.map_err(|e| e.to_string())?;

    let mut stream = resp.bytes_stream();
    let app_handle = app.clone();

    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|e| e.to_string())?;
        file.write_all(&chunk).await.map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;
        let progress = (downloaded as f64 / total_size as f64) * 100.0;
        let _ = app_handle.emit(&format!("download_{}", id), progress);
    }

    let _ = app_handle.emit(&format!("download_{}", id), true);

    Ok(())
}
