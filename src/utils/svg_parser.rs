use svgtypes::{Color, ViewBox, PathParser};
use std::path::Path;
use std::str::FromStr;

pub struct SvgInfo {
    pub width: Option<String>,
    pub height: Option<String>,
    pub view_box: Option<ViewBox>,
    pub path_count: usize,
    pub colors_used: Vec<Color>,
    pub total_path_commands: usize,
}

pub fn parse_svg_info(svg_path: &Path) -> Result<SvgInfo, Box<dyn std::error::Error>> {
    let svg_content = std::fs::read_to_string(svg_path)?;

    // Parse viewBox if present
    let view_box = if let Some(vb_str) = extract_attribute(&svg_content, "viewBox") {
        ViewBox::from_str(&vb_str).ok()
    } else {
        None
    };

    // Extract width/height
    let width = extract_attribute(&svg_content, "width");
    let height = extract_attribute(&svg_content, "height");

    // Count paths and parse their data
    let path_count = svg_content.matches("<path").count();
    let mut total_path_commands = 0;

    for path_data in extract_all_path_data(&svg_content) {
        for segment in PathParser::from(path_data.as_str()) {
            if segment.is_ok() {
                total_path_commands += 1;
            }
        }
    }

    // Extract colors
    let mut colors_used = Vec::new();
    for color_str in extract_colors(&svg_content) {
        if let Ok(color) = Color::from_str(&color_str) {
            if !colors_used.contains(&color) {
                colors_used.push(color);
            }
        }
    }

    Ok(SvgInfo {
        width,
        height,
        view_box,
        path_count,
        colors_used,
        total_path_commands,
    })
}

fn extract_attribute(content: &str, attr: &str) -> Option<String> {
    let pattern = format!("{}=\"", attr);
    if let Some(start) = content.find(&pattern) {
        let value_start = start + pattern.len();
        if let Some(end) = content[value_start..].find('"') {
            return Some(content[value_start..value_start + end].to_string());
        }
    }
    None
}

fn extract_all_path_data(content: &str) -> Vec<String> {
    let mut paths = Vec::new();
    for line in content.lines() {
        if let Some(d_attr) = extract_attribute(line, "d") {
            paths.push(d_attr);
        }
    }
    paths
}

fn extract_colors(content: &str) -> Vec<String> {
    let mut colors = Vec::new();
    for attr in &["fill", "stroke", "stop-color", "color"] {
        if let Some(color) = extract_attribute(content, attr) {
            colors.push(color);
        }
    }
    colors
}