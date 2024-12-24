use super::super::{
    instruction::{fields, Instruction},
    Exception, Interpreter,
};
use crate::constants::{fn_codes::Coprocessor0Fn, register};
use num_traits::FromPrimitive;

impl Interpreter {
    pub fn execute_coprocessor_0(&mut self, instruction: Instruction) -> Result<(), Exception> {
        use Coprocessor0Fn::*;
        let r#fn = match Coprocessor0Fn::from_u8(fields::rs(instruction)) {
            Some(r#fn) => r#fn,
            None => return Err(Exception::ReservedInstruction),
        };
        let rt = fields::rt(instruction);
        let rd = fields::rd(instruction);
        match r#fn {
            MoveFromCoprocessor0 => self.mfc0(rt, rd),
            MoveToCoprocessor0 => self.mtc0(rd, rt),
            ErrorReturn => self.eret(instruction),
        }
    }

    fn mfc0(&mut self, rt: u8, rd: u8) -> Result<(), Exception> {
        let rd_value = match rd {
            register::VADDR => self.registers.vaddr,
            register::STATUS => self.registers.status,
            register::CAUSE => self.registers.cause,
            register::EPC => self.registers.epc,
            _ => return Err(Exception::ReservedInstruction), // not exactly what i'm looking for
        };
        self.registers.write_u32_to_cpu(rt, rd_value)
    }

    fn mtc0(&mut self, rd: u8, rt: u8) -> Result<(), Exception> {
        let rt_value = self.registers.read_u32_from_cpu(rt)?;
        let destination = match rd {
            register::VADDR => &mut self.registers.vaddr,
            register::STATUS => &mut self.registers.status,
            register::CAUSE => &mut self.registers.cause,
            register::EPC => &mut self.registers.epc,
            _ => return Err(Exception::ReservedInstruction), // not exactly what i'm looking for
        };
        *destination = rt_value;
        Ok(())
    }

    fn eret(&mut self, instruction: Instruction) -> Result<(), Exception> {
        if instruction == 0x42000018 {
            self.pc = self.registers.epc;
            self.registers.status &= !(0x2u32); // set bit 1 to 0
            Ok(())
        } else {
            Err(Exception::ReservedInstruction)
        }
    }
}
