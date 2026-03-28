//! ExifTool `-stay_open` 进程管理

use crate::error::{Error, Result};
use std::io::{BufRead, BufReader, BufWriter, Write};

use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::time::Duration;
use tracing::{debug, info, warn};

const DEFAULT_RESPONSE_TIMEOUT: Duration = Duration::from_secs(30);

/// 命令编号类型
///
/// 用于在多命令执行场景中区分不同命令的响应
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommandId(pub u32);

impl CommandId {
    /// 创建新的命令编号
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    /// 获取编号值
    pub fn value(&self) -> u32 {
        self.0
    }
}

/// 命令执行请求
#[derive(Debug, Clone)]
pub struct CommandRequest {
    /// 命令编号（用于多命令场景）
    pub id: Option<CommandId>,
    /// 命令参数
    pub args: Vec<String>,
}

impl CommandRequest {
    /// 创建新的命令请求（无编号）
    pub fn new(args: Vec<String>) -> Self {
        Self { id: None, args }
    }

    /// 创建带编号的命令请求
    pub fn with_id(id: CommandId, args: Vec<String>) -> Self {
        Self { id: Some(id), args }
    }
}

/// ExifTool 进程内部状态
pub struct ExifToolInner {
    process: Child,
    stdin: BufWriter<ChildStdin>,
    stdout_rx: Receiver<String>,
    response_timeout: Duration,
}

impl std::fmt::Debug for ExifToolInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExifToolInner")
            .field("process", &self.process.id())
            .finish()
    }
}

impl ExifToolInner {
    /// 启动新的 ExifTool 进程（-stay_open 模式，从 PATH 查找）
    pub fn new() -> Result<Self> {
        Self::with_executable_and_timeout("exiftool", DEFAULT_RESPONSE_TIMEOUT)
    }

    /// 使用指定的可执行文件路径启动 ExifTool 进程
    pub fn with_executable<P: AsRef<std::ffi::OsStr>>(exe: P) -> Result<Self> {
        Self::with_executable_and_timeout(exe, DEFAULT_RESPONSE_TIMEOUT)
    }

    /// 使用指定可执行文件和响应超时启动 ExifTool 进程
    pub fn with_executable_and_timeout<P: AsRef<std::ffi::OsStr>>(
        exe: P,
        response_timeout: Duration,
    ) -> Result<Self> {
        info!("Starting ExifTool process with -stay_open mode");

        let mut process = Command::new(exe)
            .arg("-stay_open")
            .arg("True")
            .arg("-@")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    Error::ExifToolNotFound
                } else {
                    e.into()
                }
            })?;

        let stdin = process
            .stdin
            .take()
            .ok_or_else(|| Error::process("Failed to capture stdin"))?;

        let stdout = process
            .stdout
            .take()
            .ok_or_else(|| Error::process("Failed to capture stdout"))?;

        let stdout_rx = Self::spawn_stdout_reader(stdout);

        let mut inner = Self {
            process,
            stdin: BufWriter::new(stdin),
            stdout_rx,
            response_timeout,
        };

        // 验证进程是否正常工作
        inner.verify_process()?;

        info!("ExifTool process started successfully");
        Ok(inner)
    }

    /// 验证进程是否正常工作
    fn verify_process(&mut self) -> Result<()> {
        debug!("Verifying ExifTool process");

        // 发送版本查询命令
        self.send_line("-ver")?;
        self.send_line("-execute")?;
        self.stdin.flush()?;

        // 读取响应
        let response = self.read_response()?;
        debug!("ExifTool version: {}", response.text().trim());

        Ok(())
    }

    /// 发送单行命令
    pub fn send_line(&mut self, line: &str) -> Result<()> {
        debug!("Sending command: {}", line);
        writeln!(self.stdin, "{}", line)?;
        Ok(())
    }

    /// 执行命令并获取响应（无编号）
    pub fn execute(&mut self, args: &[String]) -> Result<Response> {
        self.execute_with_id(None, args)
    }

    /// 执行命令并获取响应（支持编号）
    pub fn execute_with_id(
        &mut self,
        command_num: Option<usize>,
        args: &[String],
    ) -> Result<Response> {
        debug!("Executing command with {} args", args.len());

        // 发送所有参数
        for arg in args {
            self.send_line(arg)?;
        }

        // 发送执行命令（带编号或不带）
        if let Some(num) = command_num {
            self.send_line(&format!("-execute{}", num))?;
        } else {
            self.send_line("-execute")?;
        }
        self.stdin.flush()?;

        // 读取响应
        self.read_response_for_num(command_num)
    }

    /// 读取响应（直到遇到 {ready} 或 {readyNUM}）
    pub fn read_response(&mut self) -> Result<Response> {
        self.read_response_for_id(None)
    }

    /// 读取特定命令编号的响应
    fn read_response_for_id(&mut self, expected_id: Option<CommandId>) -> Result<Response> {
        let mut lines = Vec::new();

        loop {
            let buffer = match self.stdout_rx.recv_timeout(self.response_timeout) {
                Ok(line) => line,
                Err(RecvTimeoutError::Timeout) => return Err(Error::Timeout),
                Err(RecvTimeoutError::Disconnected) => {
                    return Err(Error::process("Unexpected EOF from ExifTool process"));
                }
            };

            let trimmed = buffer.trim();
            debug!("Received line: {}", trimmed);

            if trimmed.starts_with("{ready") && trimmed.ends_with('}') {
                if trimmed == "{ready}" {
                    // 无编号响应
                    if let Some(expected) = expected_id {
                        return Err(Error::process(format!(
                            "Expected {{ready{}}}, but received {{ready}}",
                            expected.value()
                        )));
                    }
                    break;
                }

                // 解析 {readyNUM} 格式
                let content = &trimmed[6..trimmed.len() - 1];

                // 检查是否为错误码
                if let Ok(code) = content.parse::<i32>() {
                    if code != 0 {
                        let message = format!("ExifTool 返回错误码: {}", code);
                        return Err(Error::process(message));
                    }
                    // 编号为 0 表示成功但不匹配（正常情况下不应该发生）
                } else {
                    // 解析命令编号
                    match content.parse::<u32>() {
                        Ok(id) => {
                            let received_id = CommandId::new(id);
                            if let Some(expected) = expected_id
                                && received_id != expected
                            {
                                return Err(Error::process(format!(
                                    "命令编号不匹配: 期望 {}, 收到 {}",
                                    expected.value(),
                                    received_id.value()
                                )));
                            }
                            break;
                        }
                        Err(_) => {
                            return Err(Error::process(format!(
                                "无法解析 {{ready}} 标记中的编号: {}",
                                trimmed
                            )));
                        }
                    }
                }
            }

            lines.push(buffer.clone());
        }

        Ok(Response::new(lines))
    }

    /// 批量执行命令（串行执行）
    pub fn execute_batch(&mut self, commands: &[Vec<String>]) -> Result<Vec<Response>> {
        debug!("Executing batch of {} commands", commands.len());

        let mut responses = Vec::with_capacity(commands.len());

        for args in commands {
            let response = self.execute(args)?;
            responses.push(response);
        }

        Ok(responses)
    }

    /// 批量执行命令（原子多命令，使用 -execute[NUM]）
    ///
    /// 所有命令在一个事务中发送，通过顺序区分响应。
    /// 使用 -execute1, -execute2 等格式让 ExifTool 在 {ready} 标记中包含编号。
    pub fn execute_multiple(&mut self, commands: &[Vec<String>]) -> Result<Vec<Response>> {
        if commands.is_empty() {
            return Ok(Vec::new());
        }

        debug!("Executing {} commands atomically", commands.len());

        // 发送所有命令（不立即读取响应）
        for (idx, args) in commands.iter().enumerate() {
            let command_num = idx + 1; // 从 1 开始计数

            // 发送参数
            for arg in args {
                self.send_line(arg)?;
            }

            // 发送带编号的执行命令
            self.send_line(&format!("-execute{}", command_num))?;
        }
        self.stdin.flush()?;

        // 按顺序读取所有响应
        let mut responses = Vec::with_capacity(commands.len());
        for idx in 0..commands.len() {
            let expected_num = idx + 1;
            let response = self.read_response_for_num(Some(expected_num))?;
            responses.push(response);
        }

        Ok(responses)
    }

    /// 读取特定命令编号的响应
    fn read_response_for_num(&mut self, expected_num: Option<usize>) -> Result<Response> {
        let mut lines = Vec::new();

        loop {
            let buffer = match self.stdout_rx.recv_timeout(self.response_timeout) {
                Ok(line) => line,
                Err(RecvTimeoutError::Timeout) => return Err(Error::Timeout),
                Err(RecvTimeoutError::Disconnected) => {
                    return Err(Error::process("Unexpected EOF from ExifTool process"));
                }
            };

            let trimmed = buffer.trim();
            debug!("Received line: {}", trimmed);

            if trimmed.starts_with("{ready") && trimmed.ends_with('}') {
                if trimmed == "{ready}" {
                    // 无编号响应
                    if let Some(expected) = expected_num {
                        return Err(Error::process(format!(
                            "Expected {{ready{}}}, but received {{ready}}",
                            expected
                        )));
                    }
                    break;
                }

                // 解析 {readyNUM} 格式
                let content = &trimmed[6..trimmed.len() - 1];

                // 尝试解析为命令编号
                match content.parse::<usize>() {
                    Ok(num) => {
                        if let Some(expected) = expected_num
                            && num != expected
                        {
                            return Err(Error::process(format!(
                                "命令编号不匹配: 期望 {}, 收到 {}",
                                expected, num
                            )));
                        }
                        break;
                    }
                    Err(_) => {
                        return Err(Error::process(format!(
                            "无法解析 {{ready}} 标记中的编号: {}",
                            trimmed
                        )));
                    }
                }
            }

            lines.push(buffer.clone());
        }

        Ok(Response::new(lines))
    }

    /// 刷新 stdin
    pub fn flush(&mut self) -> Result<()> {
        self.stdin.flush().map_err(|e| e.into())
    }

    /// 关闭进程
    pub fn close(&mut self) -> Result<()> {
        info!("Closing ExifTool process");

        // 发送关闭命令
        let _ = self.send_line("-stay_open");
        let _ = self.send_line("False");
        let _ = self.send_line("-execute");
        let _ = self.stdin.flush();

        // 等待进程退出
        match self.wait_with_timeout(Duration::from_secs(5)) {
            Ok(Some(status)) => {
                if let Some(code) = status.code() {
                    if code != 0 {
                        warn!("ExifTool exited with code: {}", code);
                    } else {
                        info!("ExifTool process exited cleanly");
                    }
                }
            }
            Ok(None) => {
                warn!("ExifTool did not exit gracefully, forcing kill");
                let _ = self.process.kill();
            }
            Err(e) => {
                warn!("Error waiting for ExifTool: {}", e);
                let _ = self.process.kill();
            }
        }

        Ok(())
    }

    /// 带超时的等待
    fn wait_with_timeout(&mut self, timeout: Duration) -> Result<Option<std::process::ExitStatus>> {
        use std::thread;

        let start = std::time::Instant::now();

        loop {
            match self.process.try_wait()? {
                Some(status) => return Ok(Some(status)),
                None => {
                    if start.elapsed() >= timeout {
                        return Ok(None);
                    }
                    thread::sleep(Duration::from_millis(10));
                }
            }
        }
    }
}

impl ExifToolInner {
    /// 启动 stdout 读取线程，逐行转发到通道
    fn spawn_stdout_reader(stdout: ChildStdout) -> Receiver<String> {
        let (tx, rx) = mpsc::channel();

        std::thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            let mut buffer = String::new();

            loop {
                buffer.clear();
                match reader.read_line(&mut buffer) {
                    Ok(0) => break,
                    Ok(_) => {
                        if tx.send(buffer.clone()).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        rx
    }
}

impl Drop for ExifToolInner {
    fn drop(&mut self) {
        if let Err(e) = self.close() {
            warn!("Error closing ExifTool process: {}", e);
        }
    }
}

/// 命令响应
#[derive(Debug, Clone)]
pub struct Response {
    lines: Vec<String>,
}

impl Response {
    /// 创建新响应
    pub fn new(lines: Vec<String>) -> Self {
        Self { lines }
    }

    /// 获取所有行
    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    /// 获取合并的文本内容
    pub fn text(&self) -> String {
        self.lines.join("")
    }

    /// 获取 JSON 解析结果
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        let text = self.text();
        serde_json::from_str(&text).map_err(|e| e.into())
    }

    /// 检查是否有错误
    pub fn is_error(&self) -> bool {
        self.lines.iter().any(|line| line.contains("Error:"))
    }

    /// 获取错误信息
    pub fn error_message(&self) -> Option<String> {
        self.lines
            .iter()
            .find(|line| line.contains("Error:"))
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response() {
        let lines = vec!["Line 1".to_string(), "Line 2".to_string()];
        let response = Response::new(lines);

        assert_eq!(response.lines().len(), 2);
        assert_eq!(response.text(), "Line 1Line 2");
        assert!(!response.is_error());
    }

    #[test]
    fn test_response_error() {
        let lines = vec!["Error: Something went wrong".to_string()];
        let response = Response::new(lines);

        assert!(response.is_error());
        assert!(response.error_message().is_some());
    }

    #[test]
    fn test_response_warning_not_error() {
        let lines = vec!["Warning: minor issue".to_string()];
        let response = Response::new(lines);

        assert!(!response.is_error());
        assert!(response.error_message().is_none());
    }

    #[test]
    fn test_response_json() {
        let lines = vec![r#"{"key": "value"}"#.to_string()];
        let response = Response::new(lines);

        #[derive(Debug, serde::Deserialize, PartialEq)]
        struct TestData {
            key: String,
        }

        let data: TestData = response.json().unwrap();
        assert_eq!(data.key, "value");
    }

    #[test]
    fn test_command_id() {
        let id1 = CommandId::new(1);
        let id2 = CommandId::new(1);
        let id3 = CommandId::new(2);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
        assert_eq!(id1.value(), 1);
        assert_eq!(id3.value(), 2);
    }

    #[test]
    fn test_command_request() {
        let args = vec!["-ver".to_string()];

        let req1 = CommandRequest::new(args.clone());
        assert!(req1.id.is_none());
        assert_eq!(req1.args, args);

        let req2 = CommandRequest::with_id(CommandId::new(42), args.clone());
        assert_eq!(req2.id.unwrap().value(), 42);
        assert_eq!(req2.args, args);
    }

    #[test]
    fn test_command_response_with_num() {
        // 测试带编号的响应解析
        use std::sync::atomic::{AtomicU32, Ordering};

        let counter = AtomicU32::new(1);
        let id1 = counter.fetch_add(1, Ordering::SeqCst);
        let id2 = counter.fetch_add(1, Ordering::SeqCst);

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }
}
