use computer::{Computer, ROM32KBuiltIn};
use gate::Bus;

mod arithmetic;
mod computer;
mod gate;
mod sequential;

fn main() {
    let address = Bus::<15>::all0().to_shared_bus();
    // 2 + 3 = 5 のコード
    let code = "0000000000000010
                      1110110000010000
                      0000000000000011
                      1110000010010000
                      0000000000000000
                      1110001100001000";
    let rom = ROM32KBuiltIn::from_rom_str(code, address);
    let reset = Bus::<1>::all0().to_shared_bus();
    // Computerを作成
    let computer = Computer::new(reset, rom);

    // 初期状態を表示
    print_computer_status(&computer);

    // 6サイクル回しつつ状態を表示
    for _ in 0..6 {
        computer.tick();
        computer.tock();
        print_computer_status(&computer);
    }
    // 実行結果
    // r0: 0, A: 0, D: 0, PC: 0
    // r0: 0, A: 2, D: 0, PC: 1
    // r0: 0, A: 2, D: 2, PC: 2
    // r0: 0, A: 3, D: 2, PC: 3
    // r0: 0, A: 3, D: 5, PC: 4
    // r0: 0, A: 0, D: 5, PC: 5
    // r0: 5, A: 0, D: 5, PC: 6
}

fn print_computer_status(computer: &Computer) -> () {
    println!(
        "r0: {}, A: {}, D: {}, PC: {}",
        computer.get_r0(),
        computer.cpu.get_a_register_value(),
        computer.cpu.get_d_register_value(),
        computer.cpu.pc.to_u16(),
    );
}
