// Modified version of https://github.com/zonyitoo/rust-ini
// To work with pacman-conf

// The MIT License (MIT)

// Copyright (c) 2014 Y. T. CHUNG

// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! Ini

use std::borrow::Borrow;
use std::char;
use std::cmp::Eq;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::hash_map::{IntoIter, Iter, IterMut, Keys};
use std::error;
use std::fmt::{self, Display};
use std::fs::{File, OpenOptions};
use std::hash::Hash;
use std::io::{self, Read, Write};
use std::ops::{Index, IndexMut};
use std::path::Path;
use std::str::Chars;

use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt::{Formatter};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EscapePolicy {
    /// escape absolutely nothing (dangerous)
    Nothing,
    /// only escape the most necessary things
    Basics,
    /// escape basics and non-ascii characters
    BasicsUnicode,
    /// Escape reserved symbols.
    Reserved,
    /// Escape reserved symbols and non-ascii characters
    ReservedUnicode,
    /// Escape everything that some INI implementations assume
    Everything,
}

impl EscapePolicy {
    fn escape_basics(&self) -> bool {
        match *self {
            EscapePolicy::Nothing => false,
            _ => true,
        }
    }

    fn escape_reserved(&self) -> bool {
        match *self {
            EscapePolicy::Reserved => true,
            EscapePolicy::ReservedUnicode => true,
            EscapePolicy::Everything => true,
            _ => false,
        }
    }

    fn escape_unicode(&self) -> bool {
        match *self {
            EscapePolicy::BasicsUnicode => true,
            EscapePolicy::ReservedUnicode => true,
            EscapePolicy::Everything => true,
            _ => false,
        }
    }

    /// Given a character this returns true if it should be escaped as
    /// per this policy or false if not.
    pub fn should_escape(&self, c: char) -> bool {
        match c {
            '\\' | '\x00'...'\x1f' | '\x7f'...'\u{00ff}' => self.escape_basics(),
            ';' | '#' | '=' | ':' => self.escape_reserved(),
            '\u{0080}'...'\u{FFFF}' => self.escape_unicode(),
            _ => false,
        }
    }
}

// Escape non-INI characters
//
// Common escape sequences: https://en.wikipedia.org/wiki/INI_file#Escape_characters
//
// * `\\` \ (a single backslash, escaping the escape character)
// * `\0` Null character
// * `\a` Bell/Alert/Audible
// * `\b` Backspace, Bell character for some applications
// * `\t` Tab character
// * `\r` Carriage return
// * `\n` Line feed
// * `\;` Semicolon
// * `\#` Number sign
// * `\=` Equals sign
// * `\:` Colon
// * `\x????` Unicode character with hexadecimal code point corresponding to ????
fn escape_str(s: &str, policy: EscapePolicy) -> String {
    let mut escaped: String = String::with_capacity(s.len());
    for c in s.chars() {
        // if we know this is not something to escape as per policy, we just
        // write it and continue.
        if !policy.should_escape(c) {
            escaped.push(c);
            continue;
        }

        match c {
            '\\' => escaped.push_str("\\\\"),
            '\0' => escaped.push_str("\\0"),
            '\x01'...'\x06' | '\x0e'...'\x1f' | '\x7f'...'\u{00ff}' => {
                escaped.push_str(&format!("\\x{:04x}", c as isize)[..])
            }
            '\x07' => escaped.push_str("\\a"),
            '\x08' => escaped.push_str("\\b"),
            '\x0c' => escaped.push_str("\\f"),
            '\x0b' => escaped.push_str("\\v"),
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            '\r' => escaped.push_str("\\r"),
            '\u{0080}'...'\u{FFFF}' => escaped.push_str(&format!("\\x{:04x}", c as isize)[..]),
            _ => {
                escaped.push('\\');
                escaped.push(c);
            }
        }
    }
    escaped
}

/// Parsing configuration
pub struct ParseOption {
    /// Allow quote (" or ') in value
    /// For example
    /// ```ini
    /// [Section]
    /// Key1="Quoted value"
    /// Key2='Single Quote' with extra value
    /// ```
    ///
    /// In this example, Value of `Key1` is `Quoted value`,
    /// and value of `Key2` is `Single Quote with extra value`
    /// if `enabled_quote` is set to `true`.
    pub enabled_quote: bool,

    /// Interpret `\` as an escape character
    /// For example
    /// ```ini
    /// [Section]
    /// Key1=C:\Windows
    /// ```
    ///
    /// If `enabled_escape` is true, then the value of `Key` will become `C:Windows` (`\W` equals to `W`).
    pub enabled_escape: bool,
}

impl Default for ParseOption {
    fn default() -> ParseOption {
        ParseOption { enabled_quote: true,
                      enabled_escape: true, }
    }
}

/// A setter which could be used to set key-value pair in a specified section
pub struct SectionSetter<'a> {
    ini: &'a mut Ini,
    section_name: Option<String>,
}

impl<'a> SectionSetter<'a> {
    fn new(ini: &'a mut Ini, section_name: Option<String>) -> SectionSetter<'a> {
        SectionSetter { ini, section_name, }
    }

    /// Set key-value pair in this section
    pub fn set<K, V>(&'a mut self, key: K, value: V) -> &'a mut SectionSetter<'a>
        where K: Into<String>,
              V: Into<String>
    {
        {
            let prop = match self.ini.sections.entry(self.section_name.clone()) {
                Entry::Vacant(entry) => entry.insert(HashMap::new()),
                Entry::Occupied(entry) => entry.into_mut(),
            };
            let str_val : String = value.into();
            prop.insert(key.into(), str_val.into());
        }
        self
    }

    /// Delete the entry in this section with `key`
    pub fn delete<K>(&'a mut self, key: &K) -> &'a mut SectionSetter<'a>
        where String: Borrow<K>,
              K: Hash + Eq + ?Sized
    {
        if let Some(prop) = self.ini.sections.get_mut(&self.section_name) {
            prop.remove(key);
        }
        self
    }

    /// Get the entry in this section with `key`
    pub fn get<K>(&'a mut self, key: &K) -> Option<Vec<&'a str>>
        where String: Borrow<K>,
              K: Hash + Eq + ?Sized
    {
        self.ini.sections
            .get(&self.section_name)
            .and_then(|prop| prop.get(key)
                .map(|s| 
                    s.iter()
                    .map(|ss| &ss[..])
                    .collect()
                )
            )
    }
}

#[derive(Clone, Default)]
pub struct PropertyValueList {
    values: Vec<String>,
}

impl<I> Index<I> for PropertyValueList
where
    I: ::core::slice::SliceIndex<[String]>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&self.values, index)
    }
}

impl PropertyValueList{
    pub fn new() -> Self{
        PropertyValueList{
            values: Vec::new(),
        }
    }
    pub fn to_string(&self) -> String {
        self.values.join(",")
    }

    pub fn parse<T: FromStr>(&self) -> Result<T, T::Err> {
        self.to_string().parse()
    }

    pub fn iter(&self) -> std::slice::Iter<String> {
        self.values.iter()
    }

    pub fn push(&mut self, v : &mut String){
        self.values.push(v.to_string())
    }
}

impl Display for PropertyValueList{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.to_string().fmt(f)
    }
}

impl Iterator for PropertyValueList{
    type Item = String;
    fn next(&mut self) -> Option<String> {
        self.values
            .iter()
            .map(|s| s.to_string())
            .next()
    }
}

/*
impl Into<bool> for PropertyValueList {
    fn into(self) -> bool {
        self.to_string().into()
    }
}
*/


impl From<String> for PropertyValueList {
    fn from(v: String) -> Self{
        PropertyValueList{
            values: vec!(v),
        }
    }
}


impl From<&str> for PropertyValueList {
    fn from(v: &str) -> Self{
        PropertyValueList{
            values: vec!(v.into()),
        }
    }
}

/// Properties type (key-value pairs)
pub type Properties = HashMap<String, PropertyValueList>; // Key-value pairs

/// Ini struct
#[derive(Clone, Default)]
pub struct Ini {
    sections: HashMap<Option<String>, Properties>,
}

impl Ini {
    /// Create an instance
    pub fn new() -> Ini {
        Default::default()
    }

    /// Set with a specified section, `None` is for the general section
    pub fn with_section<S>(&mut self, section: Option<S>) -> SectionSetter
        where S: Into<String>
    {
        SectionSetter::new(self, section.map(|s| s.into()))
    }

    /// Get the immmutable general section
    pub fn general_section(&self) -> &Properties {
        self.section(None::<String>).expect("There is no general section in this Ini")
    }

    /// Get the mutable general section
    pub fn general_section_mut(&mut self) -> &mut Properties {
        self.section_mut(None::<String>).expect("There is no general section in this Ini")
    }

    /// Get a immutable section
    pub fn section<S>(&self, name: Option<S>) -> Option<&Properties>
        where S: Into<String>
    {
        self.sections.get(&name.map(|s| s.into()))
    }

    /// Get a mutable section
    pub fn section_mut<S>(&mut self, name: Option<S>) -> Option<&mut Properties>
        where S: Into<String>
    {
        self.sections.get_mut(&name.map(|s| s.into()))
    }

    /// Get the entry
    pub fn entry(&mut self, name: Option<String>) -> Entry<Option<String>, Properties> {
        self.sections.entry(name.map(|s| s))
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.sections.clear()
    }

    /// Iterate with sections
    pub fn sections(&self) -> Keys<Option<String>, Properties> {
        self.sections.keys()
    }

    /// Set key-value to a section
    pub fn set_to<S>(&mut self, section: Option<S>, key: String, value: String)
        where S: Into<String>
    {
        self.with_section(section).set(key, value);
    }

    /// Get the value from a section with key
    ///
    /// Example:
    ///
    /// ```
    /// use pacman::ini::Ini;
    /// let input = "[sec]\nabc = def\n";
    /// let ini = Ini::load_from_str(input).unwrap();
    /// ```
    pub fn get_from<'a, S>(&'a self, section: Option<S>, key: &str) -> Option<&'a [String]>
        where S: Into<String>
    {
        match self.sections.get(&section.map(|s| s.into())) {
            None => None,
            Some(ref prop) => match prop.get(key) {
                Some(p) => Some(&p[..]),
                None => None,
            },
        }
    }

    /// Delete a section, return the properties if it exists
    pub fn delete<S>(&mut self, section: Option<S>) -> Option<Properties>
        where S: Into<String>
    {
        self.sections.remove(&section.map(|s| s.into()))
    }

}

impl<'q> Index<&'q Option<String>> for Ini {
    type Output = Properties;

    fn index<'a>(&'a self, index: &'q Option<String>) -> &'a Properties {
        match self.sections.get(index) {
            Some(p) => p,
            None => panic!("Section `{:?}` does not exist", index),
        }
    }
}

impl<'i> IndexMut<&'i Option<String>> for Ini {
    fn index_mut<'a>(&'a mut self, index: &Option<String>) -> &'a mut Properties {
        match self.sections.get_mut(index) {
            Some(p) => p,
            None => panic!("Section `{:?}` does not exist", index),
        }
    }
}

impl<'q> Index<&'q str> for Ini {
    type Output = Properties;

    fn index<'a>(&'a self, index: &'q str) -> &'a Properties {
        match self.sections.get(&Some(index.into())) {
            Some(p) => p,
            None => panic!("Section `{}` does not exist", index),
        }
    }
}

impl<'q> IndexMut<&'q str> for Ini {
    fn index_mut<'a>(&'a mut self, index: &'q str) -> &'a mut Properties {
        match self.sections.get_mut(&Some(index.into())) {
            Some(p) => p,
            None => panic!("Section `{}` does not exist", index),
        }
    }
}

impl Ini {
    /// Load from a string
    pub fn load_from_str(buf: &str) -> Result<Ini, ParseError> {
        Ini::load_from_str_opt(buf, ParseOption::default())
    }

    /// Load from a string, but do not interpret '\' as an escape character
    pub fn load_from_str_noescape(buf: &str) -> Result<Ini, ParseError> {
        Ini::load_from_str_opt(buf,
                               ParseOption { enabled_escape: false,
                                             ..ParseOption::default() })
    }

    /// Load from a string with options
    pub fn load_from_str_opt(buf: &str, opt: ParseOption) -> Result<Ini, ParseError> {
        let mut parser = Parser::new(buf.chars(), opt);
        parser.parse()
    }

    /// Load from a reader
    pub fn read_from<R: Read>(reader: &mut R) -> Result<Ini, Error> {
        Ini::read_from_opt(reader, ParseOption::default())
    }

    /// Load from a reader, but do not interpret '\' as an escape character
    pub fn read_from_noescape<R: Read>(reader: &mut R) -> Result<Ini, Error> {
        Ini::read_from_opt(reader,
                           ParseOption { enabled_escape: false,
                                         ..ParseOption::default() })
    }

    /// Load from a reader with options
    pub fn read_from_opt<R: Read>(reader: &mut R, opt: ParseOption) -> Result<Ini, Error> {
        let mut s = String::new();
        reader.read_to_string(&mut s).map_err(Error::Io)?;
        let mut parser = Parser::new(s.chars(), opt);
        match parser.parse() {
            Err(e) => Err(Error::Parse(e)),
            Ok(success) => Ok(success),
        }
    }

    /// Load from a file
    pub fn load_from_file<P: AsRef<Path>>(filename: P) -> Result<Ini, Error> {
        Ini::load_from_file_opt(filename, ParseOption::default())
    }

    /// Load from a file, but do not interpret '\' as an escape character
    pub fn load_from_file_noescape<P: AsRef<Path>>(filename: P) -> Result<Ini, Error> {
        Ini::load_from_file_opt(filename,
                                ParseOption { enabled_escape: false,
                                              ..ParseOption::default() })
    }

    /// Load from a file with options
    pub fn load_from_file_opt<P: AsRef<Path>>(filename: P, opt: ParseOption) -> Result<Ini, Error> {
        let mut reader = match File::open(filename.as_ref()) {
            Err(e) => {
                return Err(Error::Io(e));
            }
            Ok(r) => r,
        };
        Ini::read_from_opt(&mut reader, opt)
    }
}

/// Iterator for sections
pub struct SectionIterator<'a> {
    mapiter: Iter<'a, Option<String>, Properties>,
}

/// Iterator for mutable sections
pub struct SectionMutIterator<'a> {
    mapiter: IterMut<'a, Option<String>, Properties>,
}

impl<'a> Ini {
    /// Immutable iterate though sections
    pub fn iter(&'a self) -> SectionIterator<'a> {
        SectionIterator { mapiter: self.sections.iter(), }
    }

    /// Mutable iterate though sections
    /// *Deprecated! Use `iter_mut` instead!*
    pub fn mut_iter(&'a mut self) -> SectionMutIterator<'a> {
        SectionMutIterator { mapiter: self.sections.iter_mut(), }
    }

    /// Mutable iterate though sections
    pub fn iter_mut(&'a mut self) -> SectionMutIterator<'a> {
        SectionMutIterator { mapiter: self.sections.iter_mut(), }
    }
}

impl<'a> Iterator for SectionIterator<'a> {
    type Item = (&'a Option<String>, &'a Properties);

    #[inline]
    fn next(&mut self) -> Option<(&'a Option<String>, &'a Properties)> {
        self.mapiter.next()
    }
}

impl<'a> Iterator for SectionMutIterator<'a> {
    type Item = (&'a Option<String>, &'a mut Properties);

    #[inline]
    fn next(&mut self) -> Option<(&'a Option<String>, &'a mut Properties)> {
        self.mapiter.next()
    }
}

impl<'a> IntoIterator for &'a Ini {
    type Item = (&'a Option<String>, &'a Properties);
    type IntoIter = SectionIterator<'a>;

    fn into_iter(self) -> SectionIterator<'a> {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Ini {
    type Item = (&'a Option<String>, &'a mut Properties);
    type IntoIter = SectionMutIterator<'a>;

    fn into_iter(self) -> SectionMutIterator<'a> {
        self.iter_mut()
    }
}

pub struct SectionIntoIter {
    iter: IntoIter<Option<String>, Properties>,
}

impl Iterator for SectionIntoIter {
    type Item = (Option<String>, Properties);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl IntoIterator for Ini {
    type Item = (Option<String>, Properties);
    type IntoIter = SectionIntoIter;

    fn into_iter(self) -> SectionIntoIter {
        SectionIntoIter { iter: self.sections.into_iter(), }
    }
}

// Ini parser
struct Parser<'a> {
    ch: Option<char>,
    rdr: Chars<'a>,
    line: usize,
    col: usize,
    opt: ParseOption,
}

#[derive(Debug)]
/// Parse error
pub struct ParseError {
    pub line: usize,
    pub col: usize,
    pub msg: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{} {}", self.line, self.col, self.msg)
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        self.msg.as_str()
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Debug)]
pub enum Error {
 Io(io::Error),
 Parse(ParseError),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::Parse(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::Parse(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => err.cause(),
            Error::Parse(ref err) => err.cause(),
        }
    }
}

impl<'a> Parser<'a> {
    // Create a parser
    pub fn new(rdr: Chars<'a>, opt: ParseOption) -> Parser<'a> {
        let mut p = Parser { ch: None,
                             line: 0,
                             col: 0,
                             rdr,
                             opt, };
        p.bump();
        p
    }

    fn eof(&self) -> bool {
        self.ch.is_none()
    }

    fn bump(&mut self) {
        self.ch = self.rdr.next();
        match self.ch {
            Some('\n') => {
                self.line += 1;
                self.col = 0;
            }
            Some(..) => {
                self.col += 1;
            }
            None => {}
        }
    }

    fn error<U>(&self, msg: String) -> Result<U, ParseError> {
        Err(ParseError { line: self.line,
                         col: self.col,
                         msg, })
    }

    /// Consume all the white space until the end of the line or a tab
    fn parse_whitespace(&mut self) {
        while let Some(c) = self.ch {
            if !c.is_whitespace() && c != '\n' && c != '\t' && c != '\r' {
                break;
            }
            self.bump();
        }
    }

    /// Consume all the white space except line break
    fn parse_whitespace_except_line_break(&mut self) {
        while let Some(c) = self.ch {
            if (c == '\n' || c == '\r' || !c.is_whitespace()) && c != '\t' {
                break;
            }
            self.bump();
        }
    }

    /// Parse the whole INI input
    pub fn parse(&mut self) -> Result<Ini, ParseError> {
        let mut result = Ini::new();
        let mut curkey: String = "".into();
        let mut cursec: Option<String> = None;

        self.parse_whitespace();
        while let Some(cur_ch) = self.ch {
            match cur_ch {
                ';' | '#' => {
                    self.parse_comment();
                }
                '[' => match self.parse_section() {
                    Ok(sec) => {
                        let msec = &sec[..].trim();
                        cursec = Some(msec.to_string());
                        result.sections.entry(cursec.clone()).or_insert_with(HashMap::new);
                        self.bump();
                    }
                    Err(e) => return Err(e),
                },
                '=' | ':' => {
                    if (&curkey[..]).is_empty() {
                        return self.error("Missing key".to_string());
                    }
                    match self.parse_val() {
                        Ok(val) => {
                            let mut mval = val[..].trim().to_owned();
                            let sec = result.sections.entry(cursec.clone()).or_insert_with(HashMap::new);
                            let val = sec.entry(curkey).or_insert_with(PropertyValueList::new);
                            val.push(&mut mval);
                            curkey = "".into();
                        }
                        Err(e) => return Err(e),
                    }
                }
                _ => match self.parse_key() {
                    
                    Ok(key) => {
                        if !(&curkey[..]).is_empty() {
                            let sec = result.sections.entry(cursec.clone()).or_insert_with(HashMap::new);
                            sec.insert(curkey, "true".into());
                       }
                        let mkey: String = key[..].trim().to_owned();
                        curkey = mkey;
                    }
                    Err(e) => return Err(e),
                },
            }

            self.parse_whitespace();
        }

        Ok(result)
    }

    fn parse_comment(&mut self) {
        while let Some(c) = self.ch {
            self.bump();
            if c == '\n' {
                break;
            }
        }
    }

    fn parse_str_until(&mut self, endpoint: &[Option<char>]) -> Result<String, ParseError> {
        let mut result: String = String::new();

        while !endpoint.contains(&self.ch) {
            match self.ch {
                None => {
                    return self.error(format!("Expecting \"{:?}\" but found EOF.", endpoint));
                }
                Some('\\') if self.opt.enabled_escape => {
                    self.bump();
                    if self.eof() {
                        return self.error(format!("Expecting \"{:?}\" but found EOF.", endpoint));
                    }
                    match self.ch.unwrap() {
                        '0' => result.push('\0'),
                        'a' => result.push('\x07'),
                        'b' => result.push('\x08'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        'n' => result.push('\n'),
                        '\n' => (),
                        'x' => {
                            // Unicode 4 character
                            let mut code: String = String::with_capacity(4);
                            for _ in 0..4 {
                                self.bump();
                                if self.eof() {
                                    return self.error(format!("Expecting \"{:?}\" but found EOF.", endpoint));
                                } else if let Some('\\') = self.ch {
                                    self.bump();
                                    if self.ch != Some('\n') {
                                        return self.error(format!("Expecting \"\\\\n\" but \
                                                                   found \"{:?}\".",
                                                                  self.ch));
                                    }
                                }
                                code.push(self.ch.unwrap());
                            }
                            let r = u32::from_str_radix(&code[..], 16);
                            match r {
                                Ok(c) => result.push(char::from_u32(c).unwrap()),
                                Err(_) => return self.error("Unknown character.".to_string()),
                            }
                        }
                        c => result.push(c),
                    }
                }
                Some(c) => {
                    result.push(c);
                }
            }
            self.bump();
        }
        Ok(result)
    }

    fn parse_section(&mut self) -> Result<String, ParseError> {
        // Skip [
        self.bump();
        self.parse_str_until(&[Some(']')])
    }

    fn parse_key(&mut self) -> Result<String, ParseError> {
        self.parse_str_until(&[Some('='), Some(':'), Some('\n')])
    }

    fn parse_val(&mut self) -> Result<String, ParseError> {
        self.bump();
        // Issue #35: Allow empty value
        self.parse_whitespace_except_line_break();

        match self.ch {
            None => Ok(String::new()),
            Some('"') if self.opt.enabled_quote => {
                self.bump();
                self.parse_str_until(&[Some('"')]).and_then(|s| {
                                                                self.bump(); // Eats the last "
                                                                             // Parse until EOL
                                                                self.parse_str_until_eol().map(|x| s + &x)
                                                            })
            }
            Some('\'') if self.opt.enabled_quote => {
                self.bump();
                self.parse_str_until(&[Some('\'')]).and_then(|s| {
                                                                 self.bump(); // Eats the last '
                                                                              // Parse until EOL
                                                                 self.parse_str_until_eol().map(|x| s + &x)
                                                             })
            }
            _ => self.parse_str_until_eol(),
        }
    }

    fn parse_str_until_eol(&mut self) -> Result<String, ParseError> {
        self.parse_str_until(&[Some('\n'), Some('\r'), Some(';'), Some('#'), None])
    }
}
