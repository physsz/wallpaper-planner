# Development Journal: Building a Native Desktop Overlay with Tauri

This journal documents the transformation of a static HTML dashboard into a standalone macOS desktop application. 

---

## 1. The Architectural Shift
**The Problem:** The original project relied on **Plash** (a third-party app) to render HTML as a wallpaper. This created "friction": users had to install another app, manually point it to files, and manually refresh when they updated their tasks.

**The Solution:** Use **Tauri v2**. 
Tauri allows us to write our UI in standard Web tech (HTML/JS/CSS) while using **Rust** to perform "privileged" system operations—like telling macOS to treat our window as part of the desktop.

---

## 2. Directory Structure
A modern Tauri project separates the "Frontend" (what you see) from the "Core" (the native logic).

```text
wallpaper-planner/
├── src/                <-- FRONTEND: Your original dashboard logic
│   ├── index.html      <-- The UI layout (Glassmorphism + SVG)
│   ├── tasks.js        <-- The "Database" (User-edited config)
│   └── wallpaper.jpg   <-- Static assets
├── src-tauri/          <-- NATIVE CORE: The Rust implementation
│   ├── Cargo.toml      <-- Rust dependencies (cocoa, objc, notify)
│   ├── tauri.conf.json <-- App configuration (Window size, permissions)
│   └── src/
│       ├── main.rs     <-- The entry point (starts the app)
│       └── lib.rs      <-- THE "BRAINS": All native logic lives here
├── package.json        <-- Node scripts (npm run dev/build)
└── node_modules/       <-- Tauri CLI and development tools
```

---

## 3. Implementation Logic: The "Desktop Pinning" Magic
The most complex part of this app is making a window "sticky" on the desktop. In macOS, every window has a "Level." Standard apps are at `kCGNormalWindowLevel`. We need to move ours to `kCGDesktopWindowLevel`.

In `src-tauri/src/lib.rs`, we used the **Cocoa** crate to talk directly to macOS:

```rust
// 1. Get the raw macOS window handle (NSWindow)
let ns_window = window.ns_window().unwrap() as id;

unsafe {
  // 2. Set the Level to 'Desktop' (-2147483624)
  // This places the window behind all other apps and icons.
  let _: () = msg_send![ns_window, setLevel: -2147483624];

  // 3. Set Collection Behavior
  // .stationary: Doesn't move when switching Spaces.
  // .canJoinAllSpaces: Visible on every virtual desktop.
  // .ignoresCycle: Doesn't show up in Cmd+Tab.
  let mut behavior = NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                   | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
                   | NSWindowCollectionBehavior::NSWindowCollectionBehaviorIgnoresCycle;
  ns_window.setCollectionBehavior_(behavior);

  // 4. Enable Click-Through
  // This tells macOS to ignore mouse events and pass them to whatever is underneath.
  let _: () = msg_send![ns_window, setIgnoresMouseEvents: true];
}
```

---

## 4. The Live-Update Logic (User Experience)
Since users edit `tasks.js` manually, we wanted a "save-to-apply" experience. We implemented a **File Watcher** using the `notify` crate.

1.  **The Watcher**: Rust monitors the `src/` folder for any "Write" events.
2.  **The Trigger**: When `tasks.js` is saved, Rust sends a command to the webview: `window.eval("location.reload()")`.
3.  **The Result**: The UI refreshes instantly, re-reading the new data from `tasks.js`.

---

## 5. Lessons for Future Apps
If you want to build another app using this as a template, remember these three "Pillars of Tauri":

*   **Pillar 1: Capabilities**: In Tauri v2, permissions are strict. If you want to open a file (like the "Edit Tasks" button does), you *must* explicitly allow it in `src-tauri/capabilities/default.json`.
*   **Pillar 2: The Setup Hook**: The `.setup(|app| { ... })` block in `lib.rs` is where you initialize everything that should happen *before* the user sees the window (like the pinning and the tray icon).
*   **Pillar 3: Transparency**: To get a truly "borderless" feel, you must set `transparent: true` and `decorations: false` in `tauri.conf.json`. Without this, you'll have a white background and a title bar.
