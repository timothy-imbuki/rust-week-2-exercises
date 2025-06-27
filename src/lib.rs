use hex::{decode, encode};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    decode(hex_str).map_err(|e| e.to_string())
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    let mut reversed = bytes.to_vec();
    reversed.reverse();
    reversed
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    encode(bytes)
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    decode(hex)
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    num.to_le_bytes()
}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    input.trim().parse::<u64>().map_err(|e| e.to_string())
}

#[derive(Debug)]
pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}

pub fn classify_script(script: &[u8]) -> ScriptType {
    if script.len() == 25 && script[0] == 0x76 && script[1] == 0xa9 {
        ScriptType::P2PKH
    } else if script.len() == 22 && script[0] == 0x00 && script[1] == 0x14 {
        ScriptType::P2WPKH
    } else {
        ScriptType::Unknown
    }
}

#[derive(Debug, Clone)]
pub struct Outpoint {
    pub txid: Vec<u8>,
    pub vout: u32,
}

pub fn read_pushdata(script: &[u8]) -> &[u8] {
    &script[2..]
}

pub trait Wallet {
    fn balance(&self) -> u64;
}

pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(&self) -> u64 {
        self.confirmed
    }
}

pub fn apply_fee(balance: &mut u64, fee: u64) {
    if *balance >= fee {
        *balance -= fee;
    } else {
        *balance = 0;
    }
}

pub fn move_txid(txid: String) -> String {
    format!("Transaction ID: {}", txid)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    OpChecksig,
    OpDup,
    OpInvalid,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        match byte {
            0xac => Ok(Opcode::OpChecksig),
            0x76 => Ok(Opcode::OpDup),
            _ => Ok(Opcode::OpInvalid),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

pub fn consume_utxo(utxo: UTXO) -> UTXO {
    utxo
}
