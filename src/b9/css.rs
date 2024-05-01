use crate::parser;
use std::{collections::HashMap, fs, sync::Mutex};

use gtk::{gdk::Display, prelude::*, CssProvider};

static CSS_RULES: Mutex<Option<HashMap<String, Vec<(String, String)>>>> = Mutex::new(None); // shut the fuck up

struct Properties {
    line_height: String,
    color: String,
    background_color: String,
    font_family: String,
    font_weight: String,
    text_align: String,
    underline: String,
    underline_color: String,
    overline: String,
    overline_color: String,
    strikethrough: String,
    strikethrough_color: String,
    margin_top: String,
    margin_bottom: String,
    margin_left: String,
    margin_right: String,
    border_style: String,
    border_color: String,
    border_width: String,
    border_radius: String,
    padding: String,
    font_size: String,
}

pub(crate) trait Styleable {
    fn style(&self);
}

impl Styleable for gtk::Label {
    fn style(&self) {
        let guard = CSS_RULES.lock().unwrap();
        let css = guard.as_ref().unwrap();
        let mut classes = self.css_classes();
        let mut final_css = "".to_string();

        classes.push(self.css_name());

        self.set_use_markup(true);

        for class in classes {
            if let Some(rules) = css.get(&class.to_string()) {
                let mut properties = get_properties(rules);

                match self.css_name().as_str() {
                    "h1" => properties.font_size = "24px".to_string(),
                    "h2" => properties.font_size = "22px".to_string(),
                    "h3" => properties.font_size = "20px".to_string(),
                    "h4" => properties.font_size = "18px".to_string(),
                    "h5" => properties.font_size = "16px".to_string(),
                    "h6" => properties.font_size = "14px".to_string(),
                    _ => {}
                };

                properties.font_size = properties.font_size.replace("px", "pt");

                let markup = &format!(
                    "<span foreground=\"{}\" size=\"{}\" line_height=\"{}\" font_family=\"{}\" font_weight=\"{}\" underline=\"{}\" underline_color=\"{}\" overline=\"{}\" overline_color=\"{}\" strikethrough=\"{}\" strikethrough_color=\"{}\">{}</span>",
                    properties.color,
                    properties.font_size,
                    properties.line_height,
                    properties.font_family,
                    properties.font_weight,
                    properties.underline,
                    properties.underline_color,
                    properties.overline,
                    properties.overline_color,
                    properties.strikethrough,
                    properties.strikethrough_color,
                    self.label(),
                );

                self.set_markup(markup);

                final_css += &format!(
                    "
                {} {{
                    margin-top: {};
                    margin-bottom: {};
                    margin-left: {};
                    margin-right: {};

                    border-style: {};
                    border-color: {};
                    border-width: {};
                    border-radius: {};
                    padding: {};
                }}
                ",
                    class,
                    properties.margin_top + "px",
                    properties.margin_bottom + "px",
                    properties.margin_left + "px",
                    properties.margin_right + "px",
                    properties.border_style,
                    properties.border_color,
                    properties.border_width,
                    properties.border_radius,
                    properties.padding
                );
            }

            load_css_into_app(&final_css);
        }
    }
}

impl Styleable for gtk::DropDown {
    fn style(&self) {
        let guard = CSS_RULES.lock().unwrap();
        let css = guard.as_ref().unwrap();
        let mut classes = self.css_classes();
        let mut final_css = "".to_string();

        classes.push("select".into());

        for class in classes {

            if let Some(rules) = css.get(&class.to_string()) {
                let properties: Properties = get_properties(rules);

                final_css += &format!(
                    "
                .{} {{
                    color: {};
                    background-color: {};
                    font-size: {};
                    font-family: {};

                    margin-top: {};
                    margin-bottom: {};
                    margin-left: {};
                    margin-right: {};

                    border-style: {};
                    border-color: {};
                    border-width: {};
                    border-radius: {};
                    padding: {};
                }}
                ",
                    class,
                    properties.color,
                    properties.background_color,
                    properties.font_size,
                    properties.font_family,
                    properties.margin_top + "px",
                    properties.margin_bottom + "px",
                    properties.margin_left + "px",
                    properties.margin_right + "px",
                    properties.border_style,
                    properties.border_color,
                    properties.border_width,
                    properties.border_radius,
                    properties.padding
                );
            }

            load_css_into_app(&final_css);
        }
    }
}

impl Styleable for gtk::LinkButton {
    fn style(&self) {
        let lbl = gtk::Label::builder()
            .css_name("a")
            .label(
                self.child()
                    .unwrap()
                    .downcast::<gtk::Label>()
                    .unwrap()
                    .label(),
            )
            .build();
        self.set_child(Some(&lbl));

        Styleable::style(&lbl);
    }
}

impl Styleable for gtk::Box {
    fn style(&self) {}
}

impl Styleable for gtk::TextView {
    fn style(&self) {}
}

impl Styleable for gtk::Separator {
    fn style(&self) {}
}

impl Styleable for gtk::Picture {
    fn style(&self) {}
}

impl Styleable for gtk::Entry {
    fn style(&self) {}
}

pub(crate) fn load_css() {
    let stylesheet_utf8_string = fs::read_to_string("test/styles.css").unwrap();
    let res = parser::parse(&stylesheet_utf8_string).unwrap();

    CSS_RULES.lock().unwrap().replace(res);
}

pub(crate) fn perform_styling<T: Styleable>(_element: &html_parser::Element, styleable: &T) {
    styleable.style();
}

fn get_rule(rules: &Vec<(String, String)>, property: &str, default_value: &str) -> String {
    rules
        .iter()
        .find(|(name, _)| name.as_str() == property)
        .map(|(_, value)| value.as_str())
        .unwrap_or(default_value)
        .to_owned()
}

pub(crate) fn load_css_into_app(content: &str) {
    let provider = CssProvider::new();
    provider.load_from_string(content);

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

// shithole
fn get_properties(rules: &Vec<(String, String)>) -> Properties {
    let line_height = get_rule(&rules, "line-height", &"1");
    let font_size = get_rule(&rules, "font-size", &"11px");
    let color = get_rule(&rules, "color", &"#ffffff");
    let background_color = get_rule(&rules, "background-color", &"#202020");
    let font_family = get_rule(&rules, "font-family", &"Noto Sans");
    let font_weight = get_rule(&rules, "font-weight", &"normal");
    let text_align = get_rule(&rules, "text_align", &"start");
    let underline = get_rule(&rules, "underline", &"none");
    let underline_color = get_rule(&rules, "underline-color", &"black");
    let overline = get_rule(&rules, "overline", &"none");
    let overline_color = get_rule(&rules, "overline-color", &"black");
    let strikethrough = get_rule(&rules, "strikethrough", &"false");
    let strikethrough_color = get_rule(&rules, "strikethrough-color", &"black");

    let margin_top = get_rule(&rules, "margin-top", "0").replace("px", "");
    let margin_bottom = get_rule(&rules, "margin-bottom", "0").replace("px", "");
    let margin_left = get_rule(&rules, "margin-left", "0").replace("px", "");
    let margin_right = get_rule(&rules, "margin-right", "0").replace("px", "");

    let border_style = get_rule(&rules, "border-style", "none");
    let border_color = get_rule(&rules, "border-color", "black");
    let border_width = get_rule(&rules, "border-width", "0");
    let border_radius = get_rule(&rules, "border-radius", "0");
    let padding = get_rule(&rules, "padding", "0");

    Properties {
        line_height,
        color,
        background_color,
        font_family,
        font_weight,
        text_align,
        underline,
        underline_color,
        overline,
        overline_color,
        strikethrough,
        strikethrough_color,
        margin_top,
        margin_bottom,
        margin_left,
        margin_right,
        border_style,
        border_color,
        border_width,
        border_radius,
        padding,
        font_size,
    }
}