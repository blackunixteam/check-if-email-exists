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

extern crate lettre;
#[macro_use]
extern crate log;
extern crate native_tls;
extern crate rand;
extern crate rayon;
extern crate trust_dns_resolver;

mod mx;
mod smtp;
mod syntax;

use lettre::error::Error as LettreError;
use lettre::smtp::SMTP_PORT;
use lettre::EmailAddress;
use mx::MxLookupError;
use rayon::prelude::*;
use smtp::SmtpEmailDetails;
use std::collections::HashMap;
use std::str::FromStr;
use syntax::{address_syntax, AddressSyntax};
use trust_dns_resolver::lookup::MxLookup;

#[derive(Debug)]
/// Errors that can happen on MX lookups
pub enum MxError {
	/// Skipped checking MX records
	Skipped,
	/// Error while resolving MX lookups
	Mx(MxLookupError),
}

/// All details about email address, MX records and SMTP responses
#[derive(Debug)]
pub struct SingleEmail {
	/// Details about the MX host
	pub mx: Result<MxLookup, MxError>,
	/// Details about the SMTP responses of the email
	pub smtp: Result<SmtpEmailDetails, ()>, // TODO Better Err type
	/// Details about the email address
	pub syntax: Result<AddressSyntax, LettreError>,
}

// /// Check if all usernames exist on one domain
// fn email_details_one_domain(from_email: &str, usernames: Vec<&str>, domain: &str) -> Vec<Result<SingleEmail,SingleEmailError>> {
// 	debug!("Checking following usernames on domain '{}': {:?}", domain, usernames);

// 	debug!("Getting MX lookup...");
// 	let hosts = match mx_hosts::get_mx_lookup(domain) {
// 		Ok(h) => h,
// 		Err(MxLookupError::Io(err)) => {
// 			return vec![Err(DomainError::Io(err)), ..usernames.len()];
// 		}
// 		Err(MxLookupError::ResolveError(err)) => {
// 			return vec![Err(DomainError::MxLookup(err)), ..usernames.len()];
// 		}
// 	};
// 	let mut combinations = Vec::new(); // `(host, port)` combination
// 	for host in hosts.iter() {
// 		// We could add ports 465 and 587 too
// 		combinations.push((host.exchange(), SMTP_PORT));
// 	}
// 	let mx_details = combinations
// 		.iter()
// 		.map(|(host, _)| host.into())
// 		.collect::<Vec<String>>();
// 	debug!("Found the following MX hosts {:?}", mx_details);

// }

/// The main function: checks email format, checks MX records, and checks SMTP
/// responses to the email inbox.
pub fn emails_exist(email_addresses: Vec<&str>, from_email: &str) -> Vec<Result<(), ()>> {
	debug!("Checking list of {} emails", email_addresses.len());

	let from_email = EmailAddress::from_str(from_email).unwrap_or(
		EmailAddress::from_str("user@example.org").expect("This is a valid email. qed."),
	);

	// We want to get at the end: a HashMap between email_address (as &str), and
	// Result<SingleEmail, SingleEmailError>. To do so, we separate the
	// task into 3 steps

	// Step 1: create a HashMap between email_address and AddressSyntax
	let syntax_map: HashMap<&str, Result<AddressSyntax, LettreError>> = email_addresses
		.iter()
		.fold(HashMap::new(), |mut acc, value| {
			acc.entry(value).or_insert(address_syntax(value));
			acc
		});

	// Number of valid ones
	let valid_count = syntax_map
		.values()
		.filter(|s| s.is_ok())
		.collect::<Vec<_>>()
		.len();
	debug!(
		"Found {} valid emails, {} invalid ones",
		valid_count,
		email_addresses.len() - valid_count
	);

	// Step 2: create a HashMap between domain and Result<MxLookup, MxLookupError>
	// Partition the emails by host name
	let partition: HashMap<String, Vec<&AddressSyntax>> = syntax_map
		.values()
		.filter_map(|value| value.as_ref().ok())
		.fold(HashMap::new(), |mut acc, value| {
			let entry = acc.entry(value.domain.clone()).or_insert(vec![]);
			entry.push(&value);
			acc
		});
	let mut all_domains: Vec<&str> = Vec::new();
	for (k, _) in partition.iter() {
		all_domains.push(k);
	}
	let mx_map = all_domains
		.into_par_iter()
		.fold(
			|| HashMap::new(),
			|mut acc, domain| {
				acc.entry(domain).or_insert(mx::get_mx_lookup(domain));
				acc
			},
		)
		.reduce(
			|| HashMap::new(),
			|mut m1, m2| {
				m1.extend(m2);
				m1
			},
		);

	println!("{:?}", mx_map);

	// Step 3: create a HashMap between email_address and
	println!("{:?}", syntax_map);

	// Finally, create a map between email_address and SingleEmail
	let single_email_map: HashMap<&str, SingleEmail> =email_addresses
		.iter()
		.fold(HashMap::new(),|mut acc, value| {
			{
				let current_syntax = syntax_map.get(value).expect("We created syntax_map with email_addresses as keys. qed.");
				match current_syntax {
					Ok(s) => {
						let current_mx = mx_map.get::<str>(&s.domain).expect("We created mx_map with all email_addresses' domains. qed.");
						match current_mx {
							Ok(m) => {
								acc.entry(value).or_insert(SingleEmail {
									mx: Ok(*m),
									smtp: Err(()),
									syntax: Ok(*s)
								});
							},
							Err(err)=> {
								acc.entry(value).or_insert(SingleEmail {
									mx: Err(MxError::Mx(*err)),
									smtp: Err(()),
									syntax: Ok(*s)
								});
							}
						}
					},
					Err(err)=> {
						acc.entry(value).or_insert(SingleEmail {
									mx: Err(MxError::Skipped),
									smtp: Err(()),
									syntax: Err(*err)
								});
					}
				}
			}

			acc
		});

	// println!("{:?}", single_email_map);

	vec![Ok(())]

	// let smtp_details = combinations
	// 	// Concurrently find any combination that returns true for email_exists
	// 	.par_iter()
	// 	// Attempt to make a SMTP call to host
	// 	.flat_map(|(host, port)| {
	// 		smtp::email_details(
	// 			&from_email,
	// 			vec![&address_details.username],
	// 			host,
	// 			*port,
	// 			address_details.domain.as_str(),
	// 		)
	// 	})
	// 	.find_any(|_| true)
	// 	// If all smtp calls timed out/got refused/errored, we assume that the
	// 	// ISP is blocking relevant ports
	// 	.ok_or(EmailExistsError::BlockedByIsp)?;

	// Ok(EmailDetails {
	// 	address: address_details,
	// 	mx: mx_details,
	// 	smtp: smtp_details,
	// })
}
