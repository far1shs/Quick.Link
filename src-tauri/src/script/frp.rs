use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use directories::ProjectDirs;
use tauri::Emitter;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

#[derive(Debug, Clone)]
struct ProcessInfo {
    logs: Vec<String>,
    pid: Option<u32>,
}

#[derive(Debug, Default)]
pub struct FrpState {
    processes: Arc<Mutex<HashMap<u32, ProcessInfo>>>,
}

lazy_static::lazy_static! {
    static ref FRP_STATE: FrpState = FrpState::default();
}

#[derive(serde::Serialize)]
pub struct Response {
    status: bool,
    message: String,
}

#[tauri::command]
pub async fn run_frp(
    id: u32,
    config: String,
    app: tauri::AppHandle,
) -> Result<Response, Response> {
    let parent_dir = ProjectDirs::from("", "", "icu.far1sh.app.quick-link")
        .map(|proj_dirs| proj_dirs.data_dir().parent().unwrap().to_path_buf())
        .unwrap_or_default();
    
    let mut cmd = Command::new(parent_dir.display().to_string());
    cmd.creation_flags(0x08000000);
    for arg in config.split_whitespace() {
        cmd.arg(arg);
    }
    cmd.arg("--disable-log-color");
    cmd.stdout(std::process::Stdio::piped());

    // 启动进程并获取 PID
    let mut child = cmd.spawn().map_err(|e| Response {
        status: false,
        message: format!("启动失败 {}", e),
    })?;
    let pid = child.id().ok_or(Response {
        status: false,
        message: "无法获取进程ID".to_string(),
    })?;
    let stdout = match child.stdout.take() {
        Some(os) => os,
        None => {
            return Err(Response {
                status: false,
                message: "无法获取标准输出".to_string(),
            })
        }
    };

    // 初始化进程日志存储
    FRP_STATE.processes.lock().unwrap().insert(
        id,
        ProcessInfo {
            logs: Vec::new(),
            pid: Some(pid),
        },
    );

    // 捕获 stdout 并异步读取每一行
    let processes = FRP_STATE.processes.clone();
    let app_handle = app.clone();
    tokio::spawn(async move {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            // 保存日志到对应进程
            let mut processes = processes.lock().unwrap();
            if let Some(info) = processes.get_mut(&id) {
                info.logs.push(line.clone());
            }

            // 发送事件给前端，channel 为 "frp_logs_${id}"
            let _ = app_handle.emit(&format!("frp_logs_{}", id), line);
        }
    });

    Ok(Response {
        status: true,
        message: "启动成功".to_string(),
    })
}

#[tauri::command]
pub async fn stop_frp(id: u32) -> Result<Response, Response> {
    // 先获取 PID，再移除或操作
    let (pid, exists) = {
        let processes = FRP_STATE.processes.lock().unwrap();
        if let Some(info) = processes.get(&id) {
            (info.pid, true)
        } else {
            (None, false)
        }
    };

    if !exists {
        return Ok(Response {
            status: false,
            message: "进程不存在".to_string(),
        });
    }

    if let Some(pid) = pid {
        #[cfg(target_os = "windows")]
        let output = Command::new("taskkill")
            .creation_flags(0x08000000)
            .arg("/F")
            .arg("/PID")
            .arg(pid.to_string())
            .output()
            .await;

        #[cfg(not(target_os = "windows"))]
        let output = Command::new("kill")
            .arg("-9")
            .arg(pid.to_string())
            .output()
            .await;

        if let Err(e) = output {
            return Ok(Response {
                status: false,
                message: format!("停止失败: {}", e),
            });
        }
    }

    // 确保删除记录
    let mut processes = FRP_STATE.processes.lock().unwrap();
    processes.remove(&id);

    Ok(Response {
        status: true,
        message: "进程已停止".to_string(),
    })
}

#[tauri::command]
pub async fn get_frp_logs(id: u32) -> Result<serde_json::Value, Response> {
    let logs = {
        let processes = FRP_STATE.processes.lock().unwrap();
        processes.get(&id).map(|info| info.logs.clone())
    };

    match logs {
        Some(logs) => Ok(serde_json::json!({
            "status": true,
            "data": logs,
        })),
        None => Ok(serde_json::json!({
            "status": false,
            "message": "进程不存在",
        })),
    }
}
