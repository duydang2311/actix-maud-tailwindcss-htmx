use actix_web::Responder;
use maud::html;
use www_components::page;

fn markup() -> maud::Markup {
    html! {
        (page("Home", html! {
            h1 class="bg-blue-500" { "Heading 1" }
            button class="px-4 py-2 border border-gray-200 bg-gray-50" { "Log in" }
        }))
    }
}

pub async fn route() -> impl Responder {
    markup()
}
