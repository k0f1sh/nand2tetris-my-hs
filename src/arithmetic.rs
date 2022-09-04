use crate::gate::*;

#[derive(Debug)]
pub struct HalfAdder {
    pub sum: SharedBus<1>,
    pub carry: SharedBus<1>,
    and: And<1>,
    xor: Xor<1>,
}

impl HalfAdder {
    pub fn new(a: SharedBus<1>, b: SharedBus<1>) -> HalfAdder {
        let and = And::new(a.clone(), b.clone());
        let xor = Xor::new(a.clone(), b.clone());
        HalfAdder {
            sum: xor.out.clone(),
            carry: and.out.clone(),
            and,
            xor,
        }
    }
}

impl Gate for HalfAdder {
    fn re_compute(&self) -> () {
        self.and.re_compute();
        self.xor.re_compute();
    }
}

#[derive(Debug)]
pub struct FullAdder {
    pub sum: SharedBus<1>,
    pub carry: SharedBus<1>,
    half_adder1: HalfAdder,
    half_adder2: HalfAdder,
    or: Or<1>,
}

impl FullAdder {
    pub fn new(a: SharedBus<1>, b: SharedBus<1>, c: SharedBus<1>) -> FullAdder {
        let half_adder1 = HalfAdder::new(a.clone(), b.clone());
        let half_adder2 = HalfAdder::new(half_adder1.sum.clone(), c.clone());
        let or = Or::new(half_adder1.carry.clone(), half_adder2.carry.clone());
        FullAdder {
            sum: half_adder2.sum.clone(),
            carry: or.out.clone(),
            half_adder1,
            half_adder2,
            or,
        }
    }
}

impl Gate for FullAdder {
    fn re_compute(&self) -> () {
        self.half_adder1.re_compute();
        self.half_adder2.re_compute();
        self.or.re_compute();
    }
}

#[derive(Debug)]
pub struct Add16 {
    pub out: SharedBus<16>,
    half_adder: HalfAdder,
    full_adder1: FullAdder,
    full_adder2: FullAdder,
    full_adder3: FullAdder,
    full_adder4: FullAdder,
    full_adder5: FullAdder,
    full_adder6: FullAdder,
    full_adder7: FullAdder,
    full_adder8: FullAdder,
    full_adder9: FullAdder,
    full_adder10: FullAdder,
    full_adder11: FullAdder,
    full_adder12: FullAdder,
    full_adder13: FullAdder,
    full_adder14: FullAdder,
    full_adder15: FullAdder,
}

impl Add16 {
    pub fn new(a: SharedBus<16>, b: SharedBus<16>) -> Add16 {
        // FIXME もうちょっとスッキリ書きたい
        let a0 = a.reconnect([0]);
        let a1 = a.reconnect([1]);
        let a2 = a.reconnect([2]);
        let a3 = a.reconnect([3]);
        let a4 = a.reconnect([4]);
        let a5 = a.reconnect([5]);
        let a6 = a.reconnect([6]);
        let a7 = a.reconnect([7]);
        let a8 = a.reconnect([8]);
        let a9 = a.reconnect([9]);
        let a10 = a.reconnect([10]);
        let a11 = a.reconnect([11]);
        let a12 = a.reconnect([12]);
        let a13 = a.reconnect([13]);
        let a14 = a.reconnect([14]);
        let a15 = a.reconnect([15]);

        let b0 = b.reconnect([0]);
        let b1 = b.reconnect([1]);
        let b2 = b.reconnect([2]);
        let b3 = b.reconnect([3]);
        let b4 = b.reconnect([4]);
        let b5 = b.reconnect([5]);
        let b6 = b.reconnect([6]);
        let b7 = b.reconnect([7]);
        let b8 = b.reconnect([8]);
        let b9 = b.reconnect([9]);
        let b10 = b.reconnect([10]);
        let b11 = b.reconnect([11]);
        let b12 = b.reconnect([12]);
        let b13 = b.reconnect([13]);
        let b14 = b.reconnect([14]);
        let b15 = b.reconnect([15]);

        let half_adder = HalfAdder::new(a0, b0);
        let full_adder1 = FullAdder::new(a1, b1, half_adder.carry.clone());
        let full_adder2 = FullAdder::new(a2, b2, full_adder1.carry.clone());
        let full_adder3 = FullAdder::new(a3, b3, full_adder2.carry.clone());
        let full_adder4 = FullAdder::new(a4, b4, full_adder3.carry.clone());
        let full_adder5 = FullAdder::new(a5, b5, full_adder4.carry.clone());
        let full_adder6 = FullAdder::new(a6, b6, full_adder5.carry.clone());
        let full_adder7 = FullAdder::new(a7, b7, full_adder6.carry.clone());
        let full_adder8 = FullAdder::new(a8, b8, full_adder7.carry.clone());
        let full_adder9 = FullAdder::new(a9, b9, full_adder8.carry.clone());
        let full_adder10 = FullAdder::new(a10, b10, full_adder9.carry.clone());
        let full_adder11 = FullAdder::new(a11, b11, full_adder10.carry.clone());
        let full_adder12 = FullAdder::new(a12, b12, full_adder11.carry.clone());
        let full_adder13 = FullAdder::new(a13, b13, full_adder12.carry.clone());
        let full_adder14 = FullAdder::new(a14, b14, full_adder13.carry.clone());
        let full_adder15 = FullAdder::new(a15, b15, full_adder14.carry.clone());

        let out = Bus::new([
            half_adder.sum.get_shared_bit(0),
            full_adder1.sum.get_shared_bit(0),
            full_adder2.sum.get_shared_bit(0),
            full_adder3.sum.get_shared_bit(0),
            full_adder4.sum.get_shared_bit(0),
            full_adder5.sum.get_shared_bit(0),
            full_adder6.sum.get_shared_bit(0),
            full_adder7.sum.get_shared_bit(0),
            full_adder8.sum.get_shared_bit(0),
            full_adder9.sum.get_shared_bit(0),
            full_adder10.sum.get_shared_bit(0),
            full_adder11.sum.get_shared_bit(0),
            full_adder12.sum.get_shared_bit(0),
            full_adder13.sum.get_shared_bit(0),
            full_adder14.sum.get_shared_bit(0),
            full_adder15.sum.get_shared_bit(0),
        ])
        .to_shared_bus();

        Add16 {
            out,
            half_adder,
            full_adder1,
            full_adder2,
            full_adder3,
            full_adder4,
            full_adder5,
            full_adder6,
            full_adder7,
            full_adder8,
            full_adder9,
            full_adder10,
            full_adder11,
            full_adder12,
            full_adder13,
            full_adder14,
            full_adder15,
        }
    }
}

impl Gate for Add16 {
    fn re_compute(&self) -> () {
        self.half_adder.re_compute();
        self.full_adder1.re_compute();
        self.full_adder2.re_compute();
        self.full_adder3.re_compute();
        self.full_adder4.re_compute();
        self.full_adder5.re_compute();
        self.full_adder6.re_compute();
        self.full_adder7.re_compute();
        self.full_adder8.re_compute();
        self.full_adder9.re_compute();
        self.full_adder10.re_compute();
        self.full_adder11.re_compute();
        self.full_adder12.re_compute();
        self.full_adder13.re_compute();
        self.full_adder14.re_compute();
        self.full_adder15.re_compute();
    }
}

#[derive(Debug)]
pub struct Inc16 {
    pub out: SharedBus<16>,
    add16: Add16,
}

impl Inc16 {
    pub fn new(input: SharedBus<16>) -> Inc16 {
        let all0 = Bus::<1>::all0().to_shared_bus();
        let all1 = Bus::<1>::all1().to_shared_bus();
        let one = Bus::new([
            all1.get_shared_bit(0), // ここだけ1
            all0.get_shared_bit(0),
            all0.get_shared_bit(0),
            all0.get_shared_bit(0),
            all0.get_shared_bit(0),
            all0.get_shared_bit(0),
            all0.get_shared_bit(0),
            all0.get_shared_bit(0),
            all0.get_shared_bit(0),
            all0.get_shared_bit(0),
            all0.get_shared_bit(0),
            all0.get_shared_bit(0),
            all0.get_shared_bit(0),
            all0.get_shared_bit(0),
            all0.get_shared_bit(0),
            all0.get_shared_bit(0),
        ])
        .to_shared_bus();

        let add16 = Add16::new(input.clone(), one.clone());

        Inc16 {
            out: add16.out.clone(),
            add16,
        }
    }
}

impl Gate for Inc16 {
    fn re_compute(&self) -> () {
        self.add16.re_compute();
    }
}

#[derive(Debug)]
pub struct ALU {
    pub out: SharedBus<16>,
    pub zr: SharedBus<1>,
    pub ng: SharedBus<1>,
    mux1: Mux<16>,
    mux2: Mux<16>,
    not1: Not<16>,
    mux3: Mux<16>,
    not2: Not<16>,
    mux4: Mux<16>,
    and1: And<16>,
    add1: Add16,
    mux5: Mux<16>,
    not3: Not<16>,
    mux6: Mux<16>,
    or8way1: Or8Way,
    or8way2: Or8Way,
    or1: Or<1>,
    not4: Not<1>,
}

impl ALU {
    pub fn new(
        x: SharedBus<16>,
        y: SharedBus<16>,
        zx: SharedBus<1>,
        nx: SharedBus<1>,
        zy: SharedBus<1>,
        ny: SharedBus<1>,
        f: SharedBus<1>,
        no: SharedBus<1>,
    ) -> ALU {
        let mux1 = Mux::new(x.clone(), Bus::all0().to_shared_bus(), zx.clone());
        let mux2 = Mux::new(y.clone(), Bus::all0().to_shared_bus(), zy.clone());

        let not1 = Not::new(mux1.out.clone());
        let mux3 = Mux::new(mux1.out.clone(), not1.out.clone(), nx.clone());

        let not2 = Not::new(mux2.out.clone());
        let mux4 = Mux::new(mux2.out.clone(), not2.out.clone(), ny.clone());

        let and1 = And::new(mux3.out.clone(), mux4.out.clone());
        let add1 = Add16::new(mux3.out.clone(), mux4.out.clone());
        let mux5 = Mux::new(and1.out.clone(), add1.out.clone(), f.clone());

        let not3 = Not::new(mux5.out.clone());
        let mux6 = Mux::new(mux5.out.clone(), not3.out.clone(), no.clone());

        let or8way1 = Or8Way::new(mux6.out.reconnect([0, 1, 2, 3, 4, 5, 6, 7]));
        let or8way2 = Or8Way::new(mux6.out.reconnect([8, 9, 10, 11, 12, 13, 14, 15]));
        let or1 = Or::new(or8way1.out.clone(), or8way2.out.clone());
        let not4 = Not::new(or1.out.clone());

        let ng = Bus::new([mux6.out.get_shared_bit(15)]).to_shared_bus();

        ALU {
            out: mux6.out.clone(),
            zr: not4.out.clone(),
            ng: ng.clone(),
            mux1,
            mux2,
            not1,
            mux3,
            not2,
            mux4,
            and1,
            add1,
            mux5,
            not3,
            mux6,
            or8way1,
            or8way2,
            or1,
            not4,
        }
    }
}

impl Gate for ALU {
    fn re_compute(&self) -> () {
        self.mux1.re_compute();
        self.mux2.re_compute();
        self.not1.re_compute();
        self.mux3.re_compute();
        self.not2.re_compute();
        self.mux4.re_compute();
        self.and1.re_compute();
        self.add1.re_compute();
        self.mux5.re_compute();
        self.not3.re_compute();
        self.mux6.re_compute();
        self.or8way1.re_compute();
        self.or8way2.re_compute();
        self.or1.re_compute();
        self.not4.re_compute();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn half_adder_re_compute() {
        let cases = vec![
            // a, b, sum, carry
            ["0", "0", "0", "0"],
            ["0", "1", "1", "0"],
            ["1", "0", "1", "0"],
            ["1", "1", "0", "1"],
        ];
        for case in cases {
            let a = case[0].parse::<Bus<1>>().unwrap().to_shared_bus();
            let b = case[1].parse::<Bus<1>>().unwrap().to_shared_bus();
            let sum = case[2].parse::<Bus<1>>().unwrap().to_shared_bus();
            let carry = case[3].parse::<Bus<1>>().unwrap().to_shared_bus();

            let half_adder = HalfAdder::new(a, b);

            half_adder.re_compute();
            assert_eq!(half_adder.carry, carry);
            assert_eq!(half_adder.sum, sum);
        }
    }

    #[test]
    fn full_adder_re_compute() {
        let cases = vec![
            // a, b, c, sum carry
            ["0", "0", "0", "0", "0"],
            ["0", "0", "1", "1", "0"],
            ["0", "1", "0", "1", "0"],
            ["0", "1", "1", "0", "1"],
            ["1", "0", "0", "1", "0"],
            ["1", "0", "1", "0", "1"],
            ["1", "1", "0", "0", "1"],
            ["1", "1", "1", "1", "1"],
        ];
        for case in cases {
            let a = case[0].parse::<Bus<1>>().unwrap().to_shared_bus();
            let b = case[1].parse::<Bus<1>>().unwrap().to_shared_bus();
            let c = case[2].parse::<Bus<1>>().unwrap().to_shared_bus();
            let sum = case[3].parse::<Bus<1>>().unwrap().to_shared_bus();
            let carry = case[4].parse::<Bus<1>>().unwrap().to_shared_bus();

            let full_adder = FullAdder::new(a, b, c);

            full_adder.re_compute();
            assert_eq!(full_adder.sum, sum);
            assert_eq!(full_adder.carry, carry);
        }
    }

    #[test]
    fn add16_re_compute() {
        let cases = vec![
            // a, b, out
            ["0000000000000000", "0000000000000000", "0000000000000000"],
            ["0000000000000000", "1111111111111111", "1111111111111111"],
            ["1111111111111111", "1111111111111111", "1111111111111110"],
            ["1010101010101010", "0101010101010101", "1111111111111111"],
            ["0011110011000011", "0000111111110000", "0100110010110011"],
            ["0001001000110100", "1001100001110110", "1010101010101010"],
        ];
        for case in cases {
            let a = case[0].parse::<Bus<16>>().unwrap().to_shared_bus();
            let b = case[1].parse::<Bus<16>>().unwrap().to_shared_bus();
            let out = case[2].parse::<Bus<16>>().unwrap().to_shared_bus();

            let add16 = Add16::new(a, b);
            add16.re_compute();
            assert_eq!(add16.out, out);
        }
    }

    #[test]
    fn inc16_re_compute() {
        let cases = vec![
            // input, out
            ["0000000000000000", "0000000000000001"],
            ["1111111111111111", "0000000000000000"],
            ["0000000000000101", "0000000000000110"],
            ["1111111111111011", "1111111111111100"],
        ];
        for case in cases {
            let input = case[0].parse::<Bus<16>>().unwrap().to_shared_bus();
            let out = case[1].parse::<Bus<16>>().unwrap().to_shared_bus();

            let inc16 = Inc16::new(input);
            inc16.re_compute();
            assert_eq!(inc16.out, out);
        }
    }

    #[test]
    fn alu_re_compute() {
        let cases = vec![
            // x, y, zx, nx, zy, ny, f, no, out, zr, ng
            [
                "0000000000000000",
                "1111111111111111",
                "1",
                "0",
                "1",
                "0",
                "1",
                "0",
                "0000000000000000",
                "1",
                "0",
            ],
            [
                "0000000000000000",
                "1111111111111111",
                "1",
                "1",
                "1",
                "1",
                "1",
                "1",
                "0000000000000001",
                "0",
                "0",
            ],
            [
                "0000000000000000",
                "1111111111111111",
                "1",
                "1",
                "1",
                "0",
                "1",
                "0",
                "1111111111111111",
                "0",
                "1",
            ],
            [
                "0000000000000000",
                "1111111111111111",
                "0",
                "0",
                "1",
                "1",
                "0",
                "0",
                "0000000000000000",
                "1",
                "0",
            ],
            [
                "0000000000000000",
                "1111111111111111",
                "1",
                "1",
                "0",
                "0",
                "0",
                "0",
                "1111111111111111",
                "0",
                "1",
            ],
            [
                "0000000000000000",
                "1111111111111111",
                "0",
                "0",
                "1",
                "1",
                "0",
                "1",
                "1111111111111111",
                "0",
                "1",
            ],
            [
                "0000000000000000",
                "1111111111111111",
                "1",
                "1",
                "0",
                "0",
                "0",
                "1",
                "0000000000000000",
                "1",
                "0",
            ],
            [
                "0000000000000000",
                "1111111111111111",
                "0",
                "0",
                "1",
                "1",
                "1",
                "1",
                "0000000000000000",
                "1",
                "0",
            ],
            [
                "0000000000000000",
                "1111111111111111",
                "1",
                "1",
                "0",
                "0",
                "1",
                "1",
                "0000000000000001",
                "0",
                "0",
            ],
            [
                "0000000000000000",
                "1111111111111111",
                "0",
                "1",
                "1",
                "1",
                "1",
                "1",
                "0000000000000001",
                "0",
                "0",
            ],
            [
                "0000000000000000",
                "1111111111111111",
                "1",
                "1",
                "0",
                "1",
                "1",
                "1",
                "0000000000000000",
                "1",
                "0",
            ],
            [
                "0000000000000000",
                "1111111111111111",
                "0",
                "0",
                "1",
                "1",
                "1",
                "0",
                "1111111111111111",
                "0",
                "1",
            ],
            [
                "0000000000000000",
                "1111111111111111",
                "1",
                "1",
                "0",
                "0",
                "1",
                "0",
                "1111111111111110",
                "0",
                "1",
            ],
            [
                "0000000000000000",
                "1111111111111111",
                "0",
                "0",
                "0",
                "0",
                "1",
                "0",
                "1111111111111111",
                "0",
                "1",
            ],
            [
                "0000000000000000",
                "1111111111111111",
                "0",
                "1",
                "0",
                "0",
                "1",
                "1",
                "0000000000000001",
                "0",
                "0",
            ],
            [
                "0000000000000000",
                "1111111111111111",
                "0",
                "0",
                "0",
                "1",
                "1",
                "1",
                "1111111111111111",
                "0",
                "1",
            ],
            [
                "0000000000000000",
                "1111111111111111",
                "0",
                "0",
                "0",
                "0",
                "0",
                "0",
                "0000000000000000",
                "1",
                "0",
            ],
            [
                "0000000000000000",
                "1111111111111111",
                "0",
                "1",
                "0",
                "1",
                "0",
                "1",
                "1111111111111111",
                "0",
                "1",
            ],
            [
                "0000000000010001",
                "0000000000000011",
                "1",
                "0",
                "1",
                "0",
                "1",
                "0",
                "0000000000000000",
                "1",
                "0",
            ],
            [
                "0000000000010001",
                "0000000000000011",
                "1",
                "1",
                "1",
                "1",
                "1",
                "1",
                "0000000000000001",
                "0",
                "0",
            ],
            [
                "0000000000010001",
                "0000000000000011",
                "1",
                "1",
                "1",
                "0",
                "1",
                "0",
                "1111111111111111",
                "0",
                "1",
            ],
            [
                "0000000000010001",
                "0000000000000011",
                "0",
                "0",
                "1",
                "1",
                "0",
                "0",
                "0000000000010001",
                "0",
                "0",
            ],
            [
                "0000000000010001",
                "0000000000000011",
                "1",
                "1",
                "0",
                "0",
                "0",
                "0",
                "0000000000000011",
                "0",
                "0",
            ],
            [
                "0000000000010001",
                "0000000000000011",
                "0",
                "0",
                "1",
                "1",
                "0",
                "1",
                "1111111111101110",
                "0",
                "1",
            ],
            [
                "0000000000010001",
                "0000000000000011",
                "1",
                "1",
                "0",
                "0",
                "0",
                "1",
                "1111111111111100",
                "0",
                "1",
            ],
            [
                "0000000000010001",
                "0000000000000011",
                "0",
                "0",
                "1",
                "1",
                "1",
                "1",
                "1111111111101111",
                "0",
                "1",
            ],
            [
                "0000000000010001",
                "0000000000000011",
                "1",
                "1",
                "0",
                "0",
                "1",
                "1",
                "1111111111111101",
                "0",
                "1",
            ],
            [
                "0000000000010001",
                "0000000000000011",
                "0",
                "1",
                "1",
                "1",
                "1",
                "1",
                "0000000000010010",
                "0",
                "0",
            ],
            [
                "0000000000010001",
                "0000000000000011",
                "1",
                "1",
                "0",
                "1",
                "1",
                "1",
                "0000000000000100",
                "0",
                "0",
            ],
            [
                "0000000000010001",
                "0000000000000011",
                "0",
                "0",
                "1",
                "1",
                "1",
                "0",
                "0000000000010000",
                "0",
                "0",
            ],
            [
                "0000000000010001",
                "0000000000000011",
                "1",
                "1",
                "0",
                "0",
                "1",
                "0",
                "0000000000000010",
                "0",
                "0",
            ],
            [
                "0000000000010001",
                "0000000000000011",
                "0",
                "0",
                "0",
                "0",
                "1",
                "0",
                "0000000000010100",
                "0",
                "0",
            ],
            [
                "0000000000010001",
                "0000000000000011",
                "0",
                "1",
                "0",
                "0",
                "1",
                "1",
                "0000000000001110",
                "0",
                "0",
            ],
            [
                "0000000000010001",
                "0000000000000011",
                "0",
                "0",
                "0",
                "1",
                "1",
                "1",
                "1111111111110010",
                "0",
                "1",
            ],
            [
                "0000000000010001",
                "0000000000000011",
                "0",
                "0",
                "0",
                "0",
                "0",
                "0",
                "0000000000000001",
                "0",
                "0",
            ],
            [
                "0000000000010001",
                "0000000000000011",
                "0",
                "1",
                "0",
                "1",
                "0",
                "1",
                "0000000000010011",
                "0",
                "0",
            ],
        ];
        for case in cases {
            let x = case[0].parse::<Bus<16>>().unwrap().to_shared_bus();
            let y = case[1].parse::<Bus<16>>().unwrap().to_shared_bus();
            let zx = case[2].parse::<Bus<1>>().unwrap().to_shared_bus();
            let nx = case[3].parse::<Bus<1>>().unwrap().to_shared_bus();
            let zy = case[4].parse::<Bus<1>>().unwrap().to_shared_bus();
            let ny = case[5].parse::<Bus<1>>().unwrap().to_shared_bus();
            let f = case[6].parse::<Bus<1>>().unwrap().to_shared_bus();
            let no = case[7].parse::<Bus<1>>().unwrap().to_shared_bus();

            let out = case[8].parse::<Bus<16>>().unwrap().to_shared_bus();
            let zr = case[9].parse::<Bus<1>>().unwrap().to_shared_bus();
            let ng = case[10].parse::<Bus<1>>().unwrap().to_shared_bus();

            let alu = ALU::new(x, y, zx, nx, zy, ny, f, no);
            alu.re_compute();
            assert_eq!(alu.out, out);
            assert_eq!(alu.zr, zr);
            assert_eq!(alu.ng, ng);
        }
    }
}
