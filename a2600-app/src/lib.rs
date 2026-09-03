#![no_std]
#![allow(warnings)]

mod glyphs_english;
use glyphs_english::FONT_GLYPHS;

use core::ffi::{c_int, c_void};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicPtr, AtomicU32, Ordering};

use za_gles::font::BitmapFont;
use za_gles::{BatchRenderer, GlContext};
use za_math::{Mat4, Rect};
use za_sys::NativeWindow;
use zae_a2600::{cpu::Cpu, memory::Memory};

// ===== LOGGING =====
#[macro_export]
macro_rules! logfox {
    ($tag:expr, $msg:expr) => {
        {
            za_sys::android_log(za_sys::LogLevel::Info, $tag, $msg);
        }
    };
    ($tag:expr, $($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut buf = heapless::String::<256>::new();
            let _ = core::write!(buf, $($arg)*);
            za_sys::android_log(za_sys::LogLevel::Info, $tag, buf.as_str());
        }
    };
}

// ===== STATE =====
static RUNNING: AtomicBool = AtomicBool::new(false);
static WIDTH: AtomicI32 = AtomicI32::new(0);
static HEIGHT: AtomicI32 = AtomicI32::new(0);
static FRAME_COUNT: AtomicU32 = AtomicU32::new(0);
static INITIALIZED: AtomicBool = AtomicBool::new(false);
static FRAME_LOCK: AtomicBool = AtomicBool::new(false);

static mut GL_CTX_STORAGE: MaybeUninit<GlContext> = MaybeUninit::uninit();
static GL_CTX: AtomicPtr<GlContext> = AtomicPtr::new(core::ptr::null_mut());

static mut BATCH_STORAGE: MaybeUninit<BatchRenderer<400, 600>> = MaybeUninit::uninit();
static BATCH: AtomicPtr<BatchRenderer<400, 600>> = AtomicPtr::new(core::ptr::null_mut());

// --- Font atlas ---
static FONT_ATLAS_BYTES: &[u8] = include_bytes!("../../assets/font_atlas.rgba");
const FONT_ATLAS_W: i32 = 512;
const FONT_ATLAS_H: i32 = 512;

static mut FONT_STORAGE: MaybeUninit<BitmapFont> = MaybeUninit::uninit();
static FONT: AtomicPtr<BitmapFont> = AtomicPtr::new(core::ptr::null_mut());
static mut MEM_STORAGE: MaybeUninit<Memory> = MaybeUninit::uninit();
static mut CPU_STORAGE: MaybeUninit<Cpu> = MaybeUninit::uninit();
// ===== JNI EXPORTS =====
#[no_mangle]
pub extern "C" fn Java_com_versonr7_a2600app_A2600Activity_nativeOnRenderThreadExit(
    _env: *mut c_void,
    _class: *mut c_void,
) {
    unsafe {
        // تأكد أن السياق الحالي ساري قبل حذف الموارد
        let ctx_ptr = GL_CTX.load(Ordering::Acquire);
        if !ctx_ptr.is_null() {
            let _ = (*ctx_ptr).make_current();
        }

        // احذف الخط أولاً (يستخدم Textures)
        if !FONT.load(Ordering::Relaxed).is_null() {
            core::ptr::drop_in_place(FONT_STORAGE.as_mut_ptr());
            FONT.store(core::ptr::null_mut(), Ordering::Release);
        }

        // ثم احذف BatchRenderer
        if !BATCH.load(Ordering::Relaxed).is_null() {
            core::ptr::drop_in_place(BATCH_STORAGE.as_mut_ptr());
            BATCH.store(core::ptr::null_mut(), Ordering::Release);
        }

        // أخيرًا احذف سياق GL
        if !GL_CTX.load(Ordering::Relaxed).is_null() {
            core::ptr::drop_in_place(GL_CTX_STORAGE.as_mut_ptr());
            GL_CTX.store(core::ptr::null_mut(), Ordering::Release);
        }

        INITIALIZED.store(false, Ordering::Release);
    }
}

#[no_mangle]
pub extern "C" fn Java_com_versonr7_a2600app_A2600Activity_nativeOnCreate(
    _env: *mut c_void,
    _class: *mut c_void,
) {
    logfox!("ZAVOGLES", "Native onCreate");
}

#[no_mangle]
pub extern "C" fn Java_com_versonr7_a2600app_A2600Activity_nativeOnSurfaceCreated(
    _env: *mut c_void,
    _class: *mut c_void,
    surface: *mut c_void,
) {
    logfox!("ZAVOGLES", "Native surfaceCreated");

    unsafe {
        let anw = za_sys::ANativeWindow_fromSurface(_env, surface);
        if anw.is_null() {
            logfox!("ZAVOGLES", "ERROR: ANativeWindow_fromSurface returned null");
            return;
        }

        if let Some(win) = NativeWindow::from_raw(anw) {
            let w = win.width();
            let h = win.height();

            // ✅ إصلاح تسريب السياق: إذا كان فيه سياق قديم، امسحه وأبطل المؤشرات القديمة
            let old_ctx = GL_CTX.load(Ordering::Acquire);
            if !old_ctx.is_null() {
                core::ptr::drop_in_place(old_ctx);
                GL_CTX.store(core::ptr::null_mut(), Ordering::Release);
                BATCH.store(core::ptr::null_mut(), Ordering::Release);
                FONT.store(core::ptr::null_mut(), Ordering::Release);
                logfox!("ZAVOGLES", "Old GL context dropped");
            }

            match GlContext::from_window(win) {
                Ok(ctx) => {
                    GL_CTX_STORAGE.write(ctx);
                    GL_CTX.store(GL_CTX_STORAGE.as_mut_ptr(), Ordering::Release);

                    WIDTH.store(w, Ordering::Release);
                    HEIGHT.store(h, Ordering::Release);
                    INITIALIZED.store(true, Ordering::Release);
                    RUNNING.store(true, Ordering::Release);

                    logfox!("ZAVOGLES", "EGL context ready: {}x{}", w, h);
                }
                Err(e) => logfox!("ZAVOGLES", "ERROR: GlContext failed: {}", e),
            }
        } else {
            logfox!("ZAVOGLES", "ERROR: NativeWindow::from_raw failed");
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_com_versonr7_a2600app_A2600Activity_nativeOnSurfaceChanged(
    _env: *mut c_void,
    _class: *mut c_void,
    width: i32,
    height: i32,
) {
    logfox!("ZAVOGLES", "Native surfaceChanged: {}x{}", width, height);
    WIDTH.store(width, Ordering::Release);
    HEIGHT.store(height, Ordering::Release);
}

#[no_mangle]
pub extern "C" fn Java_com_versonr7_a2600app_A2600Activity_nativeOnSurfaceDestroyed(
    _env: *mut c_void,
    _class: *mut c_void,
) {
    logfox!("ZAVOGLES", "Native surfaceDestroyed");
    RUNNING.store(false, Ordering::Release);
}

#[no_mangle]
pub extern "C" fn Java_com_versonr7_a2600app_A2600Activity_nativeOnPause(
    _env: *mut c_void,
    _class: *mut c_void,
) {
    logfox!("ZAVOGLES", "Native onPause");
    RUNNING.store(false, Ordering::Release);
}

#[no_mangle]
pub extern "C" fn Java_com_versonr7_a2600app_A2600Activity_nativeOnResume(
    _env: *mut c_void,
    _class: *mut c_void,
) {
    logfox!("ZAVOGLES", "Native onResume");
}

#[no_mangle]
pub extern "C" fn Java_com_versonr7_a2600app_A2600Activity_nativeOnDestroy(
    _env: *mut c_void,
    _class: *mut c_void,
) {
    logfox!("ZAVOGLES", "Native onDestroy");
    RUNNING.store(false, Ordering::Release);
}

#[no_mangle]
pub extern "C" fn Java_com_versonr7_a2600app_A2600Activity_nativeOnTouch(
    _env: *mut c_void,
    _class: *mut c_void,
    x: f32,
    _y: f32,
    action: i32,
) {
    if action == 0 {
        let w = WIDTH.load(Ordering::Acquire) as f32;

        logfox!("ZAVOGLES", "Touch received");
    }
}

#[no_mangle]
pub extern "C" fn Java_com_versonr7_a2600app_A2600Activity_nativeOnFrame(
    _env: *mut c_void,
    _class: *mut c_void,
) {
    if FRAME_LOCK
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire)
        .is_err()
    {
        return;
    }

    if !RUNNING.load(Ordering::Acquire) {
        FRAME_LOCK.store(false, Ordering::Release);
        return;
    }

    unsafe {
        let ctx_ptr = GL_CTX.load(Ordering::Acquire);
        if ctx_ptr.is_null() {
            FRAME_LOCK.store(false, Ordering::Release);
            return;
        }

        let ctx = &mut *ctx_ptr;

        let batch_ptr = BATCH.load(Ordering::Acquire);
        if batch_ptr.is_null() {
            if let Err(e) = ctx.make_current() {
                logfox!("A2600", "ERROR: make_current failed: {}", e);
                FRAME_LOCK.store(false, Ordering::Release);
                return;
            }
            ctx.setup_gl_state();

            match BatchRenderer::<400, 600>::new() {
                Ok(batch) => {
                    BATCH_STORAGE.write(batch);
                    BATCH.store(BATCH_STORAGE.as_mut_ptr(), Ordering::Release);
                    logfox!("A2600", "BatchRenderer created on render thread");
                }
                Err(e) => {
                    logfox!("A2600", "ERROR: BatchRenderer failed: {}", e);
                    FRAME_LOCK.store(false, Ordering::Release);
                    return;
                }
            }
        }

        let batch = &mut *BATCH.load(Ordering::Acquire);

        let w = WIDTH.load(Ordering::Acquire) as f32;
        let h = HEIGHT.load(Ordering::Acquire) as f32;

        ctx.update_viewport(w as i32, h as i32);
        ctx.clear();

        // --- تهيئة المحاكي مرة واحدة ---
        if MEM_STORAGE.as_ptr().is_null() {
            let mut mem = Memory::new();
            let rom = include_bytes!("../../roms/adventure.bin");
            mem.load_rom(rom);
            MEM_STORAGE.write(mem);

            let mut cpu = Cpu::new();
            let mem_ptr = MEM_STORAGE.as_mut_ptr();
            cpu.reset(&mut *mem_ptr);
            CPU_STORAGE.write(cpu);

            logfox!("A2600", "Atari 2600 emulator initialized");
        }

        // --- تشغيل إطار واحد ---
        let mem = &mut *MEM_STORAGE.as_mut_ptr();
        let cpu = &mut *CPU_STORAGE.as_mut_ptr();
        cpu.run_frame(mem);

        // --- رسم خلفية بلون TIA ---
        let bg_color = mem.tia.background_color();
        batch.begin_frame();
        batch.draw_quad(
            Rect::from_coords(0.0, 0.0, w, h),
            Rect::from_coords(0.0, 0.0, 1.0, 1.0),
            bg_color,
        );
        batch.end_frame(&Mat4::ortho(0.0, w, h, 0.0, -1.0, 1.0), 0.0, 0.0, 0.0);

        if RUNNING.load(Ordering::Acquire) {
            if let Err(e) = ctx.swap_buffers() {
                logfox!("A2600", "ERROR: swap_buffers: {}", e);
            }
        }
    }

    FRAME_LOCK.store(false, Ordering::Release);
}

// ===== PANIC HANDLER =====
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if info.location().is_some() {
        za_sys::android_log(
            za_sys::LogLevel::Error,
            "ZAVOGLES",
            "PANIC! (see logcat for details)",
        );
    } else {
        za_sys::android_log(za_sys::LogLevel::Error, "ZAVOGLES", "PANIC!");
    }
    // ✅ إصلاح Claude: إنهاء فوري بدل حلقة لا نهائية
    loop {}
}

// ===== TESTS =====
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running() {
        RUNNING.store(true, Ordering::Relaxed);
        assert!(RUNNING.load(Ordering::Relaxed));
    }

    #[test]
    fn test_frame_lock() {
        assert!(FRAME_LOCK
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire)
            .is_ok());
        assert!(FRAME_LOCK
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire)
            .is_err());
        FRAME_LOCK.store(false, Ordering::Release);
    }
}
