// Copyright (c) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;

use proptest::prelude::*;
use tempfile::TempDir;

use frm::commands::EtcFile;

proptest! {
    #[test]
    fn etc_file_roundtrip(idx in 0usize..4) {
        let files = EtcFile::ALL;
        let file = files[idx];
        let s = file.as_str();
        let parsed: EtcFile = s.parse().unwrap();
        prop_assert_eq!(file, parsed);
    }

    #[test]
    fn etc_file_display_matches_as_str(idx in 0usize..4) {
        let files = EtcFile::ALL;
        let file = files[idx];
        prop_assert_eq!(file.to_string(), file.as_str());
    }

    #[test]
    fn all_names_contains_all_files(idx in 0usize..4) {
        let files = EtcFile::ALL;
        let file = files[idx];
        let names = EtcFile::all_names();
        prop_assert!(names.contains(&file.as_str()));
    }

    #[test]
    fn invalid_etc_file_is_rejected(s in "[a-z]{1,20}\\.[a-z]{1,10}") {
        let valid_names = EtcFile::all_names();
        if !valid_names.contains(&s.as_str()) {
            let result: Result<EtcFile, _> = s.parse();
            prop_assert!(result.is_err());
        }
    }

    #[test]
    fn copied_file_preserves_content(content in prop::collection::vec(any::<u8>(), 0..1024)) {
        let temp = TempDir::new().unwrap();
        let src_path = temp.path().join("source");
        fs::write(&src_path, &content).unwrap();

        let dest_dir = temp.path().join("dest");
        fs::create_dir_all(&dest_dir).unwrap();
        let dest_path = dest_dir.join("rabbitmq.conf");

        fs::copy(&src_path, &dest_path).unwrap();

        let copied = fs::read(&dest_path).unwrap();
        prop_assert_eq!(content, copied);
    }
}
