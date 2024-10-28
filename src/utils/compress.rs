use std::io::prelude::*;
use zip::write::FileOptions;
use std::fs;
use std::path::Path;

pub fn zip_file(
    src_file: &str,
    dst_file: &str,
    level: Option<i32>,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()> {

    let src_path = Path::new(src_file);
    let dst_path = Path::new(dst_file);
    let zip_file = fs::File::create(dst_path).unwrap();

    let mut zip = zip::ZipWriter::new(zip_file);

    let options = FileOptions::default()
        .compression_method(method)
        .compression_level(level)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    
    //Path에서 파일명만 추출
    let fname = src_path.file_name().unwrap();

    #[allow(deprecated)]
    zip.start_file(fname.to_str().unwrap(), options)?;
    let mut f = fs::File::open(src_path)?;

    f.read_to_end(&mut buffer)?;
    zip.write_all(&buffer)?;

    zip.finish()?;
    Ok(())
}