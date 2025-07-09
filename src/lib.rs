use bbcode_tagger::{BBCode, BBNode, BBTag, BBTree};
use eframe::egui::{
    self, Align, Color32, FontId, RichText, Stroke, TextFormat, TextWrapMode, text::LayoutJob,
};
use std::collections::HashMap;

pub struct NodeFormatter {}

pub struct BBCodeCache {
    parser: BBCode,
    cache: HashMap<String, BBTree>,
}
impl BBCodeCache {
    pub fn get_bbtree(&mut self, input: &str) -> &BBTree {
        if !self.cache.contains_key(input) {
            self.cache
                .insert(input.to_string(), self.parser.parse(input));
        }
        self.cache.get(input).unwrap()
    }
}

pub struct BBCodeViewer {}
impl BBCodeViewer {
    pub fn new() {}

    pub fn show(&self, ui: &mut egui::Ui, cache: &mut BBCodeCache, input: &str) {
        let bbtree = cache.get_bbtree(input);
        show_bbnode(ui, bbtree, 0, &TextFormat::default());
    }
}

fn show_bbnode(ui: &mut egui::Ui, tree: &BBTree, i: i32, parent_fmt: &TextFormat) {
    let node = tree.get_node(i);
    let text = node.text.as_str();
    let mut text_fmt = parent_fmt.clone();

    // TODO: take in to account the parent tags

    let mut children_handled = false;

    match node.tag {
        BBTag::None => {
            ui.label(node.text.as_str());
        }
        BBTag::Bold
        | BBTag::Italic
        | BBTag::Underline
        | BBTag::Strikethrough
        | BBTag::FontSize
        | BBTag::FontColor
        | BBTag::Center
        | BBTag::Left
        | BBTag::Right
        | BBTag::Superscript
        | BBTag::Subscript
        | BBTag::ListItem => {
            ui_handle_text(ui, node, &mut text_fmt);
        }
        BBTag::Quote => {
            ui.label(text);
        }
        BBTag::Spoiler => {
            ui.label(text);
        }
        BBTag::Link => {
            // no URL to create link, use text
            let mut value = text;
            if node.value.is_some() {
                value = node.value.as_ref().unwrap().as_str();
            }
            if text.is_empty() {
                ui.hyperlink(value);
            } else {
                ui.hyperlink_to(text, value);
            }
        }
        BBTag::Image => {
            ui.label(text);
        }
        BBTag::ListOrdered => {
            children_handled = true;
            let id = ui.make_persistent_id(format!("{}_list", i));
            egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, true)
                .show_header(ui, |ui| {
                    ui.label(node.text.as_str());
                })
                .body(|ui| {
                    ui.vertical(|ui| {
                        for (index, node) in node.children.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{index}.) "));
                                show_bbnode(ui, tree, *node, &text_fmt);
                            });
                        }
                    });
                });
        }
        BBTag::ListUnordered => {
            children_handled = true;
            ui.vertical(|ui| {
                ui.style_mut().wrap_mode = Some(TextWrapMode::Wrap);
                if let Some(title) = &node.value {
                    if !title.is_empty() {
                        ui.label(node.text.as_str());
                    }
                }
                for node in node.children.iter() {
                    ui.horizontal(|ui| {
                        ui.label("– ");
                        show_bbnode(ui, tree, *node, &text_fmt);
                    });
                }
            });
        } // "–"
        BBTag::Code => {
            ui.label(RichText::new(text).code());
        }
        BBTag::Preformatted => {
            ui.label(text);
        }
        BBTag::Table => {
            ui.label(text);
        }
        BBTag::TableHeading => {
            ui.label(text);
        }
        BBTag::TableRow => {
            ui.label(text);
        }
        BBTag::TableCell => {
            ui.label(text);
        }
        BBTag::YouTube => {
            ui.label(text);
        }
        BBTag::Blur => {
            ui.label(text);
        }
        BBTag::Email => {
            ui.label(text);
        }
        BBTag::Unknown => {
            ui.label(text);
        }
    };

    if children_handled {
        return;
    }
    for child in node.children.iter() {
        show_bbnode(ui, tree, *child, &text_fmt);
    }
}
fn ui_handle_text(ui: &mut egui::Ui, node: &BBNode, text_fmt: &mut TextFormat) {
    // skip empty text
    if node.text.trim().is_empty() {
        return;
    }
    let mut job = LayoutJob::default();

    let (default_color, strong_color) = if ui.visuals().dark_mode {
        (Color32::LIGHT_GRAY, Color32::WHITE)
    } else {
        (Color32::DARK_GRAY, Color32::BLACK)
    };
    text_fmt.color = default_color;

    // tag on the current node to apply the same formatting
    match node.tag {
        BBTag::Bold => {
            text_fmt.color = strong_color;
        }
        BBTag::Italic => {
            text_fmt.italics = true;
        }
        BBTag::Underline => {
            text_fmt.underline = Stroke::new(1.0, text_fmt.color);
        }
        BBTag::Strikethrough => {
            text_fmt.strikethrough = Stroke::new(1.0, text_fmt.color);
        }
        BBTag::FontColor => {
            // if n.value.is_some_and(|x| x.starts_with("#")) {
            // let text_color = node.value.unwrap().as_str();
            // text_fmt.color = hex_color!(text_color);
            // }
        }
        BBTag::FontSize => {
            if let Some(size) = &node.value {
                match size.as_str() {
                    "1" => text_fmt.font_id = FontId::proportional(32.0),
                    "2" => text_fmt.font_id = FontId::proportional(24.0),
                    "3" => text_fmt.font_id = FontId::proportional(20.8),
                    "4" => text_fmt.font_id = FontId::proportional(16.0),
                    "5" => text_fmt.font_id = FontId::proportional(12.8),
                    "6" => text_fmt.font_id = FontId::proportional(11.2),
                    _ => {}
                }
            }
        }
        BBTag::Center => {
            text_fmt.valign = Align::Center;
        }
        BBTag::Left => {
            text_fmt.valign = Align::LEFT;
        }
        BBTag::Right => text_fmt.valign = Align::RIGHT,
        BBTag::Superscript => {
            text_fmt.font_id = FontId::proportional(7.0);
            text_fmt.valign = Align::TOP;
        }
        BBTag::Subscript => {
            text_fmt.font_id = FontId::proportional(7.0);
            text_fmt.valign = Align::BOTTOM;
        }
        _ => {}
    }
    job.append(node.text.as_str(), 0.0, text_fmt.clone());
    ui.label(job);
}

#[cfg(test)]
mod tests {
    use super::*;
}
