use zae_a2600::{cpu::Cpu, memory::Memory};

fn main() {
    let rom = include_bytes!("../../roms/adventure.bin");
    let mut mem = Memory::new();
    mem.load_rom(rom);

    let mut cpu = Cpu::new();
    cpu.reset(&mut mem);
    println!("PC after reset: 0x{:04X}", cpu.pc);

    // تشغيل 60 إطاراً (ثانية واحدة تقريباً)
    println!("\n=== Frame-by-Frame TIA State ===");
    for frame in 0..60 {
        cpu.run_frame(&mut mem);

        // نطبع فقط كل 5 إطارات لتجنب الزحام الشديد
        if frame % 5 == 0 {
            println!(
                "[f{:>2}] pc=0x{:04X} | CTRLPF=0x{:02X} | COLUPF=0x{:02X} | COLUBK=0x{:02X} | PF0=0x{:02X} PF1=0x{:02X} PF2=0x{:02X}",
                frame,
                cpu.pc,
                mem.tia.ctrlpf,
                mem.tia.colupf,
                mem.tia.colubk,
                mem.tia.pf0,
                mem.tia.pf1,
                mem.tia.pf2
            );
        }
    }
}
