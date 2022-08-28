use std::{fs, fmt::{Result, Debug}, io::Stdin};
use chrono::{NaiveDate};

// TODO this dir will be different when run as a binary
const BLOG_ROOT: &str = "..";

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ParsedPage {
    date: String,
    title: String,
    filename: String,
}

fn error(message: &str) -> ! {
    eprintln!("Error: {}", message);
    std::process::exit(1);
}

fn main() {
    // get blog posts
    let mut posts = get_post_files(BLOG_ROOT);
    
    posts.sort_by(|a, b| b.filename.cmp(&a.filename));

    println!("{:?}", posts);

    let contents = generate_contents_page(posts);

    println!("{}", contents);

    // TODO convert contents page and others into html

}

fn get_post_files(dir: &str) -> Vec<ParsedPage> {
    let paths = fs::read_dir(dir).unwrap();

    let mut parsed_posts: Vec<ParsedPage> = Vec::new();

    for path in paths {
        if let Ok(path) = path {
            if (path.file_type().unwrap().is_file()) {
                let post_option = parse_filename(&path.file_name().to_str().unwrap());
                if let Some(post) = post_option {
                    parsed_posts.push(post);
                }
            }
        }
    }

    return parsed_posts;
}

fn parse_filename(filename: &str) -> Option<ParsedPage> {
    // expect in format `YYYYMMDD name`
    let split = match filename.split_once(" ") {
        Some(split) => split,
        None => return None,
    };
    let date_str = split.0;
    let title = match split.1.rsplit_once(".") {
        Some(split) => split.0,
        None => return None,
    };

    return Some(ParsedPage {
        title: title.to_string(),
        date: date_str.to_string(),
        filename: filename.to_string(),
    })
}

fn generate_contents_page(pages: Vec<ParsedPage>) -> String {
    let mut contents = String::new();

    for page in pages {
        let text = format_page_row(page);
        contents.push_str(&text);
    }

    // TODO insert header and footer
    let header = match fs::read_to_string(BLOG_ROOT.to_owned() + "/.config/header.md") {
        Ok(header) => header,
        Err(_) => String::new(),
    };
    
    let footer = match fs::read_to_string(BLOG_ROOT.to_owned() + "/.config/footer.md") {
        Ok(footer) => footer,
        Err(_) => String::new(),
    };

    let full_page = format!("{header}\n\n{contents}{footer}");

    return full_page;
}

fn format_page_row(page: ParsedPage) -> String {
    let date = page.date;
    let title = page.title;
    let date_parsed = match NaiveDate::parse_from_str(&date, "%Y%m%d") {
        Ok(date_parsed) => date_parsed,
        Err(e)  => {
            println!("{}", e);
            error("Invalid date provided")
        },
    };
    let date_formatted = date_parsed.format("%B %d %Y");
    // TODO the title needs to be a link to the correct post
    return format!("{date_formatted} -- [{title}]()\n\n");
}

// get list of post files
// get their date and title
// ensure they are in order of date
// 
// generate markdown contents list for these
// convert contents page and pages to html