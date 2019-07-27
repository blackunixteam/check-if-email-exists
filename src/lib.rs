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

mod mx_hosts;
mod smtp;

use lettre::error::Error as LettreError;
use lettre::smtp::SMTP_PORT;
use lettre::EmailAddress;
use mx_hosts::MxLookupError;
use rayon::prelude::*;
use smtp::SmtpEmailDetails;
use std::io::Error as IoError;
use std::str::FromStr;
use trust_dns_resolver::error::ResolveError;

/// Errors generated while connecting to the domain host
#[derive(Debug)]
pub enum DomainError {
	/// ISP is blocking SMTP ports
	BlockedByIsp,
	/// To email address formatting error
	FromAddressError(LettreError),
	/// IO error
	Io(IoError),
	///Error while resolving MX lookups
	MxLookup(ResolveError),
	/// To email address formatting error
	ToAddressError(LettreError),
}

/// Errors concerning one single email address
#[derive(Debug)]
pub enum SingleEmailError {
	/// Error related to the domain host
	DomainError,
	/// To email address formatting error
	ToAddressError(LettreError),
}

/// Information after parsing an email address
#[derive(Debug)]
pub struct AddressDetails {
	/// The email address as a lettre EmailAddress
	pub address: EmailAddress,
	/// The domain name, after "@"
	pub domain: String,
	/// The username, before "@"
	pub username: String,
	/// Is the email in a valid format?
	pub valid_format: bool,
}

/// All details about the host domain
pub struct DomainDetails {
	/// Details about the MX records of the domain
	pub mx: Vec<String>,
}

/// All details about email address, MX records and SMTP responses
#[derive(Debug)]
pub struct SingleEmailDetails {
	/// Details about the email address
	pub address: AddressDetails,
	/// Details about the SMTP responses of the email
	pub smtp: SmtpEmailDetails,
}

// /// Check if all usernames exist on one domain
// fn email_details_one_domain(from_email: &str, usernames: Vec<&str>, domain: &str) -> Vec<Result<SingleEmailDetails,SingleEmailError>> {
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
// 		.map(|(host, _)| host.to_string())
// 		.collect::<Vec<String>>();
// 	debug!("Found the following MX hosts {:?}", mx_details);


// }

/// The main function: checks email format, checks MX records, and checks SMTP
/// responses to the email inbox.
pub fn email_exists(from_email: &str, to_emails: Vec<&str>) -> () {
	debug!("Preparing to check for {} emails", to_emails.len());

	// let from_email = match EmailAddress::from_str(from_email) {
	// 	Ok(email) => email,
	// 	Err(err) => return Err(EmailExistsError::FromAddressError(err)),
	// };


	// let iter: &str = to_email.as_ref();
	// let mut iter = iter.split("@");
	// let username = iter
	// 	.next()
	// 	.expect("We checked above that email is valid. qed.")
	// 	.to_string();
	// let domain = iter
	// 	.next()
	// 	.expect("We checked above that email is valid. qed.")
	// 	.to_string();

	// let address_details = AddressDetails {
	// 	address: to_email,
	// 	domain,
	// 	username,
	// 	valid_format: true,
	// };
	// debug!("Details of the email address: {:?}", address_details);

	// let to_email = match EmailAddress::from_str(to_email) {
	// 	Ok(email) => email,
	// 	Err(err) => return Err(EmailExistsError::ToAddressError(err)),
	// };

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
