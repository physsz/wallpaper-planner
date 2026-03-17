# Wallpaper Planner (Standalone Desktop App)

A productivity planner overlay for your macOS desktop wallpaper. It layers Gantt-style project timelines and a daily task list directly on top of your existing wallpaper, turning your desktop into an always-visible planning dashboard.

![Planner with wallpaper](examples/planner+wallpaper.png)

This is a **modern standalone desktop app** built with [Tauri v2](https://v2.tauri.app/). It "pins" itself to your desktop level, stays visible across all spaces, and is completely click-through.

---

## Features

- **Desktop Overlay**: Sits behind your icons but above your wallpaper.
- **Click-Through**: You can interact with your desktop icons right through the planner.
- **Live Sync**: Edits to `tasks.js` are detected instantly; the wallpaper updates the moment you save.
- **Menu Bar Integration**: Access "Edit Tasks" and "Quit" directly from your macOS menu bar.
- **No Dependencies**: No need for Plash, Chrome, or shell scripts.

---

## Getting Started

### 1 — Install Dependencies

You'll need [Node.js](https://nodejs.org/) and [Rust](https://www.rust-lang.org/tools/install) installed on your Mac.

```bash
# Clone the repo
git clone https://github.com/physsz/wallpaper-planner.git
cd wallpaper-planner

# Install Node dependencies
npm install
```

### 2 — Run the App

```bash
npm run dev
```

The planner will appear on your desktop. You'll also see a new icon in your menu bar.

### 3 — Build for Production

To create a standalone `.app` or `.dmg`:

```bash
npm run build
```

---

## Configuration (`src/tasks.js`)

The app watches `src/tasks.js`. To change your tasks:

1. Click the **Menu Bar Icon** (top right of your screen).
2. Select **Edit Tasks**.
3. Update your tasks or Gantt bars in your text editor.
4. Save the file — the desktop overlay will refresh instantly.

### Appearance Options

In `src/tasks.js`, you can customize everything:

```js
appearance: {
  wallpaper:      "wallpaper.jpg", // filename in src/, or "" for transparent
  scrimOpacity:   0.38,            // dark overlay: 0.0 (none) → 1.0 (black)
  panelOpacity:   0.62,            // glass fill:   0.0 (clear) → 1.0 (solid)
  panelBlur:      22,              // backdrop blur in px
  panelRadius:    14,              // corner radius in px
}
```

---

## Old Method (Plash / Shell Script)

The original files are still available in the `src/` directory if you prefer using [Plash](https://sindresorhus.com/plash) or the `export-wallpaper.sh` script.

---

## Project Structure

```
wallpaper-planner/
├── src/                ← Web assets (HTML/JS/CSS)
│   ├── index.html      ← UI Layout
│   └── tasks.js        ← Your configuration (EDIT THIS)
├── src-tauri/          ← Native macOS logic (Rust)
├── package.json        ← App scripts
└── examples/           ← Screenshots
```
