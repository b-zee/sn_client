// Copyright 2016 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.


use nfs::metadata::FileMetadata;
use routing::DataIdentifier;
use self_encryption::DataMap;
use std::fmt;

/// Representation of a File to be put into the network. Could be any kind of file: text, music, video, etc.
#[derive(RustcEncodable, RustcDecodable, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum File {
    /// Represents unversioned file
    Unversioned(FileMetadata),

    /// Represents versioned file that contains a pointer to a list of previous versions and
    /// metadata for the current version.
    Versioned {
        /// Pointer to a list of versions (should be DataIdentifier::ImmutableData, contains
        /// Vec<FileMetadata>)
        ptr_versions: DataIdentifier,

        /// Total number of versions for this file
        num_of_versions: u64,

        /// Metadata for the current version
        latest_version: FileMetadata,
    },
}

impl File {
    /// Get metadata associated with the file
    pub fn metadata(&self) -> &FileMetadata {
        match *self {
            File::Unversioned(ref metadata) => metadata,
            File::Versioned { ref latest_version, .. } => latest_version,
        }
    }

    /// Get metadata associated with the file, with mutability to allow updation
    pub fn metadata_mut(&mut self) -> &mut FileMetadata {
        match *self {
            File::Unversioned(ref mut id) => id,
            File::Versioned { ref mut latest_version, .. } => latest_version,
        }
    }

    /// Get a name for the current version of the file
    pub fn name(&self) -> &str {
        self.metadata().name()
    }

    /// Get the data-map of the File. This is generated by passing the contents of the File to
    /// self-encryption
    pub fn datamap(&self) -> &DataMap {
        self.metadata().datamap()
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File > metadata: {:?}", self.metadata())
    }
}

#[cfg(test)]
mod tests {
    use maidsafe_utilities::serialisation::{deserialise, serialise};
    use nfs::metadata::FileMetadata;
    use self_encryption::DataMap;
    use super::*;

    #[test]
    fn serialise_deserialise() {
        let obj_before = File::Unversioned(FileMetadata::new("Home".to_string(),
                                                             "{mime:\"application/json\"}"
                                                                 .to_string()
                                                                 .into_bytes(),
                                           DataMap::None));
        let serialised_data = unwrap!(serialise(&obj_before));
        let obj_after = unwrap!(deserialise(&serialised_data));
        assert_eq!(obj_before, obj_after);
    }
}
