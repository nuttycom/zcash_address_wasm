use wasm_bindgen::prelude::*;
use zcash_address::{
    unified::{self, Container, Encoding},
    Network, ToAddress, TryFromAddress, ZcashAddress,
};

struct TraceableAddress {
    net: Network,
    validating_key_hash: [u8; 20],
    expiry_height: Option<u32>,
    expiry_time: Option<u64>,
}

impl TraceableAddress {
    pub fn with_expiry_height(self, height: u32) -> Self {
        Self {
            net: self.net,
            validating_key_hash: self.validating_key_hash,
            expiry_height: Some(height),
            expiry_time: self.expiry_time,
        }
    }

    pub fn with_expiry_time(self, epoch_time: u64) -> Self {
        Self {
            net: self.net,
            validating_key_hash: self.validating_key_hash,
            expiry_height: self.expiry_height,
            expiry_time: Some(epoch_time),
        }
    }

    pub fn to_address(&self) -> ZcashAddress {
        let items = Some(unified::Receiver::Unknown {
            typecode: 0x04,
            data: self.validating_key_hash.to_vec(),
        })
        .into_iter()
        .chain(
            self.expiry_height
                .iter()
                .map(|h| unified::Receiver::Unknown {
                    typecode: 0xE0,
                    data: h.to_le_bytes().to_vec(),
                }),
        )
        .chain(self.expiry_time.iter().map(|t| unified::Receiver::Unknown {
            typecode: 0xE1,
            data: t.to_le_bytes().to_vec(),
        }))
        .collect::<Vec<_>>();

        ZcashAddress::from_unified(
            self.net,
            unified::Address::try_from_items(items)
                .expect("We know that this produces a valid address."),
        )
    }

    pub fn into_p2pkh(self) -> ZcashAddress {
        ZcashAddress::from_transparent_p2pkh(self.net, self.validating_key_hash)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceableAddressError {
    P2pkhReceiverNotFound,
    ReceiverLengthInvalid,
    ExpiryHeightInvalid,
    ExpiryTimeInvalid,
}

impl std::fmt::Display for TraceableAddressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TraceableAddressError::P2pkhReceiverNotFound =>
                write!(f, "No P2PKH Receiver found"),
            TraceableAddressError::ReceiverLengthInvalid =>
                write!(f, "Receiver length invalid for typecode 0x04: must be 20 bytes."),
            TraceableAddressError::ExpiryHeightInvalid =>
                write!(f, "Expiry height invalid: the value of typecode 0xE0 must be a 4-byte integer in little-endian order."),
            TraceableAddressError::ExpiryTimeInvalid =>
                write!(f, "Expiry height invalid: the value of typecode 0xE1 must be an 8-byte integer in little-endian order."),
        }
    }
}

impl std::error::Error for TraceableAddressError {}

impl TryFromAddress for TraceableAddress {
    type Error = TraceableAddressError;

    fn try_from_unified(
        net: Network,
        data: unified::Address,
    ) -> Result<Self, zcash_address::ConversionError<Self::Error>> {
        let validating_key_hash = data
            .items_as_parsed()
            .iter()
            .find_map(|item| match item {
                unified::Receiver::P2pkh(data) => Some(Ok(data.clone())),
                unified::Receiver::Unknown {
                    typecode: 0x04,
                    data,
                } => Some(
                    <[u8; 20]>::try_from(&data[..])
                        .map_err(|_| TraceableAddressError::ReceiverLengthInvalid),
                ),
                _ => None,
            })
            .transpose()?
            .ok_or_else(|| TraceableAddressError::P2pkhReceiverNotFound)?;

        let expiry_height = data
            .items_as_parsed()
            .iter()
            .find_map(|item| match item {
                unified::Receiver::Unknown {
                    typecode: 0xE0,
                    data,
                } => Some(
                    <[u8; 4]>::try_from(&data[..])
                        .map_err(|_| TraceableAddressError::ExpiryHeightInvalid)
                        .map(u32::from_le_bytes),
                ),
                _ => None,
            })
            .transpose()?;

        let expiry_time = data
            .items_as_parsed()
            .iter()
            .find_map(|item| match item {
                unified::Receiver::Unknown {
                    typecode: 0xE1,
                    data,
                } => Some(
                    <[u8; 8]>::try_from(&data[..])
                        .map(u64::from_le_bytes)
                        .map_err(|_| TraceableAddressError::ExpiryTimeInvalid),
                ),
                _ => None,
            })
            .transpose()?;

        Ok(Self {
            net,
            validating_key_hash,
            expiry_height,
            expiry_time,
        })
    }

    fn try_from_transparent_p2pkh(
        net: Network,
        data: [u8; 20],
    ) -> Result<Self, zcash_address::ConversionError<Self::Error>> {
        Ok(TraceableAddress {
            net,
            validating_key_hash: data,
            expiry_height: None,
            expiry_time: None,
        })
    }
}

#[wasm_bindgen]
pub fn to_traceable_address(address: &str, expiry_time: u64) -> Result<String, JsError> {
    let zaddr = ZcashAddress::try_from_encoded(address)?;
    let traceable = zaddr.convert::<TraceableAddress>()?;

    Ok(traceable
        .with_expiry_time(expiry_time)
        .to_address()
        .encode())
}

#[wasm_bindgen]
pub fn addr_expiry_height(address: &str) -> Result<Option<u32>, JsError> {
    let zaddr = ZcashAddress::try_from_encoded(address)?;
    Ok(zaddr.convert::<TraceableAddress>()?.expiry_height)
}

#[wasm_bindgen]
pub fn addr_expiry_time(address: &str) -> Result<Option<u64>, JsError> {
    let zaddr = ZcashAddress::try_from_encoded(address)?;
    Ok(zaddr.convert::<TraceableAddress>()?.expiry_time)
}

#[wasm_bindgen]
pub fn traceable_to_p2pkh(address: &str) -> Result<String, JsError> {
    let zaddr = ZcashAddress::try_from_encoded(address)?;
    let traceable = zaddr.convert::<TraceableAddress>()?;

    Ok(traceable.into_p2pkh().encode())
}
