use anyhow::Result;
use rswin32::ntexapi2::psinfo::query_system_process_information;

fn main() -> Result<()> {
    println!("正在查询系统进程信息...");

    let processes = query_system_process_information()?;

    println!("找到 {} 个进程:", processes.len());
    println!(
        "{:<10} {:<20} {:<15} {:<15}",
        "PID", "进程名", "线程数", "内存使用"
    );
    println!("{}", "-".repeat(70));

    for process in processes.iter().take(20) {
        // 只显示前20个进程
        let process_id = process.UniqueProcessId as usize;
        // 获取进程名
        let process_name = if process.ImageName.Length > 0 {
            let name_ptr = process.ImageName.Buffer;
            let name_len = process.ImageName.Length as usize / 2; // Unicode字符
            let name_slice =
                unsafe { std::slice::from_raw_parts(name_ptr as *const u16, name_len) };
            String::from_utf16_lossy(name_slice)
        } else {
            "System Idle Process".to_string()
        };
        // 格式化内存使用量
        let memory_usage = format_memory_size(process.WorkingSetSize as u64);
        println!(
            "{:<10} {:<20} {:<15} {:<15}",
            process_id, process_name, process.NumberOfThreads, memory_usage
        );
    }

    if processes.len() > 20 {
        println!("... 还有 {} 个进程未显示", processes.len() - 20);
    }

    Ok(())
}
// 格式化内存大小，将字节转换为可读格式
fn format_memory_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * 1024;
    const GB: u64 = 1024 * 1024 * 1024;

    if bytes == 0 {
        return "0 KB".to_string();
    } else if bytes < KB {
        return format!("{} B", bytes);
    } else if bytes < MB {
        return format!("{} KB", bytes / KB);
    } else if bytes < GB {
        return format!("{:.1} MB", bytes as f64 / MB as f64);
    } else {
        return format!("{:.1} GB", bytes as f64 / GB as f64);
    }
}
