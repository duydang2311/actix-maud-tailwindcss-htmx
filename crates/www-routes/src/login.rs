use actix_web::Responder;
use maud::html;

fn markup() -> maud::Markup {
    html! {
        html {
            body {
                h1 { "Login route" }
            }
        }
    }
}

pub async fn route() -> impl Responder {
    markup()
}
