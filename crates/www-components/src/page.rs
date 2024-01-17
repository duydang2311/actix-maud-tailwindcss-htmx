use maud::{html, Markup, DOCTYPE};

pub fn page(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) }
                link href="/static/base.css" rel="stylesheet";
            }
            body {
                (body)
            }
        }
    }
}
