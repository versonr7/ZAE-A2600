#![no_std]

pub mod cpu;
pub mod memory;
pub mod tia;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_rom() {
        let mut mem = memory::Memory::new();
        let rom = include_bytes!("../../roms/adventure.bin");
        mem.load_rom(rom);
        assert_ne!(rom.len(), 0, "ROM empty");
        assert_eq!(mem.read(0x1000), rom[0]);
    }

    #[test]
    fn test_tia_background() {
        let mut mem = memory::Memory::new();
        mem.write(0x09, 0x7F); // COLUBK
        let color = mem.tia.background_color();
        assert!(color.r > 0.9 && color.g > 0.9 && color.b > 0.9);
    }

    #[test]
    fn test_cpu_execute() {
        let rom = include_bytes!("../../roms/adventure.bin");
        let mut mem = memory::Memory::new();
        mem.load_rom(rom);

        let mut cpu = cpu::Cpu::new();
        cpu.reset(&mut mem);

        // نفذ إطارًا كاملاً (19912 دورة)
        let cycles_used = cpu.run_frame(&mut mem);

        assert!(cycles_used > 0, "CPU should execute some cycles");
        assert!(cpu.cycles > 0, "CPU cycles counter should increase");
        assert_ne!(cpu.pc, 0, "PC should not be zero after execution");
    }
}
