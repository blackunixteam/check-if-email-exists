// check_if_email_exists
// Copyright (C) 2018-2019 Amaury Martiny

// check_if_email_exists is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// check_if_email_exists is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with check_if_email_exists.  If not, see <http://www.gnu.org/licenses/>.

extern crate env_logger;
#[macro_use]
extern crate clap;
extern crate lettre;

// use check_if_email_exists::email_exists;
use clap::App;
use std::error::Error;
use std::io;
use std::process;

fn read_csv() -> Result<Vec<String>, Box<dyn Error>> {
	// Build the CSV reader and iterate over each record.
	let mut rdr = csv::Reader::from_reader(io::stdin());
	let mut emails = vec![];

	for result in rdr.records() {
		// The iterator yields Result<StringRecord, Error>, so we check the
		// error here.
		let mut record = result?;
		// If the csv has more than 1 column, we ignore
		record.truncate(1);
		emails.push(record.as_slice().to_string())
	}

	Ok(emails)
}

fn main() {
	env_logger::init();

	// The YAML file is found relative to the current file, similar to how modules are found
	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml).get_matches();

	let from_email = matches.value_of("FROM_EMAIL").unwrap_or("user@example.org");
	// let to_email = matches
	// 	.value_of("TO_EMAIL")
	// 	.expect("'TO_EMAIL' is required. qed.");

	match read_csv() {
		Ok(emails) => {
			println!("{:?}", emails);
			process::exit(0)
		}
		Err(err) => {
			println!("error running example: {}", err);
			process::exit(1);
		}
	}
}
