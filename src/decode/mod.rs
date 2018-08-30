use super::consts::{ADDI, ALU, CMP, EXP, JCC, JMP, JPR, LD, MOV, NOP, SPC, ST};

#[macro_use]
mod macros;

pub fn decode(inst_reg: u32) -> Instruction {
    let mut x = Instruction::new(inst_reg);

    let i_immd_zex = (inst_reg >> 20) & 0xfff;
    let i_immd_sex = to_i_immd_sex!(i_immd_zex);
    let s_immd_zex = to_s_immd_zex!(inst_reg);
    let s_immd_sex = to_s_immd_sex!(s_immd_zex);
    let u_immd_zex = ((inst_reg >> 12) & 0xfffff) << 12;
    let u_immd_sex = u_immd_zex;
    let b_immd_zex = to_b_immd_zex!(inst_reg);
    let b_immd_sex = to_b_immd_sex!(b_immd_zex);
    let j_immd_zex = to_j_immd_zex!(inst_reg);
    let j_immd_sex = to_j_immd_sex!(j_immd_zex);

    let o = x.opcode;
    match o {
        o if o == (InstructionType::A as u32) => {
            x.match_0x37(u_immd_sex);
            return x;
        }
        o if o == (InstructionType::B as u32) => {
            x.match_0x17(u_immd_sex);
            return x;
        }
        o if o == (InstructionType::C as u32) => {
            x.match_0x6f(j_immd_sex);
            return x;
        }
        o if o == (InstructionType::D as u32) => {
            x.match_0x67();
            return x;
        }
        o if o == (InstructionType::E as u32) => {
            x.match_0x63(b_immd_sex);
            return x;
        }
        o if o == (InstructionType::F as u32) => {
            x.match_0x03(i_immd_sex);
            return x;
        }
        o if o == (InstructionType::G as u32) => {
            x.match_0x23(s_immd_sex);
            return x;
        }
        o if o == (InstructionType::H as u32) => {
            x.match_0x13(i_immd_sex);
            return x;
        }
        o if o == (InstructionType::I as u32) => {
            x.match_0x33(b_immd_sex);
            return x;
        }
        o if o == (InstructionType::J as u32) => {
            x.match_0x0f(i_immd_sex);
            return x;
        }
        o if o == (InstructionType::K as u32) => {
            x.match_0x73(i_immd_sex);
            return x;
        }
        _ => {
            x.match_default();
            return x;
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: u32,
    pub rd: u32,
    pub funct3: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub rs3: u32,
    pub funct7: u32,
    pub immd: u32,
    pub op: u32,
    pub op2: u32,
}

impl Instruction {
    fn new(inst_reg: u32) -> Instruction {
        return Instruction {
            opcode: inst_reg & 0x7f,
            rd: (inst_reg >> 7) & 0x1f,
            funct3: (inst_reg >> 12) & 0x7,
            rs1: (inst_reg >> 15) & 0x1f,
            rs2: (inst_reg >> 20) & 0x1f,
            rs3: 0,
            funct7: (inst_reg >> 25) & 0x3f,
            immd: 0,
            op: 0,
            op2: 0,
        };
    }

    fn match_0x37(&mut self, u_immd_sex: u32) {
        self.immd = u_immd_sex;
        self.rs1 = self.rs1;
        self.rs2 = 32; // immd
        self.rd = self.rd;
        self.op = MOV;
        self.op2 = NOP;
    }

    fn match_0x17(&mut self, u_immd_sex: u32) {
        self.immd = u_immd_sex;
        self.rs1 = 33; // PC
        self.rs2 = 32; // immd
        self.rd = self.rd;
        self.op = ADDI;
        self.op2 = NOP;
    }

    fn match_0x6f(&mut self, j_immd_sex: u32) {
        self.immd = j_immd_sex;
        self.rs1 = self.rs1;
        self.rs2 = 34; // PC+4
        self.rd = self.rd;
        self.op = MOV;
        self.op2 = JMP;
    }

    fn match_0x67(&mut self) {
        self.rs1 = self.rs1;
        self.rs2 = 34; // PC+4
        self.rd = self.rd;
        self.op = MOV;
        self.op2 = JPR;
    }

    fn match_0x63(&mut self, b_immd_sex: u32) {
        self.immd = b_immd_sex;
        self.rs1 = self.rs1;
        self.rs2 = self.rs2;
        self.rd = 0;
        self.op = CMP; // see also funct3
        self.op2 = JCC;
    }

    fn match_0x03(&mut self, i_immd_sex: u32) {
        self.immd = i_immd_sex;
        self.rs1 = self.rs1;
        self.rs2 = 32; // immd
        self.rd = self.rd;
        self.op = ADDI;
        self.op2 = LD; // see also funct3
    }

    fn match_0x23(&mut self, s_immd_sex: u32) {
        self.immd = s_immd_sex;
        self.rs1 = self.rs1;
        self.rs3 = self.rs2;
        self.rs2 = 32; // immd
        self.rd = 0;
        self.op = ADDI;
        self.op2 = ST; // see also funct3
    }

    fn match_0x13(&mut self, i_immd_sex: u32) {
        self.immd = if (self.funct3 == 1) || (self.funct3 == 5) {
            i_immd_sex & 0x1f
        } else {
            i_immd_sex
        };
        self.rs1 = self.rs1;
        self.rs2 = 32; // immd
        self.rd = self.rd;
        self.op = if self.funct3 == 0 { ADDI } else { ALU }; // see also funct3
        self.op2 = NOP;
    }

    fn match_0x33(&mut self, b_immd_sex: u32) {
        self.immd = b_immd_sex;
        self.rs1 = self.rs1;
        self.rs2 = self.rs2;
        self.rd = self.rd;
        self.op = ALU; // see also funct3
        self.op2 = NOP;
    }

    fn match_0x0f(&mut self, i_immd_sex: u32) {
        // FENCE
        self.immd = i_immd_sex;
        self.rs1 = self.rs1;
        self.rs2 = self.rs2;
        self.rd = 0;
        self.op = NOP;
        self.op2 = NOP;
    }

    fn match_0x73(&mut self, i_immd_sex: u32) {
        self.immd = i_immd_sex;
        self.rs1 = self.rs1;
        self.rs2 = self.rs2;
        self.rd = 0;
        self.op = NOP;
        self.op2 = SPC;
    }

    fn match_default(&mut self) {
        self.rs1 = 0;
        self.rs2 = 0;
        self.rd = 0;
        self.op = NOP;
        self.op2 = EXP;
    }
}

enum InstructionType {
    A = 0x37,
    B = 0x17,
    C = 0x6f,
    D = 0x67,
    E = 0x63,
    F = 0x03,
    G = 0x23,
    H = 0x13,
    I = 0x33,
    J = 0x0f,
    K = 0x73,
}
