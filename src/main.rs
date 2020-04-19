extern crate scraper;

use std::env;
use std::io::{self, prelude::*};
use std::process;

use scraper::{html::Html, selector::Selector};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut handler = stdin.lock();

	let mut html = String::new();

	handler.read_to_string(&mut html)?;

	if html.is_empty() {
		eprintln!("err: no HTML supplied to stdin");
		process::exit(1);
	}

	let args: Vec<String> = env::args().collect();
	let query = args.get(1).expect("err: expected an HTML query");
	let attr = args.get(2);
	let parsed_html = Html::parse_document(html.as_str());
	let selector = Selector::parse(query).expect("err: invalid selector found");

	for el in parsed_html.select(&selector) {
		match attr {
			Some(a) => println!("{}", el.value().attr(a).unwrap_or("")),
			None => println!("{:?}", el.value()),
		}
	}

	Ok(())
}
