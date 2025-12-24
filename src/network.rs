//! Network definitions and known token deployments.
//!
//! This module defines supported networks and their chain IDs,
//! and provides statically known USDC deployments per network.

use crate::types::{MixedAddress, TokenAsset, TokenDeployment, TokenDeploymentEip712};
use alloy::primitives::address;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde::de::Error as DeError;
use solana_sdk::pubkey::Pubkey;
use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

/// Supported Ethereum-compatible networks.
///
/// Used to differentiate between testnet and mainnet environments for the x402 protocol.
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    /// Base Sepolia testnet (chain ID 84532).
    BaseSepolia,
    /// Base mainnet (chain ID 8453).
    Base,
    /// XDC mainnet (chain ID 50).
    XdcMainnet,
    /// Avalanche Fuji testnet (chain ID 43113)
    AvalancheFuji,
    /// Avalanche Mainnet (chain ID 43114)
    Avalanche,
    /// XRPL EVM mainnet (chain ID 1440000)
    XrplEvm,
    /// Solana Mainnet - Live production environment for deployed applications
    Solana,
    /// Solana Devnet - Testing with public accessibility for developers experimenting with their applications
    SolanaDevnet,
    /// Polygon Amoy testnet (chain ID 80002).
    PolygonAmoy,
    /// Polygon mainnet (chain ID 137).
    Polygon,
    /// Sei mainnet (chain ID 1329).
    Sei,
    /// Sei testnet (chain ID 1328).
    SeiTestnet,
    /// Besu private network (chain ID 1337).
    BesuPrivate,
}

impl Display for Network {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_caip2().unwrap_or(self.as_legacy()))
    }
}

impl Serialize for Network {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_caip2().unwrap_or(self.as_legacy()))
    }
}

impl<'de> Deserialize<'de> for Network {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Network::from_str_any(&s).ok_or_else(|| {
            D::Error::custom(format!(
                "Unknown network '{}'. Supported legacy: {}; supported CAIP-2: {}",
                s,
                Network::legacy_variants().join(", "),
                Network::caip2_variants().join(", ")
            ))
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NetworkFamily {
    Evm,
    Solana,
}

impl From<Network> for NetworkFamily {
    fn from(value: Network) -> Self {
        match value {
            Network::BaseSepolia => NetworkFamily::Evm,
            Network::Base => NetworkFamily::Evm,
            Network::XdcMainnet => NetworkFamily::Evm,
            Network::AvalancheFuji => NetworkFamily::Evm,
            Network::Avalanche => NetworkFamily::Evm,
            Network::XrplEvm => NetworkFamily::Evm,
            Network::Solana => NetworkFamily::Solana,
            Network::SolanaDevnet => NetworkFamily::Solana,
            Network::PolygonAmoy => NetworkFamily::Evm,
            Network::Polygon => NetworkFamily::Evm,
            Network::Sei => NetworkFamily::Evm,
            Network::SeiTestnet => NetworkFamily::Evm,
            Network::BesuPrivate => NetworkFamily::Evm,
        }
    }
}

impl Network {
    /// Return all known [`Network`] variants.
    pub fn variants() -> &'static [Network] {
        &[
            Network::BaseSepolia,
            Network::Base,
            Network::XdcMainnet,
            Network::AvalancheFuji,
            Network::Avalanche,
            Network::XrplEvm,
            Network::Solana,
            Network::SolanaDevnet,
            Network::PolygonAmoy,
            Network::Polygon,
            Network::Sei,
            Network::SeiTestnet,
            Network::BesuPrivate,
        ]
    }

    fn as_legacy(&self) -> &'static str {
        match self {
            Network::BaseSepolia => "base-sepolia",
            Network::Base => "base",
            Network::XdcMainnet => "xdc",
            Network::AvalancheFuji => "avalanche-fuji",
            Network::Avalanche => "avalanche",
            Network::XrplEvm => "xrpl-evm",
            Network::Solana => "solana",
            Network::SolanaDevnet => "solana-devnet",
            Network::PolygonAmoy => "polygon-amoy",
            Network::Polygon => "polygon",
            Network::Sei => "sei",
            Network::SeiTestnet => "sei-testnet",
            Network::BesuPrivate => "besu-private",
        }
    }

    fn as_caip2(&self) -> Option<&'static str> {
        match self {
            Network::BaseSepolia => Some("eip155:84532"),
            Network::Base => Some("eip155:8453"),
            Network::XdcMainnet => Some("eip155:50"),
            Network::AvalancheFuji => Some("eip155:43113"),
            Network::Avalanche => Some("eip155:43114"),
            Network::XrplEvm => Some("eip155:1440000"),
            Network::PolygonAmoy => Some("eip155:80002"),
            Network::Polygon => Some("eip155:137"),
            Network::Sei => Some("eip155:1329"),
            Network::SeiTestnet => Some("eip155:1328"),
            // No clear CAIP-2 ids for Solana in this codebase; fall back to legacy.
            Network::Solana | Network::SolanaDevnet => None,
            Network::BesuPrivate => Some("eip155:1337"),
        }
    }

    fn legacy_variants() -> Vec<&'static str> {
        Network::variants().iter().map(|n| n.as_legacy()).collect()
    }

    fn caip2_variants() -> Vec<&'static str> {
        Network::variants()
            .iter()
            .filter_map(|n| n.as_caip2())
            .collect()
    }

    fn from_str_any(s: &str) -> Option<Self> {
        // Try CAIP-2 first, then legacy.
        let matched = match s {
            "eip155:84532" => Some(Network::BaseSepolia),
            "eip155:8453" => Some(Network::Base),
            "eip155:50" => Some(Network::XdcMainnet),
            "eip155:43113" => Some(Network::AvalancheFuji),
            "eip155:43114" => Some(Network::Avalanche),
            "eip155:1440000" => Some(Network::XrplEvm),
            "eip155:80002" => Some(Network::PolygonAmoy),
            "eip155:137" => Some(Network::Polygon),
            "eip155:1329" => Some(Network::Sei),
            "eip155:1328" => Some(Network::SeiTestnet),
            "eip155:1337" => Some(Network::BesuPrivate),
            "solana:mainnet" => Some(Network::Solana),
            "solana:devnet" => Some(Network::SolanaDevnet),
            _ => None,
        };
        if matched.is_some() {
            return matched;
        }
        match s {
            "base-sepolia" => Some(Network::BaseSepolia),
            "base" => Some(Network::Base),
            "xdc" => Some(Network::XdcMainnet),
            "avalanche-fuji" => Some(Network::AvalancheFuji),
            "avalanche" => Some(Network::Avalanche),
            "xrpl-evm" => Some(Network::XrplEvm),
            "solana" => Some(Network::Solana),
            "solana-devnet" => Some(Network::SolanaDevnet),
            "polygon-amoy" => Some(Network::PolygonAmoy),
            "polygon" => Some(Network::Polygon),
            "sei" => Some(Network::Sei),
            "sei-testnet" => Some(Network::SeiTestnet),
            "besu-private" => Some(Network::BesuPrivate),
            _ => None,
        }
    }
}

/// Lazily initialized known USDC deployment on Base Sepolia as [`USDCDeployment`].
static USDC_BASE_SEPOLIA: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x036CbD53842c5426634e7929541eC2318f3dCF7e").into(),
            network: Network::BaseSepolia,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USDC".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized known USDC deployment on Base mainnet as [`USDCDeployment`].
static USDC_BASE: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913").into(),
            network: Network::Base,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USD Coin".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized known USDC deployment on XDC mainnet as [`USDCDeployment`].
static USDC_XDC: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x2A8E898b6242355c290E1f4Fc966b8788729A4D4").into(),
            network: Network::XdcMainnet,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "Bridged USDC(XDC)".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized known USDC deployment on Avalanche Fuji testnet as [`USDCDeployment`].
static USDC_AVALANCHE_FUJI: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x5425890298aed601595a70AB815c96711a31Bc65").into(),
            network: Network::AvalancheFuji,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USD Coin".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized known USDC deployment on Avalanche Fuji testnet as [`USDCDeployment`].
static USDC_AVALANCHE: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0xB97EF9Ef8734C71904D8002F8b6Bc66Dd9c48a6E").into(),
            network: Network::Avalanche,
        },
        decimals: 6,

        eip712: Some(TokenDeploymentEip712 {
            name: "USD Coin".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized known USDC deployment on Solana mainnet as [`USDCDeployment`].
static USDC_SOLANA: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: MixedAddress::Solana(
                Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap(),
            ),
            network: Network::Solana,
        },
        decimals: 6,
        eip712: None,
    })
});

/// Lazily initialized known USDC deployment on Solana mainnet as [`USDCDeployment`].
static USDC_SOLANA_DEVNET: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: MixedAddress::Solana(
                Pubkey::from_str("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU").unwrap(),
            ),
            network: Network::SolanaDevnet,
        },
        decimals: 6,
        eip712: None,
    })
});

/// Lazily initialized known USDC deployment on Polygon Amoy testnet as [`USDCDeployment`].
static USDC_POLYGON_AMOY: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x41E94Eb019C0762f9Bfcf9Fb1E58725BfB0e7582").into(),
            network: Network::PolygonAmoy,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USDC".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized known USDC deployment on Polygon mainnet as [`USDCDeployment`].
static USDC_POLYGON: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359").into(),
            network: Network::Polygon,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USDC".into(),
            version: "2".into(),
        }),
    })
});

static USDC_SEI: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0xe15fC38F6D8c56aF07bbCBe3BAf5708A2Bf42392").into(),
            network: Network::Sei,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USDC".into(),
            version: "2".into(),
        }),
    })
});

static USDC_SEI_TESTNET: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x4fCF1784B31630811181f670Aea7A7bEF803eaED").into(),
            network: Network::SeiTestnet,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USDC".into(),
            version: "2".into(),
        }),
    })
});

/// Placeholder USDC deployment for Besu private networks.
///
/// Address/metadata should be supplied via `PaymentRequirements.extra` when using this network.
static USDC_BESU_PRIVATE: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x0000000000000000000000000000000000000000").into(),
            network: Network::BesuPrivate,
        },
        decimals: 6,
        eip712: None,
    })
});

/// A known USDC deployment as a wrapper around [`TokenDeployment`].
#[derive(Clone, Debug)]
pub struct USDCDeployment(pub TokenDeployment);

impl Deref for USDCDeployment {
    type Target = TokenDeployment;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Lazily initialized known USDC deployment on XRPL EVM mainnet as [`USDCDeployment`].
static USDC_XRPL_EVM: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0xDaF4556169c4F3f2231d8ab7BC8772Ddb7D4c84C").into(),
            network: Network::XrplEvm,
        },
        decimals: 6,
        // EIP-712 domain fields (name/version) are resolved dynamically if not provided.
        eip712: None,
    })
});

impl From<&USDCDeployment> for TokenDeployment {
    fn from(deployment: &USDCDeployment) -> Self {
        deployment.0.clone()
    }
}

impl From<USDCDeployment> for Vec<TokenAsset> {
    fn from(deployment: USDCDeployment) -> Self {
        vec![deployment.asset.clone()]
    }
}

impl From<&USDCDeployment> for Vec<TokenAsset> {
    fn from(deployment: &USDCDeployment) -> Self {
        vec![deployment.asset.clone()]
    }
}

impl USDCDeployment {
    /// Return the known USDC deployment for the given network.
    ///
    /// Panic if the network is unsupported (not expected in practice).
    pub fn by_network<N: Borrow<Network>>(network: N) -> &'static USDCDeployment {
        match network.borrow() {
            Network::BaseSepolia => &USDC_BASE_SEPOLIA,
            Network::Base => &USDC_BASE,
            Network::XdcMainnet => &USDC_XDC,
            Network::AvalancheFuji => &USDC_AVALANCHE_FUJI,
            Network::Avalanche => &USDC_AVALANCHE,
            Network::XrplEvm => &USDC_XRPL_EVM,
            Network::Solana => &USDC_SOLANA,
            Network::SolanaDevnet => &USDC_SOLANA_DEVNET,
            Network::PolygonAmoy => &USDC_POLYGON_AMOY,
            Network::Polygon => &USDC_POLYGON,
            Network::Sei => &USDC_SEI,
            Network::SeiTestnet => &USDC_SEI_TESTNET,
            Network::BesuPrivate => &USDC_BESU_PRIVATE,
        }
    }
}
