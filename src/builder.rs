// Copyright 2013-2014 The Rust Project Developers.
// Copyright 2018 The Uuid Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A Builder type for [`Uuid`]s.
//!
//! [`Uuid`]: ../struct.Uuid.html

use prelude::*;
use BytesError;

/// A builder struct for creating a [`Uuid`]
#[allow(missing_copy_implementations)]
#[derive(Debug)]
pub struct Builder(Uuid);

impl Builder {
    /// Creates a `Builder` using the supplied big-endian bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use uuid::Builder;
    /// use uuid::Bytes;
    ///
    /// let bytes: Bytes = [
    ///     70, 235, 208, 238, 14, 109, 67, 201, 185, 13, 204, 195, 90, 145, 63,
    ///     62,
    /// ];
    ///
    /// let mut builder = Builder::from_bytes_be(bytes);
    /// let uuid = builder.build().to_hyphenated().to_string();
    ///
    /// let expected_uuid = String::from("46ebd0ee-0e6d-43c9-b90d-ccc35a913f3e");
    ///
    /// assert_eq!(expected_uuid, uuid);
    /// ```
    ///
    /// An incorrect number of bytes:
    ///
    /// ```compile_fail
    /// use uuid::Builder;
    /// use uuid::Bytes;
    ///
    /// let bytes: Bytes = [4, 54, 67, 12, 43, 2, 98, 76]; // doesn't compile
    ///
    /// let uuid = Builder::from_bytes_le(bytes);
    /// ```
    pub fn from_bytes_be(b: Bytes) -> Self {
        Builder(Uuid::from_bytes_be(b))
    }

    /// Creates a `Builder` using the supplied little-endian bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use uuid::Builder;
    /// use uuid::Bytes;
    ///
    /// let bytes: Bytes = [
    ///     70, 235, 208, 238, 14, 109, 67, 201, 185, 13, 204, 195, 90, 145, 63,
    ///     62,
    /// ];
    ///
    /// let mut builder = Builder::from_bytes_le(bytes);
    /// let uuid = builder.build().to_hyphenated().to_string();
    ///
    /// let expected_uuid = String::from("eed0eb46-6d0e-c943-b90d-ccc35a913f3e");
    ///
    /// assert_eq!(expected_uuid, uuid);
    /// ```
    ///
    /// An incorrect number of bytes:
    ///
    /// ```compile_fail
    /// use uuid::Builder;
    /// use uuid::Bytes;
    ///
    /// let bytes: Bytes = [4, 54, 67, 12, 43, 2, 98, 76]; // doesn't compile
    ///
    /// let uuid = Builder::from_bytes_le(bytes);
    /// ```
    pub fn from_bytes_le(b: Bytes) -> Self {
        Builder(Uuid::from_bytes_le(b))
    }

    /// Creates a `Builder` using the supplied big-endian bytes.
    ///
    /// # Errors
    ///
    /// This function will return an error if `b` has any length other than 16.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use uuid::Builder;
    ///
    /// let bytes = [4, 54, 67, 12, 43, 2, 98, 76, 32, 50, 87, 5, 1, 33, 43, 87];
    ///
    /// let builder = Builder::from_slice_be(&bytes);
    /// let uuid =
    ///     builder.map(|mut builder| builder.build().to_hyphenated().to_string());
    ///
    /// let expected_uuid =
    ///     Ok(String::from("0436430c-2b02-624c-2032-570501212b57"));
    ///
    /// assert_eq!(expected_uuid, uuid);
    /// ```
    ///
    /// An incorrect number of bytes:
    ///
    /// ```
    /// use uuid::prelude::*;
    /// use uuid::Builder;
    ///
    /// let bytes = [4, 54, 67, 12, 43, 2, 98, 76];
    ///
    /// let builder = Builder::from_slice_be(&bytes);
    ///
    /// assert!(builder.is_err());
    /// ```
    pub fn from_slice_be(b: &[u8]) -> Result<Self, BytesError> {
        const BYTES_LEN: usize = 16;

        let len = b.len();

        if len != BYTES_LEN {
            return Err(BytesError::new(BYTES_LEN, len));
        }

        let mut bytes: Bytes = [0; 16];
        bytes.copy_from_slice(b);
        Ok(Self::from_bytes_be(bytes))
    }

    /// Creates a `Builder` using the supplied little-endian bytes.
    ///
    /// # Errors
    ///
    /// This function will return an error if `b` has any length other than 16.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use uuid::Builder;
    ///
    /// let bytes = [4, 54, 67, 12, 43, 2, 98, 76, 32, 50, 87, 5, 1, 33, 43, 87];
    ///
    /// let builder = Builder::from_slice_le(&bytes);
    /// let uuid =
    ///     builder.map(|mut builder| builder.build().to_hyphenated().to_string());
    ///
    /// let expected_uuid =
    ///     Ok(String::from("0c433604-022b-4c62-2032-570501212b57"));
    ///
    /// assert_eq!(expected_uuid, uuid);
    /// ```
    ///
    /// An incorrect number of bytes:
    ///
    /// ```
    /// use uuid::prelude::*;
    /// use uuid::Builder;
    ///
    /// let bytes = [4, 54, 67, 12, 43, 2, 98, 76];
    ///
    /// let builder = Builder::from_slice_le(&bytes);
    ///
    /// assert!(builder.is_err());
    /// ```
    pub fn from_slice_le(b: &[u8]) -> Result<Self, BytesError> {
        const BYTES_LEN: usize = 16;

        let len = b.len();

        if len != BYTES_LEN {
            return Err(BytesError::new(BYTES_LEN, len));
        }

        let mut bytes: Bytes = [0; 16];
        bytes.copy_from_slice(b);
        Ok(Self::from_bytes_le(bytes))
    }

    /// Creates a `Builder` from four field values.
    ///
    /// # Errors
    ///
    /// This function will return an error if `d4`'s length is not 8 bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use uuid::Builder;
    ///
    /// let d4 = [12, 3, 9, 56, 54, 43, 8, 9];
    ///
    /// let builder = Builder::from_fields(42, 12, 5, &d4);
    /// let uuid =
    ///     builder.map(|mut builder| builder.build().to_hyphenated().to_string());
    ///
    /// let expected_uuid =
    ///     Ok(String::from("0000002a-000c-0005-0c03-0938362b0809"));
    ///
    /// assert_eq!(expected_uuid, uuid);
    /// ```
    ///
    /// An invalid length:
    ///
    /// ```
    /// use uuid::prelude::*;
    ///
    /// let d4 = [12];
    ///
    /// let builder = uuid::Builder::from_fields(42, 12, 5, &d4);
    ///
    /// assert!(builder.is_err());
    /// ```
    pub fn from_fields(
        d1: u32,
        d2: u16,
        d3: u16,
        d4: &[u8],
    ) -> Result<Self, BytesError> {
        Uuid::from_fields(d1, d2, d3, d4).map(Builder)
    }

    /// Creates a `Builder` with an initial [`Uuid::nil`]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use uuid::Builder;
    ///
    /// let mut builder = Builder::nil();
    ///
    /// assert_eq!(
    ///     builder.build().to_hyphenated().to_string(),
    ///     "00000000-0000-0000-0000-000000000000"
    /// );
    /// ```
    pub fn nil() -> Self {
        Builder(Uuid::nil())
    }

    /// Specifies the variant of the internal [`Uuid`].
    pub fn set_variant(&mut self, v: Variant) -> &mut Self {
        self.0.set_variant(v);
        self
    }

    /// Specifies the version number of the internal [`Uuid`].
    pub fn set_version(&mut self, v: Version) -> &mut Self {
        self.0.set_version(v);
        self
    }

    /// Hands over the internal constructed [`Uuid`]
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use uuid::Builder;
    ///
    /// let uuid = Builder::nil().build();
    ///
    /// assert_eq!(
    ///     uuid.to_hyphenated().to_string(),
    ///     "00000000-0000-0000-0000-000000000000"
    /// );
    /// ```
    pub fn build(&mut self) -> Uuid {
        self.0
    }
}
