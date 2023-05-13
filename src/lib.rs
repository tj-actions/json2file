// Copyright ⓒ 2021 Tonye Jack.
// Licensed under the MIT license
// (see LICENSE or <http://opensource.org/licenses/MIT>).


//! # json2file provides a convenient way to write JSON data to file(s).
//! 
//! It includes a [writer](crate::writer) module that takes input JSON data and a set of arguments specifying the output file(s) to write to, and writes the data to the specified file(s) in the specified format.
//! 
//! This library is designed to be easy to use and flexible, with support for writing JSON data to multiple files in different formats, including JSON, and plain text.

#![deny(missing_docs)]

pub mod writer;
