use za_disasm;
use zae_a2600::{cpu::Cpu, memory::Memory};

fn main() {
    let rom = include_bytes!("../../roms/adventure.bin");
    println!("ROM size: {}", rom.len());

    let mut mem = Memory::new();
    mem.load_rom(rom);

    let mut cpu = Cpu::new();
    cpu.reset(&mut mem);
    println!("PC after reset: 0x{:04X}\n", cpu.pc);

    let mut out = [0u8; 32];

    for i in 0..200_000 {
        let pc = cpu.pc;

        if i % 5000 == 0 {
            let offset = (pc & 0x0FFF) as usize;
            let code = &mem.rom[offset..(offset + 3).min(mem.rom.len())];
            let ins = za_disasm::decode(code, pc);
            let n = za_disasm::format(&mut out, &ins);
            let text = core::str::from_utf8(&out[..n]).unwrap_or("???");

            println!(
                "[{:>7}] PC=0x{:04X}  A=0x{:02X} X=0x{:02X} Y=0x{:02X}  | {:<16} | TIA: bg={} pf={} ctrl=0x{:02X}  PF: 0x{:02X}/0x{:02X}/0x{:02X}",
                i, pc, cpu.a, cpu.x, cpu.y, text,
                mem.tia.colubk, mem.tia.colupf, mem.tia.ctrlpf,
                mem.tia.pf0, mem.tia.pf1, mem.tia.pf2
            );
        }
        cpu.step(&mut mem);
    }

    println!("\n--- TIA في النهاية ---");
    println!(
        "colubk={} colupf={} ctrlpf={} pf0=0x{:02X} pf1=0x{:02X} pf2=0x{:02X}",
        mem.tia.colubk, mem.tia.colupf, mem.tia.ctrlpf, mem.tia.pf0, mem.tia.pf1, mem.tia.pf2
    );
}
