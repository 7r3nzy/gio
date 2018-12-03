#![cfg(feature = "v2_36")]

extern crate glib;
extern crate gio;

use std::io;
use gio::prelude::*;

#[test]
fn std_io_copy_with_gio() {
    let bytes = glib::Bytes::from_owned([1, 2, 3]);
    let mut read = gio::MemoryInputStream::new_from_bytes(&bytes).into_read();
    let mut write = gio::MemoryOutputStream::new_resizable().into_write();

    let result = io::copy(&mut read, &mut write);

    let out_stream = write.into_output_stream();
    out_stream.close(None).unwrap();
    assert_eq!(result.unwrap(), 3);
    assert_eq!(out_stream.steal_as_bytes().unwrap(), bytes);
}
