use std::{
    borrow::Borrow,
    cell::{Cell, RefCell},
};

use crate::{arithmetic::Inc16, gate::*};

/// 順序回路を使う回路ですべてclock_up(), clock_down()をちゃんと呼ぶ必要がある

#[derive(Debug)]
pub struct DFF {
    pub out: SharedBus<1>,
    input: SharedBus<1>,
    state: SharedBus<1>,
}

impl DFF {
    pub fn new(input: SharedBus<1>) -> DFF {
        let state = Bus::all0().to_shared_bus();
        let out = Bus::all0().to_shared_bus();
        DFF { out, input, state }
    }
}

impl Gate for DFF {
    fn clock_up(&self) -> () {
        // clock_upではINPUTの値をstatenにセットするだけ
        // このときのoutは古いまま
        let input_bit = self.input.get_shared_bit(0).get();
        self.state.get_shared_bit(0).set(input_bit);
    }

    fn clock_down(&self) -> () {
        // clock_upでstateをoutに更新する
        let state_bit = self.state.get_shared_bit(0).get();
        self.out.get_shared_bit(0).set(state_bit);
    }
}

#[derive(Debug)]
pub struct OneBitRegister {
    pub out: SharedBus<1>,
    mux: Mux<1>,
    dff: DFF,
    feedback: SharedBus<1>,
}

impl OneBitRegister {
    pub fn new(input: SharedBus<1>, load: SharedBus<1>) -> OneBitRegister {
        let feedback = Bus::<1>::all0().to_shared_bus();
        let mux = Mux::<1>::new(feedback.clone(), input.clone(), load.clone());
        let dff = DFF::new(mux.out.clone());

        OneBitRegister {
            out: dff.out.clone(),
            mux,
            dff,
            feedback,
        }
    }
}

impl Gate for OneBitRegister {
    fn clock_up(&self) -> () {
        self.dff.clock_up();
    }

    fn clock_down(&self) -> () {
        self.dff.clock_down();
    }

    fn re_compute(&self) -> () {
        let dff_out_bit = self.dff.out.get_shared_bit(0).get();
        self.feedback.borrow().get_shared_bit(0).set(dff_out_bit);

        self.mux.re_compute();
    }
}

#[derive(Debug)]
pub struct Register {
    pub out: SharedBus<16>,
    one_bit0: OneBitRegister,
    one_bit1: OneBitRegister,
    one_bit2: OneBitRegister,
    one_bit3: OneBitRegister,
    one_bit4: OneBitRegister,
    one_bit5: OneBitRegister,
    one_bit6: OneBitRegister,
    one_bit7: OneBitRegister,
    one_bit8: OneBitRegister,
    one_bit9: OneBitRegister,
    one_bit10: OneBitRegister,
    one_bit11: OneBitRegister,
    one_bit12: OneBitRegister,
    one_bit13: OneBitRegister,
    one_bit14: OneBitRegister,
    one_bit15: OneBitRegister,
}

impl Register {
    pub fn new(input: SharedBus<16>, load: SharedBus<1>) -> Register {
        let one_bit0 = OneBitRegister::new(input.reconnect([0]), load.clone());
        let one_bit1 = OneBitRegister::new(input.reconnect([1]), load.clone());
        let one_bit2 = OneBitRegister::new(input.reconnect([2]), load.clone());
        let one_bit3 = OneBitRegister::new(input.reconnect([3]), load.clone());
        let one_bit4 = OneBitRegister::new(input.reconnect([4]), load.clone());
        let one_bit5 = OneBitRegister::new(input.reconnect([5]), load.clone());
        let one_bit6 = OneBitRegister::new(input.reconnect([6]), load.clone());
        let one_bit7 = OneBitRegister::new(input.reconnect([7]), load.clone());
        let one_bit8 = OneBitRegister::new(input.reconnect([8]), load.clone());
        let one_bit9 = OneBitRegister::new(input.reconnect([9]), load.clone());
        let one_bit10 = OneBitRegister::new(input.reconnect([10]), load.clone());
        let one_bit11 = OneBitRegister::new(input.reconnect([11]), load.clone());
        let one_bit12 = OneBitRegister::new(input.reconnect([12]), load.clone());
        let one_bit13 = OneBitRegister::new(input.reconnect([13]), load.clone());
        let one_bit14 = OneBitRegister::new(input.reconnect([14]), load.clone());
        let one_bit15 = OneBitRegister::new(input.reconnect([15]), load.clone());

        let out = Bus::new([
            one_bit0.out.get_shared_bit(0),
            one_bit1.out.get_shared_bit(0),
            one_bit2.out.get_shared_bit(0),
            one_bit3.out.get_shared_bit(0),
            one_bit4.out.get_shared_bit(0),
            one_bit5.out.get_shared_bit(0),
            one_bit6.out.get_shared_bit(0),
            one_bit7.out.get_shared_bit(0),
            one_bit8.out.get_shared_bit(0),
            one_bit9.out.get_shared_bit(0),
            one_bit10.out.get_shared_bit(0),
            one_bit11.out.get_shared_bit(0),
            one_bit12.out.get_shared_bit(0),
            one_bit13.out.get_shared_bit(0),
            one_bit14.out.get_shared_bit(0),
            one_bit15.out.get_shared_bit(0),
        ])
        .to_shared_bus();

        Register {
            out,
            one_bit0,
            one_bit1,
            one_bit2,
            one_bit3,
            one_bit4,
            one_bit5,
            one_bit6,
            one_bit7,
            one_bit8,
            one_bit9,
            one_bit10,
            one_bit11,
            one_bit12,
            one_bit13,
            one_bit14,
            one_bit15,
        }
    }
}

impl Gate for Register {
    fn clock_up(&self) -> () {
        self.one_bit0.clock_up();
        self.one_bit1.clock_up();
        self.one_bit2.clock_up();
        self.one_bit3.clock_up();
        self.one_bit4.clock_up();
        self.one_bit5.clock_up();
        self.one_bit6.clock_up();
        self.one_bit7.clock_up();
        self.one_bit8.clock_up();
        self.one_bit9.clock_up();
        self.one_bit10.clock_up();
        self.one_bit11.clock_up();
        self.one_bit12.clock_up();
        self.one_bit13.clock_up();
        self.one_bit14.clock_up();
        self.one_bit15.clock_up();
    }

    fn clock_down(&self) -> () {
        self.one_bit0.clock_down();
        self.one_bit1.clock_down();
        self.one_bit2.clock_down();
        self.one_bit3.clock_down();
        self.one_bit4.clock_down();
        self.one_bit5.clock_down();
        self.one_bit6.clock_down();
        self.one_bit7.clock_down();
        self.one_bit8.clock_down();
        self.one_bit9.clock_down();
        self.one_bit10.clock_down();
        self.one_bit11.clock_down();
        self.one_bit12.clock_down();
        self.one_bit13.clock_down();
        self.one_bit14.clock_down();
        self.one_bit15.clock_down();
    }

    fn re_compute(&self) -> () {
        self.one_bit0.re_compute();
        self.one_bit1.re_compute();
        self.one_bit2.re_compute();
        self.one_bit3.re_compute();
        self.one_bit4.re_compute();
        self.one_bit5.re_compute();
        self.one_bit6.re_compute();
        self.one_bit7.re_compute();
        self.one_bit8.re_compute();
        self.one_bit9.re_compute();
        self.one_bit10.re_compute();
        self.one_bit11.re_compute();
        self.one_bit12.re_compute();
        self.one_bit13.re_compute();
        self.one_bit14.re_compute();
        self.one_bit15.re_compute();
    }
}

#[derive(Debug)]
pub struct RAM8 {
    pub out: SharedBus<16>,
    dmux8way: DMux8Way,
    reg1: Register,
    reg2: Register,
    reg3: Register,
    reg4: Register,
    reg5: Register,
    reg6: Register,
    reg7: Register,
    reg8: Register,
    mux8way16: Mux8Way16,
}

#[allow(dead_code)]
impl RAM8 {
    pub fn new(input: SharedBus<16>, load: SharedBus<1>, address: SharedBus<3>) -> RAM8 {
        let dmux8way = DMux8Way::new(load.clone(), address.clone());
        let reg1 = Register::new(input.clone(), dmux8way.out1.clone());
        let reg2 = Register::new(input.clone(), dmux8way.out2.clone());
        let reg3 = Register::new(input.clone(), dmux8way.out3.clone());
        let reg4 = Register::new(input.clone(), dmux8way.out4.clone());
        let reg5 = Register::new(input.clone(), dmux8way.out5.clone());
        let reg6 = Register::new(input.clone(), dmux8way.out6.clone());
        let reg7 = Register::new(input.clone(), dmux8way.out7.clone());
        let reg8 = Register::new(input.clone(), dmux8way.out8.clone());
        let mux8way16 = Mux8Way16::new(
            reg1.out.clone(),
            reg2.out.clone(),
            reg3.out.clone(),
            reg4.out.clone(),
            reg5.out.clone(),
            reg6.out.clone(),
            reg7.out.clone(),
            reg8.out.clone(),
            address.clone(),
        );

        RAM8 {
            out: mux8way16.out.clone(),
            dmux8way,
            reg1,
            reg2,
            reg3,
            reg4,
            reg5,
            reg6,
            reg7,
            reg8,
            mux8way16,
        }
    }
}

impl Gate for RAM8 {
    fn clock_up(&self) -> () {
        self.dmux8way.clock_up();
        self.reg1.clock_up();
        self.reg2.clock_up();
        self.reg3.clock_up();
        self.reg4.clock_up();
        self.reg5.clock_up();
        self.reg6.clock_up();
        self.reg7.clock_up();
        self.reg8.clock_up();
        self.mux8way16.clock_up();
    }

    fn clock_down(&self) -> () {
        self.dmux8way.clock_down();
        self.reg1.clock_down();
        self.reg2.clock_down();
        self.reg3.clock_down();
        self.reg4.clock_down();
        self.reg5.clock_down();
        self.reg6.clock_down();
        self.reg7.clock_down();
        self.reg8.clock_down();
        self.mux8way16.clock_down();
    }

    fn re_compute(&self) -> () {
        self.dmux8way.re_compute();
        self.reg1.re_compute();
        self.reg2.re_compute();
        self.reg3.re_compute();
        self.reg4.re_compute();
        self.reg5.re_compute();
        self.reg6.re_compute();
        self.reg7.re_compute();
        self.reg8.re_compute();
        self.mux8way16.re_compute();
    }
}

#[derive(Debug)]
pub struct RAM64 {
    pub out: SharedBus<16>,
    dmux8way: Box<DMux8Way>,
    ram8_1: Box<RAM8>,
    ram8_2: Box<RAM8>,
    ram8_3: Box<RAM8>,
    ram8_4: Box<RAM8>,
    ram8_5: Box<RAM8>,
    ram8_6: Box<RAM8>,
    ram8_7: Box<RAM8>,
    ram8_8: Box<RAM8>,
    mux8way16: Box<Mux8Way16>,
}

#[allow(dead_code)]
impl RAM64 {
    pub fn new(input: SharedBus<16>, load: SharedBus<1>, address: SharedBus<6>) -> RAM64 {
        let low_address = address.reconnect([0, 1, 2]);
        let high_address = address.reconnect([3, 4, 5]);
        let dmux8way = Box::new(DMux8Way::new(load.clone(), high_address.clone()));
        let ram8_1 = Box::new(RAM8::new(
            input.clone(),
            dmux8way.out1.clone(),
            low_address.clone(),
        ));
        let ram8_2 = Box::new(RAM8::new(
            input.clone(),
            dmux8way.out2.clone(),
            low_address.clone(),
        ));
        let ram8_3 = Box::new(RAM8::new(
            input.clone(),
            dmux8way.out3.clone(),
            low_address.clone(),
        ));
        let ram8_4 = Box::new(RAM8::new(
            input.clone(),
            dmux8way.out4.clone(),
            low_address.clone(),
        ));
        let ram8_5 = Box::new(RAM8::new(
            input.clone(),
            dmux8way.out5.clone(),
            low_address.clone(),
        ));
        let ram8_6 = Box::new(RAM8::new(
            input.clone(),
            dmux8way.out6.clone(),
            low_address.clone(),
        ));
        let ram8_7 = Box::new(RAM8::new(
            input.clone(),
            dmux8way.out7.clone(),
            low_address.clone(),
        ));
        let ram8_8 = Box::new(RAM8::new(
            input.clone(),
            dmux8way.out8.clone(),
            low_address.clone(),
        ));
        let mux8way16 = Box::new(Mux8Way16::new(
            ram8_1.out.clone(),
            ram8_2.out.clone(),
            ram8_3.out.clone(),
            ram8_4.out.clone(),
            ram8_5.out.clone(),
            ram8_6.out.clone(),
            ram8_7.out.clone(),
            ram8_8.out.clone(),
            high_address.clone(),
        ));

        RAM64 {
            out: mux8way16.out.clone(),
            dmux8way,
            ram8_1,
            ram8_2,
            ram8_3,
            ram8_4,
            ram8_5,
            ram8_6,
            ram8_7,
            ram8_8,
            mux8way16,
        }
    }
}

impl Gate for RAM64 {
    fn clock_up(&self) -> () {
        self.dmux8way.clock_up();
        self.ram8_1.clock_up();
        self.ram8_2.clock_up();
        self.ram8_3.clock_up();
        self.ram8_4.clock_up();
        self.ram8_5.clock_up();
        self.ram8_6.clock_up();
        self.ram8_7.clock_up();
        self.ram8_8.clock_up();
        self.mux8way16.clock_up();
    }

    fn clock_down(&self) -> () {
        self.dmux8way.clock_down();
        self.ram8_1.clock_down();
        self.ram8_2.clock_down();
        self.ram8_3.clock_down();
        self.ram8_4.clock_down();
        self.ram8_5.clock_down();
        self.ram8_6.clock_down();
        self.ram8_7.clock_down();
        self.ram8_8.clock_down();
        self.mux8way16.clock_down();
    }

    fn re_compute(&self) -> () {
        self.dmux8way.re_compute();
        self.ram8_1.re_compute();
        self.ram8_2.re_compute();
        self.ram8_3.re_compute();
        self.ram8_4.re_compute();
        self.ram8_5.re_compute();
        self.ram8_6.re_compute();
        self.ram8_7.re_compute();
        self.ram8_8.re_compute();
        self.mux8way16.re_compute();
    }
}

#[derive(Debug)]
pub struct RAM512 {
    pub out: SharedBus<16>,
    dmux8way: Box<DMux8Way>,
    ram64_1: Box<RAM64>,
    ram64_2: Box<RAM64>,
    ram64_3: Box<RAM64>,
    ram64_4: Box<RAM64>,
    ram64_5: Box<RAM64>,
    ram64_6: Box<RAM64>,
    ram64_7: Box<RAM64>,
    ram64_8: Box<RAM64>,
    mux8way16: Box<Mux8Way16>,
}

#[allow(dead_code)]
impl RAM512 {
    pub fn new(input: SharedBus<16>, load: SharedBus<1>, address: SharedBus<9>) -> RAM512 {
        let low_address = address.reconnect([0, 1, 2, 3, 4, 5]);
        let high_address = address.reconnect([6, 7, 8]);
        let dmux8way = Box::new(DMux8Way::new(load.clone(), high_address.clone()));
        let ram64_1 = Box::new(RAM64::new(
            input.clone(),
            dmux8way.out1.clone(),
            low_address.clone(),
        ));
        let ram64_2 = Box::new(RAM64::new(
            input.clone(),
            dmux8way.out2.clone(),
            low_address.clone(),
        ));
        let ram64_3 = Box::new(RAM64::new(
            input.clone(),
            dmux8way.out3.clone(),
            low_address.clone(),
        ));
        let ram64_4 = Box::new(RAM64::new(
            input.clone(),
            dmux8way.out4.clone(),
            low_address.clone(),
        ));
        let ram64_5 = Box::new(RAM64::new(
            input.clone(),
            dmux8way.out5.clone(),
            low_address.clone(),
        ));
        let ram64_6 = Box::new(RAM64::new(
            input.clone(),
            dmux8way.out6.clone(),
            low_address.clone(),
        ));
        let ram64_7 = Box::new(RAM64::new(
            input.clone(),
            dmux8way.out7.clone(),
            low_address.clone(),
        ));
        let ram64_8 = Box::new(RAM64::new(
            input.clone(),
            dmux8way.out8.clone(),
            low_address.clone(),
        ));
        let mux8way16 = Box::new(Mux8Way16::new(
            ram64_1.out.clone(),
            ram64_2.out.clone(),
            ram64_3.out.clone(),
            ram64_4.out.clone(),
            ram64_5.out.clone(),
            ram64_6.out.clone(),
            ram64_7.out.clone(),
            ram64_8.out.clone(),
            high_address.clone(),
        ));

        RAM512 {
            out: mux8way16.out.clone(),
            dmux8way,
            ram64_1,
            ram64_2,
            ram64_3,
            ram64_4,
            ram64_5,
            ram64_6,
            ram64_7,
            ram64_8,
            mux8way16,
        }
    }
}

impl Gate for RAM512 {
    fn clock_up(&self) -> () {
        self.dmux8way.clock_up();
        self.ram64_1.clock_up();
        self.ram64_2.clock_up();
        self.ram64_3.clock_up();
        self.ram64_4.clock_up();
        self.ram64_5.clock_up();
        self.ram64_6.clock_up();
        self.ram64_7.clock_up();
        self.ram64_8.clock_up();
        self.mux8way16.clock_up();
    }

    fn clock_down(&self) -> () {
        self.dmux8way.clock_down();
        self.ram64_1.clock_down();
        self.ram64_2.clock_down();
        self.ram64_3.clock_down();
        self.ram64_4.clock_down();
        self.ram64_5.clock_down();
        self.ram64_6.clock_down();
        self.ram64_7.clock_down();
        self.ram64_8.clock_down();
        self.mux8way16.clock_down();
    }

    fn re_compute(&self) -> () {
        self.dmux8way.re_compute();
        self.ram64_1.re_compute();
        self.ram64_2.re_compute();
        self.ram64_3.re_compute();
        self.ram64_4.re_compute();
        self.ram64_5.re_compute();
        self.ram64_6.re_compute();
        self.ram64_7.re_compute();
        self.ram64_8.re_compute();
        self.mux8way16.re_compute();
    }
}

#[derive(Debug)]
pub struct RAM4K {
    pub out: SharedBus<16>,
    dmux8way: Box<DMux8Way>,
    ram512_1: Box<RAM512>,
    ram512_2: Box<RAM512>,
    ram512_3: Box<RAM512>,
    ram512_4: Box<RAM512>,
    ram512_5: Box<RAM512>,
    ram512_6: Box<RAM512>,
    ram512_7: Box<RAM512>,
    ram512_8: Box<RAM512>,
    mux8way16: Box<Mux8Way16>,
}

#[allow(dead_code)]
impl RAM4K {
    pub fn new(input: SharedBus<16>, load: SharedBus<1>, address: SharedBus<12>) -> RAM4K {
        let low_address = address.reconnect([0, 1, 2, 3, 4, 5, 6, 7, 8]);
        let high_address = address.reconnect([9, 10, 11]);
        let dmux8way = Box::new(DMux8Way::new(load.clone(), high_address.clone()));
        let ram512_1 = Box::new(RAM512::new(
            input.clone(),
            dmux8way.out1.clone(),
            low_address.clone(),
        ));
        let ram512_2 = Box::new(RAM512::new(
            input.clone(),
            dmux8way.out2.clone(),
            low_address.clone(),
        ));
        let ram512_3 = Box::new(RAM512::new(
            input.clone(),
            dmux8way.out3.clone(),
            low_address.clone(),
        ));
        let ram512_4 = Box::new(RAM512::new(
            input.clone(),
            dmux8way.out4.clone(),
            low_address.clone(),
        ));
        let ram512_5 = Box::new(RAM512::new(
            input.clone(),
            dmux8way.out5.clone(),
            low_address.clone(),
        ));
        let ram512_6 = Box::new(RAM512::new(
            input.clone(),
            dmux8way.out6.clone(),
            low_address.clone(),
        ));
        let ram512_7 = Box::new(RAM512::new(
            input.clone(),
            dmux8way.out7.clone(),
            low_address.clone(),
        ));
        let ram512_8 = Box::new(RAM512::new(
            input.clone(),
            dmux8way.out8.clone(),
            low_address.clone(),
        ));
        let mux8way16 = Box::new(Mux8Way16::new(
            ram512_1.out.clone(),
            ram512_2.out.clone(),
            ram512_3.out.clone(),
            ram512_4.out.clone(),
            ram512_5.out.clone(),
            ram512_6.out.clone(),
            ram512_7.out.clone(),
            ram512_8.out.clone(),
            high_address.clone(),
        ));

        RAM4K {
            out: mux8way16.out.clone(),
            dmux8way,
            ram512_1,
            ram512_2,
            ram512_3,
            ram512_4,
            ram512_5,
            ram512_6,
            ram512_7,
            ram512_8,
            mux8way16,
        }
    }
}

impl Gate for RAM4K {
    fn clock_up(&self) -> () {
        self.dmux8way.clock_up();
        self.ram512_1.clock_up();
        self.ram512_2.clock_up();
        self.ram512_3.clock_up();
        self.ram512_4.clock_up();
        self.ram512_5.clock_up();
        self.ram512_6.clock_up();
        self.ram512_7.clock_up();
        self.ram512_8.clock_up();
        self.mux8way16.clock_up();
    }

    fn clock_down(&self) -> () {
        self.dmux8way.clock_down();
        self.ram512_1.clock_down();
        self.ram512_2.clock_down();
        self.ram512_3.clock_down();
        self.ram512_4.clock_down();
        self.ram512_5.clock_down();
        self.ram512_6.clock_down();
        self.ram512_7.clock_down();
        self.ram512_8.clock_down();
        self.mux8way16.clock_down();
    }

    fn re_compute(&self) -> () {
        self.dmux8way.re_compute();
        self.ram512_1.re_compute();
        self.ram512_2.re_compute();
        self.ram512_3.re_compute();
        self.ram512_4.re_compute();
        self.ram512_5.re_compute();
        self.ram512_6.re_compute();
        self.ram512_7.re_compute();
        self.ram512_8.re_compute();
        self.mux8way16.re_compute();
    }
}

#[derive(Debug)]
pub struct RAM16K {
    pub out: SharedBus<16>,
    dmux4way: Box<DMux4Way>,
    ram4k_1: Box<RAM4K>,
    ram4k_2: Box<RAM4K>,
    ram4k_3: Box<RAM4K>,
    ram4k_4: Box<RAM4K>,
    mux4way16: Box<Mux4Way16>,
}

#[allow(dead_code)]
impl RAM16K {
    pub fn new(input: SharedBus<16>, load: SharedBus<1>, address: SharedBus<14>) -> RAM16K {
        let low_address = address.reconnect([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        let high_address = address.reconnect([12, 13]);
        let dmux4way = Box::new(DMux4Way::new(load.clone(), high_address.clone()));
        let ram4k_1 = Box::new(RAM4K::new(
            input.clone(),
            dmux4way.out1.clone(),
            low_address.clone(),
        ));
        let ram4k_2 = Box::new(RAM4K::new(
            input.clone(),
            dmux4way.out2.clone(),
            low_address.clone(),
        ));
        let ram4k_3 = Box::new(RAM4K::new(
            input.clone(),
            dmux4way.out3.clone(),
            low_address.clone(),
        ));
        let ram4k_4 = Box::new(RAM4K::new(
            input.clone(),
            dmux4way.out4.clone(),
            low_address.clone(),
        ));
        let mux4way16 = Box::new(Mux4Way16::new(
            ram4k_1.out.clone(),
            ram4k_2.out.clone(),
            ram4k_3.out.clone(),
            ram4k_4.out.clone(),
            high_address.clone(),
        ));

        RAM16K {
            out: mux4way16.out.clone(),
            dmux4way,
            ram4k_1,
            ram4k_2,
            ram4k_3,
            ram4k_4,
            mux4way16,
        }
    }
}

impl Gate for RAM16K {
    fn clock_up(&self) -> () {
        self.dmux4way.clock_up();
        self.ram4k_1.clock_up();
        self.ram4k_2.clock_up();
        self.ram4k_3.clock_up();
        self.ram4k_4.clock_up();
        self.mux4way16.clock_up();
    }

    fn clock_down(&self) -> () {
        self.dmux4way.clock_down();
        self.ram4k_1.clock_down();
        self.ram4k_2.clock_down();
        self.ram4k_3.clock_down();
        self.ram4k_4.clock_down();
        self.mux4way16.clock_down();
    }

    fn re_compute(&self) -> () {
        self.dmux4way.re_compute();
        self.ram4k_1.re_compute();
        self.ram4k_2.re_compute();
        self.ram4k_3.re_compute();
        self.ram4k_4.re_compute();
        self.mux4way16.re_compute();
    }
}

#[derive(Debug)]
pub struct RAM16KBuiltIn {
    pub out: SharedBus<16>,
    pub ram: RefCell<Box<[u16; 16384]>>,
    input: SharedBus<16>,
    load: SharedBus<1>,
    address: SharedBus<14>,
    next: Cell<u16>,
}

impl RAM16KBuiltIn {
    pub fn new(input: SharedBus<16>, load: SharedBus<1>, address: SharedBus<14>) -> RAM16KBuiltIn {
        let ram: RefCell<Box<[u16; 16384]>> = RefCell::new(Box::new([0; 16384]));
        let out = Bus::all0().to_shared_bus();
        let next = Cell::new(0);
        RAM16KBuiltIn {
            out,
            ram,
            input,
            load,
            address,
            next,
        }
    }

    fn bus_to_u16<const N: usize>(bus: SharedBus<N>) -> u16 {
        let mut u = 0;
        for i in 0..N {
            u += match bus.get_shared_bit(i).get() {
                I => 2_u16.pow(i.try_into().unwrap()),
                O => 0,
            }
        }
        u
    }
}

impl Gate for RAM16KBuiltIn {
    fn clock_up(&self) -> () {
        if self.load.get_shared_bit(0).get() == I {
            let value = Self::bus_to_u16::<16>(self.input.clone());
            self.next.set(value);
        }
    }

    fn clock_down(&self) -> () {
        if self.load.get_shared_bit(0).get() == I {
            let load_address = Self::bus_to_u16::<14>(self.address.clone()) as usize;
            let mut ram = self.ram.borrow_mut();
            ram[load_address] = self.next.get();
        }
    }

    fn re_compute(&self) -> () {
        let address = Self::bus_to_u16::<14>(self.address.clone()) as usize;
        let mut value = self.ram.borrow()[address];

        let mut bits = vec![];
        loop {
            bits.push(match value % 2 {
                1 => I,
                _ => O,
            });
            value = value / 2;
            if value < 1 {
                break;
            }
        }

        for i in 0..16 {
            self.out
                .get_shared_bit(i)
                .set(bits.get(i).map(|b| b.clone()).unwrap_or(O));
        }
    }
}

#[derive(Debug)]
pub struct PC {
    pub out: SharedBus<16>,
    feedback: SharedBus<16>,
    inc16: Inc16,
    mux16_1: Mux<16>,
    mux16_2: Mux<16>,
    mux16_3: Mux<16>,
    reg: Register,
}

impl PC {
    pub fn new(
        input: SharedBus<16>,
        load: SharedBus<1>,
        inc: SharedBus<1>,
        reset: SharedBus<1>,
    ) -> PC {
        let feedback = Bus::<16>::all0().to_shared_bus();
        let inc16 = Inc16::new(feedback.clone());
        let mux16_1 = Mux::new(feedback.clone(), inc16.out.clone(), inc.clone());
        let mux16_2 = Mux::new(mux16_1.out.clone(), input.clone(), load.clone());
        let mux16_3 = Mux::new(
            mux16_2.out.clone(),
            Bus::all0().to_shared_bus(),
            reset.clone(),
        );
        let reg = Register::new(mux16_3.out.clone(), Bus::all1().to_shared_bus());

        PC {
            out: reg.out.clone(),
            feedback,
            inc16,
            mux16_1,
            mux16_2,
            mux16_3,
            reg,
        }
    }
}

impl Gate for PC {
    fn clock_up(&self) -> () {
        self.inc16.clock_up();
        self.mux16_1.clock_up();
        self.mux16_2.clock_up();
        self.mux16_3.clock_up();
        self.reg.clock_up();
    }

    fn clock_down(&self) -> () {
        self.inc16.clock_down();
        self.mux16_1.clock_down();
        self.mux16_2.clock_down();
        self.mux16_3.clock_down();
        self.reg.clock_down();
    }

    fn re_compute(&self) -> () {
        // regのoutをmuxのaに反映
        for i in 0..16 {
            let reg_out_bit = self.reg.out.get_shared_bit(i).get();
            self.feedback.borrow().get_shared_bit(i).set(reg_out_bit);
        }

        self.inc16.re_compute();
        self.mux16_1.re_compute();
        self.mux16_2.re_compute();
        self.mux16_3.re_compute();
        self.reg.re_compute();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dff() {
        let input = Bus::<1>::all0().to_shared_bus();
        let dff = DFF::new(input.clone());

        assert_eq!(dff.out.clone(), Bus::<1>::all0().to_shared_bus());

        dff.clock_up();
        assert_eq!(dff.out.clone(), Bus::<1>::all0().to_shared_bus());

        dff.clock_down();
        assert_eq!(dff.out.clone(), Bus::<1>::all0().to_shared_bus());

        input.get_shared_bit(0).set(I);

        dff.clock_up();
        assert_eq!(dff.out.clone(), Bus::<1>::all0().to_shared_bus());

        dff.clock_down();
        assert_eq!(dff.out.clone(), Bus::<1>::all1().to_shared_bus());
    }

    #[test]
    fn one_bit_register() {
        let input = Bus::<1>::all0().to_shared_bus();
        let load = Bus::<1>::all0().to_shared_bus();
        let one_bit_register = OneBitRegister::new(input.clone(), load.clone());

        assert_eq!(
            one_bit_register.out.clone(),
            Bus::<1>::all0().to_shared_bus()
        );

        // 1を記憶させる
        input.get_shared_bit(0).set(I);
        load.get_shared_bit(0).set(I);

        // tick
        one_bit_register.re_compute();
        one_bit_register.clock_up();
        assert_eq!(
            one_bit_register.out.clone(),
            Bus::<1>::all0().to_shared_bus()
        );
        assert_eq!(
            one_bit_register.mux.out.clone(),
            Bus::<1>::all1().to_shared_bus()
        );
        // この時点ではdffは0
        // 内部のstateは1のはず
        assert_eq!(
            one_bit_register.dff.out.clone(),
            Bus::<1>::all0().to_shared_bus()
        );

        // tock
        one_bit_register.clock_down();
        one_bit_register.re_compute();
        // ここで1が記憶されているはず
        assert_eq!(
            one_bit_register.out.clone(),
            Bus::<1>::all1().to_shared_bus()
        );
        assert_eq!(
            one_bit_register.dff.out.clone(),
            Bus::<1>::all1().to_shared_bus()
        );

        // なにもしない
        input.get_shared_bit(0).set(O);
        load.get_shared_bit(0).set(O);

        // ここではinput=0, load=0なので、dffからのフィードバックの1がDFFに出力されるはず
        assert_eq!(
            one_bit_register.dff.out.clone(),
            Bus::<1>::all1().to_shared_bus()
        );
        // よってmuxも1を出力しているはず
        assert_eq!(
            one_bit_register.mux.out.clone(),
            Bus::<1>::all1().to_shared_bus()
        );

        // tick
        one_bit_register.re_compute();
        one_bit_register.clock_up();
        // まだ1が記憶されているはず
        assert_eq!(
            one_bit_register.out.clone(),
            Bus::<1>::all1().to_shared_bus()
        );
        // dff
        // ここでもinput=0, load=0なので、dffからのフィードバックの1がDFFに出力されるはず
        // よってmuxも1を出力しているはず
        assert_eq!(
            one_bit_register.dff.out.clone(),
            Bus::<1>::all1().to_shared_bus()
        );
        assert_eq!(one_bit_register.dff.out.get_shared_bit(0).get(), I);
        assert_eq!(
            one_bit_register.mux.out.clone(),
            Bus::<1>::all1().to_shared_bus()
        );

        // tock
        one_bit_register.clock_down();
        one_bit_register.re_compute();
        // まだ1が記憶されているはず
        assert_eq!(
            one_bit_register.out.clone(),
            Bus::<1>::all1().to_shared_bus()
        );

        // tick
        one_bit_register.re_compute();
        one_bit_register.clock_up();
        // tock
        one_bit_register.clock_down();
        one_bit_register.re_compute();
        // まだ1が記憶されているはず
        assert_eq!(
            one_bit_register.out.clone(),
            Bus::<1>::all1().to_shared_bus()
        );

        // 0にする
        input.get_shared_bit(0).set(O);
        load.get_shared_bit(0).set(I);

        // tick
        one_bit_register.re_compute();
        one_bit_register.clock_up();
        // tickの直後ではまだ1
        assert_eq!(
            one_bit_register.out.clone(),
            Bus::<1>::all1().to_shared_bus()
        );

        // tock
        one_bit_register.clock_down();
        one_bit_register.re_compute();
        // tockの直後に0になる
        assert_eq!(
            one_bit_register.out.clone(),
            Bus::<1>::all0().to_shared_bus()
        );

        // tick
        one_bit_register.re_compute();
        one_bit_register.clock_up();
        // tock
        one_bit_register.clock_down();
        one_bit_register.re_compute();
        // まだ0が記憶されているはず
        assert_eq!(
            one_bit_register.out.clone(),
            Bus::<1>::all0().to_shared_bus()
        );
    }

    #[test]
    fn register() {
        // 順番に依存している
        let cases: Vec<(i16, &str, i16)> = vec![
            // in, load, out
            (0, "0", 0),
            (0, "0", 0),
            (0, "1", 0),
            (0, "1", 0),
            (-32123, "0", 0),
            (-32123, "0", 0),
            (11111, "0", 0),
            (11111, "0", 0),
            (-32123, "1", 0),
            (-32123, "1", -32123),
            (-32123, "1", -32123),
            (-32123, "1", -32123),
            (-32123, "0", -32123),
            (-32123, "0", -32123),
            (12345, "1", -32123),
            (12345, "1", 12345),
            (0, "0", 12345),
            (0, "0", 12345),
            (0, "1", 12345),
            (0, "1", 0),
            (1, "0", 0),
            (1, "0", 0),
            (1, "1", 0),
            (1, "1", 1),
            (2, "0", 1),
            (2, "0", 1),
            (2, "1", 1),
            (2, "1", 2),
            (4, "0", 2),
            (4, "0", 2),
            (4, "1", 2),
            (4, "1", 4),
            (8, "0", 4),
            (8, "0", 4),
            (8, "1", 4),
            (8, "1", 8),
            (16, "0", 8),
            (16, "0", 8),
            (16, "1", 8),
            (16, "1", 16),
            (32, "0", 16),
            (32, "0", 16),
            (32, "1", 16),
            (32, "1", 32),
            (64, "0", 32),
            (64, "0", 32),
            (64, "1", 32),
            (64, "1", 64),
            (128, "0", 64),
            (128, "0", 64),
            (128, "1", 64),
            (128, "1", 128),
            (256, "0", 128),
            (256, "0", 128),
            (256, "1", 128),
            (256, "1", 256),
            (512, "0", 256),
            (512, "0", 256),
            (512, "1", 256),
            (512, "1", 512),
            (1024, "0", 512),
            (1024, "0", 512),
            (1024, "1", 512),
            (1024, "1", 1024),
            (2048, "0", 1024),
            (2048, "0", 1024),
            (2048, "1", 1024),
            (2048, "1", 2048),
            (4096, "0", 2048),
            (4096, "0", 2048),
            (4096, "1", 2048),
            (4096, "1", 4096),
            (8192, "0", 4096),
            (8192, "0", 4096),
            (8192, "1", 4096),
            (8192, "1", 8192),
            (16384, "0", 8192),
            (16384, "0", 8192),
            (16384, "1", 8192),
            (16384, "1", 16384),
            (-32768, "0", 16384),
            (-32768, "0", 16384),
            (-32768, "1", 16384),
            (-32768, "1", -32768),
            (-2, "0", -32768),
            (-2, "0", -32768),
            (-2, "1", -32768),
            (-2, "1", -2),
            (-3, "0", -2),
            (-3, "0", -2),
            (-3, "1", -2),
            (-3, "1", -3),
            (-5, "0", -3),
            (-5, "0", -3),
            (-5, "1", -3),
            (-5, "1", -5),
            (-9, "0", -5),
            (-9, "0", -5),
            (-9, "1", -5),
            (-9, "1", -9),
            (-17, "0", -9),
            (-17, "0", -9),
            (-17, "1", -9),
            (-17, "1", -17),
            (-33, "0", -17),
            (-33, "0", -17),
            (-33, "1", -17),
            (-33, "1", -33),
            (-65, "0", -33),
            (-65, "0", -33),
            (-65, "1", -33),
            (-65, "1", -65),
            (-129, "0", -65),
            (-129, "0", -65),
            (-129, "1", -65),
            (-129, "1", -129),
            (-257, "0", -129),
            (-257, "0", -129),
            (-257, "1", -129),
            (-257, "1", -257),
            (-513, "0", -257),
            (-513, "0", -257),
            (-513, "1", -257),
            (-513, "1", -513),
            (-1025, "0", -513),
            (-1025, "0", -513),
            (-1025, "1", -513),
            (-1025, "1", -1025),
            (-2049, "0", -1025),
            (-2049, "0", -1025),
            (-2049, "1", -1025),
            (-2049, "1", -2049),
            (-4097, "0", -2049),
            (-4097, "0", -2049),
            (-4097, "1", -2049),
            (-4097, "1", -4097),
            (-8193, "0", -4097),
            (-8193, "0", -4097),
            (-8193, "1", -4097),
            (-8193, "1", -8193),
            (-16385, "0", -8193),
            (-16385, "0", -8193),
            (-16385, "1", -8193),
            (-16385, "1", -16385),
            (32767, "0", -16385),
            (32767, "0", -16385),
            (32767, "1", -16385),
            (32767, "1", 32767),
        ];

        let input: SharedBus<16> = Bus::all0().to_shared_bus();
        let load: SharedBus<1> = Bus::all0().to_shared_bus();
        let register = Register::new(input.clone(), load.clone());

        cases.chunks(2).for_each(|case| {
            let _input = i16_to_bus16(case[0].0);
            let _sel = case[0].1.parse::<Bus<1>>().unwrap();

            let out = i16_to_bus16(case[0].2).to_shared_bus();

            // inputとloadの中身を変える
            input.overwrite(&_input);
            load.overwrite(&_sel);

            // tick
            register.re_compute();
            register.clock_up();

            assert_eq!(register.out.clone(), out.clone());

            // tock
            register.clock_down();
            register.re_compute();

            let out = i16_to_bus16(case[1].2).to_shared_bus();
            assert_eq!(register.out.clone(), out.clone());
        });
    }

    #[test]
    fn ram8() {
        // 順番に依存している
        let cases: Vec<(i16, i16, i16, i16, bool)> = vec![
            // in, load, address,  out, tick?
            (0, 0, 0, 0, true),
            (0, 0, 0, 0, false),
            (0, 1, 0, 0, true),
            (0, 1, 0, 0, false),
            (11111, 0, 0, 0, true),
            (11111, 0, 0, 0, false),
            (11111, 1, 1, 0, true),
            (11111, 1, 1, 11111, false),
            (11111, 0, 0, 0, true),
            (11111, 0, 0, 0, false),
            (3333, 0, 3, 0, true),
            (3333, 0, 3, 0, false),
            (3333, 1, 3, 0, true),
            (3333, 1, 3, 3333, false),
            (3333, 0, 3, 3333, true),
            (3333, 0, 3, 3333, false),
            (3333, 0, 1, 11111, false),
            (7777, 0, 1, 11111, true),
            (7777, 0, 1, 11111, false),
            (7777, 1, 7, 0, true),
            (7777, 1, 7, 7777, false),
            (7777, 0, 7, 7777, true),
            (7777, 0, 7, 7777, false),
            (7777, 0, 3, 3333, false),
            (7777, 0, 7, 7777, false),
            (7777, 0, 0, 0, true),
            (7777, 0, 0, 0, false),
            (7777, 0, 1, 11111, false),
            (7777, 0, 2, 0, false),
            (7777, 0, 3, 3333, false),
            (7777, 0, 4, 0, false),
            (7777, 0, 5, 0, false),
            (7777, 0, 6, 0, false),
            (7777, 0, 7, 7777, false),
            (21845, 1, 0, 0, true),
            (21845, 1, 0, 21845, false),
            (21845, 1, 1, 11111, true),
            (21845, 1, 1, 21845, false),
            (21845, 1, 2, 0, true),
            (21845, 1, 2, 21845, false),
            (21845, 1, 3, 3333, true),
            (21845, 1, 3, 21845, false),
            (21845, 1, 4, 0, true),
            (21845, 1, 4, 21845, false),
            (21845, 1, 5, 0, true),
            (21845, 1, 5, 21845, false),
            (21845, 1, 6, 0, true),
            (21845, 1, 6, 21845, false),
            (21845, 1, 7, 7777, true),
            (21845, 1, 7, 21845, false),
            (21845, 0, 0, 21845, true),
            (21845, 0, 0, 21845, false),
            (21845, 0, 1, 21845, false),
            (21845, 0, 2, 21845, false),
            (21845, 0, 3, 21845, false),
            (21845, 0, 4, 21845, false),
            (21845, 0, 5, 21845, false),
            (21845, 0, 6, 21845, false),
            (21845, 0, 7, 21845, false),
            (-21846, 1, 0, 21845, true),
            (-21846, 1, 0, -21846, false),
            (-21846, 0, 0, -21846, true),
            (-21846, 0, 0, -21846, false),
            (-21846, 0, 1, 21845, false),
            (-21846, 0, 2, 21845, false),
            (-21846, 0, 3, 21845, false),
            (-21846, 0, 4, 21845, false),
            (-21846, 0, 5, 21845, false),
            (-21846, 0, 6, 21845, false),
            (-21846, 0, 7, 21845, false),
            (21845, 1, 0, -21846, true),
            (21845, 1, 0, 21845, false),
            (-21846, 1, 1, 21845, true),
            (-21846, 1, 1, -21846, false),
            (-21846, 0, 0, 21845, true),
            (-21846, 0, 0, 21845, false),
            (-21846, 0, 1, -21846, false),
            (-21846, 0, 2, 21845, false),
            (-21846, 0, 3, 21845, false),
            (-21846, 0, 4, 21845, false),
            (-21846, 0, 5, 21845, false),
            (-21846, 0, 6, 21845, false),
            (-21846, 0, 7, 21845, false),
            (21845, 1, 1, -21846, true),
            (21845, 1, 1, 21845, false),
            (-21846, 1, 2, 21845, true),
            (-21846, 1, 2, -21846, false),
            (-21846, 0, 0, 21845, true),
            (-21846, 0, 0, 21845, false),
            (-21846, 0, 1, 21845, false),
            (-21846, 0, 2, -21846, false),
            (-21846, 0, 3, 21845, false),
            (-21846, 0, 4, 21845, false),
            (-21846, 0, 5, 21845, false),
            (-21846, 0, 6, 21845, false),
            (-21846, 0, 7, 21845, false),
            (21845, 1, 2, -21846, true),
            (21845, 1, 2, 21845, false),
            (-21846, 1, 3, 21845, true),
            (-21846, 1, 3, -21846, false),
            (-21846, 0, 0, 21845, true),
            (-21846, 0, 0, 21845, false),
            (-21846, 0, 1, 21845, false),
            (-21846, 0, 2, 21845, false),
            (-21846, 0, 3, -21846, false),
            (-21846, 0, 4, 21845, false),
            (-21846, 0, 5, 21845, false),
            (-21846, 0, 6, 21845, false),
            (-21846, 0, 7, 21845, false),
            (21845, 1, 3, -21846, true),
            (21845, 1, 3, 21845, false),
            (-21846, 1, 4, 21845, true),
            (-21846, 1, 4, -21846, false),
            (-21846, 0, 0, 21845, true),
            (-21846, 0, 0, 21845, false),
            (-21846, 0, 1, 21845, false),
            (-21846, 0, 2, 21845, false),
            (-21846, 0, 3, 21845, false),
            (-21846, 0, 4, -21846, false),
            (-21846, 0, 5, 21845, false),
            (-21846, 0, 6, 21845, false),
            (-21846, 0, 7, 21845, false),
            (21845, 1, 4, -21846, true),
            (21845, 1, 4, 21845, false),
            (-21846, 1, 5, 21845, true),
            (-21846, 1, 5, -21846, false),
            (-21846, 0, 0, 21845, true),
            (-21846, 0, 0, 21845, false),
            (-21846, 0, 1, 21845, false),
            (-21846, 0, 2, 21845, false),
            (-21846, 0, 3, 21845, false),
            (-21846, 0, 4, 21845, false),
            (-21846, 0, 5, -21846, false),
            (-21846, 0, 6, 21845, false),
            (-21846, 0, 7, 21845, false),
            (21845, 1, 5, -21846, true),
            (21845, 1, 5, 21845, false),
            (-21846, 1, 6, 21845, true),
            (-21846, 1, 6, -21846, false),
            (-21846, 0, 0, 21845, true),
            (-21846, 0, 0, 21845, false),
            (-21846, 0, 1, 21845, false),
            (-21846, 0, 2, 21845, false),
            (-21846, 0, 3, 21845, false),
            (-21846, 0, 4, 21845, false),
            (-21846, 0, 5, 21845, false),
            (-21846, 0, 6, -21846, false),
            (-21846, 0, 7, 21845, false),
            (21845, 1, 6, -21846, true),
            (21845, 1, 6, 21845, false),
            (-21846, 1, 7, 21845, true),
            (-21846, 1, 7, -21846, false),
            (-21846, 0, 0, 21845, true),
            (-21846, 0, 0, 21845, false),
            (-21846, 0, 1, 21845, false),
            (-21846, 0, 2, 21845, false),
            (-21846, 0, 3, 21845, false),
            (-21846, 0, 4, 21845, false),
            (-21846, 0, 5, 21845, false),
            (-21846, 0, 6, 21845, false),
            (-21846, 0, 7, -21846, false),
            (21845, 1, 7, -21846, true),
            (21845, 1, 7, 21845, false),
            (21845, 0, 0, 21845, true),
            (21845, 0, 0, 21845, false),
            (21845, 0, 1, 21845, false),
            (21845, 0, 2, 21845, false),
            (21845, 0, 3, 21845, false),
            (21845, 0, 4, 21845, false),
            (21845, 0, 5, 21845, false),
            (21845, 0, 6, 21845, false),
            (21845, 0, 7, 21845, false),
        ];

        let input: SharedBus<16> = Bus::all0().to_shared_bus();
        let load: SharedBus<1> = Bus::all0().to_shared_bus();
        let address: SharedBus<3> = Bus::all0().to_shared_bus();
        let ram8 = RAM8::new(input.clone(), load.clone(), address.clone());

        for case in cases {
            let _input = i16_to_bus16(case.0);
            let _sel = i16_to_bus1(case.1);
            let _address = i16_to_bus3(case.2);

            let out = i16_to_bus16(case.3).to_shared_bus();

            // 中身を変える
            input.overwrite(&_input);
            load.overwrite(&_sel);
            address.overwrite(&_address);

            // tick
            if case.4 {
                ram8.re_compute();
                ram8.clock_up();
                assert_eq!(ram8.out.clone(), out.clone());
            } else {
                // tock
                ram8.clock_down();
                ram8.re_compute();
                assert_eq!(ram8.out.clone(), out.clone());
            }
        }
    }

    #[test]
    fn ram64() {
        // 順番に依存している
        let cases: Vec<(i16, i16, i16, i16, bool)> = vec![
            // in, load, address,  out, tick?
            (0, 0, 0, 0, true),
            (0, 0, 0, 0, false),
            (0, 1, 0, 0, true),
            (0, 1, 0, 0, false),
            (1313, 0, 0, 0, true),
            (1313, 0, 0, 0, false),
            (1313, 1, 13, 0, true),
            (1313, 1, 13, 1313, false),
            (1313, 0, 0, 0, true),
            (1313, 0, 0, 0, false),
            (4747, 0, 47, 0, true),
            (4747, 0, 47, 0, false),
            (4747, 1, 47, 0, true),
            (4747, 1, 47, 4747, false),
            (4747, 0, 47, 4747, true),
            (4747, 0, 47, 4747, false),
            (4747, 0, 13, 1313, false),
            (6363, 0, 13, 1313, true),
            (6363, 0, 13, 1313, false),
            (6363, 1, 63, 0, true),
            (6363, 1, 63, 6363, false),
            (6363, 0, 63, 6363, true),
            (6363, 0, 63, 6363, false),
            (6363, 0, 47, 4747, false),
            (6363, 0, 63, 6363, false),
            (6363, 0, 40, 0, true),
            (6363, 0, 40, 0, false),
            (6363, 0, 41, 0, false),
            (6363, 0, 42, 0, false),
            (6363, 0, 43, 0, false),
            (6363, 0, 44, 0, false),
            (6363, 0, 45, 0, false),
            (6363, 0, 46, 0, false),
            (6363, 0, 47, 4747, false),
            (21845, 1, 40, 0, true),
            (21845, 1, 40, 21845, false),
            (21845, 1, 41, 0, true),
            (21845, 1, 41, 21845, false),
            (21845, 1, 42, 0, true),
            (21845, 1, 42, 21845, false),
            (21845, 1, 43, 0, true),
            (21845, 1, 43, 21845, false),
            (21845, 1, 44, 0, true),
            (21845, 1, 44, 21845, false),
            (21845, 1, 45, 0, true),
            (21845, 1, 45, 21845, false),
            (21845, 1, 46, 0, true),
            (21845, 1, 46, 21845, false),
            (21845, 1, 47, 4747, true),
            (21845, 1, 47, 21845, false),
            (21845, 0, 40, 21845, true),
            (21845, 0, 40, 21845, false),
            (21845, 0, 41, 21845, false),
            (21845, 0, 42, 21845, false),
            (21845, 0, 43, 21845, false),
            (21845, 0, 44, 21845, false),
            (21845, 0, 45, 21845, false),
            (21845, 0, 46, 21845, false),
            (21845, 0, 47, 21845, false),
            (-21846, 1, 40, 21845, true),
            (-21846, 1, 40, -21846, false),
            (-21846, 0, 40, -21846, true),
            (-21846, 0, 40, -21846, false),
            (-21846, 0, 41, 21845, false),
            (-21846, 0, 42, 21845, false),
            (-21846, 0, 43, 21845, false),
            (-21846, 0, 44, 21845, false),
            (-21846, 0, 45, 21845, false),
            (-21846, 0, 46, 21845, false),
            (-21846, 0, 47, 21845, false),
            (21845, 1, 40, -21846, true),
            (21845, 1, 40, 21845, false),
            (-21846, 1, 41, 21845, true),
            (-21846, 1, 41, -21846, false),
            (-21846, 0, 40, 21845, true),
            (-21846, 0, 40, 21845, false),
            (-21846, 0, 41, -21846, false),
            (-21846, 0, 42, 21845, false),
            (-21846, 0, 43, 21845, false),
            (-21846, 0, 44, 21845, false),
            (-21846, 0, 45, 21845, false),
            (-21846, 0, 46, 21845, false),
            (-21846, 0, 47, 21845, false),
            (21845, 1, 41, -21846, true),
            (21845, 1, 41, 21845, false),
            (-21846, 1, 42, 21845, true),
            (-21846, 1, 42, -21846, false),
            (-21846, 0, 40, 21845, true),
            (-21846, 0, 40, 21845, false),
            (-21846, 0, 41, 21845, false),
            (-21846, 0, 42, -21846, false),
            (-21846, 0, 43, 21845, false),
            (-21846, 0, 44, 21845, false),
            (-21846, 0, 45, 21845, false),
            (-21846, 0, 46, 21845, false),
            (-21846, 0, 47, 21845, false),
            (21845, 1, 42, -21846, true),
            (21845, 1, 42, 21845, false),
            (-21846, 1, 43, 21845, true),
            (-21846, 1, 43, -21846, false),
            (-21846, 0, 40, 21845, true),
            (-21846, 0, 40, 21845, false),
            (-21846, 0, 41, 21845, false),
            (-21846, 0, 42, 21845, false),
            (-21846, 0, 43, -21846, false),
            (-21846, 0, 44, 21845, false),
            (-21846, 0, 45, 21845, false),
            (-21846, 0, 46, 21845, false),
            (-21846, 0, 47, 21845, false),
            (21845, 1, 43, -21846, true),
            (21845, 1, 43, 21845, false),
            (-21846, 1, 44, 21845, true),
            (-21846, 1, 44, -21846, false),
            (-21846, 0, 40, 21845, true),
            (-21846, 0, 40, 21845, false),
            (-21846, 0, 41, 21845, false),
            (-21846, 0, 42, 21845, false),
            (-21846, 0, 43, 21845, false),
            (-21846, 0, 44, -21846, false),
            (-21846, 0, 45, 21845, false),
            (-21846, 0, 46, 21845, false),
            (-21846, 0, 47, 21845, false),
            (21845, 1, 44, -21846, true),
            (21845, 1, 44, 21845, false),
            (-21846, 1, 45, 21845, true),
            (-21846, 1, 45, -21846, false),
            (-21846, 0, 40, 21845, true),
            (-21846, 0, 40, 21845, false),
            (-21846, 0, 41, 21845, false),
            (-21846, 0, 42, 21845, false),
            (-21846, 0, 43, 21845, false),
            (-21846, 0, 44, 21845, false),
            (-21846, 0, 45, -21846, false),
            (-21846, 0, 46, 21845, false),
            (-21846, 0, 47, 21845, false),
            (21845, 1, 45, -21846, true),
            (21845, 1, 45, 21845, false),
            (-21846, 1, 46, 21845, true),
            (-21846, 1, 46, -21846, false),
            (-21846, 0, 40, 21845, true),
            (-21846, 0, 40, 21845, false),
            (-21846, 0, 41, 21845, false),
            (-21846, 0, 42, 21845, false),
            (-21846, 0, 43, 21845, false),
            (-21846, 0, 44, 21845, false),
            (-21846, 0, 45, 21845, false),
            (-21846, 0, 46, -21846, false),
            (-21846, 0, 47, 21845, false),
            (21845, 1, 46, -21846, true),
            (21845, 1, 46, 21845, false),
            (-21846, 1, 47, 21845, true),
            (-21846, 1, 47, -21846, false),
            (-21846, 0, 40, 21845, true),
            (-21846, 0, 40, 21845, false),
            (-21846, 0, 41, 21845, false),
            (-21846, 0, 42, 21845, false),
            (-21846, 0, 43, 21845, false),
            (-21846, 0, 44, 21845, false),
            (-21846, 0, 45, 21845, false),
            (-21846, 0, 46, 21845, false),
            (-21846, 0, 47, -21846, false),
            (21845, 1, 47, -21846, true),
            (21845, 1, 47, 21845, false),
            (21845, 0, 40, 21845, true),
            (21845, 0, 40, 21845, false),
            (21845, 0, 41, 21845, false),
            (21845, 0, 42, 21845, false),
            (21845, 0, 43, 21845, false),
            (21845, 0, 44, 21845, false),
            (21845, 0, 45, 21845, false),
            (21845, 0, 46, 21845, false),
            (21845, 0, 47, 21845, false),
            (21845, 0, 5, 0, true),
            (21845, 0, 5, 0, false),
            (21845, 0, 13, 1313, false),
            (21845, 0, 21, 0, false),
            (21845, 0, 29, 0, false),
            (21845, 0, 37, 0, false),
            (21845, 0, 45, 21845, false),
            (21845, 0, 53, 0, false),
            (21845, 0, 61, 0, false),
            (21845, 1, 5, 0, true),
            (21845, 1, 5, 21845, false),
            (21845, 1, 13, 1313, true),
            (21845, 1, 13, 21845, false),
            (21845, 1, 21, 0, true),
            (21845, 1, 21, 21845, false),
            (21845, 1, 29, 0, true),
            (21845, 1, 29, 21845, false),
            (21845, 1, 37, 0, true),
            (21845, 1, 37, 21845, false),
            (21845, 1, 45, 21845, true),
            (21845, 1, 45, 21845, false),
            (21845, 1, 53, 0, true),
            (21845, 1, 53, 21845, false),
            (21845, 1, 61, 0, true),
            (21845, 1, 61, 21845, false),
            (21845, 0, 5, 21845, true),
            (21845, 0, 5, 21845, false),
            (21845, 0, 13, 21845, false),
            (21845, 0, 21, 21845, false),
            (21845, 0, 29, 21845, false),
            (21845, 0, 37, 21845, false),
            (21845, 0, 45, 21845, false),
            (21845, 0, 53, 21845, false),
            (21845, 0, 61, 21845, false),
            (-21846, 1, 5, 21845, true),
            (-21846, 1, 5, -21846, false),
            (-21846, 0, 5, -21846, true),
            (-21846, 0, 5, -21846, false),
            (-21846, 0, 13, 21845, false),
            (-21846, 0, 21, 21845, false),
            (-21846, 0, 29, 21845, false),
            (-21846, 0, 37, 21845, false),
            (-21846, 0, 45, 21845, false),
            (-21846, 0, 53, 21845, false),
            (-21846, 0, 61, 21845, false),
            (21845, 1, 5, -21846, true),
            (21845, 1, 5, 21845, false),
            (-21846, 1, 13, 21845, true),
            (-21846, 1, 13, -21846, false),
            (-21846, 0, 5, 21845, true),
            (-21846, 0, 5, 21845, false),
            (-21846, 0, 13, -21846, false),
            (-21846, 0, 21, 21845, false),
            (-21846, 0, 29, 21845, false),
            (-21846, 0, 37, 21845, false),
            (-21846, 0, 45, 21845, false),
            (-21846, 0, 53, 21845, false),
            (-21846, 0, 61, 21845, false),
            (21845, 1, 13, -21846, true),
            (21845, 1, 13, 21845, false),
            (-21846, 1, 21, 21845, true),
            (-21846, 1, 21, -21846, false),
            (-21846, 0, 5, 21845, true),
            (-21846, 0, 5, 21845, false),
            (-21846, 0, 13, 21845, false),
            (-21846, 0, 21, -21846, false),
            (-21846, 0, 29, 21845, false),
            (-21846, 0, 37, 21845, false),
            (-21846, 0, 45, 21845, false),
            (-21846, 0, 53, 21845, false),
            (-21846, 0, 61, 21845, false),
            (21845, 1, 21, -21846, true),
            (21845, 1, 21, 21845, false),
            (-21846, 1, 29, 21845, true),
            (-21846, 1, 29, -21846, false),
            (-21846, 0, 5, 21845, true),
            (-21846, 0, 5, 21845, false),
            (-21846, 0, 13, 21845, false),
            (-21846, 0, 21, 21845, false),
            (-21846, 0, 29, -21846, false),
            (-21846, 0, 37, 21845, false),
            (-21846, 0, 45, 21845, false),
            (-21846, 0, 53, 21845, false),
            (-21846, 0, 61, 21845, false),
            (21845, 1, 29, -21846, true),
            (21845, 1, 29, 21845, false),
            (-21846, 1, 37, 21845, true),
            (-21846, 1, 37, -21846, false),
            (-21846, 0, 5, 21845, true),
            (-21846, 0, 5, 21845, false),
            (-21846, 0, 13, 21845, false),
            (-21846, 0, 21, 21845, false),
            (-21846, 0, 29, 21845, false),
            (-21846, 0, 37, -21846, false),
            (-21846, 0, 45, 21845, false),
            (-21846, 0, 53, 21845, false),
            (-21846, 0, 61, 21845, false),
            (21845, 1, 37, -21846, true),
            (21845, 1, 37, 21845, false),
            (-21846, 1, 45, 21845, true),
            (-21846, 1, 45, -21846, false),
            (-21846, 0, 5, 21845, true),
            (-21846, 0, 5, 21845, false),
            (-21846, 0, 13, 21845, false),
            (-21846, 0, 21, 21845, false),
            (-21846, 0, 29, 21845, false),
            (-21846, 0, 37, 21845, false),
            (-21846, 0, 45, -21846, false),
            (-21846, 0, 53, 21845, false),
            (-21846, 0, 61, 21845, false),
            (21845, 1, 45, -21846, true),
            (21845, 1, 45, 21845, false),
            (-21846, 1, 53, 21845, true),
            (-21846, 1, 53, -21846, false),
            (-21846, 0, 5, 21845, true),
            (-21846, 0, 5, 21845, false),
            (-21846, 0, 13, 21845, false),
            (-21846, 0, 21, 21845, false),
            (-21846, 0, 29, 21845, false),
            (-21846, 0, 37, 21845, false),
            (-21846, 0, 45, 21845, false),
            (-21846, 0, 53, -21846, false),
            (-21846, 0, 61, 21845, false),
            (21845, 1, 53, -21846, true),
            (21845, 1, 53, 21845, false),
            (-21846, 1, 61, 21845, true),
            (-21846, 1, 61, -21846, false),
            (-21846, 0, 5, 21845, true),
            (-21846, 0, 5, 21845, false),
            (-21846, 0, 13, 21845, false),
            (-21846, 0, 21, 21845, false),
            (-21846, 0, 29, 21845, false),
            (-21846, 0, 37, 21845, false),
            (-21846, 0, 45, 21845, false),
            (-21846, 0, 53, 21845, false),
            (-21846, 0, 61, -21846, false),
            (21845, 1, 61, -21846, true),
            (21845, 1, 61, 21845, false),
            (21845, 0, 5, 21845, true),
            (21845, 0, 5, 21845, false),
            (21845, 0, 13, 21845, false),
            (21845, 0, 21, 21845, false),
            (21845, 0, 29, 21845, false),
            (21845, 0, 37, 21845, false),
            (21845, 0, 45, 21845, false),
            (21845, 0, 53, 21845, false),
            (21845, 0, 61, 21845, false),
        ];

        let input: SharedBus<16> = Bus::all0().to_shared_bus();
        let load: SharedBus<1> = Bus::all0().to_shared_bus();
        let address: SharedBus<6> = Bus::all0().to_shared_bus();
        let ram64 = RAM64::new(input.clone(), load.clone(), address.clone());

        for case in cases {
            println!(
                "{}, {}, {}, {}, {}",
                &case.0, &case.1, &case.2, &case.3, &case.4
            );
            let _input = i16_to_bus16(case.0);
            let _sel = i16_to_bus1(case.1);
            let _address = i16_to_bus6(case.2);

            let out = i16_to_bus16(case.3).to_shared_bus();

            // 中身を変える
            input.overwrite(&_input);
            load.overwrite(&_sel);
            address.overwrite(&_address);

            // tick
            if case.4 {
                ram64.re_compute();
                ram64.clock_up();

                assert_eq!(ram64.out.clone(), out.clone());
            } else {
                // tock
                ram64.clock_down();
                ram64.re_compute();
                assert_eq!(ram64.out.clone(), out.clone());
            }
        }
    }

    #[test]
    fn pc() {
        // 順番に依存している
        let cases: Vec<(i16, i16, i16, i16, i16, bool)> = vec![
            // in, load, inc,  reset, out, tick?
            (0, 0, 0, 0, 0, true),
            (0, 0, 0, 0, 0, false),
            (0, 0, 0, 1, 0, true),
            (0, 0, 0, 1, 1, false),
            (-32123, 0, 0, 1, 1, true),
            (-32123, 0, 0, 1, 2, false),
            (-32123, 0, 1, 1, 2, true),
            (-32123, 0, 1, 1, -32123, false),
            (-32123, 0, 0, 1, -32123, true),
            (-32123, 0, 0, 1, -32122, false),
            (-32123, 0, 0, 1, -32122, true),
            (-32123, 0, 0, 1, -32121, false),
            (12345, 0, 1, 0, -32121, true),
            (12345, 0, 1, 0, 12345, false),
            (12345, 1, 1, 0, 12345, true),
            (12345, 1, 1, 0, 0, false),
            (12345, 0, 1, 1, 0, true),
            (12345, 0, 1, 1, 12345, false),
            (12345, 1, 1, 1, 12345, true),
            (12345, 1, 1, 1, 0, false),
            (12345, 0, 0, 1, 0, true),
            (12345, 0, 0, 1, 1, false),
            (12345, 1, 0, 1, 1, true),
            (12345, 1, 0, 1, 0, false),
            (0, 0, 1, 1, 0, true),
            (0, 0, 1, 1, 0, false),
            (0, 0, 0, 1, 0, true),
            (0, 0, 0, 1, 1, false),
            (22222, 1, 0, 0, 1, true),
            (22222, 1, 0, 0, 0, false),
        ];

        let input: SharedBus<16> = Bus::all0().to_shared_bus();
        let load: SharedBus<1> = Bus::all0().to_shared_bus();
        let inc: SharedBus<1> = Bus::all0().to_shared_bus();
        let reset: SharedBus<1> = Bus::all0().to_shared_bus();
        let pc = PC::new(input.clone(), load.clone(), inc.clone(), reset.clone());

        for case in cases {
            println!(
                "{}, {}, {}, {}, {}, {}",
                &case.0, &case.1, &case.2, &case.3, &case.4, &case.5
            );
            let _input = i16_to_bus16(case.0);
            let _reset = i16_to_bus1(case.1);
            let _load = i16_to_bus1(case.2);
            let _inc = i16_to_bus1(case.3);

            let out = i16_to_bus16(case.4).to_shared_bus();

            // 中身を変える
            input.overwrite(&_input);
            load.overwrite(&_load);
            inc.overwrite(&_inc);
            reset.overwrite(&_reset);

            // tick
            if case.5 {
                pc.re_compute();
                pc.clock_up();
                assert_eq!(pc.out.clone(), out.clone());
            } else {
                // tock
                pc.clock_down();
                pc.re_compute();
                assert_eq!(pc.out.clone(), out.clone());
            }
        }
    }

    #[test]
    #[ignore]
    fn ram512() {
        // 順番に依存している
        let cases: Vec<(i16, i16, i16, i16, bool)> = vec![
            // in, load, address,  out, tick?
            (0, 0, 0, 0, true),
            (0, 0, 0, 0, false),
            (0, 1, 0, 0, true),
            (0, 1, 0, 0, false),
            (13099, 0, 0, 0, true),
            (13099, 0, 0, 0, false),
            (13099, 1, 130, 0, true),
            (13099, 1, 130, 13099, false),
            (13099, 0, 0, 0, true),
            (13099, 0, 0, 0, false),
            (4729, 0, 472, 0, true),
            (4729, 0, 472, 0, false),
            (4729, 1, 472, 0, true),
            (4729, 1, 472, 4729, false),
            (4729, 0, 472, 4729, true),
            (4729, 0, 472, 4729, false),
            (4729, 0, 130, 13099, false),
            (5119, 0, 130, 13099, true),
            (5119, 0, 130, 13099, false),
            (5119, 1, 511, 0, true),
            (5119, 1, 511, 5119, false),
            (5119, 0, 511, 5119, true),
            (5119, 0, 511, 5119, false),
            (5119, 0, 472, 4729, false),
            (5119, 0, 511, 5119, false),
            (5119, 0, 168, 0, true),
            (5119, 0, 168, 0, false),
            (5119, 0, 169, 0, false),
            (5119, 0, 170, 0, false),
            (5119, 0, 171, 0, false),
            (5119, 0, 172, 0, false),
            (5119, 0, 173, 0, false),
            (5119, 0, 174, 0, false),
            (5119, 0, 175, 0, false),
            (21845, 1, 168, 0, true),
            (21845, 1, 168, 21845, false),
            (21845, 1, 169, 0, true),
            (21845, 1, 169, 21845, false),
            (21845, 1, 170, 0, true),
            (21845, 1, 170, 21845, false),
            (21845, 1, 171, 0, true),
            (21845, 1, 171, 21845, false),
            (21845, 1, 172, 0, true),
            (21845, 1, 172, 21845, false),
            (21845, 1, 173, 0, true),
            (21845, 1, 173, 21845, false),
            (21845, 1, 174, 0, true),
            (21845, 1, 174, 21845, false),
            (21845, 1, 175, 0, true),
            (21845, 1, 175, 21845, false),
            (21845, 0, 168, 21845, true),
            (21845, 0, 168, 21845, false),
            (21845, 0, 169, 21845, false),
            (21845, 0, 170, 21845, false),
            (21845, 0, 171, 21845, false),
            (21845, 0, 172, 21845, false),
            (21845, 0, 173, 21845, false),
            (21845, 0, 174, 21845, false),
            (21845, 0, 175, 21845, false),
            (-21846, 1, 168, 21845, true),
            (-21846, 1, 168, -21846, false),
            (-21846, 0, 168, -21846, true),
            (-21846, 0, 168, -21846, false),
            (-21846, 0, 169, 21845, false),
            (-21846, 0, 170, 21845, false),
            (-21846, 0, 171, 21845, false),
            (-21846, 0, 172, 21845, false),
            (-21846, 0, 173, 21845, false),
            (-21846, 0, 174, 21845, false),
            (-21846, 0, 175, 21845, false),
            (21845, 1, 168, -21846, true),
            (21845, 1, 168, 21845, false),
            (-21846, 1, 169, 21845, true),
            (-21846, 1, 169, -21846, false),
            (-21846, 0, 168, 21845, true),
            (-21846, 0, 168, 21845, false),
            (-21846, 0, 169, -21846, false),
            (-21846, 0, 170, 21845, false),
            (-21846, 0, 171, 21845, false),
            (-21846, 0, 172, 21845, false),
            (-21846, 0, 173, 21845, false),
            (-21846, 0, 174, 21845, false),
            (-21846, 0, 175, 21845, false),
            (21845, 1, 169, -21846, true),
            (21845, 1, 169, 21845, false),
            (-21846, 1, 170, 21845, true),
            (-21846, 1, 170, -21846, false),
            (-21846, 0, 168, 21845, true),
            (-21846, 0, 168, 21845, false),
            (-21846, 0, 169, 21845, false),
            (-21846, 0, 170, -21846, false),
            (-21846, 0, 171, 21845, false),
            (-21846, 0, 172, 21845, false),
            (-21846, 0, 173, 21845, false),
            (-21846, 0, 174, 21845, false),
            (-21846, 0, 175, 21845, false),
            (21845, 1, 170, -21846, true),
            (21845, 1, 170, 21845, false),
            (-21846, 1, 171, 21845, true),
            (-21846, 1, 171, -21846, false),
            (-21846, 0, 168, 21845, true),
            (-21846, 0, 168, 21845, false),
            (-21846, 0, 169, 21845, false),
            (-21846, 0, 170, 21845, false),
            (-21846, 0, 171, -21846, false),
            (-21846, 0, 172, 21845, false),
            (-21846, 0, 173, 21845, false),
            (-21846, 0, 174, 21845, false),
            (-21846, 0, 175, 21845, false),
            (21845, 1, 171, -21846, true),
            (21845, 1, 171, 21845, false),
            (-21846, 1, 172, 21845, true),
            (-21846, 1, 172, -21846, false),
            (-21846, 0, 168, 21845, true),
            (-21846, 0, 168, 21845, false),
            (-21846, 0, 169, 21845, false),
            (-21846, 0, 170, 21845, false),
            (-21846, 0, 171, 21845, false),
            (-21846, 0, 172, -21846, false),
            (-21846, 0, 173, 21845, false),
            (-21846, 0, 174, 21845, false),
            (-21846, 0, 175, 21845, false),
            (21845, 1, 172, -21846, true),
            (21845, 1, 172, 21845, false),
            (-21846, 1, 173, 21845, true),
            (-21846, 1, 173, -21846, false),
            (-21846, 0, 168, 21845, true),
            (-21846, 0, 168, 21845, false),
            (-21846, 0, 169, 21845, false),
            (-21846, 0, 170, 21845, false),
            (-21846, 0, 171, 21845, false),
            (-21846, 0, 172, 21845, false),
            (-21846, 0, 173, -21846, false),
            (-21846, 0, 174, 21845, false),
            (-21846, 0, 175, 21845, false),
            (21845, 1, 173, -21846, true),
            (21845, 1, 173, 21845, false),
            (-21846, 1, 174, 21845, true),
            (-21846, 1, 174, -21846, false),
            (-21846, 0, 168, 21845, true),
            (-21846, 0, 168, 21845, false),
            (-21846, 0, 169, 21845, false),
            (-21846, 0, 170, 21845, false),
            (-21846, 0, 171, 21845, false),
            (-21846, 0, 172, 21845, false),
            (-21846, 0, 173, 21845, false),
            (-21846, 0, 174, -21846, false),
            (-21846, 0, 175, 21845, false),
            (21845, 1, 174, -21846, true),
            (21845, 1, 174, 21845, false),
            (-21846, 1, 175, 21845, true),
            (-21846, 1, 175, -21846, false),
            (-21846, 0, 168, 21845, true),
            (-21846, 0, 168, 21845, false),
            (-21846, 0, 169, 21845, false),
            (-21846, 0, 170, 21845, false),
            (-21846, 0, 171, 21845, false),
            (-21846, 0, 172, 21845, false),
            (-21846, 0, 173, 21845, false),
            (-21846, 0, 174, 21845, false),
            (-21846, 0, 175, -21846, false),
            (21845, 1, 175, -21846, true),
            (21845, 1, 175, 21845, false),
            (21845, 0, 168, 21845, true),
            (21845, 0, 168, 21845, false),
            (21845, 0, 169, 21845, false),
            (21845, 0, 170, 21845, false),
            (21845, 0, 171, 21845, false),
            (21845, 0, 172, 21845, false),
            (21845, 0, 173, 21845, false),
            (21845, 0, 174, 21845, false),
            (21845, 0, 175, 21845, false),
            (21845, 0, 42, 0, true),
            (21845, 0, 42, 0, false),
            (21845, 0, 106, 0, false),
            (21845, 0, 170, 21845, false),
            (21845, 0, 234, 0, false),
            (21845, 0, 298, 0, false),
            (21845, 0, 362, 0, false),
            (21845, 0, 426, 0, false),
            (21845, 0, 490, 0, false),
            (21845, 1, 42, 0, true),
            (21845, 1, 42, 21845, false),
            (21845, 1, 106, 0, true),
            (21845, 1, 106, 21845, false),
            (21845, 1, 170, 21845, true),
            (21845, 1, 170, 21845, false),
            (21845, 1, 234, 0, true),
            (21845, 1, 234, 21845, false),
            (21845, 1, 298, 0, true),
            (21845, 1, 298, 21845, false),
            (21845, 1, 362, 0, true),
            (21845, 1, 362, 21845, false),
            (21845, 1, 426, 0, true),
            (21845, 1, 426, 21845, false),
            (21845, 1, 490, 0, true),
            (21845, 1, 490, 21845, false),
            (21845, 0, 42, 21845, true),
            (21845, 0, 42, 21845, false),
            (21845, 0, 106, 21845, false),
            (21845, 0, 170, 21845, false),
            (21845, 0, 234, 21845, false),
            (21845, 0, 298, 21845, false),
            (21845, 0, 362, 21845, false),
            (21845, 0, 426, 21845, false),
            (21845, 0, 490, 21845, false),
            (-21846, 1, 42, 21845, true),
            (-21846, 1, 42, -21846, false),
            (-21846, 0, 42, -21846, true),
            (-21846, 0, 42, -21846, false),
            (-21846, 0, 106, 21845, false),
            (-21846, 0, 170, 21845, false),
            (-21846, 0, 234, 21845, false),
            (-21846, 0, 298, 21845, false),
            (-21846, 0, 362, 21845, false),
            (-21846, 0, 426, 21845, false),
            (-21846, 0, 490, 21845, false),
            (21845, 1, 42, -21846, true),
            (21845, 1, 42, 21845, false),
            (-21846, 1, 106, 21845, true),
            (-21846, 1, 106, -21846, false),
            (-21846, 0, 42, 21845, true),
            (-21846, 0, 42, 21845, false),
            (-21846, 0, 106, -21846, false),
            (-21846, 0, 170, 21845, false),
            (-21846, 0, 234, 21845, false),
            (-21846, 0, 298, 21845, false),
            (-21846, 0, 362, 21845, false),
            (-21846, 0, 426, 21845, false),
            (-21846, 0, 490, 21845, false),
            (21845, 1, 106, -21846, true),
            (21845, 1, 106, 21845, false),
            (-21846, 1, 170, 21845, true),
            (-21846, 1, 170, -21846, false),
            (-21846, 0, 42, 21845, true),
            (-21846, 0, 42, 21845, false),
            (-21846, 0, 106, 21845, false),
            (-21846, 0, 170, -21846, false),
            (-21846, 0, 234, 21845, false),
            (-21846, 0, 298, 21845, false),
            (-21846, 0, 362, 21845, false),
            (-21846, 0, 426, 21845, false),
            (-21846, 0, 490, 21845, false),
            (21845, 1, 170, -21846, true),
            (21845, 1, 170, 21845, false),
            (-21846, 1, 234, 21845, true),
            (-21846, 1, 234, -21846, false),
            (-21846, 0, 42, 21845, true),
            (-21846, 0, 42, 21845, false),
            (-21846, 0, 106, 21845, false),
            (-21846, 0, 170, 21845, false),
            (-21846, 0, 234, -21846, false),
            (-21846, 0, 298, 21845, false),
            (-21846, 0, 362, 21845, false),
            (-21846, 0, 426, 21845, false),
            (-21846, 0, 490, 21845, false),
            (21845, 1, 234, -21846, true),
            (21845, 1, 234, 21845, false),
            (-21846, 1, 298, 21845, true),
            (-21846, 1, 298, -21846, false),
            (-21846, 0, 42, 21845, true),
            (-21846, 0, 42, 21845, false),
            (-21846, 0, 106, 21845, false),
            (-21846, 0, 170, 21845, false),
            (-21846, 0, 234, 21845, false),
            (-21846, 0, 298, -21846, false),
            (-21846, 0, 362, 21845, false),
            (-21846, 0, 426, 21845, false),
            (-21846, 0, 490, 21845, false),
            (21845, 1, 298, -21846, true),
            (21845, 1, 298, 21845, false),
            (-21846, 1, 362, 21845, true),
            (-21846, 1, 362, -21846, false),
            (-21846, 0, 42, 21845, true),
            (-21846, 0, 42, 21845, false),
            (-21846, 0, 106, 21845, false),
            (-21846, 0, 170, 21845, false),
            (-21846, 0, 234, 21845, false),
            (-21846, 0, 298, 21845, false),
            (-21846, 0, 362, -21846, false),
            (-21846, 0, 426, 21845, false),
            (-21846, 0, 490, 21845, false),
            (21845, 1, 362, -21846, true),
            (21845, 1, 362, 21845, false),
            (-21846, 1, 426, 21845, true),
            (-21846, 1, 426, -21846, false),
            (-21846, 0, 42, 21845, true),
            (-21846, 0, 42, 21845, false),
            (-21846, 0, 106, 21845, false),
            (-21846, 0, 170, 21845, false),
            (-21846, 0, 234, 21845, false),
            (-21846, 0, 298, 21845, false),
            (-21846, 0, 362, 21845, false),
            (-21846, 0, 426, -21846, false),
            (-21846, 0, 490, 21845, false),
            (21845, 1, 426, -21846, true),
            (21845, 1, 426, 21845, false),
            (-21846, 1, 490, 21845, true),
            (-21846, 1, 490, -21846, false),
            (-21846, 0, 42, 21845, true),
            (-21846, 0, 42, 21845, false),
            (-21846, 0, 106, 21845, false),
            (-21846, 0, 170, 21845, false),
            (-21846, 0, 234, 21845, false),
            (-21846, 0, 298, 21845, false),
            (-21846, 0, 362, 21845, false),
            (-21846, 0, 426, 21845, false),
            (-21846, 0, 490, -21846, false),
            (21845, 1, 490, -21846, true),
            (21845, 1, 490, 21845, false),
            (21845, 0, 42, 21845, true),
            (21845, 0, 42, 21845, false),
            (21845, 0, 106, 21845, false),
            (21845, 0, 170, 21845, false),
            (21845, 0, 234, 21845, false),
            (21845, 0, 298, 21845, false),
            (21845, 0, 362, 21845, false),
            (21845, 0, 426, 21845, false),
            (21845, 0, 490, 21845, false),
        ];

        let input: SharedBus<16> = Bus::all0().to_shared_bus();
        let load: SharedBus<1> = Bus::all0().to_shared_bus();
        let address: SharedBus<9> = Bus::all0().to_shared_bus();
        let ram512 = RAM512::new(input.clone(), load.clone(), address.clone());

        for case in cases {
            println!(
                "{}, {}, {}, {}, {}",
                &case.0, &case.1, &case.2, &case.3, &case.4
            );
            let _input = i16_to_bus16(case.0);
            let _sel = i16_to_bus1(case.1);
            let _address = i16_to_bus9(case.2);

            let out = i16_to_bus16(case.3).to_shared_bus();

            // 中身を変える
            input.overwrite(&_input);
            load.overwrite(&_sel);
            address.overwrite(&_address);

            // tick
            if case.4 {
                ram512.re_compute();
                ram512.clock_up();
                assert_eq!(ram512.out.clone(), out.clone());
            } else {
                // tock
                ram512.clock_down();
                ram512.re_compute();
                assert_eq!(ram512.out.clone(), out.clone());
            }
        }
    }

    #[test]
    #[ignore]
    fn ram4k() {
        // 順番に依存している
        let cases: Vec<(i16, i16, i16, i16, bool)> = vec![
            // in, load, address,  out, tick?
            (0, 0, 0, 0, true),
            (0, 0, 0, 0, false),
            (0, 1, 0, 0, true),
            (0, 1, 0, 0, false),
            (1111, 0, 0, 0, true),
            (1111, 0, 0, 0, false),
            (1111, 1, 1111, 0, true),
            (1111, 1, 1111, 1111, false),
            (1111, 0, 0, 0, true),
            (1111, 0, 0, 0, false),
            (3513, 0, 3513, 0, true),
            (3513, 0, 3513, 0, false),
            (3513, 1, 3513, 0, true),
            (3513, 1, 3513, 3513, false),
            (3513, 0, 3513, 3513, true),
            (3513, 0, 3513, 3513, false),
            (3513, 0, 1111, 1111, false),
            (4095, 0, 1111, 1111, true),
            (4095, 0, 1111, 1111, false),
            (4095, 1, 4095, 0, true),
            (4095, 1, 4095, 4095, false),
            (4095, 0, 4095, 4095, true),
            (4095, 0, 4095, 4095, false),
            (4095, 0, 3513, 3513, false),
            (4095, 0, 4095, 4095, false),
            (4095, 0, 2728, 0, true),
            (4095, 0, 2728, 0, false),
            (4095, 0, 2729, 0, false),
            (4095, 0, 2730, 0, false),
            (4095, 0, 2731, 0, false),
            (4095, 0, 2732, 0, false),
            (4095, 0, 2733, 0, false),
            (4095, 0, 2734, 0, false),
            (4095, 0, 2735, 0, false),
            (21845, 1, 2728, 0, true),
            (21845, 1, 2728, 21845, false),
            (21845, 1, 2729, 0, true),
            (21845, 1, 2729, 21845, false),
            (21845, 1, 2730, 0, true),
            (21845, 1, 2730, 21845, false),
            (21845, 1, 2731, 0, true),
            (21845, 1, 2731, 21845, false),
            (21845, 1, 2732, 0, true),
            (21845, 1, 2732, 21845, false),
            (21845, 1, 2733, 0, true),
            (21845, 1, 2733, 21845, false),
            (21845, 1, 2734, 0, true),
            (21845, 1, 2734, 21845, false),
            (21845, 1, 2735, 0, true),
            (21845, 1, 2735, 21845, false),
            (21845, 0, 2728, 21845, true),
            (21845, 0, 2728, 21845, false),
            (21845, 0, 2729, 21845, false),
            (21845, 0, 2730, 21845, false),
            (21845, 0, 2731, 21845, false),
            (21845, 0, 2732, 21845, false),
            (21845, 0, 2733, 21845, false),
            (21845, 0, 2734, 21845, false),
            (21845, 0, 2735, 21845, false),
            (-21846, 1, 2728, 21845, true),
            (-21846, 1, 2728, -21846, false),
            (-21846, 0, 2728, -21846, true),
            (-21846, 0, 2728, -21846, false),
            (-21846, 0, 2729, 21845, false),
            (-21846, 0, 2730, 21845, false),
            (-21846, 0, 2731, 21845, false),
            (-21846, 0, 2732, 21845, false),
            (-21846, 0, 2733, 21845, false),
            (-21846, 0, 2734, 21845, false),
            (-21846, 0, 2735, 21845, false),
            (21845, 1, 2728, -21846, true),
            (21845, 1, 2728, 21845, false),
            (-21846, 1, 2729, 21845, true),
            (-21846, 1, 2729, -21846, false),
            (-21846, 0, 2728, 21845, true),
            (-21846, 0, 2728, 21845, false),
            (-21846, 0, 2729, -21846, false),
            (-21846, 0, 2730, 21845, false),
            (-21846, 0, 2731, 21845, false),
            (-21846, 0, 2732, 21845, false),
            (-21846, 0, 2733, 21845, false),
            (-21846, 0, 2734, 21845, false),
            (-21846, 0, 2735, 21845, false),
            (21845, 1, 2729, -21846, true),
            (21845, 1, 2729, 21845, false),
            (-21846, 1, 2730, 21845, true),
            (-21846, 1, 2730, -21846, false),
            (-21846, 0, 2728, 21845, true),
            (-21846, 0, 2728, 21845, false),
            (-21846, 0, 2729, 21845, false),
            (-21846, 0, 2730, -21846, false),
            (-21846, 0, 2731, 21845, false),
            (-21846, 0, 2732, 21845, false),
            (-21846, 0, 2733, 21845, false),
            (-21846, 0, 2734, 21845, false),
            (-21846, 0, 2735, 21845, false),
            (21845, 1, 2730, -21846, true),
            (21845, 1, 2730, 21845, false),
            (-21846, 1, 2731, 21845, true),
            (-21846, 1, 2731, -21846, false),
            (-21846, 0, 2728, 21845, true),
            (-21846, 0, 2728, 21845, false),
            (-21846, 0, 2729, 21845, false),
            (-21846, 0, 2730, 21845, false),
            (-21846, 0, 2731, -21846, false),
            (-21846, 0, 2732, 21845, false),
            (-21846, 0, 2733, 21845, false),
            (-21846, 0, 2734, 21845, false),
            (-21846, 0, 2735, 21845, false),
            (21845, 1, 2731, -21846, true),
            (21845, 1, 2731, 21845, false),
            (-21846, 1, 2732, 21845, true),
            (-21846, 1, 2732, -21846, false),
            (-21846, 0, 2728, 21845, true),
            (-21846, 0, 2728, 21845, false),
            (-21846, 0, 2729, 21845, false),
            (-21846, 0, 2730, 21845, false),
            (-21846, 0, 2731, 21845, false),
            (-21846, 0, 2732, -21846, false),
            (-21846, 0, 2733, 21845, false),
            (-21846, 0, 2734, 21845, false),
            (-21846, 0, 2735, 21845, false),
            (21845, 1, 2732, -21846, true),
            (21845, 1, 2732, 21845, false),
            (-21846, 1, 2733, 21845, true),
            (-21846, 1, 2733, -21846, false),
            (-21846, 0, 2728, 21845, true),
            (-21846, 0, 2728, 21845, false),
            (-21846, 0, 2729, 21845, false),
            (-21846, 0, 2730, 21845, false),
            (-21846, 0, 2731, 21845, false),
            (-21846, 0, 2732, 21845, false),
            (-21846, 0, 2733, -21846, false),
            (-21846, 0, 2734, 21845, false),
            (-21846, 0, 2735, 21845, false),
            (21845, 1, 2733, -21846, true),
            (21845, 1, 2733, 21845, false),
            (-21846, 1, 2734, 21845, true),
            (-21846, 1, 2734, -21846, false),
            (-21846, 0, 2728, 21845, true),
            (-21846, 0, 2728, 21845, false),
            (-21846, 0, 2729, 21845, false),
            (-21846, 0, 2730, 21845, false),
            (-21846, 0, 2731, 21845, false),
            (-21846, 0, 2732, 21845, false),
            (-21846, 0, 2733, 21845, false),
            (-21846, 0, 2734, -21846, false),
            (-21846, 0, 2735, 21845, false),
            (21845, 1, 2734, -21846, true),
            (21845, 1, 2734, 21845, false),
            (-21846, 1, 2735, 21845, true),
            (-21846, 1, 2735, -21846, false),
            (-21846, 0, 2728, 21845, true),
            (-21846, 0, 2728, 21845, false),
            (-21846, 0, 2729, 21845, false),
            (-21846, 0, 2730, 21845, false),
            (-21846, 0, 2731, 21845, false),
            (-21846, 0, 2732, 21845, false),
            (-21846, 0, 2733, 21845, false),
            (-21846, 0, 2734, 21845, false),
            (-21846, 0, 2735, -21846, false),
            (21845, 1, 2735, -21846, true),
            (21845, 1, 2735, 21845, false),
            (21845, 0, 2728, 21845, true),
            (21845, 0, 2728, 21845, false),
            (21845, 0, 2729, 21845, false),
            (21845, 0, 2730, 21845, false),
            (21845, 0, 2731, 21845, false),
            (21845, 0, 2732, 21845, false),
            (21845, 0, 2733, 21845, false),
            (21845, 0, 2734, 21845, false),
            (21845, 0, 2735, 21845, false),
            (21845, 0, 341, 0, true),
            (21845, 0, 341, 0, false),
            (21845, 0, 853, 0, false),
            (21845, 0, 1365, 0, false),
            (21845, 0, 1877, 0, false),
            (21845, 0, 2389, 0, false),
            (21845, 0, 2901, 0, false),
            (21845, 0, 3413, 0, false),
            (21845, 0, 3925, 0, false),
            (21845, 1, 341, 0, true),
            (21845, 1, 341, 21845, false),
            (21845, 1, 853, 0, true),
            (21845, 1, 853, 21845, false),
            (21845, 1, 1365, 0, true),
            (21845, 1, 1365, 21845, false),
            (21845, 1, 1877, 0, true),
            (21845, 1, 1877, 21845, false),
            (21845, 1, 2389, 0, true),
            (21845, 1, 2389, 21845, false),
            (21845, 1, 2901, 0, true),
            (21845, 1, 2901, 21845, false),
            (21845, 1, 3413, 0, true),
            (21845, 1, 3413, 21845, false),
            (21845, 1, 3925, 0, true),
            (21845, 1, 3925, 21845, false),
            (21845, 0, 341, 21845, true),
            (21845, 0, 341, 21845, false),
            (21845, 0, 853, 21845, false),
            (21845, 0, 1365, 21845, false),
            (21845, 0, 1877, 21845, false),
            (21845, 0, 2389, 21845, false),
            (21845, 0, 2901, 21845, false),
            (21845, 0, 3413, 21845, false),
            (21845, 0, 3925, 21845, false),
            (-21846, 1, 341, 21845, true),
            (-21846, 1, 341, -21846, false),
            (-21846, 0, 341, -21846, true),
            (-21846, 0, 341, -21846, false),
            (-21846, 0, 853, 21845, false),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 1877, 21845, false),
            (-21846, 0, 2389, 21845, false),
            (-21846, 0, 2901, 21845, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 3925, 21845, false),
            (21845, 1, 341, -21846, true),
            (21845, 1, 341, 21845, false),
            (-21846, 1, 853, 21845, true),
            (-21846, 1, 853, -21846, false),
            (-21846, 0, 341, 21845, true),
            (-21846, 0, 341, 21845, false),
            (-21846, 0, 853, -21846, false),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 1877, 21845, false),
            (-21846, 0, 2389, 21845, false),
            (-21846, 0, 2901, 21845, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 3925, 21845, false),
            (21845, 1, 853, -21846, true),
            (21845, 1, 853, 21845, false),
            (-21846, 1, 1365, 21845, true),
            (-21846, 1, 1365, -21846, false),
            (-21846, 0, 341, 21845, true),
            (-21846, 0, 341, 21845, false),
            (-21846, 0, 853, 21845, false),
            (-21846, 0, 1365, -21846, false),
            (-21846, 0, 1877, 21845, false),
            (-21846, 0, 2389, 21845, false),
            (-21846, 0, 2901, 21845, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 3925, 21845, false),
            (21845, 1, 1365, -21846, true),
            (21845, 1, 1365, 21845, false),
            (-21846, 1, 1877, 21845, true),
            (-21846, 1, 1877, -21846, false),
            (-21846, 0, 341, 21845, true),
            (-21846, 0, 341, 21845, false),
            (-21846, 0, 853, 21845, false),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 1877, -21846, false),
            (-21846, 0, 2389, 21845, false),
            (-21846, 0, 2901, 21845, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 3925, 21845, false),
            (21845, 1, 1877, -21846, true),
            (21845, 1, 1877, 21845, false),
            (-21846, 1, 2389, 21845, true),
            (-21846, 1, 2389, -21846, false),
            (-21846, 0, 341, 21845, true),
            (-21846, 0, 341, 21845, false),
            (-21846, 0, 853, 21845, false),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 1877, 21845, false),
            (-21846, 0, 2389, -21846, false),
            (-21846, 0, 2901, 21845, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 3925, 21845, false),
            (21845, 1, 2389, -21846, true),
            (21845, 1, 2389, 21845, false),
            (-21846, 1, 2901, 21845, true),
            (-21846, 1, 2901, -21846, false),
            (-21846, 0, 341, 21845, true),
            (-21846, 0, 341, 21845, false),
            (-21846, 0, 853, 21845, false),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 1877, 21845, false),
            (-21846, 0, 2389, 21845, false),
            (-21846, 0, 2901, -21846, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 3925, 21845, false),
            (21845, 1, 2901, -21846, true),
            (21845, 1, 2901, 21845, false),
            (-21846, 1, 3413, 21845, true),
            (-21846, 1, 3413, -21846, false),
            (-21846, 0, 341, 21845, true),
            (-21846, 0, 341, 21845, false),
            (-21846, 0, 853, 21845, false),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 1877, 21845, false),
            (-21846, 0, 2389, 21845, false),
            (-21846, 0, 2901, 21845, false),
            (-21846, 0, 3413, -21846, false),
            (-21846, 0, 3925, 21845, false),
            (21845, 1, 3413, -21846, true),
            (21845, 1, 3413, 21845, false),
            (-21846, 1, 3925, 21845, true),
            (-21846, 1, 3925, -21846, false),
            (-21846, 0, 341, 21845, true),
            (-21846, 0, 341, 21845, false),
            (-21846, 0, 853, 21845, false),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 1877, 21845, false),
            (-21846, 0, 2389, 21845, false),
            (-21846, 0, 2901, 21845, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 3925, -21846, false),
            (21845, 1, 3925, -21846, true),
            (21845, 1, 3925, 21845, false),
            (21845, 0, 341, 21845, true),
            (21845, 0, 341, 21845, false),
            (21845, 0, 853, 21845, false),
            (21845, 0, 1365, 21845, false),
            (21845, 0, 1877, 21845, false),
            (21845, 0, 2389, 21845, false),
            (21845, 0, 2901, 21845, false),
            (21845, 0, 3413, 21845, false),
            (21845, 0, 3925, 21845, false),
        ];

        let input: SharedBus<16> = Bus::all0().to_shared_bus();
        let load: SharedBus<1> = Bus::all0().to_shared_bus();
        let address: SharedBus<12> = Bus::all0().to_shared_bus();
        let ram4k = RAM4K::new(input.clone(), load.clone(), address.clone());

        for case in cases {
            println!(
                "{}, {}, {}, {}, {}",
                &case.0, &case.1, &case.2, &case.3, &case.4
            );
            let _input = i16_to_bus16(case.0);
            let _sel = i16_to_bus1(case.1);
            let _address = i16_to_bus12(case.2);

            let out = i16_to_bus16(case.3).to_shared_bus();

            // 中身を変える
            input.overwrite(&_input);
            load.overwrite(&_sel);
            address.overwrite(&_address);

            // tick
            if case.4 {
                ram4k.re_compute();
                ram4k.clock_up();
                assert_eq!(ram4k.out.clone(), out.clone());
            } else {
                // tock
                ram4k.clock_down();
                ram4k.re_compute();
                assert_eq!(ram4k.out.clone(), out.clone());
            }
        }
    }

    #[test]
    #[ignore]
    fn ram16k() {
        // 順番に依存している
        let cases: Vec<(i16, i16, i16, i16, bool)> = vec![
            (0, 0, 0, 0, true),
            (0, 0, 0, 0, false),
            (0, 1, 0, 0, true),
            (0, 1, 0, 0, false),
            (4321, 0, 0, 0, true),
            (4321, 0, 0, 0, false),
            (4321, 1, 4321, 0, true),
            (4321, 1, 4321, 4321, false),
            (4321, 0, 0, 0, true),
            (4321, 0, 0, 0, false),
            (12345, 0, 12345, 0, true),
            (12345, 0, 12345, 0, false),
            (12345, 1, 12345, 0, true),
            (12345, 1, 12345, 12345, false),
            (12345, 0, 12345, 12345, true),
            (12345, 0, 12345, 12345, false),
            (12345, 0, 4321, 4321, false),
            (16383, 0, 4321, 4321, true),
            (16383, 0, 4321, 4321, false),
            (16383, 1, 16383, 0, true),
            (16383, 1, 16383, 16383, false),
            (16383, 0, 16383, 16383, true),
            (16383, 0, 16383, 16383, false),
            (16383, 0, 12345, 12345, false),
            (16383, 0, 16383, 16383, false),
            (16383, 0, 10920, 0, true),
            (16383, 0, 10920, 0, false),
            (16383, 0, 10921, 0, false),
            (16383, 0, 10922, 0, false),
            (16383, 0, 10923, 0, false),
            (16383, 0, 10924, 0, false),
            (16383, 0, 10925, 0, false),
            (16383, 0, 10926, 0, false),
            (16383, 0, 10927, 0, false),
            (21845, 1, 10920, 0, true),
            (21845, 1, 10920, 21845, false),
            (21845, 1, 10921, 0, true),
            (21845, 1, 10921, 21845, false),
            (21845, 1, 10922, 0, true),
            (21845, 1, 10922, 21845, false),
            (21845, 1, 10923, 0, true),
            (21845, 1, 10923, 21845, false),
            (21845, 1, 10924, 0, true),
            (21845, 1, 10924, 21845, false),
            (21845, 1, 10925, 0, true),
            (21845, 1, 10925, 21845, false),
            (21845, 1, 10926, 0, true),
            (21845, 1, 10926, 21845, false),
            (21845, 1, 10927, 0, true),
            (21845, 1, 10927, 21845, false),
            (21845, 0, 10920, 21845, true),
            (21845, 0, 10920, 21845, false),
            (21845, 0, 10921, 21845, false),
            (21845, 0, 10922, 21845, false),
            (21845, 0, 10923, 21845, false),
            (21845, 0, 10924, 21845, false),
            (21845, 0, 10925, 21845, false),
            (21845, 0, 10926, 21845, false),
            (21845, 0, 10927, 21845, false),
            (-21846, 1, 10920, 21845, true),
            (-21846, 1, 10920, -21846, false),
            (-21846, 0, 10920, -21846, true),
            (-21846, 0, 10920, -21846, false),
            (-21846, 0, 10921, 21845, false),
            (-21846, 0, 10922, 21845, false),
            (-21846, 0, 10923, 21845, false),
            (-21846, 0, 10924, 21845, false),
            (-21846, 0, 10925, 21845, false),
            (-21846, 0, 10926, 21845, false),
            (-21846, 0, 10927, 21845, false),
            (21845, 1, 10920, -21846, true),
            (21845, 1, 10920, 21845, false),
            (-21846, 1, 10921, 21845, true),
            (-21846, 1, 10921, -21846, false),
            (-21846, 0, 10920, 21845, true),
            (-21846, 0, 10920, 21845, false),
            (-21846, 0, 10921, -21846, false),
            (-21846, 0, 10922, 21845, false),
            (-21846, 0, 10923, 21845, false),
            (-21846, 0, 10924, 21845, false),
            (-21846, 0, 10925, 21845, false),
            (-21846, 0, 10926, 21845, false),
            (-21846, 0, 10927, 21845, false),
            (21845, 1, 10921, -21846, true),
            (21845, 1, 10921, 21845, false),
            (-21846, 1, 10922, 21845, true),
            (-21846, 1, 10922, -21846, false),
            (-21846, 0, 10920, 21845, true),
            (-21846, 0, 10920, 21845, false),
            (-21846, 0, 10921, 21845, false),
            (-21846, 0, 10922, -21846, false),
            (-21846, 0, 10923, 21845, false),
            (-21846, 0, 10924, 21845, false),
            (-21846, 0, 10925, 21845, false),
            (-21846, 0, 10926, 21845, false),
            (-21846, 0, 10927, 21845, false),
            (21845, 1, 10922, -21846, true),
            (21845, 1, 10922, 21845, false),
            (-21846, 1, 10923, 21845, true),
            (-21846, 1, 10923, -21846, false),
            (-21846, 0, 10920, 21845, true),
            (-21846, 0, 10920, 21845, false),
            (-21846, 0, 10921, 21845, false),
            (-21846, 0, 10922, 21845, false),
            (-21846, 0, 10923, -21846, false),
            (-21846, 0, 10924, 21845, false),
            (-21846, 0, 10925, 21845, false),
            (-21846, 0, 10926, 21845, false),
            (-21846, 0, 10927, 21845, false),
            (21845, 1, 10923, -21846, true),
            (21845, 1, 10923, 21845, false),
            (-21846, 1, 10924, 21845, true),
            (-21846, 1, 10924, -21846, false),
            (-21846, 0, 10920, 21845, true),
            (-21846, 0, 10920, 21845, false),
            (-21846, 0, 10921, 21845, false),
            (-21846, 0, 10922, 21845, false),
            (-21846, 0, 10923, 21845, false),
            (-21846, 0, 10924, -21846, false),
            (-21846, 0, 10925, 21845, false),
            (-21846, 0, 10926, 21845, false),
            (-21846, 0, 10927, 21845, false),
            (21845, 1, 10924, -21846, true),
            (21845, 1, 10924, 21845, false),
            (-21846, 1, 10925, 21845, true),
            (-21846, 1, 10925, -21846, false),
            (-21846, 0, 10920, 21845, true),
            (-21846, 0, 10920, 21845, false),
            (-21846, 0, 10921, 21845, false),
            (-21846, 0, 10922, 21845, false),
            (-21846, 0, 10923, 21845, false),
            (-21846, 0, 10924, 21845, false),
            (-21846, 0, 10925, -21846, false),
            (-21846, 0, 10926, 21845, false),
            (-21846, 0, 10927, 21845, false),
            (21845, 1, 10925, -21846, true),
            (21845, 1, 10925, 21845, false),
            (-21846, 1, 10926, 21845, true),
            (-21846, 1, 10926, -21846, false),
            (-21846, 0, 10920, 21845, true),
            (-21846, 0, 10920, 21845, false),
            (-21846, 0, 10921, 21845, false),
            (-21846, 0, 10922, 21845, false),
            (-21846, 0, 10923, 21845, false),
            (-21846, 0, 10924, 21845, false),
            (-21846, 0, 10925, 21845, false),
            (-21846, 0, 10926, -21846, false),
            (-21846, 0, 10927, 21845, false),
            (21845, 1, 10926, -21846, true),
            (21845, 1, 10926, 21845, false),
            (-21846, 1, 10927, 21845, true),
            (-21846, 1, 10927, -21846, false),
            (-21846, 0, 10920, 21845, true),
            (-21846, 0, 10920, 21845, false),
            (-21846, 0, 10921, 21845, false),
            (-21846, 0, 10922, 21845, false),
            (-21846, 0, 10923, 21845, false),
            (-21846, 0, 10924, 21845, false),
            (-21846, 0, 10925, 21845, false),
            (-21846, 0, 10926, 21845, false),
            (-21846, 0, 10927, -21846, false),
            (21845, 1, 10927, -21846, true),
            (21845, 1, 10927, 21845, false),
            (21845, 0, 10920, 21845, true),
            (21845, 0, 10920, 21845, false),
            (21845, 0, 10921, 21845, false),
            (21845, 0, 10922, 21845, false),
            (21845, 0, 10923, 21845, false),
            (21845, 0, 10924, 21845, false),
            (21845, 0, 10925, 21845, false),
            (21845, 0, 10926, 21845, false),
            (21845, 0, 10927, 21845, false),
            (21845, 0, 1365, 0, true),
            (21845, 0, 1365, 0, false),
            (21845, 0, 3413, 0, false),
            (21845, 0, 5461, 0, false),
            (21845, 0, 7509, 0, false),
            (21845, 0, 9557, 0, false),
            (21845, 0, 11605, 0, false),
            (21845, 0, 13653, 0, false),
            (21845, 0, 15701, 0, false),
            (21845, 1, 1365, 0, true),
            (21845, 1, 1365, 21845, false),
            (21845, 1, 3413, 0, true),
            (21845, 1, 3413, 21845, false),
            (21845, 1, 5461, 0, true),
            (21845, 1, 5461, 21845, false),
            (21845, 1, 7509, 0, true),
            (21845, 1, 7509, 21845, false),
            (21845, 1, 9557, 0, true),
            (21845, 1, 9557, 21845, false),
            (21845, 1, 11605, 0, true),
            (21845, 1, 11605, 21845, false),
            (21845, 1, 13653, 0, true),
            (21845, 1, 13653, 21845, false),
            (21845, 1, 15701, 0, true),
            (21845, 1, 15701, 21845, false),
            (21845, 0, 1365, 21845, true),
            (21845, 0, 1365, 21845, false),
            (21845, 0, 3413, 21845, false),
            (21845, 0, 5461, 21845, false),
            (21845, 0, 7509, 21845, false),
            (21845, 0, 9557, 21845, false),
            (21845, 0, 11605, 21845, false),
            (21845, 0, 13653, 21845, false),
            (21845, 0, 15701, 21845, false),
            (-21846, 1, 1365, 21845, true),
            (-21846, 1, 1365, -21846, false),
            (-21846, 0, 1365, -21846, true),
            (-21846, 0, 1365, -21846, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 5461, 21845, false),
            (-21846, 0, 7509, 21845, false),
            (-21846, 0, 9557, 21845, false),
            (-21846, 0, 11605, 21845, false),
            (-21846, 0, 13653, 21845, false),
            (-21846, 0, 15701, 21845, false),
            (21845, 1, 1365, -21846, true),
            (21845, 1, 1365, 21845, false),
            (-21846, 1, 3413, 21845, true),
            (-21846, 1, 3413, -21846, false),
            (-21846, 0, 1365, 21845, true),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 3413, -21846, false),
            (-21846, 0, 5461, 21845, false),
            (-21846, 0, 7509, 21845, false),
            (-21846, 0, 9557, 21845, false),
            (-21846, 0, 11605, 21845, false),
            (-21846, 0, 13653, 21845, false),
            (-21846, 0, 15701, 21845, false),
            (21845, 1, 3413, -21846, true),
            (21845, 1, 3413, 21845, false),
            (-21846, 1, 5461, 21845, true),
            (-21846, 1, 5461, -21846, false),
            (-21846, 0, 1365, 21845, true),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 5461, -21846, false),
            (-21846, 0, 7509, 21845, false),
            (-21846, 0, 9557, 21845, false),
            (-21846, 0, 11605, 21845, false),
            (-21846, 0, 13653, 21845, false),
            (-21846, 0, 15701, 21845, false),
            (21845, 1, 5461, -21846, true),
            (21845, 1, 5461, 21845, false),
            (-21846, 1, 7509, 21845, true),
            (-21846, 1, 7509, -21846, false),
            (-21846, 0, 1365, 21845, true),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 5461, 21845, false),
            (-21846, 0, 7509, -21846, false),
            (-21846, 0, 9557, 21845, false),
            (-21846, 0, 11605, 21845, false),
            (-21846, 0, 13653, 21845, false),
            (-21846, 0, 15701, 21845, false),
            (21845, 1, 7509, -21846, true),
            (21845, 1, 7509, 21845, false),
            (-21846, 1, 9557, 21845, true),
            (-21846, 1, 9557, -21846, false),
            (-21846, 0, 1365, 21845, true),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 5461, 21845, false),
            (-21846, 0, 7509, 21845, false),
            (-21846, 0, 9557, -21846, false),
            (-21846, 0, 11605, 21845, false),
            (-21846, 0, 13653, 21845, false),
            (-21846, 0, 15701, 21845, false),
            (21845, 1, 9557, -21846, true),
            (21845, 1, 9557, 21845, false),
            (-21846, 1, 11605, 21845, true),
            (-21846, 1, 11605, -21846, false),
            (-21846, 0, 1365, 21845, true),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 5461, 21845, false),
            (-21846, 0, 7509, 21845, false),
            (-21846, 0, 9557, 21845, false),
            (-21846, 0, 11605, -21846, false),
            (-21846, 0, 13653, 21845, false),
            (-21846, 0, 15701, 21845, false),
            (21845, 1, 11605, -21846, true),
            (21845, 1, 11605, 21845, false),
            (-21846, 1, 13653, 21845, true),
            (-21846, 1, 13653, -21846, false),
            (-21846, 0, 1365, 21845, true),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 5461, 21845, false),
            (-21846, 0, 7509, 21845, false),
            (-21846, 0, 9557, 21845, false),
            (-21846, 0, 11605, 21845, false),
            (-21846, 0, 13653, -21846, false),
            (-21846, 0, 15701, 21845, false),
            (21845, 1, 13653, -21846, true),
            (21845, 1, 13653, 21845, false),
            (-21846, 1, 15701, 21845, true),
            (-21846, 1, 15701, -21846, false),
            (-21846, 0, 1365, 21845, true),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 5461, 21845, false),
            (-21846, 0, 7509, 21845, false),
            (-21846, 0, 9557, 21845, false),
            (-21846, 0, 11605, 21845, false),
            (-21846, 0, 13653, 21845, false),
            (-21846, 0, 15701, -21846, false),
            (21845, 1, 15701, -21846, true),
            (21845, 1, 15701, 21845, false),
            (21845, 0, 1365, 21845, true),
            (21845, 0, 1365, 21845, false),
            (21845, 0, 3413, 21845, false),
            (21845, 0, 5461, 21845, false),
            (21845, 0, 7509, 21845, false),
            (21845, 0, 9557, 21845, false),
            (21845, 0, 11605, 21845, false),
            (21845, 0, 13653, 21845, false),
            (21845, 0, 15701, 21845, false),
        ];

        let input: SharedBus<16> = Bus::all0().to_shared_bus();
        let load: SharedBus<1> = Bus::all0().to_shared_bus();
        let address: SharedBus<14> = Bus::all0().to_shared_bus();
        let ram16k = RAM16K::new(input.clone(), load.clone(), address.clone());

        for case in cases {
            println!(
                "{}, {}, {}, {}, {}",
                &case.0, &case.1, &case.2, &case.3, &case.4
            );
            let _input = i16_to_bus16(case.0);
            let _sel = i16_to_bus1(case.1);
            let _address = i16_to_bus14(case.2);

            let out = i16_to_bus16(case.3).to_shared_bus();

            // 中身を変える
            input.overwrite(&_input);
            load.overwrite(&_sel);
            address.overwrite(&_address);

            // tick
            if case.4 {
                ram16k.re_compute();
                ram16k.clock_up();
                assert_eq!(ram16k.out.clone(), out.clone());
            } else {
                // tock
                ram16k.clock_down();
                ram16k.re_compute();
                assert_eq!(ram16k.out.clone(), out.clone());
            }
        }
    }

    #[test]
    fn ram16kbuiltin() {
        // 順番に依存している
        let cases: Vec<(i16, i16, i16, i16, bool)> = vec![
            (0, 0, 0, 0, true),
            (0, 0, 0, 0, false),
            (0, 1, 0, 0, true),
            (0, 1, 0, 0, false),
            (4321, 0, 0, 0, true),
            (4321, 0, 0, 0, false),
            (4321, 1, 4321, 0, true),
            (4321, 1, 4321, 4321, false),
            (4321, 0, 0, 0, true),
            (4321, 0, 0, 0, false),
            (12345, 0, 12345, 0, true),
            (12345, 0, 12345, 0, false),
            (12345, 1, 12345, 0, true),
            (12345, 1, 12345, 12345, false),
            (12345, 0, 12345, 12345, true),
            (12345, 0, 12345, 12345, false),
            (12345, 0, 4321, 4321, false),
            (16383, 0, 4321, 4321, true),
            (16383, 0, 4321, 4321, false),
            (16383, 1, 16383, 0, true),
            (16383, 1, 16383, 16383, false),
            (16383, 0, 16383, 16383, true),
            (16383, 0, 16383, 16383, false),
            (16383, 0, 12345, 12345, false),
            (16383, 0, 16383, 16383, false),
            (16383, 0, 10920, 0, true),
            (16383, 0, 10920, 0, false),
            (16383, 0, 10921, 0, false),
            (16383, 0, 10922, 0, false),
            (16383, 0, 10923, 0, false),
            (16383, 0, 10924, 0, false),
            (16383, 0, 10925, 0, false),
            (16383, 0, 10926, 0, false),
            (16383, 0, 10927, 0, false),
            (21845, 1, 10920, 0, true),
            (21845, 1, 10920, 21845, false),
            (21845, 1, 10921, 0, true),
            (21845, 1, 10921, 21845, false),
            (21845, 1, 10922, 0, true),
            (21845, 1, 10922, 21845, false),
            (21845, 1, 10923, 0, true),
            (21845, 1, 10923, 21845, false),
            (21845, 1, 10924, 0, true),
            (21845, 1, 10924, 21845, false),
            (21845, 1, 10925, 0, true),
            (21845, 1, 10925, 21845, false),
            (21845, 1, 10926, 0, true),
            (21845, 1, 10926, 21845, false),
            (21845, 1, 10927, 0, true),
            (21845, 1, 10927, 21845, false),
            (21845, 0, 10920, 21845, true),
            (21845, 0, 10920, 21845, false),
            (21845, 0, 10921, 21845, false),
            (21845, 0, 10922, 21845, false),
            (21845, 0, 10923, 21845, false),
            (21845, 0, 10924, 21845, false),
            (21845, 0, 10925, 21845, false),
            (21845, 0, 10926, 21845, false),
            (21845, 0, 10927, 21845, false),
            (-21846, 1, 10920, 21845, true),
            (-21846, 1, 10920, -21846, false),
            (-21846, 0, 10920, -21846, true),
            (-21846, 0, 10920, -21846, false),
            (-21846, 0, 10921, 21845, false),
            (-21846, 0, 10922, 21845, false),
            (-21846, 0, 10923, 21845, false),
            (-21846, 0, 10924, 21845, false),
            (-21846, 0, 10925, 21845, false),
            (-21846, 0, 10926, 21845, false),
            (-21846, 0, 10927, 21845, false),
            (21845, 1, 10920, -21846, true),
            (21845, 1, 10920, 21845, false),
            (-21846, 1, 10921, 21845, true),
            (-21846, 1, 10921, -21846, false),
            (-21846, 0, 10920, 21845, true),
            (-21846, 0, 10920, 21845, false),
            (-21846, 0, 10921, -21846, false),
            (-21846, 0, 10922, 21845, false),
            (-21846, 0, 10923, 21845, false),
            (-21846, 0, 10924, 21845, false),
            (-21846, 0, 10925, 21845, false),
            (-21846, 0, 10926, 21845, false),
            (-21846, 0, 10927, 21845, false),
            (21845, 1, 10921, -21846, true),
            (21845, 1, 10921, 21845, false),
            (-21846, 1, 10922, 21845, true),
            (-21846, 1, 10922, -21846, false),
            (-21846, 0, 10920, 21845, true),
            (-21846, 0, 10920, 21845, false),
            (-21846, 0, 10921, 21845, false),
            (-21846, 0, 10922, -21846, false),
            (-21846, 0, 10923, 21845, false),
            (-21846, 0, 10924, 21845, false),
            (-21846, 0, 10925, 21845, false),
            (-21846, 0, 10926, 21845, false),
            (-21846, 0, 10927, 21845, false),
            (21845, 1, 10922, -21846, true),
            (21845, 1, 10922, 21845, false),
            (-21846, 1, 10923, 21845, true),
            (-21846, 1, 10923, -21846, false),
            (-21846, 0, 10920, 21845, true),
            (-21846, 0, 10920, 21845, false),
            (-21846, 0, 10921, 21845, false),
            (-21846, 0, 10922, 21845, false),
            (-21846, 0, 10923, -21846, false),
            (-21846, 0, 10924, 21845, false),
            (-21846, 0, 10925, 21845, false),
            (-21846, 0, 10926, 21845, false),
            (-21846, 0, 10927, 21845, false),
            (21845, 1, 10923, -21846, true),
            (21845, 1, 10923, 21845, false),
            (-21846, 1, 10924, 21845, true),
            (-21846, 1, 10924, -21846, false),
            (-21846, 0, 10920, 21845, true),
            (-21846, 0, 10920, 21845, false),
            (-21846, 0, 10921, 21845, false),
            (-21846, 0, 10922, 21845, false),
            (-21846, 0, 10923, 21845, false),
            (-21846, 0, 10924, -21846, false),
            (-21846, 0, 10925, 21845, false),
            (-21846, 0, 10926, 21845, false),
            (-21846, 0, 10927, 21845, false),
            (21845, 1, 10924, -21846, true),
            (21845, 1, 10924, 21845, false),
            (-21846, 1, 10925, 21845, true),
            (-21846, 1, 10925, -21846, false),
            (-21846, 0, 10920, 21845, true),
            (-21846, 0, 10920, 21845, false),
            (-21846, 0, 10921, 21845, false),
            (-21846, 0, 10922, 21845, false),
            (-21846, 0, 10923, 21845, false),
            (-21846, 0, 10924, 21845, false),
            (-21846, 0, 10925, -21846, false),
            (-21846, 0, 10926, 21845, false),
            (-21846, 0, 10927, 21845, false),
            (21845, 1, 10925, -21846, true),
            (21845, 1, 10925, 21845, false),
            (-21846, 1, 10926, 21845, true),
            (-21846, 1, 10926, -21846, false),
            (-21846, 0, 10920, 21845, true),
            (-21846, 0, 10920, 21845, false),
            (-21846, 0, 10921, 21845, false),
            (-21846, 0, 10922, 21845, false),
            (-21846, 0, 10923, 21845, false),
            (-21846, 0, 10924, 21845, false),
            (-21846, 0, 10925, 21845, false),
            (-21846, 0, 10926, -21846, false),
            (-21846, 0, 10927, 21845, false),
            (21845, 1, 10926, -21846, true),
            (21845, 1, 10926, 21845, false),
            (-21846, 1, 10927, 21845, true),
            (-21846, 1, 10927, -21846, false),
            (-21846, 0, 10920, 21845, true),
            (-21846, 0, 10920, 21845, false),
            (-21846, 0, 10921, 21845, false),
            (-21846, 0, 10922, 21845, false),
            (-21846, 0, 10923, 21845, false),
            (-21846, 0, 10924, 21845, false),
            (-21846, 0, 10925, 21845, false),
            (-21846, 0, 10926, 21845, false),
            (-21846, 0, 10927, -21846, false),
            (21845, 1, 10927, -21846, true),
            (21845, 1, 10927, 21845, false),
            (21845, 0, 10920, 21845, true),
            (21845, 0, 10920, 21845, false),
            (21845, 0, 10921, 21845, false),
            (21845, 0, 10922, 21845, false),
            (21845, 0, 10923, 21845, false),
            (21845, 0, 10924, 21845, false),
            (21845, 0, 10925, 21845, false),
            (21845, 0, 10926, 21845, false),
            (21845, 0, 10927, 21845, false),
            (21845, 0, 1365, 0, true),
            (21845, 0, 1365, 0, false),
            (21845, 0, 3413, 0, false),
            (21845, 0, 5461, 0, false),
            (21845, 0, 7509, 0, false),
            (21845, 0, 9557, 0, false),
            (21845, 0, 11605, 0, false),
            (21845, 0, 13653, 0, false),
            (21845, 0, 15701, 0, false),
            (21845, 1, 1365, 0, true),
            (21845, 1, 1365, 21845, false),
            (21845, 1, 3413, 0, true),
            (21845, 1, 3413, 21845, false),
            (21845, 1, 5461, 0, true),
            (21845, 1, 5461, 21845, false),
            (21845, 1, 7509, 0, true),
            (21845, 1, 7509, 21845, false),
            (21845, 1, 9557, 0, true),
            (21845, 1, 9557, 21845, false),
            (21845, 1, 11605, 0, true),
            (21845, 1, 11605, 21845, false),
            (21845, 1, 13653, 0, true),
            (21845, 1, 13653, 21845, false),
            (21845, 1, 15701, 0, true),
            (21845, 1, 15701, 21845, false),
            (21845, 0, 1365, 21845, true),
            (21845, 0, 1365, 21845, false),
            (21845, 0, 3413, 21845, false),
            (21845, 0, 5461, 21845, false),
            (21845, 0, 7509, 21845, false),
            (21845, 0, 9557, 21845, false),
            (21845, 0, 11605, 21845, false),
            (21845, 0, 13653, 21845, false),
            (21845, 0, 15701, 21845, false),
            (-21846, 1, 1365, 21845, true),
            (-21846, 1, 1365, -21846, false),
            (-21846, 0, 1365, -21846, true),
            (-21846, 0, 1365, -21846, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 5461, 21845, false),
            (-21846, 0, 7509, 21845, false),
            (-21846, 0, 9557, 21845, false),
            (-21846, 0, 11605, 21845, false),
            (-21846, 0, 13653, 21845, false),
            (-21846, 0, 15701, 21845, false),
            (21845, 1, 1365, -21846, true),
            (21845, 1, 1365, 21845, false),
            (-21846, 1, 3413, 21845, true),
            (-21846, 1, 3413, -21846, false),
            (-21846, 0, 1365, 21845, true),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 3413, -21846, false),
            (-21846, 0, 5461, 21845, false),
            (-21846, 0, 7509, 21845, false),
            (-21846, 0, 9557, 21845, false),
            (-21846, 0, 11605, 21845, false),
            (-21846, 0, 13653, 21845, false),
            (-21846, 0, 15701, 21845, false),
            (21845, 1, 3413, -21846, true),
            (21845, 1, 3413, 21845, false),
            (-21846, 1, 5461, 21845, true),
            (-21846, 1, 5461, -21846, false),
            (-21846, 0, 1365, 21845, true),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 5461, -21846, false),
            (-21846, 0, 7509, 21845, false),
            (-21846, 0, 9557, 21845, false),
            (-21846, 0, 11605, 21845, false),
            (-21846, 0, 13653, 21845, false),
            (-21846, 0, 15701, 21845, false),
            (21845, 1, 5461, -21846, true),
            (21845, 1, 5461, 21845, false),
            (-21846, 1, 7509, 21845, true),
            (-21846, 1, 7509, -21846, false),
            (-21846, 0, 1365, 21845, true),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 5461, 21845, false),
            (-21846, 0, 7509, -21846, false),
            (-21846, 0, 9557, 21845, false),
            (-21846, 0, 11605, 21845, false),
            (-21846, 0, 13653, 21845, false),
            (-21846, 0, 15701, 21845, false),
            (21845, 1, 7509, -21846, true),
            (21845, 1, 7509, 21845, false),
            (-21846, 1, 9557, 21845, true),
            (-21846, 1, 9557, -21846, false),
            (-21846, 0, 1365, 21845, true),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 5461, 21845, false),
            (-21846, 0, 7509, 21845, false),
            (-21846, 0, 9557, -21846, false),
            (-21846, 0, 11605, 21845, false),
            (-21846, 0, 13653, 21845, false),
            (-21846, 0, 15701, 21845, false),
            (21845, 1, 9557, -21846, true),
            (21845, 1, 9557, 21845, false),
            (-21846, 1, 11605, 21845, true),
            (-21846, 1, 11605, -21846, false),
            (-21846, 0, 1365, 21845, true),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 5461, 21845, false),
            (-21846, 0, 7509, 21845, false),
            (-21846, 0, 9557, 21845, false),
            (-21846, 0, 11605, -21846, false),
            (-21846, 0, 13653, 21845, false),
            (-21846, 0, 15701, 21845, false),
            (21845, 1, 11605, -21846, true),
            (21845, 1, 11605, 21845, false),
            (-21846, 1, 13653, 21845, true),
            (-21846, 1, 13653, -21846, false),
            (-21846, 0, 1365, 21845, true),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 5461, 21845, false),
            (-21846, 0, 7509, 21845, false),
            (-21846, 0, 9557, 21845, false),
            (-21846, 0, 11605, 21845, false),
            (-21846, 0, 13653, -21846, false),
            (-21846, 0, 15701, 21845, false),
            (21845, 1, 13653, -21846, true),
            (21845, 1, 13653, 21845, false),
            (-21846, 1, 15701, 21845, true),
            (-21846, 1, 15701, -21846, false),
            (-21846, 0, 1365, 21845, true),
            (-21846, 0, 1365, 21845, false),
            (-21846, 0, 3413, 21845, false),
            (-21846, 0, 5461, 21845, false),
            (-21846, 0, 7509, 21845, false),
            (-21846, 0, 9557, 21845, false),
            (-21846, 0, 11605, 21845, false),
            (-21846, 0, 13653, 21845, false),
            (-21846, 0, 15701, -21846, false),
            (21845, 1, 15701, -21846, true),
            (21845, 1, 15701, 21845, false),
            (21845, 0, 1365, 21845, true),
            (21845, 0, 1365, 21845, false),
            (21845, 0, 3413, 21845, false),
            (21845, 0, 5461, 21845, false),
            (21845, 0, 7509, 21845, false),
            (21845, 0, 9557, 21845, false),
            (21845, 0, 11605, 21845, false),
            (21845, 0, 13653, 21845, false),
            (21845, 0, 15701, 21845, false),
        ];

        let input: SharedBus<16> = Bus::all0().to_shared_bus();
        let load: SharedBus<1> = Bus::all0().to_shared_bus();
        let address: SharedBus<14> = Bus::all0().to_shared_bus();
        let ram16k = RAM16KBuiltIn::new(input.clone(), load.clone(), address.clone());

        for case in cases {
            println!(
                "{}, {}, {}, {}, {}",
                &case.0, &case.1, &case.2, &case.3, &case.4
            );
            let _input = i16_to_bus16(case.0);
            let _sel = i16_to_bus1(case.1);
            let _address = i16_to_bus14(case.2);

            let out = i16_to_bus16(case.3).to_shared_bus();

            // 中身を変える
            input.overwrite(&_input);
            load.overwrite(&_sel);
            address.overwrite(&_address);

            // tick
            if case.4 {
                ram16k.re_compute();
                ram16k.clock_up();
                assert_eq!(ram16k.out.clone(), out.clone());
            } else {
                // tock
                ram16k.clock_down();
                ram16k.re_compute();
                assert_eq!(ram16k.out.clone(), out.clone());
            }
        }
    }

    // TODO generic
    fn i16_to_bus1(x: i16) -> Bus<1> {
        let s = format!("{x:01b}");
        s.parse::<Bus<1>>().unwrap()
    }
    fn i16_to_bus3(x: i16) -> Bus<3> {
        let s = format!("{x:03b}");
        s.parse::<Bus<3>>().unwrap()
    }
    fn i16_to_bus6(x: i16) -> Bus<6> {
        let s = format!("{x:06b}");
        s.parse::<Bus<6>>().unwrap()
    }
    fn i16_to_bus9(x: i16) -> Bus<9> {
        let s = format!("{x:09b}");
        s.parse::<Bus<9>>().unwrap()
    }
    fn i16_to_bus12(x: i16) -> Bus<12> {
        let s = format!("{x:012b}");
        s.parse::<Bus<12>>().unwrap()
    }
    fn i16_to_bus14(x: i16) -> Bus<14> {
        let s = format!("{x:014b}");
        s.parse::<Bus<14>>().unwrap()
    }
    fn i16_to_bus16(x: i16) -> Bus<16> {
        let s = format!("{x:016b}");
        s.parse::<Bus<16>>().unwrap()
    }
}
