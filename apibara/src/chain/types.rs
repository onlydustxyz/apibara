//! Types common to all chains.
use anyhow::Result;
use chrono::NaiveDateTime;
use std::{fmt, str::FromStr};

/// Chain block hash.
#[derive(Clone, PartialEq)]
pub struct BlockHash(Vec<u8>);

/// Chain address.
#[derive(Clone)]
pub struct Address(Vec<u8>);

/// Data associated with an event.
#[derive(Clone)]
pub struct TopicValue(Vec<u8>);

/// Block header information needed to track information about the chain head.
#[derive(Debug, Clone)]
pub struct BlockHeader {
    /// This block's hash.
    pub hash: BlockHash,
    /// The block parent's hash.
    pub parent_hash: Option<BlockHash>,
    /// This block height.
    pub number: u64,
    /// Time the block was finalized.
    pub timestamp: NaiveDateTime,
}

/// A blockchain event.
#[derive(Debug)]
pub struct Event {
    /// Event source address.
    pub address: Address,
    /// Indexed events.
    pub topics: Vec<TopicValue>,
    /// Event data.
    pub data: Vec<u8>,
    /// Event index in the block.
    pub block_index: usize,
}

/// Events emitted in a single block.
#[derive(Debug)]
pub struct BlockEvents {
    /// The block number.
    pub number: u64,
    /// The block hash.
    pub hash: BlockHash,
    /// The events.
    pub events: Vec<Event>,
}

impl BlockHash {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Address {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl TopicValue {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl FromStr for BlockHash {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(hex_to_vec(s)?))
    }
}

impl From<Vec<u8>> for BlockHash {
    fn from(data: Vec<u8>) -> Self {
        BlockHash(data)
    }
}

impl FromStr for Address {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(hex_to_vec(s)?))
    }
}

impl From<Vec<u8>> for Address {
    fn from(data: Vec<u8>) -> Self {
        Address(data)
    }
}

impl FromStr for TopicValue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(hex_to_vec(s)?))
    }
}

impl From<Vec<u8>> for TopicValue {
    fn from(data: Vec<u8>) -> Self {
        TopicValue(data)
    }
}

impl fmt::Display for BlockHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hash_to_hex(self))
    }
}

impl fmt::Debug for BlockHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BlockHash({})", self)
    }
}

impl fmt::Display for TopicValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", topic_value_to_hex(self))
    }
}

impl fmt::Debug for TopicValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TopicValue({})", self)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", address_to_hex(self))
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address({})", self)
    }
}

fn hex_to_vec(data: &str) -> Result<Vec<u8>> {
    if let Some(data) = data.strip_prefix("0x") {
        Ok(hex::decode(data)?)
    } else {
        Ok(hex::decode(data)?)
    }
}

fn hash_to_hex(h: &BlockHash) -> String {
    hex::encode(&h.0)
}

fn topic_value_to_hex(t: &TopicValue) -> String {
    hex::encode(&t.0)
}

fn address_to_hex(a: &Address) -> String {
    hex::encode(&a.0)
}