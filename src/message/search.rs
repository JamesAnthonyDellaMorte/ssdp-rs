use std::borrow::{Cow};
use std::error::{Error};
use std::net::{ToSocketAddrs};

use hyper::header::{Headers, Header, HeaderFormat};
use time::{Duration};

use {SSDPResult, SSDPError, MsgError};
use header::{HeaderRef, HeaderMut, MX};
use message::{self, SSDPMessage, MessageType};
use net::connector::{UdpConnector};
use receiver::{SSDPReceiver, FromRawSSDP};

/// Standard requires devices to respond within 1 second of receiving message.
const DEFAULT_UNICAST_TIMEOUT: u8 = 2;

#[derive(Debug, Clone)]
pub struct SearchRequest {
    message: SSDPMessage
}

impl SearchRequest {
    /// Construct a new SearchRequest.
    pub fn new() -> SearchRequest {
        SearchRequest{ message: SSDPMessage::new(MessageType::Search) }
    }
    
    /// Send this search request to a single host.
    pub fn unicast<A: ToSocketAddrs>(&mut self, src_addr: A, dst_addr: A) -> SSDPResult<SSDPReceiver<SearchResponse>> {
        let mut connector = try!(UdpConnector::new(src_addr).map_err(|e| SSDPError::IoError(e) ));
        
        try!(self.message.send(&mut connector, dst_addr));
        
        let timeout: u8 = match self.get::<MX>() {
            Some(&MX(n)) => n,
            None         => DEFAULT_UNICAST_TIMEOUT
        };
        
        SSDPReceiver::new(connector.deconstruct(), Some(Duration::seconds(timeout as i64)))
            .map_err(|e| SSDPError::Other(Box::new(e) as Box<Error>) )
    }
    
    /// Send this search request to the standard multicast address.
    pub fn multicast<A: ToSocketAddrs>(&self, local_addr: A)
        -> SSDPReceiver<SearchResponse> {
        panic!("Unimplemented")
    }
}

impl FromRawSSDP for SearchRequest {
    fn raw_ssdp(bytes: &[u8]) -> SSDPResult<SearchRequest> {
        let message = try!(SSDPMessage::raw_ssdp(bytes));
        
        if message.message_type() != MessageType::Search {
            Err(SSDPError::Other(Box::new(MsgError::new(
                "SSDP Message Received Is Not A SearchRequest"
            )) as Box<Error>))
        } else { Ok(SearchRequest{ message: message }) }
    }
}

impl HeaderRef for SearchRequest {
    fn get<H>(&self) -> Option<&H> where H: Header + HeaderFormat {
        self.message.get::<H>()
    }
    
    fn get_raw(&self, name: &str) -> Option<&[Vec<u8>]> {
        self.message.get_raw(name)
    }
}

impl HeaderMut for SearchRequest {
    fn set<H>(&mut self, value: H) where H: Header + HeaderFormat {
        self.message.set(value)
    }
    
    fn set_raw<K>(&mut self, name: K, value: Vec<Vec<u8>>) where K: Into<Cow<'static, str>> {
        self.message.set_raw(name, value)
    }
}

#[derive(Debug, Clone)]
pub struct SearchResponse {
    message: SSDPMessage
}

impl SearchResponse {
    pub fn new() -> SearchResponse {
        SearchResponse{ message: SSDPMessage::new(MessageType::Response) }
    }
    
    pub fn unicast<A: ToSocketAddrs>(&self, dst_addr: A) {
        panic!("Unimplemented")
    }
}

impl FromRawSSDP for SearchResponse {
    fn raw_ssdp(bytes: &[u8]) -> SSDPResult<SearchResponse> {
        let message = try!(SSDPMessage::raw_ssdp(bytes));
        
        if message.message_type() != MessageType::Response {
            Err(SSDPError::Other(Box::new(MsgError::new(
                "SSDP Message Received Is Not A SearchResponse"
            )) as Box<Error>))
        } else { Ok(SearchResponse{ message: message }) }
    }
}

impl HeaderRef for SearchResponse {
    fn get<H>(&self) -> Option<&H> where H: Header + HeaderFormat {
        self.message.get::<H>()
    }
    
    fn get_raw(&self, name: &str) -> Option<&[Vec<u8>]> {
        self.message.get_raw(name)
    }
}

impl HeaderMut for SearchResponse {
    fn set<H>(&mut self, value: H) where H: Header + HeaderFormat {
        self.message.set(value)
    }
    
    fn set_raw<K>(&mut self, name: K, value: Vec<Vec<u8>>) where K: Into<Cow<'static, str>> {
        self.message.set_raw(name, value)
    }
}