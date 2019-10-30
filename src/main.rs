#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use core::result::Result;
use reqwest::Error;

use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;

pub const MAINNET_BASE_URL: &str = "https://rest.bitcoin.com/v2/";

#[derive(Debug)]
pub struct Address {}

#[derive(Deserialize, Debug)]
pub struct AddressDetails {
    legacyAddress: String,
    cashAddress: String,
    slpAddress: String,
    balance: u32,
    balanceSat: u32,
    totalReceived: f32,
    totalReceivedSat: u32,
    totalSent: f32,
    unconfirmedBalance: f32,
    unconfirmedBalanceSat: u32,
    unconfirmedTxApperances: u32,
    txApperances: u32,
    currentPage: u32,
    pagesTotal: u32,
    transactions: Vec<String>,
}

impl Serialize for AddressDetails {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("AddressDetails", 16)?;
        state.serialize_field("legacyAddress", &self.legacyAddress)?;
        state.serialize_field("cashAddress", &self.cashAddress)?;
        state.serialize_field("slpAddress", &self.slpAddress)?;
        state.serialize_field("balance", &self.balance)?;
        state.serialize_field("balanceSat", &self.balanceSat)?;
        state.serialize_field("totalReceived", &self.totalReceived)?;
        state.serialize_field("totalReceivedSat", &self.totalReceivedSat)?;
        state.serialize_field("totalSent", &self.totalSent)?;
        state.serialize_field("unconfirmedBalance", &self.unconfirmedBalance)?;
        state.serialize_field("unconfirmedBalanceSat", &self.unconfirmedBalanceSat)?;
        state.serialize_field("unconfirmedTxApperances", &self.unconfirmedTxApperances)?;
        state.serialize_field("txApperances", &self.txApperances)?;
        state.serialize_field("currentPage", &self.currentPage)?;
        state.serialize_field("pagesTotal", &self.pagesTotal)?;
        state.serialize_field("transactions", &self.transactions)?;
        state.end()
    }
}

impl Address {
    pub fn details(cash_address: &str) -> Result<AddressDetails, Error> {
        let url: String = format!(
            "{}address/details/{}",
            crate::MAINNET_BASE_URL,
            cash_address
        );
        let s_slice: &str = &url[..];
        let address_details: AddressDetails = reqwest::get(s_slice)?.json()?;
        Ok(address_details)
    }
}

fn main() {
    let cash_address: &str = "bitcoincash:qzs02v05l7qs5s24srqju498qu55dwuj0cx5ehjm2c";
    let address_details: AddressDetails = Address::details(cash_address).unwrap();
    let serialized = serde_json::to_string(&address_details).unwrap();
    println!("serialized = {:#?}", serialized);

    let deserialized: AddressDetails = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:#?}", deserialized);
}
