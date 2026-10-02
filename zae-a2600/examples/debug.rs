use za_disasm;
use zae_a2600::{cpu::Cpu, memory::Memory};

fn main() {
    let rom = include_bytes!("../../roms/adventure.bin");
    let mut mem = Memory::new();
    mem.load_rom(rom);

    let mut cpu = Cpu::new();
    cpu.reset(&mut mem);
    println!("PC after reset: 0x{:04X}", cpu.pc);

    let mut out = [0u8; 32];

    // المرحلة 1: نشغّل 60 إطاراً بدون إدخال (attract mode)
    println!("\n=== Phase 1: No input (attract mode) ===");
    for frame in 0..60 {
        cpu.run_frame(&mut mem);
        if frame % 10 == 0 {
            let offset = (cpu.pc & 0x0FFF) as usize;
            let code = &mem.rom[offset..(offset + 3).min(mem.rom.len())];
            let ins = za_disasm::decode(code, cpu.pc);
            let n = za_disasm::format(&mut out, &ins);
            let text = core::str::from_utf8(&out[..n]).unwrap_or("???");

            println!(
                "[f{:>3}] pc=0x{:04X}  {} | P0: grp={:02X} hpos={} | P1: grp={:02X} hpos={} | swcha={:02X}",
                frame, cpu.pc, text,
                mem.tia.grp0, mem.tia.hpos_p0,
                mem.tia.grp1, mem.tia.hpos_p1,
                mem.swcha
            );
        }
    }

    // المرحلة 2: نضغط RESET (5 إطارات)
    println!("\n=== Phase 2: RESET pressed ===");
    for frame in 0..5 {
        mem.set_reset(true);
        cpu.run_frame(&mut mem);
        if frame == 4 {
            println!("  Reset done");
        }
    }
    mem.set_reset(false);

    // المرحلة 3: نضغط يسار (60 إطاراً)
    println!("\n=== Phase 3: LEFT pressed (60 frames) ===");
    for frame in 0..60 {
        mem.set_joystick(false, false, true, false);
        cpu.run_frame(&mut mem);

        if frame % 10 == 0 {
            println!(
                "[f{:>3}] pc=0x{:04X} | P0: grp={:02X} hpos={} | P1: grp={:02X} hpos={} | swcha={:02X}",
                frame, cpu.pc,
                mem.tia.grp0, mem.tia.hpos_p0,
                mem.tia.grp1, mem.tia.hpos_p1,
                mem.swcha
            );
        }
    }
    mem.set_joystick(false, false, false, false);

    println!("\n=== Final TIA ===");
    println!(
        "bg={} pf={} pf0={:02X} pf1={:02X} pf2={:02X}",
        mem.tia.colubk, mem.tia.colupf, mem.tia.pf0, mem.tia.pf1, mem.tia.pf2
    );
}
