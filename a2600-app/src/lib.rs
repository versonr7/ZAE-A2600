#![no_std]
#![allow(warnings)]

use core::ffi::{c_int, c_void};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicPtr, AtomicU32, Ordering};

use za_gles::{BatchRenderer, GlContext, Texture};
use za_math::{Color, Mat4, Rect};
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
static INITIALIZED: AtomicBool = AtomicBool::new(false);
static FRAME_LOCK: AtomicBool = AtomicBool::new(false);

struct FrameLockGuard;
impl FrameLockGuard {
    fn try_acquire() -> Option<Self> {
        if FRAME_LOCK
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire)
            .is_ok()
        {
            Some(FrameLockGuard)
        } else {
            None
        }
    }
}
impl Drop for FrameLockGuard {
    fn drop(&mut self) {
        FRAME_LOCK.store(false, Ordering::Release);
    }
}

static mut GL_CTX_STORAGE: MaybeUninit<GlContext> = MaybeUninit::uninit();
static GL_CTX: AtomicPtr<GlContext> = AtomicPtr::new(core::ptr::null_mut());

static mut BATCH_STORAGE: MaybeUninit<BatchRenderer<400, 600>> = MaybeUninit::uninit();
static BATCH: AtomicPtr<BatchRenderer<400, 600>> = AtomicPtr::new(core::ptr::null_mut());

static JOY_UP: AtomicBool = AtomicBool::new(false);
static JOY_DOWN: AtomicBool = AtomicBool::new(false);
static JOY_LEFT: AtomicBool = AtomicBool::new(false);
static JOY_RIGHT: AtomicBool = AtomicBool::new(false);
static JOY_FIRE: AtomicBool = AtomicBool::new(false);
static RESET_FRAMES: AtomicU32 = AtomicU32::new(0);

static mut SCREEN_TEX_STORAGE: MaybeUninit<Texture> = MaybeUninit::uninit();
static SCREEN_TEX: AtomicPtr<Texture> = AtomicPtr::new(core::ptr::null_mut());
static mut MEM_STORAGE: MaybeUninit<Memory> = MaybeUninit::uninit();
static mut CPU_STORAGE: MaybeUninit<Cpu> = MaybeUninit::uninit();
static EMU_INITIALIZED: AtomicBool = AtomicBool::new(false);

#[no_mangle]
pub extern "C" fn Java_com_versonr7_a2600app_A2600Activity_nativeOnReset(
    _env: *mut c_void,
    _class: *mut c_void,
) {
    logfox!("A2600", "RESET pressed");
    RESET_FRAMES.store(30, Ordering::Release);
}

#[no_mangle]
pub extern "C" fn Java_com_versonr7_a2600app_A2600Activity_nativeOnRenderThreadExit(
    _env: *mut c_void,
    _class: *mut c_void,
) {
    unsafe {
        let ctx_ptr = GL_CTX.load(Ordering::Acquire);
        if !ctx_ptr.is_null() {
            let _ = (*ctx_ptr).make_current();
        }
        if !SCREEN_TEX.load(Ordering::Relaxed).is_null() {
            core::ptr::drop_in_place(SCREEN_TEX_STORAGE.as_mut_ptr());
            SCREEN_TEX.store(core::ptr::null_mut(), Ordering::Release);
        }
        if !BATCH.load(Ordering::Relaxed).is_null() {
            core::ptr::drop_in_place(BATCH_STORAGE.as_mut_ptr());
            BATCH.store(core::ptr::null_mut(), Ordering::Release);
        }
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
            return;
        }

        if let Some(win) = NativeWindow::from_raw(anw) {
            let w = win.width();
            let h = win.height();
            WIDTH.store(w, Ordering::Release);
            HEIGHT.store(h, Ordering::Release);
            INITIALIZED.store(true, Ordering::Release);
            RUNNING.store(true, Ordering::Release);

            let ctx_ptr = GL_CTX.load(Ordering::Acquire);
            if ctx_ptr.is_null() {
                match GlContext::from_window(win) {
                    Ok(ctx) => {
                        GL_CTX_STORAGE.write(ctx);
                        GL_CTX.store(GL_CTX_STORAGE.as_mut_ptr(), Ordering::Release);
                        logfox!("ZAVOGLES", "EGL context created for the first time");
                    }
                    Err(e) => logfox!("ZAVOGLES", "ERROR: GlContext failed: {}", e),
                }
            } else {
                let ctx = &mut *ctx_ptr;
                match ctx.update_surface(win) {
                    Ok(()) => logfox!("ZAVOGLES", "EGL surface updated successfully"),
                    Err(e) => logfox!("ZAVOGLES", "ERROR: update_surface failed: {}", e),
                }
            }
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
    WIDTH.store(width, Ordering::Release);
    HEIGHT.store(height, Ordering::Release);
}

#[no_mangle]
pub extern "C" fn Java_com_versonr7_a2600app_A2600Activity_nativeOnSurfaceDestroyed(
    _env: *mut c_void,
    _class: *mut c_void,
) {
    RUNNING.store(false, Ordering::Release);
}

#[no_mangle]
pub extern "C" fn Java_com_versonr7_a2600app_A2600Activity_nativeOnPause(
    _env: *mut c_void,
    _class: *mut c_void,
) {
    RUNNING.store(false, Ordering::Release);
}

#[no_mangle]
pub extern "C" fn Java_com_versonr7_a2600app_A2600Activity_nativeOnResume(
    _env: *mut c_void,
    _class: *mut c_void,
) {
}

#[no_mangle]
pub extern "C" fn Java_com_versonr7_a2600app_A2600Activity_nativeOnDestroy(
    _env: *mut c_void,
    _class: *mut c_void,
) {
    RUNNING.store(false, Ordering::Release);
}

#[no_mangle]
pub extern "C" fn Java_com_versonr7_a2600app_A2600Activity_nativeOnTouch(
    _env: *mut c_void,
    _class: *mut c_void,
    _x: f32,
    _y: f32,
    _action: i32,
) {
}

#[no_mangle]
pub extern "C" fn Java_com_versonr7_a2600app_A2600Activity_nativeOnJoystick(
    _env: *mut c_void,
    _class: *mut c_void,
    up: i32,
    down: i32,
    left: i32,
    right: i32,
    fire: i32,
) {
    JOY_UP.store(up != 0, Ordering::Release);
    JOY_DOWN.store(down != 0, Ordering::Release);
    JOY_LEFT.store(left != 0, Ordering::Release);
    JOY_RIGHT.store(right != 0, Ordering::Release);
    JOY_FIRE.store(fire != 0, Ordering::Release);
}

#[no_mangle]
pub extern "C" fn Java_com_versonr7_a2600app_A2600Activity_nativeOnFrame(
    _env: *mut c_void,
    _class: *mut c_void,
) {
    let _guard = match FrameLockGuard::try_acquire() {
        Some(g) => g,
        None => return,
    };

    if !RUNNING.load(Ordering::Acquire) {
        return;
    }

    unsafe {
        let ctx_ptr = GL_CTX.load(Ordering::Acquire);
        if ctx_ptr.is_null() {
            return;
        }
        let ctx = &mut *ctx_ptr;

        let batch_ptr = BATCH.load(Ordering::Acquire);
        if batch_ptr.is_null() {
            if let Err(_) = ctx.make_current() {
                return;
            }
            ctx.setup_gl_state();

            if let Ok(batch) = BatchRenderer::<400, 600>::new() {
                BATCH_STORAGE.write(batch);
                BATCH.store(BATCH_STORAGE.as_mut_ptr(), Ordering::Release);
            }

            if let Ok(tex) = Texture::new() {
                SCREEN_TEX_STORAGE.write(tex);
                SCREEN_TEX.store(SCREEN_TEX_STORAGE.as_mut_ptr(), Ordering::Release);
            }
        }

        let batch = &mut *BATCH.load(Ordering::Acquire);
        let w = WIDTH.load(Ordering::Acquire) as f32;
        let h = HEIGHT.load(Ordering::Acquire) as f32;

        ctx.update_viewport(w as i32, h as i32);
        ctx.clear();

        if !EMU_INITIALIZED.load(Ordering::Acquire) {
            let mut mem = Memory::new();
            let rom = include_bytes!("../../roms/adventure.bin");
            mem.load_rom(rom);
            MEM_STORAGE.write(mem);

            let mut cpu = Cpu::new();
            cpu.reset(&mut *MEM_STORAGE.as_mut_ptr());
            CPU_STORAGE.write(cpu);
            EMU_INITIALIZED.store(true, Ordering::Release);
        }

        {
            let mem = &mut *MEM_STORAGE.as_mut_ptr();
            mem.set_joystick(
                JOY_UP.load(Ordering::Acquire),
                JOY_DOWN.load(Ordering::Acquire),
                JOY_LEFT.load(Ordering::Acquire),
                JOY_RIGHT.load(Ordering::Acquire),
            );
            mem.set_fire(JOY_FIRE.load(Ordering::Acquire));

            let cnt = RESET_FRAMES.load(Ordering::Acquire);
            if cnt > 0 {
                mem.set_reset(true);
                RESET_FRAMES.store(cnt - 1, Ordering::Release);
            } else {
                mem.set_reset(false);
            }
        }

        // --- ⚠️ [Cycle-Accurate] تشغيل إطار واحد (19912 دورة CPU) ---
let mem = &mut *MEM_STORAGE.as_mut_ptr();
let cpu = &mut *CPU_STORAGE.as_mut_ptr();
cpu.run_for_cycles(mem, 19912);

        let tex_ptr = SCREEN_TEX.load(Ordering::Acquire);
        if !tex_ptr.is_null() {
            let tex = &*tex_ptr;
            let _ = tex.upload_rgba(
                zae_a2600::tia::FB_WIDTH as i32,
                zae_a2600::tia::FB_HEIGHT as i32,
                &mem.tia.framebuffer,
            );
            batch.begin_frame();
            batch.is_msdf = false; // ✅ هذه صورة لعبة وليست نص
            batch.set_texture(tex);
            batch.draw_quad(
                Rect::from_coords(0.0, 0.0, w, h),
                Rect::from_coords(0.0, 0.0, 1.0, 1.0),
                Color::WHITE,
            );
            batch.end_frame(&Mat4::ortho(0.0, w, h, 0.0, -1.0, 1.0), 0.0, 0.0, 0.0);
        }

        if RUNNING.load(Ordering::Acquire) {
            let _ = ctx.swap_buffers();
        }
    }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    za_sys::android_log(za_sys::LogLevel::Error, "ZAVOGLES", "PANIC!");
    unsafe { libc::abort() }
}
