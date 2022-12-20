//!  MIT license.

///  Parse an HTML document for its pre-format tags and extract the text.
///  # Example
/// 
///  ```
///  let expected: &str = "SPECIES\tUCSC VERSION\tRELEASE DATE\tRELEASE NAME\tSTATUS\nChicken\tgalGal6\tMar. 2018\tGRCg6\tGallus-gallus-6.0\tAvailable\n";
///  let html: &str = "
///  <!doctype HTML>
///  <html>
///  <head>
///  </head>
///  <body>
///      <pre>SPECIES\tUCSC VERSION\tRELEASE DATE\tRELEASE NAME\tSTATUS\n</pre>
///      <pre>Chicken\tgalGal6\tMar. 2018\tGRCg6\tGallus-gallus-6.0\tAvailable\n</pre>
///  </body>
///  </html>
///  ";
///  let parsed: String = utils::html::parse_pre_elements(html);
///  assert_eq!(parsed, expected);
///  ```
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
