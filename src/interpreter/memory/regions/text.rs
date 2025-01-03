use super::{super::super::Exception, Region};
use crate::{
    config::Endian,
    type_aliases::address::{is_aligned, Address},
};
use std::ops::Range;

pub struct TextRegion {
    pub addresses: Range<Address>,
    instructions: Box<[u32]>,
    pub num_instructions: usize,
    pub end_pc: Option<Address>,
}

impl Region for TextRegion {
    fn contains(&self, address: Address) -> bool {
        self.addresses.contains(&address)
    }

    fn read_u8(&self, address: Address) -> Result<u8, Exception> {
        Err(Exception::InvalidLoad(address)) // todo
    }

    fn read_u16(&self, address: Address, _assert_aligned: bool) -> Result<u16, Exception> {
        Err(Exception::InvalidLoad(address)) // todo
    }

    fn read_u32(&self, address: Address, assert_aligned: bool) -> Result<u32, Exception> {
        match self.calculate_index(address, assert_aligned) {
            Some(index) => Ok(self.instructions[index]),
            None => Err(Exception::InvalidLoad(address)),
        }
    }

    fn read_u64(&self, address: Address, _assert_aligned: bool) -> Result<u64, Exception> {
        Err(Exception::InvalidLoad(address)) // todo
    }

    fn get_slice(&self, address: Address) -> Result<&[u8], Exception> {
        Err(Exception::InvalidLoad(address)) // todo
    }

    fn get_slice_mut(&mut self, address: Address) -> Result<&mut [u8], Exception> {
        Err(Exception::InvalidLoad(address)) // todo
    }

    fn write_u8(&mut self, address: Address, _value: u8) -> Result<(), Exception> {
        Err(Exception::InvalidStore(address)) // todo
    }

    fn write_u16(
        &mut self,
        address: Address,
        _value: u16,
        _assert_aligned: bool,
    ) -> Result<(), Exception> {
        Err(Exception::InvalidStore(address)) // todo
    }

    fn write_u32(
        &mut self,
        address: Address,
        value: u32,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        if let Some(index) = self.calculate_index(address, assert_aligned) {
            self.instructions[index] = value;
            Ok(())
        } else {
            Err(Exception::InvalidStore(address))
        }
    }

    fn write_u64(
        &mut self,
        address: Address,
        _value: u64,
        _assert_aligned: bool,
    ) -> Result<(), Exception> {
        Err(Exception::InvalidStore(address)) // todo
    }
}

impl TextRegion {
    pub fn new(low_address: Address, bytes_to_allocate: usize) -> Self {
        let words_to_allocate = bytes_to_allocate >> 2;
        Self {
            addresses: low_address..(low_address + bytes_to_allocate as u32),
            instructions: vec![0u32; words_to_allocate].into_boxed_slice(),
            end_pc: None,
            num_instructions: 0,
        }
    }

    pub fn populate(&mut self, bytes: Vec<u8>, endian: Endian) {
        match endian {
            Endian::Little => self.populate_from_le(bytes),
            Endian::Big => self.populate_from_be(bytes),
        }
    }

    fn populate_from_le(&mut self, bytes: Vec<u8>) {
        let mut byte_index: usize = 0;
        let num_instructions = bytes.len() >> 2;
        for i in 0..num_instructions {
            self.instructions[i] = u32::from_le_bytes([
                bytes[byte_index],
                bytes[byte_index + 1],
                bytes[byte_index + 2],
                bytes[byte_index + 3],
            ]);
            byte_index += 4;
        }
        self.num_instructions = num_instructions;
        self.end_pc = Some(byte_index as u32 + self.addresses.start);
    }

    fn populate_from_be(&mut self, bytes: Vec<u8>) {
        let mut byte_index: usize = 0;
        let num_instructions = bytes.len() >> 2;
        for i in 0..num_instructions {
            self.instructions[i] = u32::from_be_bytes([
                bytes[byte_index],
                bytes[byte_index + 1],
                bytes[byte_index + 2],
                bytes[byte_index + 3],
            ]);
            byte_index += 4;
        }
        self.num_instructions = num_instructions;
        self.end_pc = Some(byte_index as u32 + self.addresses.start);
    }

    fn calculate_index(&self, address: Address, assert_aligned: bool) -> Option<usize> {
        if (!assert_aligned || is_aligned(address, 4)) && self.contains(address) {
            let index = (address - self.addresses.start) >> 2;
            Some(index as usize)
        } else {
            None
        }
    }
}
