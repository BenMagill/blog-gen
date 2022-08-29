use std::{fs, fmt::{Result, Debug}, io::Stdin};
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
    // wipe old build folder
    match fs::remove_dir_all(format!("{}/.build", BLOG_ROOT)) {
        Err(_) => println!("WARNING: Failed to remove .build dir, continuing"),
        Ok(_) => (),
    };

    fs::create_dir(format!("{}/.build", BLOG_ROOT));

    let mut posts = get_post_files(BLOG_ROOT);
    
    posts.sort_by(|a, b| b.filename.cmp(&a.filename));

    let contents = generate_contents_page(&posts);

    println!("{}", contents);

    let template = get_html_template();

    // create contents page
    md_to_html(&contents, "index.html", &template);

    for post in posts {
        let filename = generate_post_filename(&post);
        let md = fs::read_to_string(BLOG_ROOT.to_owned() + "/" + &post.filename).unwrap();
        md_to_html(&md, &filename, &template)
    }

    match fs::copy(format!("{}/.config/style.css", BLOG_ROOT), format!("{}/.build/style.css", BLOG_ROOT)) {
        Err(_) => error("Failed to add stlye.css to build"),
        _ => (),
    };
}
fn generate_post_filename(post: &ParsedPage) -> String {
    return format!("{}-{}.html", post.date, post.title.replace(" ", "-"));
}

fn get_html_template() -> String {
    return match fs::read_to_string(BLOG_ROOT.to_owned() + "/.config/template.html") {
        Ok(html) => html,
        Err(_) => error("Missing html template"),
    }
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

fn generate_contents_page(pages: &Vec<ParsedPage>) -> String {
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

fn format_page_row(page: &ParsedPage) -> String {
    let date = &page.date;
    let title = &page.title;
    let date_parsed = match NaiveDate::parse_from_str(&date, "%Y%m%d") {
        Ok(date_parsed) => date_parsed,
        Err(e)  => {
            println!("{}", e);
            error("Invalid date provided")
        },
    };
    let date_formatted = date_parsed.format("%B %d %Y");
    let link = generate_post_filename(page);

    return format!("{date_formatted} -- [{title}]({link})\n\n");
}

fn md_to_html(md: &str, file_name: &str, template: &str) {
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
    println!("{}", html_content);

    let html = template.replace(TEMPLATE_CONTENT_LOCATION, &html_content);

    match fs::write(BLOG_ROOT.to_owned() + "/.build/" + file_name, html) {
        Err(_) => error("Failed to store html output"),
        _ => (),
    }
}