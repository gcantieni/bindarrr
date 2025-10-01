use pulldown_cmark::html;
use pulldown_cmark::Parser;
use std::env;
use std::fs;
use std::path;

const OUTPUT_DIR: &str = "output/";

// Set DEBUG=true in the environment to trigger this
macro_rules! print_dbg {
    ($($arg:tt)*) => {
        let debug = env::var("DEBUG").is_ok();
        if debug {
            println!("[DEBUG] {}", format!($($arg)*))
        }
    };
}

fn main() {
    // Do the file related stuff to start so we panic early if they're messed
    // up.
    // NOTE: this is all intended to be run from the md-convert dir
    if !path::Path::new(OUTPUT_DIR).exists() {
        fs::create_dir(OUTPUT_DIR).unwrap();
    }

    if !path::Path::new("output/songs").exists() {
        fs::create_dir("output/songs").unwrap();
    }

    let input_md = fs::read_to_string("bindarrr.md").unwrap();

    // This implementation is going to be inherently brittle because it
    // relies on the output of the google docs markdown conversion.
    // It seems like they use pandoc style markdown output.
    //
    // First we find the first song, and then parse them into an array of songs.
    // We can skip the TOC, as we are generating that ourselves.

    // Skip the first line which is the title.
    let mut shanty_body = String::new();
    let mut shanty_title = String::new();
    let mut table_of_contents = String::new();

    // Skip the first line, as it contains the title.
    for line in input_md.lines().skip(1) {
        print_dbg!("Line: {}", line);

        if line.starts_with("# ") {
            let title_tmp = line
                .trim_start_matches('#')
                .split('{')
                .next()
                .unwrap()
                .trim();

            // Now try to get the title. Sometimes this is empty.
            if title_tmp.is_empty() {
                print_dbg!("EMPTY TITLE");
                continue;
            }

            print_dbg!("TITLE");
            // We know we have a new title, so save the previous shanty, if
            // we have a previous shanty.
            if !shanty_title.is_empty() {
                let html_str = markdown_to_html(&shanty_body);

                let file_name = shanty_title
                    .to_lowercase()
                    .replace("/", "")
                    .replace("'", "")
                    .replace(",", "")
                    .replace("’", "")
                    .replace("*", "")
                    .replace("?", "")
                    .replace(" ", "-")
                    + ".html";

                print_dbg!("FILENAME: {}", file_name);

                table_of_contents.push_str(&format!(
                    "      <li><a href='songs/{}'>{}</a></li>\n",
                    file_name, shanty_title
                ));

                let fpath = &format!("output/songs/{}", file_name);
                print_dbg!("File path {}", fpath);
                fs::write(&format!("output/songs/{}", file_name), html_str).unwrap();

                println!("Shanty added: {} : {}", shanty_title, file_name);
            }

            // Reset
            shanty_body = "".to_string();

            // Extract the title.
            // For an input line of:
            // '# All For Me Grog {#all-for-me-grog}'
            // we want 'All For Me Grog'
            //
            // Trim away the '#' and then split and discard the end starting
            // with '{'
            shanty_title = title_tmp.to_string();

            // Remove footnotes. They are of the form [^32]
            if shanty_title.contains("[^") {
                print_dbg!("Removing footnote");
                if let Some((first, rest)) = shanty_title.split_once('[') {
                    // Get rid of ^32] by finding ']' and appending the rest.
                    let r: String = rest.chars().skip_while(|&c| c != ']').skip(1).collect();

                    shanty_title = format!("{}{}", first, r);
                }
            }

            // Insert title
            shanty_body.push_str("# ");
            shanty_body.push_str(&shanty_title);
            shanty_body.push_str("\n");
        } else if !shanty_title.is_empty() {
            print_dbg!("SHANTY LINE");

            // Google docs chooses to represent the "Formerly in the Irish
            // packet" line as an h2, which feels too big. Replace it with a
            // normal italics line. It does the same thing with a line like
            // "From Long-Hard Susan". For now, choose to represent all h2's as
            // italics.
            if line.starts_with("## ") {
                // Remove h2 and add italics (by wrappinging it like this *line*)
                let l = format!("**{}**", &line[3..]);
                shanty_body.push_str(&l);
            } else {
                shanty_body.push_str(line);
            }
            shanty_body.push_str("\n");
        }
    }

    print_dbg!("TOC: {}", table_of_contents);

    // Now we can write the index.html file
    let output_toc = fs::read_to_string("index-template.html")
        .unwrap()
        .replace("REPLACEME", &table_of_contents);

    fs::write("output/index.html", output_toc).unwrap();
}

fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut body = String::new();
    html::push_html(&mut body, parser);

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
<link rel='stylesheet' href='../css/song.css'>
<link href='https://fonts.googleapis.com/css?family=Jim+Nightshade' rel='stylesheet'>
<meta name='viewport' content='width=device-width, initial-scale=1.0'>
</head>
<body>
{}
</body>
</html>"#,
        body
    )
}
