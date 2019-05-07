# strings
https://doc.rust-lang.org/stable/book/ch08-02-strings.html

Strings are collections of bytes - specifically they are `Vec<u8>`. However, strings are different from other collections ex. you can't index into them. The 
`String` type is implemented in the standard library and is:
- Mutable
- Owned
- UTF-8 encoded

String slices `str` are references to UTF-8 data. String literals are a type of string slice.
