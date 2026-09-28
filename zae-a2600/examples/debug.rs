use za_disasm;
use zae_a2600::{cpu::Cpu, memory::Memory};

fn main() {
    let rom = include_bytes!("../../roms/adventure.bin");
    let mut mem = Memory::new();
    mem.load_rom(rom);

    let mut cpu = Cpu::new();
    cpu.reset(&mut mem);
    println!("PC after reset: 0x{:04X}\n", cpu.pc);

    let mut out = [0u8; 32];

    for i in 0..100_000 {
        let pc = cpu.pc;

        // اطبع كل 5000 خطوة، مع المؤقت
        if i % 5000 == 0 {
            let offset = (pc & 0x0FFF) as usize;
            let code = &mem.rom[offset..(offset + 3).min(mem.rom.len())];
            let ins = za_disasm::decode(code, pc);
            let n = za_disasm::format(&mut out, &ins);
            let text = core::str::from_utf8(&out[..n]).unwrap_or("???");

            println!(
                "[{:>6}] PC=0x{:04X}  {} | TIA: bg={} pf={} | RIOT: t={} presc={}/{} writes={}",
                i,
                pc,
                text,
                mem.tia.colubk,
                mem.tia.colupf,
                mem.riot_timer,
                mem.riot_prescaler,
                mem.riot_prescaler_value,
                mem.tim64t_writes,
            );
        }
        cpu.step(&mut mem);
    }

    println!("\n--- TIA Final ---");
    println!(
        "colubk={} colupf={} pf0={:02X}",
        mem.tia.colubk, mem.tia.colupf, mem.tia.pf0
    );
    println!("--- RIOT Final ---");
    println!(
        "timer={} prescaler={} presc_val={}",
        mem.riot_timer, mem.riot_prescaler, mem.riot_prescaler_value
    );
}
