use std::{fs, fmt::{Debug}};
use chrono::{NaiveDate};
use comrak::{markdown_to_html, ComrakOptions};

// TODO this dir will be different when run as a binary
const BLOG_ROOT: &str = "..";
const TEMPLATE_CONTENT_LOCATION: &str = "INSERT_CONTENT_HERE";

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
    clear_dir(&format!("{}/build", BLOG_ROOT));

    let mut posts = get_posts(BLOG_ROOT);
    
    posts.sort_by(|a, b| b.filename.cmp(&a.filename));

    let contents = generate_contents_page_md(&posts);
    let template = get_html_template();

    md_to_html(&contents, "index.html", &template);
    for post in posts {
        let filename = generate_post_filename(&post);
        let md = match fs::read_to_string(format!("{}/{}", BLOG_ROOT, post.filename)) {
            Ok(md) => md,
            Err(_) => error(&format!("Failed to read {}", post.filename))
        };
        md_to_html(&md, &filename, &template)
    }

    match fs::copy(format!("{}/.config/style.css", BLOG_ROOT), format!("{}/build/style.css", BLOG_ROOT)) {
        Err(_) => error("Failed to add stlye.css to build"),
        _ => (),
    };
}

fn clear_dir(dir: &str) {
    match fs::remove_dir_all(dir) {
        Err(_) => error(&format!("Failed to remove directory '{}'", dir)),
        Ok(_) => (),
    };

    match fs::create_dir(dir) {
        Err(_) => error(&format!("Failed to re-add directory '{}'", dir)),
        Ok(_) => ()
    };
}

fn generate_post_filename(post: &ParsedPage) -> String {
    return format!("{}-{}.html", post.date, post.title.replace(" ", "-"));
}

fn get_html_template() -> String {
    return match fs::read_to_string(format!("{}/.config/template.html", BLOG_ROOT)) {
        Ok(html) => html,
        Err(_) => error("Missing html template file"),
    }
}

fn get_posts(dir: &str) -> Vec<ParsedPage> {
    let paths = match fs::read_dir(dir) {
        Ok(paths) => paths,
        Err(_) => error("Failed to read posts"),
    };

    let mut parsed_posts: Vec<ParsedPage> = Vec::new();

    for path in paths {
        if let Ok(path) = path {
            if path.file_type().unwrap().is_file() {
                let post_option = parse_filename(match &path.file_name().to_str() {
                    Some(filename) => filename,
                    None => error("Could not read post filename"),
                });
                if let Some(post) = post_option {
                    parsed_posts.push(post);
                }
            }
        }
    }

    return parsed_posts;
}

/**
 * Extract the date and title from a blogs filename
 */
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

/**
 * Create the contents page markdown
 */
fn generate_contents_page_md(pages: &Vec<ParsedPage>) -> String {
    let mut contents = String::new();

    for page in pages {
        let date = format_blog_date(&page.date);
        let link = generate_post_filename(page);
        contents.push_str(&format!("{} -- [{}]({})\n\n", date, page.title, link));
    }

    let header = match fs::read_to_string(format!("{}/.config/header.md", BLOG_ROOT)) {
        Ok(header) => header,
        Err(_) => String::new(),
    };
    
    let footer = match fs::read_to_string(format!("{}/.config/footer.md", BLOG_ROOT)) {
        Ok(footer) => footer,
        Err(_) => String::new(),
    };

    let full_page = format!("{header}\n\n{contents}{footer}");

    return full_page;
}

fn format_blog_date(date: &str) -> String {
    let date_parsed = match NaiveDate::parse_from_str(&date, "%Y%m%d") {
        Ok(date_parsed) => date_parsed,
        Err(e)  => {
            println!("{}", e);
            error("Invalid date provided")
        },
    };
    let date_formatted = date_parsed.format("%B %d %Y");
    return date_formatted.to_string();
}

fn md_to_html(md: &str, filename: &str, template: &str) {
    let mut options = ComrakOptions::default();
    options.extension.autolink = true;
    options.extension.description_lists = true;
    options.extension.footnotes = true;
    options.extension.strikethrough = true;
    options.extension.superscript = true;
    options.extension.table = true;
    options.extension.tagfilter = true;
    options.extension.tasklist = true;
    options.render.hardbreaks = true;

    let html_content = markdown_to_html(md, &options); 
    let html_file = template.replace(TEMPLATE_CONTENT_LOCATION, &html_content);

    match fs::write(format!("{}/build/{}", BLOG_ROOT, filename), html_file) {
        Err(_) => error("Failed to store html output"),
        Ok(_) => (),
    }
}