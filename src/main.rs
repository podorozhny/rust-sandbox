#[macro_use]
extern crate tera;
extern crate lazy_static;
extern crate serde_json;

use std::collections::HashMap;

use tera::{Tera, Context, Result};
use serde_json::value::{Value, to_value};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = compile_templates!("app/templates/**/*");
        tera.autoescape_on(vec!["html", ".sql"]);
        tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}

pub fn do_nothing_filter(value: Value, _: HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("do_nothing_filter", "value", String, value);
    Ok(to_value(&s).unwrap())
}

fn main() {
    let mut context = Context::new();
    context.add("username", &"Putin");

    Tera::one_off("hello", &Context::new(), true).unwrap();

    match TEMPLATES.render("layout.html", &context) {
        Ok(s) => println!("{:?}", s),
        Err(e) => {
            println!("Error: {}", e);
            for e in e.iter().skip(1) {
                println!("Reason: {}", e);
            }
        }
    };
}