//! MIT license.

pub fn parse_pre_elements(html: &str) -> String {
    let mut parsed = String::new();
    let document = scraper::Html::parse_document(html);
    let pre_selector = scraper::Selector::parse("pre").unwrap();

    for pre in document.select(&pre_selector) {
        let text_collection = pre.text().collect::<Vec<&str>>();
        for text in text_collection {
            parsed.push_str(text);
        }
    }

    return parsed;
}
