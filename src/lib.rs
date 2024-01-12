use wasm_bindgen::prelude::*;
use zcash_address::{
    unified::{self, Encoding},
    Network, ToAddress, TryFromAddress, ZcashAddress,
};

struct TraceableReceiver {
    net: Network,
    data: [u8; 20],
}

impl TraceableReceiver {
    fn to_address(&self, expiry_time: u64) -> ZcashAddress {
        let traceable_addr = unified::Address::try_from_items(vec![
            unified::Receiver::Unknown {
                typecode: 0x04,
                data: self.data.to_vec(),
            },
            unified::Receiver::Unknown {
                typecode: 0xE0,
                data: expiry_time.to_le_bytes().to_vec(),
            },
        ])
        .expect("We know that this produces a valid address.");

        ZcashAddress::from_unified(self.net, traceable_addr)
    }
}

impl TryFromAddress for TraceableReceiver {
    type Error = unified::ParseError;

    fn try_from_transparent_p2pkh(
        net: Network,
        data: [u8; 20],
    ) -> Result<Self, zcash_address::ConversionError<Self::Error>> {
        Ok(TraceableReceiver { net, data })
    }
}

#[wasm_bindgen]
pub fn to_traceable_address(address: &str, expiry_time: u64) -> Result<String, JsError> {
    let zaddr = ZcashAddress::try_from_encoded(address)?;
    let t_receiver = zaddr.convert::<TraceableReceiver>()?;

    Ok(t_receiver.to_address(expiry_time).encode())
}
