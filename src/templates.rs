use askama::Template;

#[derive(Template)]
#[template(path = "template.html")]
pub struct MyTemplate {}
