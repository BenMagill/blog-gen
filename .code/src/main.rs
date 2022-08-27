use std::fs;

struct ParsedInfo {
    date: String,
    title: String,
}

fn error(message: &str) {
    eprintln!("Error: {}", message);
    std::process::exit(1);
}

fn main() {
    // TODO this dir will be different when run as a binary
    get_post_files("..");
}

fn get_post_files(dir: &str) {
    let paths = fs::read_dir(dir).unwrap();

    let mut parsed_posts: Vec<ParsedInfo> = Vec::new();

    for path in paths {
        if let Ok(path) = path {
            if (path.file_type().unwrap().is_file()) {
                let post = parse_filename(&path.file_name().to_str().unwrap());
                parsed_posts.push(post);
            }
        }
    }
}

fn parse_filename(filename: &str) -> ParsedInfo {
    // expect in format `YYYYMMDD name`
    let split = filename.split_once(" ").unwrap();
    let date_str = split.0;
    let title = split.1.rsplit_once(".").unwrap().0;
    println!("{}", date_str);
    println!("{}", title);
    return ParsedInfo {
        title: title.to_string(),
        date: date_str.to_string(),
    }
}

// get list of post files
// get their date and title
// ensure they are in order of date
// 
// generate markdown contents list for these
// convert contents page and pages to html