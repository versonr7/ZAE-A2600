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

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        Log.i(TAG, "Activity onCreate");

        FrameLayout rootLayout = new FrameLayout(this);

        surfaceView = new SurfaceView(this);
        surfaceView.getHolder().addCallback(this);
        surfaceView.setFocusableInTouchMode(true);
        surfaceView.requestFocus();
        rootLayout.addView(surfaceView, new FrameLayout.LayoutParams(
                FrameLayout.LayoutParams.MATCH_PARENT, FrameLayout.LayoutParams.MATCH_PARENT
        ));

        View controlsOverlay = createControlsOverlay();
        rootLayout.addView(controlsOverlay, new FrameLayout.LayoutParams(
                FrameLayout.LayoutParams.MATCH_PARENT, FrameLayout.LayoutParams.MATCH_PARENT
        ));

        setContentView(rootLayout);
        nativeOnCreate();
    }

    private View createControlsOverlay() {
        RelativeLayout overlay = new RelativeLayout(this);
        Button btnUp = createTransparentButton("▲");
        Button btnDown = createTransparentButton("▼");
        Button btnLeft = createTransparentButton("◀");
        Button btnRight = createTransparentButton("▶");
        Button btnFire = createTransparentButton("🔥");
// ... (بعد إنشاء btnFire)
Button btnReset = createTransparentButton("RST");
RelativeLayout.LayoutParams resetParams = new RelativeLayout.LayoutParams(120, 120);
resetParams.addRule(RelativeLayout.ALIGN_PARENT_TOP);
resetParams.addRule(RelativeLayout.ALIGN_PARENT_RIGHT);
resetParams.setMargins(0, 80, 80, 0);
btnReset.setLayoutParams(resetParams);
overlay.addView(btnReset);
btnReset.setOnTouchListener((v, event) -> {
    if (event.getAction() == MotionEvent.ACTION_DOWN) nativeOnReset();
    return true;
});
      
        GridLayout dpadLayout = new GridLayout(this);
        dpadLayout.setColumnCount(3);
        dpadLayout.setRowCount(3);
        RelativeLayout.LayoutParams dpadParams = new RelativeLayout.LayoutParams(
                RelativeLayout.LayoutParams.WRAP_CONTENT, RelativeLayout.LayoutParams.WRAP_CONTENT
        );
        dpadParams.addRule(RelativeLayout.ALIGN_PARENT_BOTTOM);
        dpadParams.addRule(RelativeLayout.ALIGN_PARENT_LEFT);
        dpadParams.setMargins(80, 50, 0, 80);
        dpadLayout.setLayoutParams(dpadParams);

        dpadLayout.addView(new Space(this), new GridLayout.LayoutParams(GridLayout.spec(0), GridLayout.spec(0)));
        dpadLayout.addView(btnUp, createGridParams(0, 1));
        dpadLayout.addView(new Space(this), new GridLayout.LayoutParams(GridLayout.spec(0), GridLayout.spec(2)));
        dpadLayout.addView(btnLeft, createGridParams(1, 0));
        dpadLayout.addView(new Space(this), new GridLayout.LayoutParams(GridLayout.spec(1), GridLayout.spec(1)));
        dpadLayout.addView(btnRight, createGridParams(1, 2));
        dpadLayout.addView(new Space(this), new GridLayout.LayoutParams(GridLayout.spec(2), GridLayout.spec(0)));
        dpadLayout.addView(btnDown, createGridParams(2, 1));
        dpadLayout.addView(new Space(this), new GridLayout.LayoutParams(GridLayout.spec(2), GridLayout.spec(2)));

        RelativeLayout.LayoutParams fireParams = new RelativeLayout.LayoutParams(160, 160);
        fireParams.addRule(RelativeLayout.ALIGN_PARENT_BOTTOM);
        fireParams.addRule(RelativeLayout.ALIGN_PARENT_RIGHT);
        fireParams.setMargins(0, 0, 100, 100);
        btnFire.setLayoutParams(fireParams);

        overlay.addView(dpadLayout);
        overlay.addView(btnFire);

        btnUp.setOnTouchListener((v, event) -> { handleTouch(event, 1, 0, 0, 0, 0); return true; });
        btnDown.setOnTouchListener((v, event) -> { handleTouch(event, 0, 1, 0, 0, 0); return true; });
        btnLeft.setOnTouchListener((v, event) -> { handleTouch(event, 0, 0, 1, 0, 0); return true; });
        btnRight.setOnTouchListener((v, event) -> { handleTouch(event, 0, 0, 0, 1, 0); return true; });
        btnFire.setOnTouchListener((v, event) -> { handleTouch(event, 0, 0, 0, 0, 1); return true; });

        return overlay;
    }

    private Button createTransparentButton(String text) {
        Button btn = new Button(this);
        btn.setText(text);
        btn.setTextSize(24);
        btn.getBackground().setAlpha(90);
        btn.setTextColor(Color.WHITE);
        btn.setPadding(0, 0, 0, 0);
        return btn;
    }

    private GridLayout.LayoutParams createGridParams(int row, int col) {
        GridLayout.LayoutParams params = new GridLayout.LayoutParams(GridLayout.spec(row), GridLayout.spec(col));
        params.width = 110;
        params.height = 110;
        return params;
    }

    private void handleTouch(MotionEvent event, int up, int down, int left, int right, int fire) {
        if (event.getAction() == MotionEvent.ACTION_DOWN) {
            nativeOnJoystick(up, down, left, right, fire);
        } else if (event.getAction() == MotionEvent.ACTION_UP) {
            nativeOnJoystick(0, 0, 0, 0, 0);
        }
    }

    @Override
    protected void onPause() {
        super.onPause();
        running = false;
        if (renderThread != null) {
            try { renderThread.join(100); } catch (InterruptedException e) { }
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
        nativeOnDestroy();
    }

    @Override
    public boolean dispatchKeyEvent(KeyEvent event) {
        int code = event.getKeyCode();
        int action = event.getAction();
        boolean down = (action == KeyEvent.ACTION_DOWN);
        boolean up = (action == KeyEvent.ACTION_UP);

        switch (code) {
            case KeyEvent.KEYCODE_DPAD_UP: if (down) nativeOnJoystick(1, 0, 0, 0, 0); else if (up) nativeOnJoystick(0, 0, 0, 0, 0); return true;
            case KeyEvent.KEYCODE_DPAD_DOWN: if (down) nativeOnJoystick(0, 1, 0, 0, 0); else if (up) nativeOnJoystick(0, 0, 0, 0, 0); return true;
            case KeyEvent.KEYCODE_DPAD_LEFT: if (down) nativeOnJoystick(0, 0, 1, 0, 0); else if (up) nativeOnJoystick(0, 0, 0, 0, 0); return true;
            case KeyEvent.KEYCODE_DPAD_RIGHT: if (down) nativeOnJoystick(0, 0, 0, 1, 0); else if (up) nativeOnJoystick(0, 0, 0, 0, 0); return true;
            case KeyEvent.KEYCODE_BUTTON_A:
            case KeyEvent.KEYCODE_BUTTON_X:
            case KeyEvent.KEYCODE_BUTTON_Y:
            case KeyEvent.KEYCODE_BUTTON_B: if (down) nativeOnJoystick(0, 0, 0, 0, 1); else if (up) nativeOnJoystick(0, 0, 0, 0, 0); return true;
            case KeyEvent.KEYCODE_BUTTON_START:
            case KeyEvent.KEYCODE_BUTTON_SELECT: if (down) nativeOnReset(); return true;
        }
        return super.dispatchKeyEvent(event);
    }

    @Override
    public boolean onGenericMotionEvent(MotionEvent event) {
        if ((event.getSource() & android.view.InputDevice.SOURCE_JOYSTICK) == android.view.InputDevice.SOURCE_JOYSTICK) {
            float x = event.getAxisValue(MotionEvent.AXIS_X);
            float y = event.getAxisValue(MotionEvent.AXIS_Y);
            if (Math.abs(x) < 0.3f && Math.abs(y) < 0.3f) { nativeOnJoystick(0, 0, 0, 0, 0); return true; }
            int u = (y < -0.5f) ? 1 : 0;
            int d = (y > 0.5f) ? 1 : 0;
            int l = (x < -0.5f) ? 1 : 0;
            int r = (x > 0.5f) ? 1 : 0;
            nativeOnJoystick(u, d, l, r, 0);
            return true;
        }
        return super.onGenericMotionEvent(event);
    }

    // ✅ تم حذف onTouchEvent القديم بالكامل لأنه كان يسبب الجلچات

    @Override
    public void surfaceCreated(SurfaceHolder holder) {
        if (renderThread != null && renderThread.isAlive()) return;
        nativeOnSurfaceCreated(holder.getSurface());
        running = true;
        renderThread = new Thread(this::renderLoop);
        renderThread.start();
    }

    @Override
    public void surfaceChanged(SurfaceHolder holder, int format, int width, int height) {
        nativeOnSurfaceChanged(width, height);
    }

    @Override
    public void surfaceDestroyed(SurfaceHolder holder) {
        running = false;
        if (renderThread != null) {
            try { renderThread.join(); } catch (InterruptedException e) { }
            renderThread = null;
        }
        nativeOnSurfaceDestroyed();
    }

    private void renderLoop() {
        while (running) {
            nativeOnFrame();
            try { Thread.sleep(16); } catch (InterruptedException e) { break; }
        }
        nativeOnRenderThreadExit();
    }
}