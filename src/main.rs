extern crate regex;

use std::env;
use std::process::{Command, Stdio};
use regex::Regex;

const TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>{TITLE}</title>
<style>{STYLES}</style>
</head>
<body>
{BODY}
</body>
</html>"#;

// 獲取 HTML 的標題
fn get_title(output: &str) -> &str {
    let title_start = "<title>";
    let title_end = "</title>";

    let title_start_pos = output.find(title_start);
    let title_end_pos = output.find(title_end);

    match (title_start_pos, title_end_pos) {
        (Some(start), Some(end)) => {
            &output[start+title_start.len()..end]
        },
        _ => "",
    }
}

// 獲取 HTML 中的樣式
fn get_styles(output: &str) -> &str {
    let re = Regex::new(r"(?s)<style[^>]*?>(.*)</style>").unwrap();

    match re.captures(output) {
        Some(caps) => {
            caps.get(1).map_or("", |m| m.as_str()).trim()
        },
        None => "",
    }
}

// 獲取 HTML 的主體內容
fn get_content(output: &str) -> &str {
    let re = Regex::new(r#"(?s)<body>
<div class="document"[^>]*?>
(.+)
</div>
</body>"#).unwrap();

    match re.captures(output) {
        Some(caps) => {
            caps.get(1).map_or("", |m| m.as_str()).trim()
        },
        None => "",
    }
}

// 給代碼塊添加行號
fn add_code_line(content: &str) -> String {
    let mut tmp = String::from(content);

    let re = Regex::new(r"(?s)(<pre[^>]*?><code>)(.*?)(</code></pre>)").unwrap();
    let line_re = Regex::new(r"(?m)^(.*?)$").unwrap();

    for caps in re.captures_iter(content) {
        // 完整的 pre 標籤
        let full_ori = caps.get(0).map_or("", |m| m.as_str());

        // 代碼前的標籤
        let start_tag = caps.get(1).map_or("", |m| m.as_str());

        // 代碼後的標籤
        let end_tag = caps.get(3).map_or("", |m| m.as_str());

        // 代碼內容
        let code_ori = caps.get(2).map_or("", |m| m.as_str());

        // 代碼每一行使用 span.code-line 包裹起來
        let code_new = &line_re.replace_all(code_ori, r#"<span class="code-line">$1</span>"#).to_string();

        // 拼合成新的完整的 pre 標籤
        let mut full_new = String::from(start_tag);
        full_new.push_str(code_new);
        full_new.push_str(end_tag);

        // 替換
        tmp = tmp.replace(full_ori, &full_new);
    }

    tmp
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let output = Command::new("rst2html.py")
        .args(&args[1..])
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute command");

    let output_str = std::str::from_utf8(&output.stdout).unwrap();

    let title = get_title(output_str);
    if title.is_empty() {
        print!("{}", output_str);
        return
    }

    let styles = get_styles(output_str);
    let mut content = get_content(output_str);

    let tmp = add_code_line(content);
    content = tmp.as_str();

    print!("{}", TEMPLATE.replace("{TITLE}", title)
        .replace("{STYLES}", styles)
        .replace("{BODY}", content));
}
