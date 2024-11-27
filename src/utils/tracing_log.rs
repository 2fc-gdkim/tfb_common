use std::{fs::create_dir_all, path::PathBuf, str::FromStr};
use tracing::Level;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Layer};
use tracing_appender::rolling;

pub fn setup(prefix: &str, log_path: &str, max_level: Level, with_console : bool) -> tracing_appender::non_blocking::WorkerGuard {

    let timer = time::format_description::parse(
        "[year]-[month padding:zero]-[day padding:zero] [hour]:[minute]:[second]",
    )
    .expect("Invalid time format description");
    let time_offset =
        time::UtcOffset::current_local_offset().unwrap_or_else(|_| time::UtcOffset::UTC);
    let timer = fmt::time::OffsetTime::new(time_offset, timer);

    // 로그 디렉터리 생성
    let log_path_buf = PathBuf::from_str(log_path).unwrap();
    if log_path_buf.exists() == false {
        create_dir_all(&log_path_buf).unwrap();
    }

    // 파일 로거 설정
    let file_appender = rolling::daily(log_path, prefix);
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);

    // 콘솔 로거 설정
    let console_writer = std::io::stdout;

    // 파일 로거 레이어
    let file_layer = fmt::layer()
        .with_writer(file_writer)
        .with_level(true)
        .with_timer(timer.clone())
        .with_target(false)
        .with_file(false)        
        .with_line_number(false)
        .with_filter(tracing_subscriber::filter::LevelFilter::from_level(max_level));

    // 콘솔 로거 레이어
    let console_layer = fmt::layer()
        .with_writer(console_writer)
        .with_level(true)
        .with_timer(timer)
        .with_target(false)
        .with_file(false)
        .with_line_number(false)
        .with_filter(tracing_subscriber::filter::LevelFilter::from_level(max_level));

    // 두 개의 레이어를 병합하여 글로벌 설정
    if with_console {
        tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .init();        
    }
    else {
        tracing_subscriber::registry()
            .with(file_layer)
            .init();
    }

    return _guard;
}

