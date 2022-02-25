/*
 * Created on Tue Feb 22 2022
 *
 * This file is a part of Skytable
 * Skytable (formerly known as TerrabaseDB or Skybase) is a free and open-source
 * NoSQL database written by Sayan Nandan ("the Author") with the
 * vision to provide flexibility in data modelling without compromising
 * on performance, queryability or scalability.
 *
 * Copyright (c) 2022, Sayan Nandan <ohsayan@outlook.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 *
*/

mod keys {
    use super::super::keys::{generate_full, verify_key};

    #[test]
    fn test_verify_key() {
        let (key, store) = generate_full();
        assert!(verify_key(key.as_bytes(), &store));
    }
}

mod authn {
    use super::super::{AuthError, AuthProvider};
    use crate::corestore::htable::Coremap;
    use std::sync::Arc;
    #[test]
    fn claim_root_okay() {
        let orig = b"c4299d190fb9a00626797fcc138c56eae9971664";
        let authmap = Arc::new(Coremap::new());
        let provider = AuthProvider::new(authmap, Some(*orig));
        let _ = provider.claim_root(orig).unwrap();
    }
    #[test]
    fn claim_root_wrongkey() {
        let orig = b"c4299d190fb9a00626797fcc138c56eae9971664";
        let authmap = Arc::new(Coremap::new());
        let provider = AuthProvider::new(authmap, Some(*orig));
        let claim_err = provider.claim_root(&orig[1..]).unwrap_err();
        assert_eq!(claim_err, AuthError::BadCredentials);
    }
    #[test]
    fn claim_root_disabled() {
        let provider = AuthProvider::new(Arc::new(Coremap::new()), None);
        assert_eq!(
            provider.claim_root(b"abcd").unwrap_err(),
            AuthError::Disabled
        );
    }
    #[test]
    fn claim_root_already_claimed() {
        let orig = b"c4299d190fb9a00626797fcc138c56eae9971664";
        let authmap = Arc::new(Coremap::new());
        let provider = AuthProvider::new(authmap, Some(*orig));
        let _ = provider.claim_root(orig).unwrap();
        assert_eq!(
            provider.claim_root(orig).unwrap_err(),
            AuthError::AlreadyClaimed
        );
    }
}