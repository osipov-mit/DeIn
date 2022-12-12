#![no_std]

use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId, Clone, Vec};
use scale_info::TypeInfo;

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct DnsRecord {
    pub id: u32,
    pub name: String,
    pub link: String,
    pub description: String,
    pub created_by: ActorId,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum DnsAction {
    Register {
        name: String,
        link: String,
        description: String,
    },
    Remove(u32),
    Update {
        id: u32,
        name: Option<String>,
        link: Option<String>,
        description: Option<String>,
    },
    GetById(u32),
    GetByName(String),
    GetByDescription(String),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum DnsReply {
    Record(Option<DnsRecord>),
    Records(Vec<DnsRecord>),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum QueryAction {
    GetAll,
    GetById(u32),
    GetByName(String),
    GetByCreator(ActorId),
    GetByDescription(String),
    GetByPattern(String),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum QueryResult {
    Record(Option<DnsRecord>),
    Records(Vec<DnsRecord>),
}
