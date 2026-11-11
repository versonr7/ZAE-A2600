package com.versonr7.a2600app;

import android.app.Activity;
import android.os.Bundle;
import android.view.KeyEvent;
import android.view.MotionEvent;
import android.view.Surface;
import android.view.SurfaceHolder;
import android.view.SurfaceView;
import android.view.View;
import android.util.Log;
import android.widget.Button;
import android.widget.FrameLayout;
import android.widget.RelativeLayout;
import android.widget.GridLayout;
import android.widget.Space;
import android.graphics.Color;

public class A2600Activity extends Activity implements SurfaceHolder.Callback {
    private static final String TAG = "A2600";

    static {
        System.loadLibrary("a2600_app");
    }

    // Native methods
    public static native void nativeOnCreate();
    public static native void nativeOnSurfaceCreated(Surface surface);
    public static native void nativeOnSurfaceChanged(int width, int height);
    public static native void nativeOnSurfaceDestroyed();
    public static native void nativeOnPause();
    public static native void nativeOnResume();
    public static native void nativeOnDestroy();
    public static native void nativeOnTouch(float x, float y, int action);
    public static native void nativeOnFrame();
    public static native void nativeOnRenderThreadExit();
    public static native void nativeOnJoystick(int up, int down, int left, int right, int fire);
    public static native void nativeOnReset();

    private SurfaceView surfaceView;
    private boolean running = false;
    private Thread renderThread;

    // حالة أزرار التحكم
    private boolean joyUp = false, joyDown = false, joyLeft = false, joyRight = false, joyFire = false;
    private boolean resetHeld = false;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        Log.i(TAG, "Activity onCreate");

        // ✅ 1. إنشاء الحاوية الرئيسية (FrameLayout)
        FrameLayout rootLayout = new FrameLayout(this);

        // ✅ 2. إضافة شاشة اللعبة (SurfaceView)
        surfaceView = new SurfaceView(this);
        surfaceView.getHolder().addCallback(this);
        surfaceView.setFocusableInTouchMode(true);
        surfaceView.requestFocus();
        rootLayout.addView(surfaceView, new FrameLayout.LayoutParams(
                FrameLayout.LayoutParams.MATCH_PARENT,
                FrameLayout.LayoutParams.MATCH_PARENT
        ));

        // ✅ 3. إضافة أزرار التحكم الشفافة فوق الشاشة
        View controlsOverlay = createControlsOverlay();
        rootLayout.addView(controlsOverlay, new FrameLayout.LayoutParams(
                FrameLayout.LayoutParams.MATCH_PARENT,
                FrameLayout.LayoutParams.MATCH_PARENT
        ));

        setContentView(rootLayout);
        nativeOnCreate();
    }

    // ============ دالة إنشاء أزرار التحكم الشفافة ============
    private View createControlsOverlay() {
        RelativeLayout overlay = new RelativeLayout(this);
        overlay.setLayoutParams(new RelativeLayout.LayoutParams(
                RelativeLayout.LayoutParams.MATCH_PARENT,
                RelativeLayout.LayoutParams.MATCH_PARENT
        ));

        // إنشاء الأزرار
        Button btnUp = createTransparentButton("▲");
        Button btnDown = createTransparentButton("▼");
        Button btnLeft = createTransparentButton("◀");
        Button btnRight = createTransparentButton("▶");
        Button btnFire = createTransparentButton("🔥");

        // --- ترتيب D-Pad في الزاوية اليسرى السفلى ---
        GridLayout dpadLayout = new GridLayout(this);
        dpadLayout.setColumnCount(3);
        dpadLayout.setRowCount(3);
        RelativeLayout.LayoutParams dpadParams = new RelativeLayout.LayoutParams(
                RelativeLayout.LayoutParams.WRAP_CONTENT, RelativeLayout.LayoutParams.WRAP_CONTENT
        );
        dpadParams.addRule(RelativeLayout.ALIGN_PARENT_BOTTOM);
        dpadParams.addRule(RelativeLayout.ALIGN_PARENT_LEFT);
        dpadParams.setMargins(80, 50, 0, 80); // مسافة من الحافة
        dpadLayout.setLayoutParams(dpadParams);

        // إضافة الأزرار إلى GridLayout مع فراغات
        dpadLayout.addView(new Space(this), new GridLayout.LayoutParams(GridLayout.spec(0), GridLayout.spec(0)));
        dpadLayout.addView(btnUp, createGridParams(0, 1));
        dpadLayout.addView(new Space(this), new GridLayout.LayoutParams(GridLayout.spec(0), GridLayout.spec(2)));
        dpadLayout.addView(btnLeft, createGridParams(1, 0));
        dpadLayout.addView(new Space(this), new GridLayout.LayoutParams(GridLayout.spec(1), GridLayout.spec(1)));
        dpadLayout.addView(btnRight, createGridParams(1, 2));
        dpadLayout.addView(new Space(this), new GridLayout.LayoutParams(GridLayout.spec(2), GridLayout.spec(0)));
        dpadLayout.addView(btnDown, createGridParams(2, 1));
        dpadLayout.addView(new Space(this), new GridLayout.LayoutParams(GridLayout.spec(2), GridLayout.spec(2)));

        // --- زر الإطلاق في الزاوية اليمنى السفلى ---
        RelativeLayout.LayoutParams fireParams = new RelativeLayout.LayoutParams(160, 160);
        fireParams.addRule(RelativeLayout.ALIGN_PARENT_BOTTOM);
        fireParams.addRule(RelativeLayout.ALIGN_PARENT_RIGHT);
        fireParams.setMargins(0, 0, 100, 100);
        btnFire.setLayoutParams(fireParams);

        overlay.addView(dpadLayout);
        overlay.addView(btnFire);

        // ربط الأزرار بالتحكم
        btnUp.setOnTouchListener((v, event) -> { handleTouch(event, 1, 0, 0, 0, 0); return true; });
        btnDown.setOnTouchListener((v, event) -> { handleTouch(event, 0, 1, 0, 0, 0); return true; });
        btnLeft.setOnTouchListener((v, event) -> { handleTouch(event, 0, 0, 1, 0, 0); return true; });
        btnRight.setOnTouchListener((v, event) -> { handleTouch(event, 0, 0, 0, 1, 0); return true; });
        btnFire.setOnTouchListener((v, event) -> { handleTouch(event, 0, 0, 0, 0, 1); return true; });

        return overlay;
    }

    // دالة مساعدة لإنشاء زر شفاف
    private Button createTransparentButton(String text) {
        Button btn = new Button(this);
        btn.setText(text);
        btn.setTextSize(24);
        btn.getBackground().setAlpha(90); // شفافية
        btn.setTextColor(Color.WHITE);
        btn.setPadding(0, 0, 0, 0);
        return btn;
    }

    // دالة مساعدة لتحديد حجم الأزرار في الشبكة
    private GridLayout.LayoutParams createGridParams(int row, int col) {
        GridLayout.LayoutParams params = new GridLayout.LayoutParams(GridLayout.spec(row), GridLayout.spec(col));
        params.width = 110;
        params.height = 110;
        return params;
    }

    // دالة مساعدة لمعالجة ضغط الأزرار
    private void handleTouch(MotionEvent event, int up, int down, int left, int right, int fire) {
        if (event.getAction() == MotionEvent.ACTION_DOWN) {
            nativeOnJoystick(up, down, left, right, fire);
        } else if (event.getAction() == MotionEvent.ACTION_UP) {
            nativeOnJoystick(0, 0, 0, 0, 0);
        }
    }

    // ============ دورة حياة التطبيق ============

    @Override
    protected void onPause() {
        super.onPause();
        Log.i(TAG, "Activity onPause");
        running = false;
        if (renderThread != null) {
            try {
                renderThread.join(100);
            } catch (InterruptedException e) {
                Log.e(TAG, "join interrupted", e);
            }
        }
        nativeOnPause();
    }

    @Override
    protected void onResume() {
        super.onResume();
        nativeOnResume();
    }

    @Override
    protected void onDestroy() {
        super.onDestroy();
        Log.i(TAG, "Activity onDestroy");
        nativeOnDestroy();
    }

    // ============ دعم يد التحكم PS4 ============

    @Override
    public boolean dispatchKeyEvent(KeyEvent event) {
        int code = event.getKeyCode();
        int action = event.getAction();
        boolean down = (action == KeyEvent.ACTION_DOWN);
        boolean up = (action == KeyEvent.ACTION_UP);

        switch (code) {
            case KeyEvent.KEYCODE_DPAD_UP:
                if (down) nativeOnJoystick(1, 0, 0, 0, 0);
                else if (up) nativeOnJoystick(0, 0, 0, 0, 0);
                return true;
            case KeyEvent.KEYCODE_DPAD_DOWN:
                if (down) nativeOnJoystick(0, 1, 0, 0, 0);
                else if (up) nativeOnJoystick(0, 0, 0, 0, 0);
                return true;
            case KeyEvent.KEYCODE_DPAD_LEFT:
                if (down) nativeOnJoystick(0, 0, 1, 0, 0);
                else if (up) nativeOnJoystick(0, 0, 0, 0, 0);
                return true;
            case KeyEvent.KEYCODE_DPAD_RIGHT:
                if (down) nativeOnJoystick(0, 0, 0, 1, 0);
                else if (up) nativeOnJoystick(0, 0, 0, 0, 0);
                return true;

            case KeyEvent.KEYCODE_BUTTON_A:
            case KeyEvent.KEYCODE_BUTTON_X:
            case KeyEvent.KEYCODE_BUTTON_Y:
            case KeyEvent.KEYCODE_BUTTON_B:
            case KeyEvent.KEYCODE_BUTTON_R1:
            case KeyEvent.KEYCODE_BUTTON_L1:
                if (down) nativeOnJoystick(0, 0, 0, 0, 1);
                else if (up) nativeOnJoystick(0, 0, 0, 0, 0);
                return true;

            case KeyEvent.KEYCODE_BUTTON_START:
            case KeyEvent.KEYCODE_BUTTON_SELECT:
                if (down) {
                    nativeOnReset();
                }
                return true;
        }
        return super.dispatchKeyEvent(event);
    }

    @Override
    public boolean onGenericMotionEvent(MotionEvent event) {
        if ((event.getSource() & android.view.InputDevice.SOURCE_JOYSTICK)
                == android.view.InputDevice.SOURCE_JOYSTICK) {
            float x = event.getAxisValue(MotionEvent.AXIS_X);
            float y = event.getAxisValue(MotionEvent.AXIS_Y);
            if (Math.abs(x) < 0.3f && Math.abs(y) < 0.3f) {
                nativeOnJoystick(0, 0, 0, 0, 0);
                return true;
            }
            int u = (y < -0.5f) ? 1 : 0;
            int d = (y > 0.5f) ? 1 : 0;
            int l = (x < -0.5f) ? 1 : 0;
            int r = (x > 0.5f) ? 1 : 0;
            nativeOnJoystick(u, d, l, r, 0);
            return true;
        }
        return super.onGenericMotionEvent(event);
    }

    // ============ دعم اللمس (في حال لمس الشاشة العادية) ============

    @Override
    public boolean onTouchEvent(MotionEvent event) {
        int action = event.getActionMasked();
        int w = surfaceView.getWidth();
        int h = surfaceView.getHeight();

        if (action == MotionEvent.ACTION_UP || action == MotionEvent.ACTION_CANCEL) {
            joyUp = joyDown = joyLeft = joyRight = joyFire = false;
            resetHeld = false;
            nativeOnJoystick(0, 0, 0, 0, 0);
            return true;
        }

        boolean up = false, down = false, left = false, right = false, fire = false, reset = false;
        int pointerCount = event.getPointerCount();

        for (int i = 0; i < pointerCount; i++) {
            float x = event.getX(i);
            float y = event.getY(i);
            float xr = x / w;
            float yr = y / h;

            if (yr < 0.2f && xr > 0.8f) {
                reset = true;
            } else if (yr > 0.5f && xr < 0.5f) {
                if (yr < 0.7f) up = true;
                else if (yr > 0.9f) down = true;
                else if (xr < 0.25f) left = true;
                else right = true;
            } else if (yr > 0.5f && xr >= 0.5f) {
                fire = true;
            }
        }

        if (up != joyUp || down != joyDown || left != joyLeft || right != joyRight || fire != joyFire) {
            joyUp = up; joyDown = down; joyLeft = left; joyRight = right; joyFire = fire;
            nativeOnJoystick(up ? 1 : 0, down ? 1 : 0, left ? 1 : 0, right ? 1 : 0, fire ? 1 : 0);
        }

        if (reset && !resetHeld) {
            resetHeld = true;
            nativeOnReset();
        } else if (!reset) {
            resetHeld = false;
        }

        return true;
    }

    // ============ دورة حياة Surface ============

    @Override
    public void surfaceCreated(SurfaceHolder holder) {
        Log.i(TAG, "Surface created");
        if (renderThread != null && renderThread.isAlive()) {
            return;
        }
        nativeOnSurfaceCreated(holder.getSurface());
        running = true;
        renderThread = new Thread(this::renderLoop);
        renderThread.start();
    }

    @Override
    public void surfaceChanged(SurfaceHolder holder, int format, int width, int height) {
        Log.i(TAG, "Surface changed: " + width + "x" + height);
        nativeOnSurfaceChanged(width, height);
    }

    @Override
    public void surfaceDestroyed(SurfaceHolder holder) {
        Log.i(TAG, "Surface destroyed");
        running = false;
        if (renderThread != null) {
            try {
                renderThread.join();
            } catch (InterruptedException e) {
                Log.e(TAG, "Render thread interrupted", e);
            }
            renderThread = null;
        }
        nativeOnSurfaceDestroyed();
    }

    // ============ حلقة الرسم ============

    private void renderLoop() {
        while (running) {
            nativeOnFrame();
            try {
                Thread.sleep(16);
            } catch (InterruptedException e) {
                break;
            }
        }
        nativeOnRenderThreadExit();
    }
}