use std::env;
use std::process::Command;

//쉘 명령어 실행 
pub fn execute(cmd: &str, args: &[String], remove_newline: bool) -> Option<String> {
    match env::consts::OS {
        "windows" => {
            //tracing::error!("명령어 실행 [{}] [{:?}]", cmd, args);

            let output = Command::new("cmd")
            .args(&["/C", &cmd])
            .args(args)
            .output();

            match output {
                Ok(o) => {
                    // 명령어의 종료 상태를 출력합니다.
                    if o.status.success() {
                        if remove_newline {
                            return Some(String::from_utf8_lossy(&o.stdout).to_string().replace("\r\n", ""));
                        }
                        else {
                            return Some(String::from_utf8_lossy(&o.stdout).to_string());
                        }
                    } else {
                        tracing::error!("Command execution failed. status[{:?}]", o.status);
                    } 
                },
                Err(e) => {
                    tracing::error!("Command execution failed. [{:?}]", e);
                }
            }
        }
        _ => {
            let output = Command::new("sh")
            .args(&["-c", &cmd])
            .args(args)
            .output()
            .expect("Command execution failed.");

            // 명령어의 종료 상태를 출력합니다.
            if output.status.success() {
                if remove_newline {
                    return Some(String::from_utf8_lossy(&output.stdout).to_string().replace("\n", ""));
                }
                else {
                    return Some(String::from_utf8_lossy(&output.stdout).to_string());
                }
            } else {
                tracing::error!("Command execution failed. [{:?}]", output.status);
            }                  
        }
    }
    None
}

//쉘 명령어 실행
pub fn execute_noparam(cmd: &str, remove_newline: bool) -> Option<String> {
    match env::consts::OS {
        "windows" => {
            //tracing::error!("명령어 실행 [{}] [{:?}]", cmd, args);

            let output = Command::new("cmd")
            .args(&["/C", &cmd])
            .output();

            match output {
                Ok(o) => {
                    // 명령어의 종료 상태를 출력합니다.
                    if o.status.success() {
                        if remove_newline {
                            return Some(String::from_utf8_lossy(&o.stdout).to_string().replace("\r\n", ""));
                        }
                        else {
                            return Some(String::from_utf8_lossy(&o.stdout).to_string());
                        }
                    } else {
                        tracing::error!("Command execution failed. status[{:?}]", o.status);
                    } 
                },
                Err(e) => {
                    tracing::error!("Command execution failed. [{:?}]", e);
                }
            }
        }
        _ => {
            let output = Command::new("sh")
            .args(&["-c", &cmd])
            .output()
            .expect("Command execution failed.");

            // 명령어의 종료 상태를 출력합니다.
            if output.status.success() {
                if remove_newline {
                    return Some(String::from_utf8_lossy(&output.stdout).to_string().replace("\n", ""));
                }
                else {
                    return Some(String::from_utf8_lossy(&output.stdout).to_string());
                }
            } else {
                tracing::error!("Command execution failed. [{:?}]", output.status);
            }                  
        }
    }
    None
}