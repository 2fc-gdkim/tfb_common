use std::str::FromStr;
use std::fs;
use std::env;
use std::fs::create_dir_all;
use std::path::PathBuf;

//실행파일의 경로 리턴
pub fn get_exe_path() -> Option<PathBuf> {
    let exe = env::current_exe().unwrap();
    let dir = match exe.parent() {
        Some(v) => v,
        None => {
            eprintln!("Executable must be in some directory");
            return None
        },
    };
    Some(dir.to_path_buf())
}

//현재 실행파일의 경로아래 하위 폴더 
pub fn get_sub_folder(folder: &str, create : bool) -> PathBuf {
    let mut dir = get_exe_path().unwrap();
    dir.push(folder);
    if create && dir.exists() == false {
        create_dir_all(&dir).unwrap();
    }
    dir
}

//파일 삭제
pub fn remove_file(file_path: &str) {
    if let Ok(_) = fs::metadata(file_path) {
        fs::remove_file(file_path).unwrap();
    }
}

//폴더가 존재하는지 확인
pub fn is_exist_folder(path : &str) -> bool {
    let path_buf: PathBuf = PathBuf::from_str(path).unwrap();
    path_buf.exists()
}
