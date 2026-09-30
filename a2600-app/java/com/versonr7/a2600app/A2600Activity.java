package com.versonr7.a2600app;

import android.app.Activity;
import android.os.Bundle;
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

    // Native methods (نفس الأسماء لكن داخل lib.rs سنستخدم prefix جديد)
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
        if (surfaceView.getHolder().getSurface() != null && surfaceView.getHolder().getSurface().isValid()) {
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

        if (yr > 0.6f && xr < 0.4f) {
            if (yr < 0.75f) up = true;
            else if (yr > 0.9f) down = true;
            else if (xr < 0.2f) left = true;
            else right = true;
        }
        else if (yr > 0.6f && xr > 0.6f) {
            fire = true;
        }
    }

    Log.i(TAG, "Touch: xr=" + (x/w) + " yr=" + (y/h) + " → U" + (up?1:0) + " D" + (down?1:0) + " L" + (left?1:0) + " R" + (right?1:0) + " F" + (fire?1:0));

    nativeOnJoystick(up ? 1 : 0, down ? 1 : 0, left ? 1 : 0, right ? 1 : 0, fire ? 1 : 0);
    return true;
}

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
        // لا تستدع nativeOnSurfaceDestroyed هنا
    }

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

    private void setupTouchControls() {
        // لا شيء حالياً - المنطق في onTouchEvent
    }
}   // ← قوس إغلاق الكلاس هنا فقط (مرة واحدة)
