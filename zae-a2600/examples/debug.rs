use zae_a2600::{cpu::Cpu, memory::Memory};

fn main() {
    let rom = include_bytes!("../../roms/Collect_NTSC.bin");
    let mut mem = Memory::new();
    mem.load_rom(rom);
    
    // ✅ نطبع أي كتابة على سجلات TIA (العناوين من 0x00 إلى 0x3F)
    mem.trace_fn = Some(|reg, val, sl, cyc| {
        if reg < 0x40 {
            println!("[TIA W] sl={} cyc={} reg=0x{:02X} val=0x{:02X}", sl, cyc, reg, val);
        }
    });

    let mut cpu = Cpu::new();
    cpu.reset(&mut mem);

    println!("PC after reset: 0x{:04X}", cpu.pc);
    println!("=== Tracing ===");

    let mut total_cpu_cycles = 0u32;
    let mut frame = 0;
    let mut instruction_count = 0;

    // نشغل 5 إطارات أو 500 ألف تعليمة
    while frame < 5 && instruction_count < 500_000 {
        instruction_count += 1;

        let pc = cpu.pc;
        
        if mem.tia.wsync {
            let remaining_tia = 228 - mem.tia.cycle_in_scanline;
            mem.tia.tick_cycle(remaining_tia);
            mem.tick_riot(remaining_tia / 3);
            mem.tia.wsync = false;
        } else {
            let cpu_cycles = cpu.step_cycle(&mut mem) as u32;
            if cpu_cycles > 0 {
                // نطبع حالة المعالج كل 100 ألف تعليمة لنعرف أين وصل
                if instruction_count % 100_000 == 0 {
                    println!("[PC:{:04X}] Opcode: 0x{:02X} | sl={} cyc={} | A:{:02X} X:{:02X} Y:{:02X}", 
                        pc, mem.read(pc), mem.tia.scanline, mem.tia.cycle_in_scanline, cpu.a, cpu.x, cpu.y);
                }

                mem.tia.tick_cycle(cpu_cycles * 3);
                mem.tick_riot(cpu_cycles);
                total_cpu_cycles += cpu_cycles;
            }
        }
        if total_cpu_cycles >= 19912 {
            frame += 1;
            total_cpu_cycles = 0;
        }
    }
    println!("=== Done ===");
}