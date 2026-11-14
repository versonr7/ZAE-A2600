use zae_a2600::{cpu::Cpu, memory::Memory};

fn main() {
    let rom = include_bytes!("../../roms/Pong2600.bin");
    let mut mem = Memory::new();
    mem.load_rom(rom);
    
    // ✅ تفعيل التتبع من هنا (الـ debug.rs يمكنه استخدام println)
    mem.trace_fn = Some(|reg, val, sl, cyc| {
        if matches!(reg, 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x2A | 0x0D | 0x0E | 0x0F) {
            println!("[TIA] sl={} cyc={} reg=0x{:02X} val=0x{:02X}", sl, cyc, reg, val);
        }
    });

    let mut cpu = Cpu::new();
    cpu.reset(&mut mem);

    println!("PC after reset: 0x{:04X}", cpu.pc);
    println!("=== Tracing 1 Frame of Pong ===");

    let mut total_cpu_cycles = 0u32;
    let mut instruction_count = 0u64;

    while total_cpu_cycles < 19912 && instruction_count < 2_000_000 {
        instruction_count += 1;
        
        if mem.tia.wsync {
            let remaining_tia = 228 - mem.tia.cycle_in_scanline;
            mem.tia.tick_cycle(remaining_tia);
            mem.tick_riot(remaining_tia / 3);
            mem.tia.wsync = false;
        } else {
            let cpu_cycles = cpu.step_cycle(&mut mem) as u32;
            if cpu_cycles > 0 {
                mem.tia.tick_cycle(cpu_cycles * 3);
                mem.tick_riot(cpu_cycles);
                total_cpu_cycles += cpu_cycles;
            }
        }
    }
    
    println!("=== Frame Complete ===");
    println!("Final: P0_hpos={} P1_hpos={} P0_grp={:02X} P1_grp={:02X}",
        mem.tia.hpos_p0, mem.tia.hpos_p1, mem.tia.grp0, mem.tia.grp1);
}