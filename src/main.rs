use std::path::{Path, PathBuf};

use clap::Parser;
use opml::{Outline, OPML};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct Root {
	subscriptions: Vec<Subscription>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct Subscription {
	name: String,
	url: String,
}

fn parse(opml_file: &Path) -> OPML {
	let mut content = std::fs::File::open(opml_file).unwrap();
	OPML::from_reader(&mut content).unwrap()
}

fn get_subs(opml: OPML) -> Vec<Outline> {
	opml.body
		.outlines
		.into_iter()
		.find(|o| o.text == "YouTube Subscriptions")
		.unwrap()
		.outlines
}

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[clap(version, about)]
struct Args {
	input: PathBuf,
	output: PathBuf,
}

fn main() {
	let args = Args::parse();

	let input = args.input;
	let output = args.output;

	let opml = parse(Path::new(&input));
	let subs = get_subs(opml);

	let mut subscriptions = Vec::with_capacity(subs.len());

	for outline in subs {
		if let Outline { text: name, xml_url: Some(url), .. } = outline {
			let sub = Subscription {
				name,
				url: url.replace("feeds/videos.xml?channel_id=", "channel/"),
			};
			subscriptions.push(sub);
		} else {
			eprintln!("No url for {}", outline.text);
		}
	}

	let output = std::fs::OpenOptions::new()
		.create(true)
		.truncate(true)
		.write(true)
		.open(Path::new(&output))
		.unwrap();

	serde_json::to_writer_pretty(output, &Root { subscriptions }).unwrap();
}
