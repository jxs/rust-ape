//! A library for reading and writing APEv2 tags.
//!
//! An APE tag is a tag used to add metadata (title, artist, album, etc...) to digital audio files.
//!
//! Read the [specification][1] for more information.
//! [1]: http://wiki.hydrogenaud.io/index.php?title=APEv2_specification
//!
//! # Examples
//!
//! ## Creating a tag
//!
//! ```no_run
//! use ape::{Tag, Item};
//!
//! let mut tag = Tag::new();
//! let item = Item::from_text("artist", "Artist Name").unwrap();
//! tag.set_item(item);
//! tag.write("path/to/file").unwrap();
//! ```
//!
//! ## Reading a tag
//!
//! ```no_run
//! use ape::read;
//!
//! let tag = read("path/to/file").unwrap();
//! let item = tag.item("artist").unwrap();
//! println!("{:?}", item.value);
//! ```
//!
//! ## Updating a tag
//!
//! ```no_run
//! use ape::{read, Item};
//!
//! let path = "path/to/file";
//! let mut tag = read(path).unwrap();
//! let item = Item::from_text("album", "Album Name").unwrap();
//! tag.set_item(item);
//! tag.remove_item("cover");
//! tag.write(path).unwrap();
//! ```
//!
//! ## Deleting a tag
//!
//! ```no_run
//! use ape::remove;
//!
//! remove("path/to/file").unwrap();
//! ```
//!

#![warn(missing_docs)]

extern crate byteorder;

pub use error::{Result, Error};
pub use item::{Item, ItemValue};
pub use tag::{Tag, read, remove};

mod error;
mod item;
mod meta;
mod tag;
mod util;
