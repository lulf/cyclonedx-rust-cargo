/*
 * This file is part of CycloneDX Rust Cargo.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::{
    models,
    xml::{to_xml_write_error, ToXml},
};
use serde::{Deserialize, Serialize};
use xml::writer::XmlEvent;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub(crate) struct Hashes(Vec<Hash>);

impl From<models::Hashes> for Hashes {
    fn from(other: models::Hashes) -> Self {
        Hashes(other.0.into_iter().map(Into::into).collect())
    }
}

impl From<Hashes> for models::Hashes {
    fn from(other: Hashes) -> Self {
        models::Hashes(other.0.into_iter().map(Into::into).collect())
    }
}

const HASHES_TAG: &str = "hashes";

impl ToXml for Hashes {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        writer
            .write(XmlEvent::start_element(HASHES_TAG))
            .map_err(to_xml_write_error(HASHES_TAG))?;

        for hash in &self.0 {
            hash.write_xml_element(writer)?;
        }

        writer
            .write(XmlEvent::end_element())
            .map_err(to_xml_write_error(HASHES_TAG))?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Hash {
    alg: String,
    content: HashValue,
}

impl From<models::Hash> for Hash {
    fn from(other: models::Hash) -> Self {
        Self {
            alg: other.alg.to_string(),
            content: other.content.into(),
        }
    }
}

impl From<Hash> for models::Hash {
    fn from(other: Hash) -> Self {
        Self {
            alg: models::HashAlgorithm::new_unchecked(other.alg),
            content: other.content.into(),
        }
    }
}

const HASH_TAG: &str = "hash";
const ALG_ATTR: &str = "alg";

impl ToXml for Hash {
    fn write_xml_element<W: std::io::Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
    ) -> Result<(), crate::errors::XmlWriteError> {
        writer
            .write(XmlEvent::start_element(HASH_TAG).attr(ALG_ATTR, &self.alg))
            .map_err(to_xml_write_error(HASH_TAG))?;

        writer
            .write(XmlEvent::characters(&self.content.0))
            .map_err(to_xml_write_error(HASH_TAG))?;

        writer
            .write(XmlEvent::end_element())
            .map_err(to_xml_write_error(HASH_TAG))?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct HashValue(String);

impl From<models::HashValue> for HashValue {
    fn from(other: models::HashValue) -> Self {
        Self(other.0)
    }
}

impl From<HashValue> for models::HashValue {
    fn from(other: HashValue) -> Self {
        Self(other.0)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use crate::xml::test::write_element_to_string;

    use super::*;

    pub(crate) fn example_hashes() -> Hashes {
        Hashes(vec![example_hash()])
    }

    pub(crate) fn corresponding_hashes() -> models::Hashes {
        models::Hashes(vec![corresponding_hash()])
    }

    pub(crate) fn example_hash() -> Hash {
        Hash {
            alg: "algorithm".to_string(),
            content: HashValue("hash value".to_string()),
        }
    }

    pub(crate) fn corresponding_hash() -> models::Hash {
        models::Hash {
            alg: models::HashAlgorithm::UnknownHashAlgorithm("algorithm".to_string()),
            content: models::HashValue("hash value".to_string()),
        }
    }

    #[test]
    fn it_should_write_xml_full() {
        let xml_output = write_element_to_string(example_hashes());
        insta::assert_snapshot!(xml_output);
    }
}
