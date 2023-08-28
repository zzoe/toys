use std::any::Any;
use std::collections::BTreeMap;

use eframe::egui::{
    Button, FontFamily, FontId, RichText, ScrollArea, TextEdit, TextStyle, Ui, WidgetText,
};
// copy from github.com/emilk/egui/egui_demo_lib/src/demo/font_book.rs
use serde::{Deserialize, Serialize};

use crate::app::view::View;

#[derive(Serialize, Deserialize, Debug)]
pub struct FontBook {
    filter: String,
    last_filter: String,
    row_width: f32,
    row_chars: Vec<Vec<(char, String)>>,
    #[serde(skip)]
    named_chars: BTreeMap<char, String>,
    #[serde(skip)]
    need_calc: bool,
}

impl Default for FontBook {
    fn default() -> Self {
        Self {
            filter: Default::default(),
            last_filter: Default::default(),
            row_width: Default::default(),
            named_chars: Default::default(),
            row_chars: vec![],
            need_calc: true,
        }
    }
}

impl View for FontBook {
    fn name(&self) -> &str {
        "📖 字典"
    }

    fn view(&mut self, ui: &mut Ui) {
        ui.label(format!(
            "The selected font supports {} characters.",
            self.named_chars.len()
        ));

        ui.horizontal(|ui| {
            ui.label("Filter:");
            ui.add(TextEdit::singleline(&mut self.filter).desired_width(120.0));
            self.filter = self.filter.to_lowercase();
            if ui.button("⨯").clicked() {
                self.filter.clear();
            }
        });

        if self.filter != self.last_filter {
            self.last_filter = self.filter.clone();
            self.need_calc = true;
        }

        if self.named_chars.is_empty() {
            self.named_chars = available_characters(ui);
        }

        ui.separator();

        let row_height = ui.text_style_height(&TextStyle::Button);
        let rows = self.row_chars.len();
        if rows == 0 {
            self.need_calc = true;
        }

        let mut scroll_area = ScrollArea::vertical().auto_shrink([false; 2]);
        if self.need_calc {
            scroll_area = scroll_area.vertical_scroll_offset(0.0);
        }

        scroll_area.show_rows(ui, row_height, rows, |ui, row_range| {
            let available_width = ui.available_width();
            if self.row_width != available_width {
                self.need_calc = true;
            }

            if self.need_calc {
                self.row_chars = vec![vec![]];
                let mut row = 0;
                let mut used_width = 0.0;

                for (chr, name) in &self.named_chars {
                    if !self.filter.is_empty()
                        && !name.to_lowercase().contains(&self.filter)
                        && !chr.to_string().contains(&self.filter)
                    {
                        continue;
                    }

                    let width = self.button_width(ui, RichText::new(chr.to_string()).into());

                    if used_width + width > available_width {
                        row += 1;
                        self.row_chars.push(vec![]);
                        self.row_chars[row].push((*chr, name.clone()));
                        used_width = width;
                    } else {
                        self.row_chars[row].push((*chr, name.clone()));
                        used_width += width;
                    }
                }

                self.row_width = available_width;
                self.need_calc = false;
            }

            for row in row_range {
                ui.horizontal(|ui| {
                    if let Some(row_chars) = self.row_chars.get(row) {
                        for (chr, name) in row_chars {
                            self.char_button(ui, *chr, name);
                        }
                    }
                });
            }
        });
    }

    fn any(&self) -> &dyn Any {
        self
    }

    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl FontBook {
    fn button_width(&self, ui: &Ui, text: WidgetText) -> f32 {
        let galley = text.into_galley(ui, Some(false), 0.0, TextStyle::Button);
        galley.size().x + 2.0 * ui.spacing().button_padding.x + ui.spacing().item_spacing.x
    }

    fn char_button(&self, ui: &mut Ui, chr: char, name: &str) {
        let button = Button::new(RichText::new(chr)).frame(false);

        let tooltip_ui = |ui: &mut Ui| {
            ui.label(RichText::new(chr));
            ui.label(format!("{}\nU+{:X}\n\nClick to copy", name, chr as u32));
        };

        if ui.add(button).on_hover_ui(tooltip_ui).clicked() {
            ui.output_mut(|o| o.copied_text = chr.to_string());
        }
    }
}

fn available_characters(ui: &Ui) -> BTreeMap<char, String> {
    ui.fonts(|fonts| {
        fonts
            .lock()
            .fonts
            .font(&FontId::new(35.0, FontFamily::Monospace)) // size is arbitrary for getting the characters
            .characters()
            .iter()
            .filter(|chr| !chr.is_whitespace() && !chr.is_ascii_control())
            .map(|&chr| (chr, char_name(chr)))
            .collect()
    })
}

fn char_name(chr: char) -> String {
    special_char_name(chr)
        .map(|s| s.to_owned())
        .or_else(|| unicode_names2::name(chr).map(|name| name.to_string().to_lowercase()))
        .unwrap_or_else(|| "unknown".to_owned())
}

fn special_char_name(chr: char) -> Option<&'static str> {
    #[allow(clippy::match_same_arms)] // many "flag"
    match chr {
        // Special private-use-area extensions found in `emoji-icon-font.ttf`:
        // Private use area extensions:
        '\u{FE4E5}' => Some("flag japan"),
        '\u{FE4E6}' => Some("flag usa"),
        '\u{FE4E7}' => Some("flag"),
        '\u{FE4E8}' => Some("flag"),
        '\u{FE4E9}' => Some("flag"),
        '\u{FE4EA}' => Some("flag great britain"),
        '\u{FE4EB}' => Some("flag"),
        '\u{FE4EC}' => Some("flag"),
        '\u{FE4ED}' => Some("flag"),
        '\u{FE4EE}' => Some("flag south korea"),
        '\u{FE82C}' => Some("number sign in square"),
        '\u{FE82E}' => Some("digit one in square"),
        '\u{FE82F}' => Some("digit two in square"),
        '\u{FE830}' => Some("digit three in square"),
        '\u{FE831}' => Some("digit four in square"),
        '\u{FE832}' => Some("digit five in square"),
        '\u{FE833}' => Some("digit six in square"),
        '\u{FE834}' => Some("digit seven in square"),
        '\u{FE835}' => Some("digit eight in square"),
        '\u{FE836}' => Some("digit nine in square"),
        '\u{FE837}' => Some("digit zero in square"),

        // Special private-use-area extensions found in `emoji-icon-font.ttf`:
        // Web services / operating systems / browsers
        '\u{E600}' => Some("web-dribbble"),
        '\u{E601}' => Some("web-stackoverflow"),
        '\u{E602}' => Some("web-vimeo"),
        '\u{E603}' => Some("web-twitter"),
        '\u{E604}' => Some("web-facebook"),
        '\u{E605}' => Some("web-googleplus"),
        '\u{E606}' => Some("web-pinterest"),
        '\u{E607}' => Some("web-tumblr"),
        '\u{E608}' => Some("web-linkedin"),
        '\u{E60A}' => Some("web-stumbleupon"),
        '\u{E60B}' => Some("web-lastfm"),
        '\u{E60C}' => Some("web-rdio"),
        '\u{E60D}' => Some("web-spotify"),
        '\u{E60E}' => Some("web-qq"),
        '\u{E60F}' => Some("web-instagram"),
        '\u{E610}' => Some("web-dropbox"),
        '\u{E611}' => Some("web-evernote"),
        '\u{E612}' => Some("web-flattr"),
        '\u{E613}' => Some("web-skype"),
        '\u{E614}' => Some("web-renren"),
        '\u{E615}' => Some("web-sina-weibo"),
        '\u{E616}' => Some("web-paypal"),
        '\u{E617}' => Some("web-picasa"),
        '\u{E618}' => Some("os-android"),
        '\u{E619}' => Some("web-mixi"),
        '\u{E61A}' => Some("web-behance"),
        '\u{E61B}' => Some("web-circles"),
        '\u{E61C}' => Some("web-vk"),
        '\u{E61D}' => Some("web-smashing"),
        '\u{E61E}' => Some("web-forrst"),
        '\u{E61F}' => Some("os-windows"),
        '\u{E620}' => Some("web-flickr"),
        '\u{E621}' => Some("web-picassa"),
        '\u{E622}' => Some("web-deviantart"),
        '\u{E623}' => Some("web-steam"),
        '\u{E624}' => Some("web-github"),
        '\u{E625}' => Some("web-git"),
        '\u{E626}' => Some("web-blogger"),
        '\u{E627}' => Some("web-soundcloud"),
        '\u{E628}' => Some("web-reddit"),
        '\u{E629}' => Some("web-delicious"),
        '\u{E62A}' => Some("browser-chrome"),
        '\u{E62B}' => Some("browser-firefox"),
        '\u{E62C}' => Some("browser-ie"),
        '\u{E62D}' => Some("browser-opera"),
        '\u{E62E}' => Some("browser-safari"),
        '\u{E62F}' => Some("web-google-drive"),
        '\u{E630}' => Some("web-wordpress"),
        '\u{E631}' => Some("web-joomla"),
        '\u{E632}' => Some("lastfm"),
        '\u{E633}' => Some("web-foursquare"),
        '\u{E634}' => Some("web-yelp"),
        '\u{E635}' => Some("web-drupal"),
        '\u{E636}' => Some("youtube"),
        '\u{F189}' => Some("vk"),
        '\u{F1A6}' => Some("digg"),
        '\u{F1CA}' => Some("web-vine"),
        '\u{F8FF}' => Some("os-apple"),

        // Special private-use-area extensions found in `Ubuntu-Light.ttf`
        '\u{F000}' => Some("uniF000"),
        '\u{F001}' => Some("fi"),
        '\u{F002}' => Some("fl"),
        '\u{F506}' => Some("one seventh"),
        '\u{F507}' => Some("two sevenths"),
        '\u{F508}' => Some("three sevenths"),
        '\u{F509}' => Some("four sevenths"),
        '\u{F50A}' => Some("five sevenths"),
        '\u{F50B}' => Some("six sevenths"),
        '\u{F50C}' => Some("one ninth"),
        '\u{F50D}' => Some("two ninths"),
        '\u{F50E}' => Some("four ninths"),
        '\u{F50F}' => Some("five ninths"),
        '\u{F510}' => Some("seven ninths"),
        '\u{F511}' => Some("eight ninths"),
        '\u{F800}' => Some("zero.alt"),
        '\u{F801}' => Some("one.alt"),
        '\u{F802}' => Some("two.alt"),
        '\u{F803}' => Some("three.alt"),
        '\u{F804}' => Some("four.alt"),
        '\u{F805}' => Some("five.alt"),
        '\u{F806}' => Some("six.alt"),
        '\u{F807}' => Some("seven.alt"),
        '\u{F808}' => Some("eight.alt"),
        '\u{F809}' => Some("nine.alt"),
        '\u{F80A}' => Some("zero.sups"),
        '\u{F80B}' => Some("one.sups"),
        '\u{F80C}' => Some("two.sups"),
        '\u{F80D}' => Some("three.sups"),
        '\u{F80E}' => Some("four.sups"),
        '\u{F80F}' => Some("five.sups"),
        '\u{F810}' => Some("six.sups"),
        '\u{F811}' => Some("seven.sups"),
        '\u{F812}' => Some("eight.sups"),
        '\u{F813}' => Some("nine.sups"),
        '\u{F814}' => Some("zero.sinf"),
        '\u{F815}' => Some("one.sinf"),
        '\u{F816}' => Some("two.sinf"),
        '\u{F817}' => Some("three.sinf"),
        '\u{F818}' => Some("four.sinf"),
        '\u{F819}' => Some("five.sinf"),
        '\u{F81A}' => Some("six.sinf"),
        '\u{F81B}' => Some("seven.sinf"),
        '\u{F81C}' => Some("eight.sinf"),
        '\u{F81D}' => Some("nine.sinf"),

        _ => None,
    }
}
