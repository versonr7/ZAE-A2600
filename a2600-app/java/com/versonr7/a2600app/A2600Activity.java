package com.versonr7.a2600app;

import android.app.Activity;
import android.os.Bundle;
import android.view.KeyEvent;
import android.view.MotionEvent;
import android.view.Surface;
import android.view.SurfaceHolder;
import android.view.SurfaceView;
import android.util.Log;

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

    private SurfaceView surfaceView;
    private boolean running = false;
    private Thread renderThread;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        Log.i(TAG, "Activity onCreate");

        surfaceView = new SurfaceView(this);
        surfaceView.getHolder().addCallback(this);
        surfaceView.setFocusableInTouchMode(true);
        surfaceView.requestFocus();
        setContentView(surfaceView);
        nativeOnCreate();
    }

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
        if (surfaceView.getHolder().getSurface() != null
                && surfaceView.getHolder().getSurface().isValid()) {
            if (renderThread == null || !renderThread.isAlive()) {
                running = true;
                renderThread = new Thread(this::renderLoop);
                renderThread.start();
            }
        }
    }

    @Override
    protected void onDestroy() {
        super.onDestroy();
        Log.i(TAG, "Activity onDestroy");
        nativeOnDestroy();
    }

    // ============ PS4 Controller Support ============

    @Override
    public boolean onKeyDown(int keyCode, KeyEvent event) {
        Log.i(TAG, "KeyDown: " + keyCode);
        switch (keyCode) {
            // D-Pad
            case KeyEvent.KEYCODE_DPAD_UP:
                nativeOnJoystick(1, 0, 0, 0, 0); return true;
            case KeyEvent.KEYCODE_DPAD_DOWN:
                nativeOnJoystick(0, 1, 0, 0, 0); return true;
            case KeyEvent.KEYCODE_DPAD_LEFT:
                nativeOnJoystick(0, 0, 1, 0, 0); return true;
            case KeyEvent.KEYCODE_DPAD_RIGHT:
                nativeOnJoystick(0, 0, 0, 1, 0); return true;

            // Face buttons (X = fire في PS4)
            case KeyEvent.KEYCODE_BUTTON_A:   // زر X
            case KeyEvent.KEYCODE_BUTTON_X:   // زر □
            case KeyEvent.KEYCODE_BUTTON_Y:   // زر △
            case KeyEvent.KEYCODE_BUTTON_B:   // زر ○
                nativeOnJoystick(0, 0, 0, 0, 1); return true;

            // Start/Select
            case KeyEvent.KEYCODE_BUTTON_START:
            case KeyEvent.KEYCODE_BUTTON_SELECT:
                return true;

            default:
                return super.onKeyDown(keyCode, event);
        }
    }

    @Override
    public boolean onKeyUp(int keyCode, KeyEvent event) {
        Log.i(TAG, "KeyUp: " + keyCode);
        // عند رفع أي زر، صفّر الاتجاهات (ما عدا زر Fire إن كان مرفوعاً)
        if (keyCode == KeyEvent.KEYCODE_BUTTON_A
                || keyCode == KeyEvent.KEYCODE_BUTTON_X
                || keyCode == KeyEvent.KEYCODE_BUTTON_Y
                || keyCode == KeyEvent.KEYCODE_BUTTON_B) {
            nativeOnJoystick(0, 0, 0, 0, 0);
            return true;
        }
        if (keyCode == KeyEvent.KEYCODE_DPAD_UP
                || keyCode == KeyEvent.KEYCODE_DPAD_DOWN
                || keyCode == KeyEvent.KEYCODE_DPAD_LEFT
                || keyCode == KeyEvent.KEYCODE_DPAD_RIGHT) {
            nativeOnJoystick(0, 0, 0, 0, 0);
            return true;
        }
        return super.onKeyUp(keyCode, event);
    }

    @Override
    public boolean onGenericMotionEvent(MotionEvent event) {
        // Analog Stick من PS4 controller
        if ((event.getSource() & android.view.InputDevice.SOURCE_JOYSTICK)
                == android.view.InputDevice.SOURCE_JOYSTICK) {
            float x = event.getAxisValue(MotionEvent.AXIS_X);
            float y = event.getAxisValue(MotionEvent.AXIS_Y);

            Log.i(TAG, "Analog: x=" + x + " y=" + y);

            // Dead zone
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

    // ============ Touch Support ============

    @Override
    public boolean onTouchEvent(MotionEvent event) {
        float x = event.getX();
        float y = event.getY();
        int w = surfaceView.getWidth();
        int h = surfaceView.getHeight();

        boolean up = false, down = false, left = false, right = false, fire = false;

        int action = event.getActionMasked();
        if (action == MotionEvent.ACTION_DOWN || action == MotionEvent.ACTION_MOVE) {
            float xr = x / w;
            float yr = y / h;

            // النصف الأيسر السفلي = Joystick
            if (yr > 0.6f && xr < 0.5f) {
                if (yr < 0.75f) up = true;
                else if (yr > 0.9f) down = true;
                else if (xr < 0.25f) left = true;
                else right = true;
            }
            // النصف الأيمن السفلي = Fire
            else if (yr > 0.6f && xr >= 0.5f) {
                fire = true;
            }
        }

        Log.i(TAG, "Touch: xr=" + (x / w) + " yr=" + (y / h)
                + " → U" + (up ? 1 : 0) + " D" + (down ? 1 : 0)
                + " L" + (left ? 1 : 0) + " R" + (right ? 1 : 0)
                + " F" + (fire ? 1 : 0));

        nativeOnJoystick(up ? 1 : 0, down ? 1 : 0, left ? 1 : 0, right ? 1 : 0, fire ? 1 : 0);
        return true;
    }

    // ============ SurfaceHolder.Callback ============

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
        try {
            renderThread.join();
        } catch (InterruptedException e) {
            Log.e(TAG, "Render thread interrupted", e);
        }
    }

    // ============ Render Loop ============

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
