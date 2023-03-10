// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::KeyComparable;
use identity_core::common::OrderedSet;

use crate::did::DIDUrl;
use crate::did::DID;
use crate::utils::DIDUrlQuery;

/// Allows retrieving an entry from a collection using a custom query type.
///
/// Used for querying verification methods in a DID Document by either its full DID Url identifier
/// or only its fragment. See [`DIDUrlQuery`].
pub trait Queryable<D, T, Q> {
  /// Returns a reference to an entry matching the query if one exists.
  fn query(&self, query: Q) -> Option<&T>;
  /// Returns a mutable reference to an entry matching the query if one exists.
  fn query_mut(&mut self, query: Q) -> Option<&mut T>;
}

impl<'query, D, T, Q> Queryable<D, T, Q> for OrderedSet<T>
where
  D: DID,
  T: KeyComparable,
  <T as KeyComparable>::Key: AsRef<DIDUrl<D>>,
  Q: Into<DIDUrlQuery<'query>>,
{
  fn query(&self, query: Q) -> Option<&T> {
    let query: DIDUrlQuery<'query> = query.into();
    self.iter().find(|entry| query.matches(entry.key().as_ref()))
  }

  /// WARNING: improper usage of this allows violating the key-uniqueness of the OrderedSet.
  fn query_mut(&mut self, query: Q) -> Option<&mut T> {
    let query: DIDUrlQuery<'query> = query.into();
    self
      .iter_mut_unchecked()
      .find(|entry| query.matches(entry.key().as_ref()))
  }
}
