use std::path::Path;
use std::fs::File;
use std::error::Error;

pub fn create_file() {
    let path = Path::new("lorem_ipsum.txt");
    let display = path.display();

    // 以只写模式打开文件，返回 `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    dbg!(file);
}