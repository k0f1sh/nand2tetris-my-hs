use core::str::FromStr;
use std::cell::RefCell;
use std::{cell::Cell, rc::Rc};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Bit {
    O, // 0
    I, // 1
}

pub use Bit::*;

pub type SharedBit = Rc<Cell<Bit>>;

#[derive(PartialEq, Clone)]
pub struct Bus<const N: usize> {
    pub bits: [SharedBit; N],
}

impl<const N: usize> std::fmt::Debug for Bus<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self
            .bits
            .iter()
            .rev()
            .map(|b| {
                let c = b.get();
                match c {
                    O => '0',
                    I => '1',
                }
            })
            .collect();
        let intval = isize::from_str_radix(&s, 2).unwrap();

        write!(f, "Bus[{}]({})", s, intval)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SharedBus<const N: usize>(pub Rc<RefCell<Bus<N>>>);

impl Bus<1> {
    // 例: Bus<1> を Bus<4> にしたいとき
    // Bus<1> = "0"にならBus<4> = "0000"
    // Bus<1> = "1"にならBus<4> = "1111"
    pub fn widen<const N: usize>(&self) -> Bus<N> {
        let bit = self.bits[0].clone();
        let bits = [(); N].map(|_| bit.clone());
        Bus { bits }
    }
}

impl<const N: usize> Bus<N> {
    pub fn new(bits: [SharedBit; N]) -> Self {
        Bus { bits }
    }

    pub fn all0() -> Self {
        Bus {
            //bits: core::array::from_fn(|| Rc::new(Cell::new(O))),
            bits: [(); N].map(|_| Rc::new(Cell::new(O))),
        }
    }

    pub fn all1() -> Self {
        Bus {
            bits: [(); N].map(|_| Rc::new(Cell::new(I))),
        }
    }

    pub fn to_shared_bus(self) -> SharedBus<N> {
        SharedBus(Rc::new(RefCell::new(self)))
    }

    pub fn get_shared_bit(&self, index: usize) -> SharedBit {
        self.bits[index].clone()
    }
    pub fn to_u16(&self) -> u16 {
        let mut u = 0;
        for i in 0..N {
            u += match self.bits.get(i).map(|s| s.get()) {
                Some(Bit::I) => 2_u16.pow(i.try_into().unwrap()),
                _ => 0,
            }
        }
        u
    }
}

impl SharedBus<1> {
    pub fn widen<const N: usize>(&self) -> SharedBus<N> {
        self.0.borrow().widen().to_shared_bus()
    }
}

impl<const N: usize> SharedBus<N> {
    pub fn get_shared_bit(&self, index: usize) -> SharedBit {
        self.0.borrow().get_shared_bit(index)
    }

    // もとのSharedBusの指定bitをつなぎ直した新しいSharedBusを生成する
    // FIXME indexがN以上の値だとpanicになってしまう。できればコンパイル時にエラーにしたい。
    pub fn reconnect<const M: usize>(&self, bits: [usize; M]) -> SharedBus<M> {
        Bus::new(bits.map(|index| self.get_shared_bit(index).clone())).to_shared_bus()
    }

    pub fn overwrite(&self, bus: &Bus<N>) -> () {
        for i in 0..N {
            self.get_shared_bit(i).set(bus.get_shared_bit(i).get());
        }
    }

    pub fn to_u16(&self) -> u16 {
        return self.0.borrow().to_u16();
    }
}

#[derive(Debug)]
pub struct ParseBusStrError;

impl<const N: usize> FromStr for Bus<N> {
    type Err = ParseBusStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != N {
            return Err(ParseBusStrError);
        }

        let bits = s
            .chars()
            .map(|c| match c {
                '0' => Ok(O),
                '1' => Ok(I),
                _ => Err(ParseBusStrError),
            })
            .collect::<Result<Vec<Bit>, ParseBusStrError>>();

        if bits.is_err() {
            return Err(ParseBusStrError);
        }
        let mut bits = bits.unwrap();
        let bit_array: [SharedBit; N] = [(); N].map(|_| Rc::new(Cell::new(O)));
        for i in 0..N {
            bit_array[i].set(bits.pop().unwrap_or(O))
        }

        Ok(Bus { bits: bit_array })
    }
}

pub trait Gate {
    fn re_compute(&self) -> () {}
    fn clock_up(&self) -> () {}
    fn clock_down(&self) -> () {}
}

#[derive(Debug)]
pub struct Nand<const N: usize> {
    a: SharedBus<N>,
    b: SharedBus<N>,
    out: SharedBus<N>,
}

impl<const N: usize> Nand<N> {
    pub fn new(a: SharedBus<N>, b: SharedBus<N>) -> Nand<N> {
        let a = a.clone();
        let b = b.clone();
        let out = Bus::<N>::all0().to_shared_bus();
        Nand { a, b, out }
    }
}

impl<const N: usize> Gate for Nand<N> {
    fn re_compute(&self) -> () {
        let a = self.a.0.borrow();
        let b = self.b.0.borrow();
        let out = self.out.0.borrow_mut();

        for i in 0..N {
            let bit = match (a.bits[i].get(), b.bits[i].get()) {
                (O, O) => I,
                (O, I) => I,
                (I, O) => I,
                (I, I) => O,
            };
            out.bits[i].set(bit);
        }
    }
}

#[derive(Debug)]
pub struct Not<const N: usize> {
    pub out: SharedBus<N>,
    nand: Nand<N>,
}

impl<const N: usize> Not<N> {
    pub fn new(input: SharedBus<N>) -> Not<N> {
        let nand = Nand::new(input.clone(), input.clone());
        Not {
            out: nand.out.clone(),
            nand,
        }
    }
}

impl<const N: usize> Gate for Not<N> {
    fn re_compute(&self) -> () {
        self.nand.re_compute();
    }
}

#[derive(Debug)]
pub struct And<const N: usize> {
    pub out: SharedBus<N>,
    nand: Nand<N>,
    not: Not<N>,
}

impl<const N: usize> And<N> {
    pub fn new(a: SharedBus<N>, b: SharedBus<N>) -> And<N> {
        let nand = Nand::new(a.clone(), b.clone());
        let not = Not::new(nand.out.clone());
        And {
            out: not.out.clone(),
            nand,
            not,
        }
    }
}

impl<const N: usize> Gate for And<N> {
    fn re_compute(&self) -> () {
        self.nand.re_compute();
        self.not.re_compute();
    }
}

#[derive(Debug)]
pub struct Or<const N: usize> {
    pub out: SharedBus<N>,
    nand1: Nand<N>,
    nand2: Nand<N>,
    nand3: Nand<N>,
}

impl<const N: usize> Or<N> {
    pub fn new(a: SharedBus<N>, b: SharedBus<N>) -> Or<N> {
        let nand1 = Nand::new(a.clone(), a.clone());
        let nand2 = Nand::new(b.clone(), b.clone());
        let nand3 = Nand::new(nand1.out.clone(), nand2.out.clone());
        Or {
            out: nand3.out.clone(),
            nand1,
            nand2,
            nand3,
        }
    }
}

impl<const N: usize> Gate for Or<N> {
    fn re_compute(&self) -> () {
        self.nand1.re_compute();
        self.nand2.re_compute();
        self.nand3.re_compute();
    }
}

#[derive(Debug)]
pub struct Xor<const N: usize> {
    pub out: SharedBus<N>,
    nand1: Nand<N>,
    nand2: Nand<N>,
    nand3: Nand<N>,
    nand4: Nand<N>,
}

impl<const N: usize> Xor<N> {
    pub fn new(a: SharedBus<N>, b: SharedBus<N>) -> Xor<N> {
        let nand1 = Nand::new(a.clone(), b.clone());
        let nand2 = Nand::new(a.clone(), nand1.out.clone());
        let nand3 = Nand::new(b.clone(), nand1.out.clone());
        let nand4 = Nand::new(nand2.out.clone(), nand3.out.clone());

        Xor {
            out: nand4.out.clone(),
            nand1,
            nand2,
            nand3,
            nand4,
        }
    }
}

impl<const N: usize> Gate for Xor<N> {
    fn re_compute(&self) -> () {
        self.nand1.re_compute();
        self.nand2.re_compute();
        self.nand3.re_compute();
        self.nand4.re_compute();
    }
}

#[derive(Debug)]
pub struct Mux<const N: usize> {
    pub out: SharedBus<N>,
    not: Not<N>,
    and1: And<N>,
    and2: And<N>,
    or: Or<N>,
}

impl<const N: usize> Mux<N> {
    pub fn new(a: SharedBus<N>, b: SharedBus<N>, sel: SharedBus<1>) -> Mux<N> {
        let sel_n: SharedBus<N> = sel.widen();

        let not = Not::new(sel_n.clone());
        let and1 = And::new(not.out.clone(), a.clone());
        let and2 = And::new(sel_n.clone(), b.clone());
        let or = Or::new(and1.out.clone(), and2.out.clone());
        Mux {
            out: or.out.clone(),
            not,
            and1,
            and2,
            or,
        }
    }
}

impl<const N: usize> Gate for Mux<N> {
    fn re_compute(&self) -> () {
        self.not.re_compute();
        self.and1.re_compute();
        self.and2.re_compute();
        self.or.re_compute();
    }
}

#[derive(Debug)]
pub struct DMux {
    pub out1: SharedBus<1>,
    pub out2: SharedBus<1>,
    not: Not<1>,
    and1: And<1>,
    and2: And<1>,
}

impl DMux {
    pub fn new(input: SharedBus<1>, sel: SharedBus<1>) -> DMux {
        let not = Not::new(sel.clone());
        let and1 = And::new(not.out.clone(), input.clone());
        let and2 = And::new(sel.clone(), input.clone());
        DMux {
            out1: and1.out.clone(),
            out2: and2.out.clone(),
            not,
            and1,
            and2,
        }
    }
}

impl Gate for DMux {
    fn re_compute(&self) -> () {
        self.not.re_compute();
        self.and1.re_compute();
        self.and2.re_compute();
    }
}

#[derive(Debug)]
pub struct Or8Way {
    pub out: SharedBus<1>,
    or1: Or<1>,
    or2: Or<1>,
    or3: Or<1>,
    or4: Or<1>,
    or5: Or<1>,
    or6: Or<1>,
    or7: Or<1>,
}

impl Or8Way {
    pub fn new(input: SharedBus<8>) -> Or8Way {
        let i0 = Bus::new([input.get_shared_bit(0)]).to_shared_bus();
        let i1 = Bus::new([input.get_shared_bit(1)]).to_shared_bus();
        let i2 = Bus::new([input.get_shared_bit(2)]).to_shared_bus();
        let i3 = Bus::new([input.get_shared_bit(3)]).to_shared_bus();
        let i4 = Bus::new([input.get_shared_bit(4)]).to_shared_bus();
        let i5 = Bus::new([input.get_shared_bit(5)]).to_shared_bus();
        let i6 = Bus::new([input.get_shared_bit(6)]).to_shared_bus();
        let i7 = Bus::new([input.get_shared_bit(7)]).to_shared_bus();

        let or1 = Or::new(i0, i1);
        let or2 = Or::new(i2, i3);
        let or3 = Or::new(i4, i5);
        let or4 = Or::new(i6, i7);
        let or5 = Or::new(or1.out.clone(), or2.out.clone());
        let or6 = Or::new(or3.out.clone(), or4.out.clone());
        let or7 = Or::new(or5.out.clone(), or6.out.clone());
        Or8Way {
            out: or7.out.clone(),
            or1,
            or2,
            or3,
            or4,
            or5,
            or6,
            or7,
        }
    }
}

impl Gate for Or8Way {
    fn re_compute(&self) -> () {
        self.or1.re_compute();
        self.or2.re_compute();
        self.or3.re_compute();
        self.or4.re_compute();
        self.or5.re_compute();
        self.or6.re_compute();
        self.or7.re_compute();
    }
}

#[derive(Debug)]
pub struct Mux4Way16 {
    pub out: SharedBus<16>,
    mux1: Mux<16>,
    mux2: Mux<16>,
    mux3: Mux<16>,
}

impl Mux4Way16 {
    pub fn new(
        a: SharedBus<16>,
        b: SharedBus<16>,
        c: SharedBus<16>,
        d: SharedBus<16>,
        sel: SharedBus<2>,
    ) -> Mux4Way16 {
        let sel0 = sel.reconnect([0]);
        let sel1 = sel.reconnect([1]);

        let mux1 = Mux::new(a.clone(), b.clone(), sel0.clone());
        let mux2 = Mux::new(c.clone(), d.clone(), sel0.clone());
        let mux3 = Mux::new(mux1.out.clone(), mux2.out.clone(), sel1.clone());

        Mux4Way16 {
            out: mux3.out.clone(),
            mux1,
            mux2,
            mux3,
        }
    }
}

impl Gate for Mux4Way16 {
    fn re_compute(&self) -> () {
        self.mux1.re_compute();
        self.mux2.re_compute();
        self.mux3.re_compute();
    }
}

#[derive(Debug)]
pub struct Mux8Way16 {
    pub out: SharedBus<16>,
    mux1: Mux4Way16,
    mux2: Mux4Way16,
    mux3: Mux<16>,
}

#[allow(dead_code)]
impl Mux8Way16 {
    pub fn new(
        a: SharedBus<16>,
        b: SharedBus<16>,
        c: SharedBus<16>,
        d: SharedBus<16>,
        e: SharedBus<16>,
        f: SharedBus<16>,
        g: SharedBus<16>,
        h: SharedBus<16>,
        sel: SharedBus<3>,
    ) -> Mux8Way16 {
        let sel01 = sel.reconnect([0, 1]);
        let sel2 = sel.reconnect([2]);

        let mux1 = Mux4Way16::new(a.clone(), b.clone(), c.clone(), d.clone(), sel01.clone());
        let mux2 = Mux4Way16::new(e.clone(), f.clone(), g.clone(), h.clone(), sel01.clone());
        let mux3 = Mux::new(mux1.out.clone(), mux2.out.clone(), sel2.clone());

        Mux8Way16 {
            out: mux3.out.clone(),
            mux1,
            mux2,
            mux3,
        }
    }
}

impl Gate for Mux8Way16 {
    fn re_compute(&self) -> () {
        self.mux1.re_compute();
        self.mux2.re_compute();
        self.mux3.re_compute();
    }
}

#[derive(Debug)]
pub struct DMux4Way {
    pub out1: SharedBus<1>,
    pub out2: SharedBus<1>,
    pub out3: SharedBus<1>,
    pub out4: SharedBus<1>,
    dmux1: DMux,
    dmux2: DMux,
    dmux3: DMux,
}

impl DMux4Way {
    pub fn new(input: SharedBus<1>, sel: SharedBus<2>) -> DMux4Way {
        let sel0 = sel.reconnect([0]);
        let sel1 = sel.reconnect([1]);

        let dmux1 = DMux::new(input.clone(), sel1.clone());
        let dmux2 = DMux::new(dmux1.out1.clone(), sel0.clone());
        let dmux3 = DMux::new(dmux1.out2.clone(), sel0.clone());

        DMux4Way {
            out1: dmux2.out1.clone(),
            out2: dmux2.out2.clone(),
            out3: dmux3.out1.clone(),
            out4: dmux3.out2.clone(),
            dmux1,
            dmux2,
            dmux3,
        }
    }
}

impl Gate for DMux4Way {
    fn re_compute(&self) -> () {
        self.dmux1.re_compute();
        self.dmux2.re_compute();
        self.dmux3.re_compute();
    }
}

#[derive(Debug)]
pub struct DMux8Way {
    pub out1: SharedBus<1>,
    pub out2: SharedBus<1>,
    pub out3: SharedBus<1>,
    pub out4: SharedBus<1>,
    pub out5: SharedBus<1>,
    pub out6: SharedBus<1>,
    pub out7: SharedBus<1>,
    pub out8: SharedBus<1>,
    dmux1: DMux,
    dmux2: DMux,
    dmux3: DMux,
    dmux4: DMux,
    dmux5: DMux,
    dmux6: DMux,
    dmux7: DMux,
}

#[allow(dead_code)]
impl DMux8Way {
    pub fn new(input: SharedBus<1>, sel: SharedBus<3>) -> DMux8Way {
        let sel0 = sel.reconnect([0]);
        let sel1 = sel.reconnect([1]);
        let sel2 = sel.reconnect([2]);

        let dmux1 = DMux::new(input.clone(), sel2.clone());

        let dmux2 = DMux::new(dmux1.out1.clone(), sel1.clone());
        let dmux3 = DMux::new(dmux1.out2.clone(), sel1.clone());

        let dmux4 = DMux::new(dmux2.out1.clone(), sel0.clone());
        let dmux5 = DMux::new(dmux2.out2.clone(), sel0.clone());
        let dmux6 = DMux::new(dmux3.out1.clone(), sel0.clone());
        let dmux7 = DMux::new(dmux3.out2.clone(), sel0.clone());

        DMux8Way {
            out1: dmux4.out1.clone(),
            out2: dmux4.out2.clone(),
            out3: dmux5.out1.clone(),
            out4: dmux5.out2.clone(),
            out5: dmux6.out1.clone(),
            out6: dmux6.out2.clone(),
            out7: dmux7.out1.clone(),
            out8: dmux7.out2.clone(),
            dmux1,
            dmux2,
            dmux3,
            dmux4,
            dmux5,
            dmux6,
            dmux7,
        }
    }
}

impl Gate for DMux8Way {
    fn re_compute(&self) -> () {
        self.dmux1.re_compute();
        self.dmux2.re_compute();
        self.dmux3.re_compute();
        self.dmux4.re_compute();
        self.dmux5.re_compute();
        self.dmux6.re_compute();
        self.dmux7.re_compute();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widen() {
        let original_bit = Rc::new(Cell::new(O));
        let original_bus: Bus<1> = Bus::new([original_bit.clone()]);
        let widen_bus: Bus<4> = original_bus.widen();

        assert_eq!(
            widen_bus.bits,
            [
                Rc::new(Cell::new(O)),
                Rc::new(Cell::new(O)),
                Rc::new(Cell::new(O)),
                Rc::new(Cell::new(O))
            ]
        );

        // change original_bit
        original_bit.set(I);

        // also changes widen_bus
        assert_eq!(
            widen_bus.bits,
            [
                Rc::new(Cell::new(I)),
                Rc::new(Cell::new(I)),
                Rc::new(Cell::new(I)),
                Rc::new(Cell::new(I))
            ]
        )
    }

    #[test]
    fn bus_from_str() {
        assert_eq!(
            "0".parse::<Bus<1>>().unwrap(),
            Bus::<1>::new([Rc::new(Cell::new(O))])
        );
        assert_eq!(
            "1".parse::<Bus<1>>().unwrap(),
            Bus::<1>::new([Rc::new(Cell::new(I))])
        );
        assert_eq!(
            "00".parse::<Bus<2>>().unwrap(),
            Bus::<2>::new([Rc::new(Cell::new(O)), Rc::new(Cell::new(O))])
        );
        assert_eq!(
            "01".parse::<Bus<2>>().unwrap(),
            Bus::<2>::new([Rc::new(Cell::new(I)), Rc::new(Cell::new(O))])
        );
        assert_eq!(
            "10".parse::<Bus<2>>().unwrap(),
            Bus::<2>::new([Rc::new(Cell::new(O)), Rc::new(Cell::new(I))])
        );
        assert_eq!(
            "11".parse::<Bus<2>>().unwrap(),
            Bus::<2>::new([Rc::new(Cell::new(I)), Rc::new(Cell::new(I))])
        );

        assert_eq!(
            "10101010".parse::<Bus<8>>().unwrap(),
            Bus::<8>::new([
                Rc::new(Cell::new(O)),
                Rc::new(Cell::new(I)),
                Rc::new(Cell::new(O)),
                Rc::new(Cell::new(I)),
                Rc::new(Cell::new(O)),
                Rc::new(Cell::new(I)),
                Rc::new(Cell::new(O)),
                Rc::new(Cell::new(I)),
            ])
        );
    }

    #[test]
    fn nand_re_compute() {
        let cases = vec![
            // a, b, out
            ["0", "0", "1"],
            ["1", "0", "1"],
            ["0", "1", "1"],
            ["1", "1", "0"],
        ];
        for case in cases {
            let a = case[0].parse::<Bus<1>>().unwrap().to_shared_bus();
            let b = case[1].parse::<Bus<1>>().unwrap().to_shared_bus();
            let out = case[2].parse::<Bus<1>>().unwrap().to_shared_bus();
            let nand = Nand::new(a, b);
            nand.re_compute();
            assert_eq!(nand.out, out);
        }
    }

    #[test]
    fn not_re_compute() {
        let cases = vec![
            // input, out
            ["0", "1"],
            ["1", "0"],
        ];
        for case in cases {
            let input = case[0].parse::<Bus<1>>().unwrap().to_shared_bus();
            let out = case[1].parse::<Bus<1>>().unwrap().to_shared_bus();
            let not = Not::new(input);
            not.re_compute();
            assert_eq!(not.out, out);
        }
    }

    #[test]
    fn not16_re_compute() {
        let cases = vec![
            // input, out
            ["0000000000000000", "1111111111111111"],
            ["1111111111111111", "0000000000000000"],
            ["1010101010101010", "0101010101010101"],
            ["0011110011000011", "1100001100111100"],
            ["0001001000110100", "1110110111001011"],
        ];
        for case in cases {
            let input = case[0].parse::<Bus<16>>().unwrap().to_shared_bus();
            let out = case[1].parse::<Bus<16>>().unwrap().to_shared_bus();
            let not = Not::new(input);
            not.re_compute();
            assert_eq!(not.out, out);
        }
    }

    #[test]
    fn and_re_compute() {
        let cases = vec![
            // a, b, out
            ["0", "0", "0"],
            ["1", "0", "0"],
            ["0", "1", "0"],
            ["1", "1", "1"],
        ];
        for case in cases {
            let a = case[0].parse::<Bus<1>>().unwrap().to_shared_bus();
            let b = case[1].parse::<Bus<1>>().unwrap().to_shared_bus();
            let out = case[2].parse::<Bus<1>>().unwrap().to_shared_bus();
            let and = And::new(a, b);
            and.re_compute();
            assert_eq!(and.out, out);
        }
    }

    #[test]
    fn and16_re_compute() {
        let cases = vec![
            // a, b, out
            ["0000000000000000", "0000000000000000", "0000000000000000"],
            ["0000000000000000", "1111111111111111", "0000000000000000"],
            ["1111111111111111", "1111111111111111", "1111111111111111"],
            ["1010101010101010", "0101010101010101", "0000000000000000"],
            ["0011110011000011", "0000111111110000", "0000110011000000"],
            ["0001001000110100", "1001100001110110", "0001000000110100"],
        ];
        for case in cases {
            let a = case[0].parse::<Bus<16>>().unwrap().to_shared_bus();
            let b = case[1].parse::<Bus<16>>().unwrap().to_shared_bus();
            let out = case[2].parse::<Bus<16>>().unwrap().to_shared_bus();
            let and = And::new(a, b);
            and.re_compute();
            assert_eq!(and.out, out);
        }
    }

    #[test]
    fn or_re_compute() {
        let cases = vec![
            // a, b, out
            ["0", "0", "0"],
            ["1", "0", "1"],
            ["0", "1", "1"],
            ["1", "1", "1"],
        ];
        for case in cases {
            let a = case[0].parse::<Bus<1>>().unwrap().to_shared_bus();
            let b = case[1].parse::<Bus<1>>().unwrap().to_shared_bus();
            let out = case[2].parse::<Bus<1>>().unwrap().to_shared_bus();
            let or = Or::new(a, b);
            or.re_compute();
            assert_eq!(or.out, out);
        }
    }

    #[test]
    fn or16_re_compute() {
        let cases = vec![
            // a, b, out
            ["0000000000000000", "0000000000000000", "0000000000000000"],
            ["0000000000000000", "1111111111111111", "1111111111111111"],
            ["1111111111111111", "1111111111111111", "1111111111111111"],
            ["1010101010101010", "0101010101010101", "1111111111111111"],
            ["0011110011000011", "0000111111110000", "0011111111110011"],
            ["0001001000110100", "1001100001110110", "1001101001110110"],
        ];
        for case in cases {
            let a = case[0].parse::<Bus<16>>().unwrap().to_shared_bus();
            let b = case[1].parse::<Bus<16>>().unwrap().to_shared_bus();
            let out = case[2].parse::<Bus<16>>().unwrap().to_shared_bus();
            let or = Or::new(a, b);
            or.re_compute();
            assert_eq!(or.out, out);
        }
    }

    #[test]
    fn xor_re_compute() {
        let cases = vec![
            // a, b, out
            ["0", "0", "0"],
            ["1", "0", "1"],
            ["0", "1", "1"],
            ["1", "1", "0"],
        ];
        for case in cases {
            let a = case[0].parse::<Bus<1>>().unwrap().to_shared_bus();
            let b = case[1].parse::<Bus<1>>().unwrap().to_shared_bus();
            let out = case[2].parse::<Bus<1>>().unwrap().to_shared_bus();
            let xor = Xor::new(a, b);
            xor.re_compute();
            assert_eq!(xor.out, out);
        }
    }

    #[test]
    fn mux_re_compute() {
        let cases = vec![
            // a, b, sel, out
            ["0", "0", "0", "0"],
            ["0", "1", "0", "0"],
            ["1", "0", "0", "1"],
            ["1", "1", "0", "1"],
            ["0", "0", "1", "0"],
            ["0", "1", "1", "1"],
            ["1", "0", "1", "0"],
            ["1", "1", "1", "1"],
        ];
        for case in cases {
            let a = case[0].parse::<Bus<1>>().unwrap().to_shared_bus();
            let b = case[1].parse::<Bus<1>>().unwrap().to_shared_bus();
            let sel = case[2].parse::<Bus<1>>().unwrap().to_shared_bus();
            let out = case[3].parse::<Bus<1>>().unwrap().to_shared_bus();
            let mux = Mux::new(a, b, sel);
            mux.re_compute();
            assert_eq!(mux.out, out);
        }
    }

    #[test]
    fn mux16_re_compute() {
        let cases = vec![
            // a, b, sel, out
            [
                "0000000000000000",
                "0000000000000000",
                "0",
                "0000000000000000",
            ],
            [
                "0000000000000000",
                "0000000000000000",
                "1",
                "0000000000000000",
            ],
            [
                "0000000000000000",
                "0001001000110100",
                "0",
                "0000000000000000",
            ],
            [
                "0000000000000000",
                "0001001000110100",
                "1",
                "0001001000110100",
            ],
            [
                "1001100001110110",
                "0000000000000000",
                "0",
                "1001100001110110",
            ],
            [
                "1001100001110110",
                "0000000000000000",
                "1",
                "0000000000000000",
            ],
            [
                "1010101010101010",
                "0101010101010101",
                "0",
                "1010101010101010",
            ],
            [
                "1010101010101010",
                "0101010101010101",
                "1",
                "0101010101010101",
            ],
        ];
        for case in cases {
            let a = case[0].parse::<Bus<16>>().unwrap().to_shared_bus();
            let b = case[1].parse::<Bus<16>>().unwrap().to_shared_bus();
            let sel = case[2].parse::<Bus<1>>().unwrap().to_shared_bus();
            let out = case[3].parse::<Bus<16>>().unwrap().to_shared_bus();
            let mux = Mux::new(a, b, sel);
            mux.re_compute();
            assert_eq!(mux.out, out);
        }
    }

    #[test]
    fn dmux_re_compute() {
        let cases = vec![
            // input sel, out1, out2
            ["0", "0", "0", "0"],
            ["0", "1", "0", "0"],
            ["1", "0", "1", "0"],
            ["1", "1", "0", "1"],
        ];
        for case in cases {
            let input = case[0].parse::<Bus<1>>().unwrap().to_shared_bus();
            let sel = case[1].parse::<Bus<1>>().unwrap().to_shared_bus();
            let out1 = case[2].parse::<Bus<1>>().unwrap().to_shared_bus();
            let out2 = case[3].parse::<Bus<1>>().unwrap().to_shared_bus();
            let dmux = DMux::new(input, sel);
            dmux.re_compute();
            assert_eq!(dmux.out1, out1);
            assert_eq!(dmux.out2, out2);
        }
    }

    #[test]
    fn or8way_re_compute() {
        let cases = vec![
            // input, out
            ["00000000", "0"],
            ["11111111", "1"],
            ["00010000", "1"],
            ["00000001", "1"],
            ["00100110", "1"],
        ];
        for case in cases {
            let input = case[0].parse::<Bus<8>>().unwrap().to_shared_bus();
            let out = case[1].parse::<Bus<1>>().unwrap().to_shared_bus();
            let or8way = Or8Way::new(input);
            or8way.re_compute();
            assert_eq!(or8way.out, out);
        }
    }

    #[test]
    fn mux4way16_re_compute() {
        let cases = vec![
            // a, b, c, d, sel, out
            [
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "00",
                "0000000000000000",
            ],
            [
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "01",
                "0000000000000000",
            ],
            [
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "10",
                "0000000000000000",
            ],
            [
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "11",
                "0000000000000000",
            ],
            [
                "0001001000110100",
                "1001100001110110",
                "1010101010101010",
                "0101010101010101",
                "00",
                "0001001000110100",
            ],
            [
                "0001001000110100",
                "1001100001110110",
                "1010101010101010",
                "0101010101010101",
                "01",
                "1001100001110110",
            ],
            [
                "0001001000110100",
                "1001100001110110",
                "1010101010101010",
                "0101010101010101",
                "10",
                "1010101010101010",
            ],
            [
                "0001001000110100",
                "1001100001110110",
                "1010101010101010",
                "0101010101010101",
                "11",
                "0101010101010101",
            ],
        ];
        for case in cases {
            let a = case[0].parse::<Bus<16>>().unwrap().to_shared_bus();
            let b = case[1].parse::<Bus<16>>().unwrap().to_shared_bus();
            let c = case[2].parse::<Bus<16>>().unwrap().to_shared_bus();
            let d = case[3].parse::<Bus<16>>().unwrap().to_shared_bus();
            let sel = case[4].parse::<Bus<2>>().unwrap().to_shared_bus();
            let out = case[5].parse::<Bus<16>>().unwrap().to_shared_bus();
            let mux4way16 = Mux4Way16::new(a, b, c, d, sel);
            mux4way16.re_compute();
            assert_eq!(mux4way16.out, out);
        }
    }

    #[test]
    fn mux8way16_re_compute() {
        let cases = vec![
            // a, b, c, d, e, f, g, h, sel, out
            [
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "000",
                "0000000000000000",
            ],
            [
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "001",
                "0000000000000000",
            ],
            [
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "010",
                "0000000000000000",
            ],
            [
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "011",
                "0000000000000000",
            ],
            [
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "100",
                "0000000000000000",
            ],
            [
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "101",
                "0000000000000000",
            ],
            [
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "110",
                "0000000000000000",
            ],
            [
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "0000000000000000",
                "111",
                "0000000000000000",
            ],
            [
                "0001001000110100",
                "0010001101000101",
                "0011010001010110",
                "0100010101100111",
                "0101011001111000",
                "0110011110001001",
                "0111100010011010",
                "1000100110101011",
                "000",
                "0001001000110100",
            ],
            [
                "0001001000110100",
                "0010001101000101",
                "0011010001010110",
                "0100010101100111",
                "0101011001111000",
                "0110011110001001",
                "0111100010011010",
                "1000100110101011",
                "001",
                "0010001101000101",
            ],
            [
                "0001001000110100",
                "0010001101000101",
                "0011010001010110",
                "0100010101100111",
                "0101011001111000",
                "0110011110001001",
                "0111100010011010",
                "1000100110101011",
                "010",
                "0011010001010110",
            ],
            [
                "0001001000110100",
                "0010001101000101",
                "0011010001010110",
                "0100010101100111",
                "0101011001111000",
                "0110011110001001",
                "0111100010011010",
                "1000100110101011",
                "011",
                "0100010101100111",
            ],
            [
                "0001001000110100",
                "0010001101000101",
                "0011010001010110",
                "0100010101100111",
                "0101011001111000",
                "0110011110001001",
                "0111100010011010",
                "1000100110101011",
                "100",
                "0101011001111000",
            ],
            [
                "0001001000110100",
                "0010001101000101",
                "0011010001010110",
                "0100010101100111",
                "0101011001111000",
                "0110011110001001",
                "0111100010011010",
                "1000100110101011",
                "101",
                "0110011110001001",
            ],
            [
                "0001001000110100",
                "0010001101000101",
                "0011010001010110",
                "0100010101100111",
                "0101011001111000",
                "0110011110001001",
                "0111100010011010",
                "1000100110101011",
                "110",
                "0111100010011010",
            ],
            [
                "0001001000110100",
                "0010001101000101",
                "0011010001010110",
                "0100010101100111",
                "0101011001111000",
                "0110011110001001",
                "0111100010011010",
                "1000100110101011",
                "111",
                "1000100110101011",
            ],
        ];
        for case in cases {
            let a = case[0].parse::<Bus<16>>().unwrap().to_shared_bus();
            let b = case[1].parse::<Bus<16>>().unwrap().to_shared_bus();
            let c = case[2].parse::<Bus<16>>().unwrap().to_shared_bus();
            let d = case[3].parse::<Bus<16>>().unwrap().to_shared_bus();
            let e = case[4].parse::<Bus<16>>().unwrap().to_shared_bus();
            let f = case[5].parse::<Bus<16>>().unwrap().to_shared_bus();
            let g = case[6].parse::<Bus<16>>().unwrap().to_shared_bus();
            let h = case[7].parse::<Bus<16>>().unwrap().to_shared_bus();
            let sel = case[8].parse::<Bus<3>>().unwrap().to_shared_bus();
            let out = case[9].parse::<Bus<16>>().unwrap().to_shared_bus();
            let mux8way16 = Mux8Way16::new(a, b, c, d, e, f, g, h, sel);
            mux8way16.re_compute();
            assert_eq!(mux8way16.out, out);
        }
    }

    #[test]
    fn dmux4way_re_compute() {
        let cases = vec![
            // input sel, out1, out2, out3, out4
            ["0", "00", "0", "0", "0", "0"],
            ["0", "01", "0", "0", "0", "0"],
            ["0", "10", "0", "0", "0", "0"],
            ["0", "11", "0", "0", "0", "0"],
            ["1", "00", "1", "0", "0", "0"],
            ["1", "01", "0", "1", "0", "0"],
            ["1", "10", "0", "0", "1", "0"],
            ["1", "11", "0", "0", "0", "1"],
        ];
        for case in cases {
            let input = case[0].parse::<Bus<1>>().unwrap().to_shared_bus();
            let sel = case[1].parse::<Bus<2>>().unwrap().to_shared_bus();
            let out1 = case[2].parse::<Bus<1>>().unwrap().to_shared_bus();
            let out2 = case[3].parse::<Bus<1>>().unwrap().to_shared_bus();
            let out3 = case[4].parse::<Bus<1>>().unwrap().to_shared_bus();
            let out4 = case[5].parse::<Bus<1>>().unwrap().to_shared_bus();
            let dmux4way = DMux4Way::new(input, sel);
            dmux4way.re_compute();
            assert_eq!(dmux4way.out1, out1);
            assert_eq!(dmux4way.out2, out2);
            assert_eq!(dmux4way.out3, out3);
            assert_eq!(dmux4way.out4, out4);
        }
    }

    #[test]
    fn dmux8way_re_compute() {
        let cases = vec![
            // input sel, out1, out2, out3, out4
            ["0", "000", "0", "0", "0", "0", "0", "0", "0", "0"],
            ["0", "001", "0", "0", "0", "0", "0", "0", "0", "0"],
            ["0", "010", "0", "0", "0", "0", "0", "0", "0", "0"],
            ["0", "011", "0", "0", "0", "0", "0", "0", "0", "0"],
            ["0", "100", "0", "0", "0", "0", "0", "0", "0", "0"],
            ["0", "101", "0", "0", "0", "0", "0", "0", "0", "0"],
            ["0", "110", "0", "0", "0", "0", "0", "0", "0", "0"],
            ["0", "111", "0", "0", "0", "0", "0", "0", "0", "0"],
            ["1", "000", "1", "0", "0", "0", "0", "0", "0", "0"],
            ["1", "001", "0", "1", "0", "0", "0", "0", "0", "0"],
            ["1", "010", "0", "0", "1", "0", "0", "0", "0", "0"],
            ["1", "011", "0", "0", "0", "1", "0", "0", "0", "0"],
            ["1", "100", "0", "0", "0", "0", "1", "0", "0", "0"],
            ["1", "101", "0", "0", "0", "0", "0", "1", "0", "0"],
            ["1", "110", "0", "0", "0", "0", "0", "0", "1", "0"],
            ["1", "111", "0", "0", "0", "0", "0", "0", "0", "1"],
        ];
        for case in cases {
            let input = case[0].parse::<Bus<1>>().unwrap().to_shared_bus();
            let sel = case[1].parse::<Bus<3>>().unwrap().to_shared_bus();
            let out1 = case[2].parse::<Bus<1>>().unwrap().to_shared_bus();
            let out2 = case[3].parse::<Bus<1>>().unwrap().to_shared_bus();
            let out3 = case[4].parse::<Bus<1>>().unwrap().to_shared_bus();
            let out4 = case[5].parse::<Bus<1>>().unwrap().to_shared_bus();
            let out5 = case[6].parse::<Bus<1>>().unwrap().to_shared_bus();
            let out6 = case[7].parse::<Bus<1>>().unwrap().to_shared_bus();
            let out7 = case[8].parse::<Bus<1>>().unwrap().to_shared_bus();
            let out8 = case[9].parse::<Bus<1>>().unwrap().to_shared_bus();
            let dmux8way = DMux8Way::new(input, sel);
            dmux8way.re_compute();
            assert_eq!(dmux8way.out1, out1);
            assert_eq!(dmux8way.out2, out2);
            assert_eq!(dmux8way.out3, out3);
            assert_eq!(dmux8way.out4, out4);
            assert_eq!(dmux8way.out5, out5);
            assert_eq!(dmux8way.out6, out6);
            assert_eq!(dmux8way.out7, out7);
            assert_eq!(dmux8way.out8, out8);
        }
    }
}
