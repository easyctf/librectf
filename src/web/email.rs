/* Taken from Keats/validator
 *
 * The MIT License (MIT)
 *
 * Copyright (c) 2016 Vincent Prouillet
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::borrow::Cow;
use std::net::IpAddr;
use std::str::FromStr;

use idna::domain_to_ascii;
use regex::Regex;

lazy_static! {
    // Regex from the specs
    // https://html.spec.whatwg.org/multipage/forms.html#valid-e-mail-address
    // It will mark esoteric email addresses like quoted string as invalid
    static ref EMAIL_USER_RE: Regex = Regex::new(r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+\z").unwrap();
    static ref EMAIL_DOMAIN_RE: Regex = Regex::new(
        r"(?i)^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$"
    ).unwrap();
    // literal form, ipv4 or ipv6 address (SMTP 4.1.3)
    static ref EMAIL_LITERAL_RE: Regex = Regex::new(r"(?i)\[([A-f0-9:\.]+)\]\z").unwrap();
}

/// Validates whether the given string is an IP V4
pub fn validate_ip_v4<'a, T>(val: T) -> bool
where
    T: Into<Cow<'a, str>>,
{
    match IpAddr::from_str(val.into().as_ref()) {
        Ok(i) => match i {
            IpAddr::V4(_) => true,
            IpAddr::V6(_) => false,
        },
        Err(_) => false,
    }
}

/// Validates whether the given string is an IP V6
pub fn validate_ip_v6<'a, T>(val: T) -> bool
where
    T: Into<Cow<'a, str>>,
{
    match IpAddr::from_str(val.into().as_ref()) {
        Ok(i) => match i {
            IpAddr::V4(_) => false,
            IpAddr::V6(_) => true,
        },
        Err(_) => false,
    }
}

/// Validates whether the given string is an IP
pub fn validate_ip<'a, T>(val: T) -> bool
where
    T: Into<Cow<'a, str>>,
{
    match IpAddr::from_str(val.into().as_ref()) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Validates whether the given string is an email based on Django `EmailValidator` and HTML5 specs
pub fn validate_email<'a, T>(val: T) -> bool
where
    T: Into<Cow<'a, str>>,
{
    let val = val.into();
    if val.is_empty() || !val.contains('@') {
        return false;
    }
    let parts: Vec<&str> = val.rsplitn(2, '@').collect();
    let user_part = parts[1];
    let domain_part = parts[0];

    if !EMAIL_USER_RE.is_match(user_part) {
        return false;
    }

    if !validate_domain_part(domain_part) {
        // Still the possibility of an [IDN](https://en.wikipedia.org/wiki/Internationalized_domain_name)
        return match domain_to_ascii(domain_part) {
            Ok(d) => validate_domain_part(&d),
            Err(_) => false,
        };
    }

    true
}

/// Checks if the domain is a valid domain and if not, check whether it's an IP
fn validate_domain_part(domain_part: &str) -> bool {
    if EMAIL_DOMAIN_RE.is_match(domain_part) {
        return true;
    }

    // maybe we have an ip as a domain?
    match EMAIL_LITERAL_RE.captures(domain_part) {
        Some(caps) => match caps.get(1) {
            Some(c) => validate_ip(c.as_str()),
            None => false,
        },
        None => false,
    }
}
