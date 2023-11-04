use std::{fs, ffi::OsStr};


pub fn get_files(config_path: String) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    if let Ok(metadata) = fs::metadata(&config_path) {
        if metadata.is_dir() {
            let entries = fs::read_dir(&config_path);
            match entries {
                Ok(entries) => {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let file_path = entry.path();
                            if file_path.is_file() {
                                if let Some(file_name) =
                                    file_path.file_name().and_then(OsStr::to_str)
                                {
                                    files.push(file_name.to_string());
                                }
                            } else if file_path.is_dir() {
                                // 如果需要递归获取子目录中的文件，可以在这里调用 get_files 函数
                                // 例如: files.extend(get_files(file_path.to_str().unwrap().to_string()));
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("无法读取目录: {}", e);
                }
            }
        } else {
            eprintln!("指定的路径不是一个目录");
        }
    } else {
        // 创建一个空目录
        fs::create_dir(&config_path);
        eprintln!("指定的路径不存在或发生错误,已经生成文件夹:{}", config_path);
    }

    files
}