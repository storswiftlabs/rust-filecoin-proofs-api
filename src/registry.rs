use std::path::PathBuf;

use anyhow::{ensure, Result};
use filecoin_proofs_v1::types::{PoRepConfig, PoRepProofPartitions, PoStConfig, SectorSize};
use serde::{Deserialize, Serialize};

/// Available seal proofs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegisteredSealProof {
    StackedDrg1KiBV1,
    StackedDrg16MiBV1,
    StackedDrg256MiBV1,
    StackedDrg1GiBV1,
    StackedDrg32GiBV1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Version {
    V1,
}

impl RegisteredSealProof {
    /// Return the version for this proof.
    pub fn version(self) -> Version {
        use RegisteredSealProof::*;

        match self {
            StackedDrg1KiBV1 | StackedDrg16MiBV1 | StackedDrg256MiBV1 | StackedDrg1GiBV1
            | StackedDrg32GiBV1 => Version::V1,
        }
    }

    /// Return the sector size for this proof.
    pub fn sector_size(self) -> SectorSize {
        use filecoin_proofs_v1::constants;
        use RegisteredSealProof::*;
        let size = match self {
            StackedDrg1KiBV1 => constants::SECTOR_SIZE_ONE_KIB,
            StackedDrg16MiBV1 => constants::SECTOR_SIZE_16_MIB,
            StackedDrg256MiBV1 => constants::SECTOR_SIZE_256_MIB,
            StackedDrg1GiBV1 => constants::SECTOR_SIZE_1_GIB,
            StackedDrg32GiBV1 => constants::SECTOR_SIZE_32_GIB,
        };
        SectorSize(size)
    }

    /// Return the number of partitions for this proof.
    pub fn partitions(self) -> u8 {
        use filecoin_proofs_v1::constants;
        use RegisteredSealProof::*;
        match self {
            StackedDrg1KiBV1 => *constants::POREP_PARTITIONS
                .read()
                .unwrap()
                .get(&constants::SECTOR_SIZE_ONE_KIB)
                .expect("invalid sector size"),
            StackedDrg16MiBV1 => *constants::POREP_PARTITIONS
                .read()
                .unwrap()
                .get(&constants::SECTOR_SIZE_16_MIB)
                .expect("invalid sector size"),
            StackedDrg256MiBV1 => *constants::POREP_PARTITIONS
                .read()
                .unwrap()
                .get(&constants::SECTOR_SIZE_256_MIB)
                .expect("invalid sector size"),
            StackedDrg1GiBV1 => *constants::POREP_PARTITIONS
                .read()
                .unwrap()
                .get(&constants::SECTOR_SIZE_1_GIB)
                .expect("invalid sector size"),
            StackedDrg32GiBV1 => *constants::POREP_PARTITIONS
                .read()
                .unwrap()
                .get(&constants::SECTOR_SIZE_32_GIB)
                .expect("invalid sector size"),
        }
    }

    pub fn single_partition_proof_len(self) -> usize {
        use RegisteredSealProof::*;

        match self {
            StackedDrg1KiBV1 | StackedDrg16MiBV1 | StackedDrg256MiBV1 | StackedDrg1GiBV1
            | StackedDrg32GiBV1 => filecoin_proofs_v1::SINGLE_PARTITION_PROOF_LEN,
        }
    }

    pub fn as_v1_config(self) -> PoRepConfig {
        use RegisteredSealProof::*;

        assert_eq!(self.version(), Version::V1);

        match self {
            StackedDrg1KiBV1 | StackedDrg16MiBV1 | StackedDrg256MiBV1 | StackedDrg1GiBV1
            | StackedDrg32GiBV1 => PoRepConfig {
                sector_size: self.sector_size(),
                partitions: PoRepProofPartitions(self.partitions()),
            },
            // _ => panic!("Can only be called on V1 configs"),
        }
    }

    /// Returns the circuit identifier.
    pub fn circuit_identifier(self) -> Result<String> {
        match self.version() {
            Version::V1 => self.as_v1_config().get_cache_identifier(),
        }
    }

    pub fn cache_verifying_key_path(self) -> Result<PathBuf> {
        match self.version() {
            Version::V1 => self.as_v1_config().get_cache_verifying_key_path(),
        }
    }

    pub fn cache_params_path(self) -> Result<PathBuf> {
        match self.version() {
            Version::V1 => self.as_v1_config().get_cache_params_path(),
        }
    }

    pub fn verifying_key_cid(self) -> Result<String> {
        match self.version() {
            Version::V1 => {
                let id = self.as_v1_config().get_cache_identifier()?;
                let params = filecoin_proofs_v1::constants::PARAMETERS.get(&format!("{}.vk", &id));
                ensure!(params.is_some(), "missing params for {}", &id);

                Ok(params.unwrap().cid.clone())
            }
        }
    }

    pub fn params_cid(self) -> Result<String> {
        match self.version() {
            Version::V1 => {
                let id = self.as_v1_config().get_cache_identifier()?;
                let params =
                    filecoin_proofs_v1::constants::PARAMETERS.get(&format!("{}.params", &id));
                ensure!(params.is_some(), "missing params for {}", &id);

                Ok(params.unwrap().cid.clone())
            }
        }
    }
}

/// Available seal proofs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegisteredPoStProof {
    StackedDrg1KiBV1,
    StackedDrg16MiBV1,
    StackedDrg256MiBV1,
    StackedDrg1GiBV1,
    StackedDrg32GiBV1,
}

impl RegisteredPoStProof {
    /// Return the version for this proof.
    pub fn version(self) -> Version {
        use RegisteredPoStProof::*;

        match self {
            StackedDrg1KiBV1 | StackedDrg16MiBV1 | StackedDrg256MiBV1 | StackedDrg1GiBV1
            | StackedDrg32GiBV1 => Version::V1,
        }
    }

    /// Return the sector size for this proof.
    pub fn sector_size(self) -> SectorSize {
        use filecoin_proofs_v1::constants;
        use RegisteredPoStProof::*;

        let size = match self {
            StackedDrg1KiBV1 => constants::SECTOR_SIZE_ONE_KIB,
            StackedDrg16MiBV1 => constants::SECTOR_SIZE_16_MIB,
            StackedDrg256MiBV1 => constants::SECTOR_SIZE_256_MIB,
            StackedDrg1GiBV1 => constants::SECTOR_SIZE_1_GIB,
            StackedDrg32GiBV1 => constants::SECTOR_SIZE_32_GIB,
        };
        SectorSize(size)
    }

    /// Return the number of partitions for this proof.
    pub fn partitions(self) -> u8 {
        use RegisteredPoStProof::*;

        match self {
            StackedDrg1KiBV1 | StackedDrg16MiBV1 | StackedDrg256MiBV1 | StackedDrg1GiBV1
            | StackedDrg32GiBV1 => 1,
        }
    }

    pub fn single_partition_proof_len(self) -> usize {
        use RegisteredPoStProof::*;

        match self {
            StackedDrg1KiBV1 | StackedDrg16MiBV1 | StackedDrg256MiBV1 | StackedDrg1GiBV1
            | StackedDrg32GiBV1 => filecoin_proofs_v1::SINGLE_PARTITION_PROOF_LEN,
        }
    }

    pub fn as_v1_config(self) -> PoStConfig {
        assert_eq!(self.version(), Version::V1);

        use RegisteredPoStProof::*;

        match self {
            StackedDrg1KiBV1 | StackedDrg16MiBV1 | StackedDrg256MiBV1 | StackedDrg1GiBV1
            | StackedDrg32GiBV1 => PoStConfig {
                sector_size: self.sector_size(),
                challenge_count: filecoin_proofs_v1::constants::POST_CHALLENGE_COUNT,
                challenged_nodes: filecoin_proofs_v1::constants::POST_CHALLENGED_NODES,
                priority: true,
            },
            // _ => panic!("Can only be called on V1 configs"),
        }
    }

    /// Returns the circuit identifier.
    pub fn circuit_identifier(self) -> Result<String> {
        match self.version() {
            Version::V1 => self.as_v1_config().get_cache_identifier(),
        }
    }

    pub fn cache_verifying_key_path(self) -> Result<PathBuf> {
        match self.version() {
            Version::V1 => self.as_v1_config().get_cache_verifying_key_path(),
        }
    }

    pub fn cache_params_path(self) -> Result<PathBuf> {
        match self.version() {
            Version::V1 => self.as_v1_config().get_cache_params_path(),
        }
    }

    pub fn verifying_key_cid(self) -> Result<String> {
        match self.version() {
            Version::V1 => {
                let id = self.as_v1_config().get_cache_identifier()?;
                let params = filecoin_proofs_v1::constants::PARAMETERS.get(&format!("{}.vk", &id));
                ensure!(params.is_some(), "missing params for {}", &id);

                Ok(params.unwrap().cid.clone())
            }
        }
    }

    pub fn params_cid(self) -> Result<String> {
        match self.version() {
            Version::V1 => {
                let id = self.as_v1_config().get_cache_identifier()?;
                let params =
                    filecoin_proofs_v1::constants::PARAMETERS.get(&format!("{}.params", &id));
                ensure!(params.is_some(), "missing params for {}", &id);

                Ok(params.unwrap().cid.clone())
            }
        }
    }
}



#[cfg(test)]
pub mod tests {
    use crate::{RegisteredSealProof, RegisteredPoStProof};
    use crate::registry::Version;
    use anyhow::Result;

    #[test]
    fn test_registered_seal_proof_accessors() -> Result<()> {
        let rsps = vec![
            RegisteredSealProof::StackedDrg1KiBV1,
            RegisteredSealProof::StackedDrg16MiBV1,
            RegisteredSealProof::StackedDrg256MiBV1,
            RegisteredSealProof::StackedDrg1GiBV1,
            RegisteredSealProof::StackedDrg32GiBV1
        ];

        for rsp in rsps {
            let _ = rsp.as_v1_config(); // make sure doesn't panic
            let _ = rsp.cache_params_path()?;
            let _ = rsp.cache_verifying_key_path()?;
            let _ = rsp.circuit_identifier()?;
            let _ = rsp.params_cid()?;
            let _ = rsp.verifying_key_cid()?;

            assert!(rsp.partitions() > 0, "partitions() failed");
            assert!(u64::from(rsp.sector_size()) > 0, "sector_size() failed");
            assert!(rsp.single_partition_proof_len() > 0, "single_partition_proof_len() failed");
            assert_eq!(rsp.version(), Version::V1, "version() was wrong");
        }

        Ok(())
    }

    #[test]
    fn test_registered_post_proof_accessors() -> Result<()> {
        let rpps = vec![
            RegisteredPoStProof::StackedDrg1KiBV1,
            RegisteredPoStProof::StackedDrg16MiBV1,
            RegisteredPoStProof::StackedDrg256MiBV1,
            RegisteredPoStProof::StackedDrg1GiBV1,
            RegisteredPoStProof::StackedDrg32GiBV1
        ];

        for rpp in rpps {
            let _ = rpp.as_v1_config(); // make sure doesn't panic
            assert!(rpp.cache_params_path().is_ok(), "cache_params_path() failed");
            assert!(rpp.cache_verifying_key_path().is_ok(), "cache_verifying_key_path() failed");
            assert!(rpp.circuit_identifier().is_ok(), "circuit_identifier() failed");
            assert!(rpp.params_cid().is_ok(), "params_cid() failed");
            assert!(rpp.partitions() > 0, "partitions() failed");
            assert!(u64::from(rpp.sector_size()) > 0, "sector_size() failed");
            assert!(rpp.single_partition_proof_len() > 0, "single_partition_proof_len() failed");
            assert!(rpp.verifying_key_cid().is_ok(), "verifying_key_cid() failed");
            assert_eq!(rpp.version(), Version::V1, "version() was wrong");
        }

        Ok(())
    }
}
