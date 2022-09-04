use std::cell::{Cell, RefCell};

use crate::{
    arithmetic::ALU,
    gate::*,
    sequential::{RAM16KBuiltIn, Register, PC},
};

#[derive(Debug)]
pub struct ROM32KBuiltIn {
    pub out: SharedBus<16>,
    pub rom: Box<[u16; 32768]>,
    address: SharedBus<15>,
}

impl ROM32KBuiltIn {
    pub fn new(rom: Box<[u16; 32768]>, address: SharedBus<15>) -> ROM32KBuiltIn {
        let out = Bus::all0().to_shared_bus();
        ROM32KBuiltIn { out, rom, address }
    }

    // rom_str example
    // "0000000000000000
    //  1111110000010000
    //  0000000000010111
    //  1110001100000110"
    pub fn from_rom_str(rom_str: &str, address: SharedBus<15>) -> ROM32KBuiltIn {
        let mut rom: Box<[u16; 32768]> = Box::new([0; 32768]);
        for (index, line) in rom_str.lines().enumerate() {
            // FIXME bus_to_u16とロジックがかぶってるので直す
            let bits = line.chars().rev().collect::<Vec<char>>();
            let mut u = 0;
            for i in 0..16 {
                u += match bits.get(i) {
                    Some('1') => 2_u16.pow(i.try_into().unwrap()),
                    _ => 0,
                }
            }
            rom[index] = u;
        }

        ROM32KBuiltIn::new(rom, address)
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

impl Gate for ROM32KBuiltIn {
    fn re_compute(&self) -> () {
        let address = Self::bus_to_u16::<15>(self.address.clone()) as usize;
        let mut value = self.rom[address];

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
pub struct ScreenBuiltIn {
    pub out: SharedBus<16>,
    ram: RefCell<Box<[u16; 8192]>>,
    input: SharedBus<16>,
    load: SharedBus<1>,
    address: SharedBus<13>,
    next: Cell<u16>,
}

impl ScreenBuiltIn {
    pub fn new(input: SharedBus<16>, load: SharedBus<1>, address: SharedBus<13>) -> ScreenBuiltIn {
        let ram: RefCell<Box<[u16; 8192]>> = RefCell::new(Box::new([0; 8192]));
        let out = Bus::all0().to_shared_bus();
        let next = Cell::new(0);
        ScreenBuiltIn {
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

impl Gate for ScreenBuiltIn {
    fn clock_up(&self) -> () {
        if self.load.get_shared_bit(0).get() == I {
            let value = Self::bus_to_u16::<16>(self.input.clone());
            self.next.set(value);
        }
    }

    fn clock_down(&self) -> () {
        if self.load.get_shared_bit(0).get() == I {
            let load_address = Self::bus_to_u16::<13>(self.address.clone()) as usize;
            let mut ram = self.ram.borrow_mut();
            ram[load_address] = self.next.get();
        }
    }

    fn re_compute(&self) -> () {
        let address = Self::bus_to_u16::<13>(self.address.clone()) as usize;
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
pub struct KeyboardBuiltIn {
    pub out: SharedBus<16>,
}

impl KeyboardBuiltIn {
    pub fn new() -> KeyboardBuiltIn {
        let out = Bus::all0().to_shared_bus();
        KeyboardBuiltIn { out }
    }
}

impl Gate for KeyboardBuiltIn {
    // TODO
}

#[derive(Debug)]
pub struct MemoryBuiltIn {
    pub out: SharedBus<16>,
    dmux4way: DMux4Way,
    or: Or<1>,
    ram16k: RAM16KBuiltIn,
    screen: ScreenBuiltIn,
    keyboard: KeyboardBuiltIn,
    mux4way16: Mux4Way16,
}

impl MemoryBuiltIn {
    pub fn new(input: SharedBus<16>, load: SharedBus<1>, address: SharedBus<15>) -> MemoryBuiltIn {
        let dmux4way = DMux4Way::new(load.clone(), address.reconnect([13, 14]));
        let or = Or::new(dmux4way.out1.clone(), dmux4way.out2.clone());
        let ram16k = RAM16KBuiltIn::new(
            input.clone(),
            or.out.clone(),
            address.reconnect([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]),
        );
        let screen = ScreenBuiltIn::new(
            input.clone(),
            dmux4way.out3.clone(),
            address.reconnect([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]),
        );
        let keyboard = KeyboardBuiltIn::new();
        let mux4way16 = Mux4Way16::new(
            ram16k.out.clone(),
            ram16k.out.clone(),
            screen.out.clone(),
            keyboard.out.clone(),
            address.reconnect([13, 14]),
        );

        MemoryBuiltIn {
            out: mux4way16.out.clone(),
            dmux4way,
            or,
            ram16k,
            screen,
            keyboard,
            mux4way16,
        }
    }
}

impl Gate for MemoryBuiltIn {
    fn clock_up(&self) -> () {
        self.dmux4way.clock_up();
        self.or.clock_up();
        self.ram16k.clock_up();
        self.screen.clock_up();
        self.keyboard.clock_up();
        self.mux4way16.clock_up();
    }

    fn clock_down(&self) -> () {
        self.dmux4way.clock_down();
        self.or.clock_down();
        self.ram16k.clock_down();
        self.screen.clock_down();
        self.keyboard.clock_down();
        self.mux4way16.clock_down();
    }

    fn re_compute(&self) -> () {
        self.dmux4way.re_compute();
        self.or.re_compute();
        self.ram16k.re_compute();
        self.screen.re_compute();
        self.keyboard.re_compute();
        self.mux4way16.re_compute();
    }
}

#[derive(Debug)]
pub struct CPU {
    pub out_m: SharedBus<16>,
    pub write_m: SharedBus<1>,
    pub address_m: SharedBus<15>,
    pub pc: SharedBus<15>,
    not1: Not<1>,
    not2: Not<1>,
    and1: And<1>,
    alu_out: SharedBus<16>,
    mux1: Mux<16>,
    or1: Or<1>,
    a_register: Register,
    mux2: Mux<16>,
    and2: And<1>,
    d_register: Register,
    alu: ALU,
    or2: Or<16>,
    or3: Or<16>,
    and3: And<1>,
    and4: And<1>,
    and5: And<1>,
    or4: Or<1>,
    not3: Not<1>,
    and6: And<1>,
    or5: Or<1>,
    or6: Or<1>,
    and7: And<1>,
    not4: Not<1>,
    pc_gate: PC,
}

impl CPU {
    pub fn new(in_m: SharedBus<16>, instruction: SharedBus<16>, reset: SharedBus<1>) -> CPU {
        let not1 = Not::new(instruction.reconnect([15]).clone());
        let not2 = Not::new(not1.out.clone());

        let and1 = And::new(not2.out.clone(), instruction.reconnect([5]));

        let alu_out = Bus::<16>::all0().to_shared_bus();
        let mux1 = Mux::new(instruction.clone(), alu_out.clone(), and1.out.clone());

        let or1 = Or::new(not1.out.clone(), and1.out.clone());
        let a_register = Register::new(mux1.out.clone(), or1.out.clone());

        let mux2 = Mux::new(
            a_register.out.clone(),
            in_m.clone(),
            instruction.reconnect([12]),
        );

        let and2 = And::new(not2.out.clone(), instruction.reconnect([4]));
        let d_register = Register::new(alu_out.clone(), and2.out.clone());

        let alu = ALU::new(
            d_register.out.clone(),
            mux2.out.clone(),
            instruction.reconnect([11]),
            instruction.reconnect([10]),
            instruction.reconnect([9]),
            instruction.reconnect([8]),
            instruction.reconnect([7]),
            instruction.reconnect([6]),
        );

        let or2 = Or::new(Bus::all0().to_shared_bus(), a_register.out.clone());
        let or3 = Or::new(Bus::all0().to_shared_bus(), alu.out.clone());
        let and3 = And::new(not2.out.clone(), instruction.reconnect([3]));

        let and4 = And::new(alu.zr.clone(), instruction.reconnect([1]));
        let and5 = And::new(alu.ng.clone(), instruction.reconnect([2]));
        let or4 = Or::new(alu.zr.clone(), alu.ng.clone());
        let not3 = Not::new(or4.out.clone());
        let and6 = And::new(not3.out.clone(), instruction.reconnect([0]));
        let or5 = Or::new(and4.out.clone(), and5.out.clone());
        let or6 = Or::new(or5.out.clone(), and6.out.clone());
        let and7 = And::new(not2.out.clone(), or6.out.clone());
        let not4 = Not::new(and7.out.clone());
        let pc_gate = PC::new(
            a_register.out.clone(),
            and7.out.clone(),
            not4.out.clone(),
            reset.clone(),
        );

        return CPU {
            out_m: or3.out.clone(),
            write_m: and3.out.clone(),
            address_m: or2
                .out
                .reconnect([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]),
            pc: pc_gate
                .out
                .reconnect([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]),
            not1,
            not2,
            and1,
            alu_out,
            mux1,
            or1,
            a_register,
            mux2,
            and2,
            d_register,
            alu,
            or2,
            or3,
            and3,
            and4,
            and5,
            or4,
            not3,
            and6,
            or5,
            or6,
            and7,
            not4,
            pc_gate,
        };
    }

    pub fn get_a_register_value(&self) -> u16 {
        self.a_register.out.to_u16()
    }

    pub fn get_d_register_value(&self) -> u16 {
        self.d_register.out.to_u16()
    }
}

impl Gate for CPU {
    fn clock_up(&self) -> () {
        self.not1.clock_up();
        self.not2.clock_up();
        self.and1.clock_up();
        self.mux1.clock_up();
        self.or1.clock_up();
        self.a_register.clock_up();
        self.mux2.clock_up();
        self.and2.clock_up();
        self.d_register.clock_up();
        self.alu.clock_up();
        self.or2.clock_up();
        self.or3.clock_up();
        self.and3.clock_up();
        self.and4.clock_up();
        self.and5.clock_up();
        self.or4.clock_up();
        self.not3.clock_up();
        self.and6.clock_up();
        self.or5.clock_up();
        self.or6.clock_up();
        self.and7.clock_up();
        self.not4.clock_up();
        self.pc_gate.clock_up();
    }

    fn clock_down(&self) -> () {
        self.not1.clock_down();
        self.not2.clock_down();
        self.and1.clock_down();
        self.mux1.clock_down();
        self.or1.clock_down();
        self.a_register.clock_down();
        self.mux2.clock_down();
        self.and2.clock_down();
        self.d_register.clock_down();
        self.alu.clock_down();
        self.or2.clock_down();
        self.or3.clock_down();
        self.and3.clock_down();
        self.and4.clock_down();
        self.and5.clock_down();
        self.or4.clock_down();
        self.not3.clock_down();
        self.and6.clock_down();
        self.or5.clock_down();
        self.or6.clock_down();
        self.and7.clock_down();
        self.not4.clock_down();
        self.pc_gate.clock_down();
    }

    fn re_compute(&self) -> () {
        self.not1.re_compute();
        self.not2.re_compute();
        self.and1.re_compute();
        //self.mux1.re_compute();
        self.or1.re_compute();
        //self.a_register.re_compute();
        self.mux2.re_compute();
        self.and2.re_compute();
        //self.d_register.re_compute();
        self.alu.re_compute();
        self.alu_out.overwrite(&self.alu.out.0.borrow());
        self.or2.re_compute();
        self.or3.re_compute();
        self.and3.re_compute();
        self.and4.re_compute();
        self.and5.re_compute();
        self.or4.re_compute();
        self.not3.re_compute();
        self.and6.re_compute();
        self.or5.re_compute();
        self.or6.re_compute();
        self.and7.re_compute();
        self.not4.re_compute();
        self.pc_gate.re_compute();

        // alu_outの出力をレジスタまで伝播させる
        self.mux1.re_compute();
        self.a_register.re_compute();
        self.d_register.re_compute();
    }
}

#[derive(Debug)]
pub struct Computer {
    rom: ROM32KBuiltIn,
    pub cpu: CPU,
    memory: MemoryBuiltIn,
    memory_out: SharedBus<16>,
}

impl Computer {
    pub fn new(reset: SharedBus<1>, rom: ROM32KBuiltIn) -> Computer {
        let memory_out = Bus::all0().to_shared_bus();
        let cpu = CPU::new(memory_out.clone(), rom.out.clone(), reset.clone());
        let memory = MemoryBuiltIn::new(
            cpu.out_m.clone(),
            cpu.write_m.clone(),
            cpu.address_m.clone(),
        );
        return Computer {
            memory_out,
            rom,
            cpu,
            memory,
        };
    }

    pub fn tick(&self) -> () {
        self.re_compute();
        self.clock_up();
    }

    pub fn tock(&self) -> () {
        self.clock_down();
        self.re_compute();
    }

    pub fn get_r0(&self) -> u16 {
        let d = self.memory.ram16k.ram.borrow();
        d[0]
    }
}

impl Gate for Computer {
    fn clock_up(&self) -> () {
        self.rom.clock_up();
        self.cpu.clock_up();
        self.memory.clock_up();
    }

    fn clock_down(&self) -> () {
        self.rom.clock_down();
        self.cpu.clock_down();
        self.memory.clock_down();
    }

    fn re_compute(&self) -> () {
        self.rom.address.overwrite(&self.cpu.pc.0.borrow());
        self.memory_out.overwrite(&self.memory.out.0.borrow());

        self.rom.re_compute();
        self.cpu.re_compute();
        self.memory.re_compute();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::Cell;
    use std::rc::Rc;

    fn str_to_shared_bus<const N: usize>(s: &str) -> SharedBus<N> {
        let mut bits = [O; N];
        for (i, c) in s.chars().rev().enumerate() {
            let bit = match c {
                '0' => O,
                '1' => I,
                _ => O,
            };
            bits[i] = bit;
        }

        Bus::new(bits.map(|b| Rc::new(Cell::new(b)))).to_shared_bus()
    }

    #[test]
    fn rom32k() {
        let rom_str = "0000000000000000\n\
                             1111110000010000\n\
                             0000000000010111\n\
                             1110001100000110";
        // 0
        let address = str_to_shared_bus::<15>("0");
        let rom = ROM32KBuiltIn::from_rom_str(rom_str, address);
        rom.re_compute();
        assert_eq!(rom.out.to_u16(), 0);

        // 1
        let address = str_to_shared_bus::<15>("1");
        let rom = ROM32KBuiltIn::from_rom_str(rom_str, address);
        rom.re_compute();
        assert_eq!(rom.out.to_u16(), 64528);

        // 2
        let address = str_to_shared_bus::<15>("10");
        let rom = ROM32KBuiltIn::from_rom_str(rom_str, address);
        rom.re_compute();
        assert_eq!(rom.out.to_u16(), 23);

        // 3
        let address = str_to_shared_bus::<15>("11");
        let rom = ROM32KBuiltIn::from_rom_str(rom_str, address);
        rom.re_compute();
        assert_eq!(rom.out.to_u16(), 58118);
    }

    #[test]
    fn ram16kbuiltin() {
        // 順番に依存している
        let cases: Vec<(i16, i16, i16, i16)> = vec![
            (12345, 1, 8192, 0),
            (12345, 1, 8192, 12345),
            (12345, 1, 16384, 0),
            (12345, 1, 16384, 12345),
            // keyboardは未対応
        ];

        let input: SharedBus<16> = Bus::all0().to_shared_bus();
        let load: SharedBus<1> = Bus::all0().to_shared_bus();
        let address: SharedBus<15> = Bus::all0().to_shared_bus();
        let memory = MemoryBuiltIn::new(input.clone(), load.clone(), address.clone());

        for case in cases {
            println!("{}, {}, {}, {}", &case.0, &case.1, &case.2, &case.3);
            let _input = i16_to_bus16(case.0);
            let _sel = i16_to_bus1(case.1);
            let _address = i16_to_bus15(case.2);

            let out = i16_to_bus16(case.3).to_shared_bus();

            // 中身を変える
            input.overwrite(&_input);
            load.overwrite(&_sel);
            address.overwrite(&_address);

            memory.re_compute();
            memory.clock_up();
            assert_eq!(memory.out.clone(), out.clone());

            // tock
            memory.clock_down();
            memory.re_compute();
        }
    }

    #[test]
    fn cpu() {
        let in_m = Bus::<16>::all0().to_shared_bus();
        let instruction = i16_to_bus16(12345).to_shared_bus();
        let reset = Bus::<1>::all0().to_shared_bus();

        let cpu = CPU::new(in_m.clone(), instruction.clone(), reset.clone());

        // 0+------------------
        instruction.overwrite(&u16_to_bus16(12345));

        cpu.re_compute();
        cpu.clock_up();

        assert_eq!(cpu.write_m.to_u16(), 0);
        assert_eq!(cpu.address_m.to_u16(), 0);
        assert_eq!(cpu.pc.to_u16(), 0);

        // 1------------------

        cpu.clock_down();
        cpu.re_compute();

        assert_eq!(cpu.write_m.to_u16(), 0);
        assert_eq!(cpu.address_m.to_u16(), 12345);
        assert_eq!(cpu.pc.to_u16(), 1);

        // 1+ ---------------------------
        instruction.overwrite(&u16_to_bus16(60432));
        cpu.re_compute();
        cpu.clock_up();

        assert_eq!(cpu.write_m.to_u16(), 0);
        assert_eq!(cpu.address_m.to_u16(), 12345);
        assert_eq!(cpu.pc.to_u16(), 1);

        // 2------------------
        cpu.clock_down();
        cpu.re_compute();

        assert_eq!(cpu.pc.to_u16(), 2);

        // ここで D=Aの実行が終わって、DにA(12345)が入るはず
        assert_eq!(cpu.get_d_register_value(), 12345);
    }

    // TODO generic
    fn i16_to_bus1(x: i16) -> Bus<1> {
        let s = format!("{x:01b}");
        s.parse::<Bus<1>>().unwrap()
    }
    fn i16_to_bus15(x: i16) -> Bus<15> {
        let s = format!("{x:015b}");
        s.parse::<Bus<15>>().unwrap()
    }
    fn i16_to_bus16(x: i16) -> Bus<16> {
        let s = format!("{x:016b}");
        s.parse::<Bus<16>>().unwrap()
    }
    fn u16_to_bus16(x: u16) -> Bus<16> {
        let s = format!("{x:016b}");
        s.parse::<Bus<16>>().unwrap()
    }
}
