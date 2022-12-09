#![no_std]

use dns_io::*;
use gstd::{msg, prelude::*, util, ActorId, Vec};

static mut RECORDS: Vec<DnsRecord> = Vec::new();

pub trait Dns {
    fn add_record(&mut self, name: String, link: String, description: String) -> DnsRecord;

    fn remove_record(&mut self, id: u32) -> Option<DnsRecord>;

    fn update_record(
        &mut self,
        id: u32,
        name: Option<String>,
        link: Option<String>,
        description: Option<String>,
    ) -> Option<DnsRecord>;

    fn get_by_id(&self, id: u32) -> Option<DnsRecord>;

    fn get_by_name(&self, name: String) -> Vec<DnsRecord>;

    fn get_by_description(&self, description: String) -> Vec<DnsRecord>;

    fn get_by_creator(&self, creator: ActorId) -> Vec<DnsRecord>;
}

impl Dns for Vec<DnsRecord> {
    fn add_record(&mut self, name: String, link: String, description: String) -> DnsRecord {
        let id = self.len() as u32;
        let record = DnsRecord {
            id,
            name,
            link,
            description,
            created_by: msg::source(),
        };

        self.push(record.clone());

        record
    }

    fn remove_record(&mut self, id: u32) -> Option<DnsRecord> {
        let index = self.iter().position(|r| r.id == id);

        if index.is_some() {
            let record = self.get(index.unwrap()).unwrap();

            if record.created_by == msg::source() {
                Some(self.swap_remove(index.unwrap()))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn update_record(
        &mut self,
        id: u32,
        name: Option<String>,
        link: Option<String>,
        description: Option<String>,
    ) -> Option<DnsRecord> {
        let index = self.iter().position(|r| r.id == id);

        if index.is_some() {
            let record = self.get_mut(index.unwrap()).unwrap();
            if record.created_by == msg::source() {
                if name.is_some() {
                    record.name = name.unwrap()
                }

                if link.is_some() {
                    record.link = link.unwrap()
                }

                if description.is_some() {
                    record.description = description.unwrap()
                }

                Some(record.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_by_id(&self, id: u32) -> Option<DnsRecord> {
        let record = self.iter().find(|&r| r.id == id);

        if record.is_some() {
            Some(record.unwrap().clone())
        } else {
            None
        }
    }

    fn get_by_name(&self, name: String) -> Vec<DnsRecord> {
        self.into_iter()
            .filter(|r| r.name == name)
            .cloned()
            .collect()
    }

    fn get_by_description(&self, description: String) -> Vec<DnsRecord> {
        self.into_iter()
            .filter(|&r| r.description.as_str().contains(description.as_str()))
            .cloned()
            .collect()
    }

    fn get_by_creator(&self, creator: ActorId) -> Vec<DnsRecord> {
        self.into_iter()
            .filter(|&r| r.created_by == creator)
            .cloned()
            .collect()
    }
}

#[no_mangle]
unsafe extern "C" fn handle() {
    let action: DnsAction = msg::load().expect("Unable to decode message");

    let result: DnsReply = match action {
        DnsAction::Register {
            name,
            link,
            description,
        } => DnsReply::Record(Some(RECORDS.add_record(name, link, description))),
        DnsAction::Remove(id) => DnsReply::Record(RECORDS.remove_record(id)),
        DnsAction::Update {
            id,
            name,
            link,
            description,
        } => DnsReply::Record(RECORDS.update_record(id, name, link, description)),
        DnsAction::GetById(id) => DnsReply::Record(RECORDS.get_by_id(id)),
        DnsAction::GetByName(name) => DnsReply::Records(RECORDS.get_by_name(name)),
        DnsAction::GetByDescription(description) => {
            DnsReply::Records(RECORDS.get_by_description(description))
        }
    };
    msg::reply_with_gas(result, 0, 0).expect("Error in sending a reply");
}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let action: QueryAction = msg::load().expect("Unable to decode message");

    let result: QueryResult = match action {
        QueryAction::GetAll => QueryResult::Records(RECORDS.clone()),
        QueryAction::GetById(id) => QueryResult::Record(RECORDS.get_by_id(id)),
        QueryAction::GetByName(name) => QueryResult::Records(RECORDS.get_by_name(name)),
        QueryAction::GetByCreator(actor) => QueryResult::Records(RECORDS.get_by_creator(actor)),
        QueryAction::GetByDescription(description) => {
            QueryResult::Records(RECORDS.get_by_description(description))
        }
    };

    util::to_leak_ptr(result.encode())
}

gstd::metadata! {
    title: "DNS contract",
    handle:
        input: DnsAction,
        output: DnsReply,
    state:
        input: QueryAction,
        output: QueryResult,
}
