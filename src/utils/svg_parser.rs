use svgtypes::{Color, ViewBox, PathParser};
use std::path::Path;
use std::str::FromStr;
use regex::Regex;

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
    for color_str in extract_all_colors(&svg_content) {
        // Skip "none", "inherit", "currentColor", and other non-color values
        if color_str == "none" || color_str == "inherit" || color_str == "currentColor" {
            continue;
        }

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
    let pattern = format!(r#"{}="([^"]*)""#, attr);
    if let Ok(re) = Regex::new(&pattern) {
        if let Some(caps) = re.captures(content) {
            return Some(caps[1].to_string());
        }
    }
    None
}

fn extract_all_path_data(content: &str) -> Vec<String> {
    let mut paths = Vec::new();
    if let Ok(re) = Regex::new(r#"<path[^>]*\sd="([^"]*)""#) {
        for caps in re.captures_iter(content) {
            paths.push(caps[1].to_string());
        }
    }
    paths
}

fn extract_all_colors(content: &str) -> Vec<String> {
    let mut colors = Vec::new();

    // === Extract colors from CSS classes in <style> blocks ===
    if let Ok(style_re) = Regex::new(r"<style[^>]*>(.*?)</style>") {
        if let Some(style_caps) = style_re.captures(content) {
            let style_content = &style_caps[1];

            // Extract fill and stroke colors from CSS rules
            if let Ok(fill_re) = Regex::new(r"fill:\s*([^;}\s]+)") {
                for caps in fill_re.captures_iter(style_content) {
                    let color_value = caps[1].trim().to_string();
                    if !color_value.is_empty() && color_value != "none" {
                        colors.push(color_value);
                    }
                }
            }

            if let Ok(stroke_re) = Regex::new(r"stroke:\s*([^;}\s]+)") {
                for caps in stroke_re.captures_iter(style_content) {
                    let color_value = caps[1].trim().to_string();
                    if !color_value.is_empty() && color_value != "none" {
                        colors.push(color_value);
                    }
                }
            }
        }
    }

    // === Original code for inline attributes ===
    // Attributes that can contain colors
    let color_attrs = ["fill", "stroke", "stop-color", "color", "flood-color", "lighting-color"];

    for attr in &color_attrs {
        // Match attribute="value" pattern
        let pattern = format!(r#"{}="([^"]*)""#, attr);
        if let Ok(re) = Regex::new(&pattern) {
            for caps in re.captures_iter(content) {
                let color_value = caps[1].trim().to_string();
                if !color_value.is_empty() && color_value != "none" {
                    colors.push(color_value);
                }
            }
        }

        // Also match attribute='value' pattern (single quotes)
        let pattern_single = format!(r#"{}='([^']*)'"#, attr);
        if let Ok(re) = Regex::new(&pattern_single) {
            for caps in re.captures_iter(content) {
                let color_value = caps[1].trim().to_string();
                if !color_value.is_empty() && color_value != "none" {
                    colors.push(color_value);
                }
            }
        }
    }

    // Also extract colors from inline style attributes
    if let Ok(re) = Regex::new(r#"style="([^"]*)""#) {
        for caps in re.captures_iter(content) {
            let style_content = &caps[1];

            // Extract colors from CSS properties within style
            for attr in &color_attrs {
                let css_pattern = format!(r"{}:\s*([^;]+)", attr);
                if let Ok(css_re) = Regex::new(&css_pattern) {
                    for css_caps in css_re.captures_iter(style_content) {
                        let color_value = css_caps[1].trim().to_string();
                        if !color_value.is_empty() && color_value != "none" {
                            colors.push(color_value);
                        }
                    }
                }
            }
        }
    }

    colors
}