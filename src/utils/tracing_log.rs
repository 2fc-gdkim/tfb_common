use std::{fs::create_dir_all, path::PathBuf, str::FromStr};
use tracing::Level;
use tracing_subscriber::fmt;

pub fn setup(prefix: &str, log_path: &str, max_level: Level, file : bool) -> tracing_appender::non_blocking::WorkerGuard {

    let timer = time::format_description::parse(
        "[year]-[month padding:zero]-[day padding:zero] [hour]:[minute]:[second]",
    )
    .expect("Cataplum");
    let time_offset =
        time::UtcOffset::current_local_offset().unwrap_or_else(|_| time::UtcOffset::UTC);
    let timer = fmt::time::OffsetTime::new(time_offset, timer);

    if file {
        let log_path_buf = PathBuf::from_str(log_path).unwrap();
        if log_path_buf.exists() == false {
            create_dir_all(&log_path_buf).unwrap();
        }
    
        let file_appender = tracing_appender::rolling::daily(log_path, prefix);
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        let subscriber = tracing_subscriber::fmt()
        .with_writer(non_blocking)   //콘솔 출력하면 파일에는 저장안됨
        .with_max_level(max_level)
        .with_timer(timer)
        .with_target(false)     //프로젝트명 출력 여부
        .with_file(false)        //파일명 출력 여부
        .with_line_number(false) //라인번호 출력 여부
        .finish();
        tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

        return _guard;
    } else {
        let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());

        let subscriber = tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(max_level)
        .with_timer(timer)
        .with_target(false)     //프로젝트명 출력 여부
        .with_file(false)        //파일명 출력 여부
        .with_line_number(false) //라인번호 출력 여부
        .finish();        
        tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

        return _guard;
    }
}

