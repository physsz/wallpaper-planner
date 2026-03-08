#set document(title: "Wallpaper Planner — User Manual", author: "")
#set page(
  paper: "a4",
  margin: (x: 2.8cm, y: 2.5cm),
  header: context {
    if counter(page).get().first() > 1 {
      grid(
        columns: (1fr, 1fr),
        align(left, text(size: 8pt, fill: luma(140), "Wallpaper Planner")),
        align(right, text(size: 8pt, fill: luma(140), counter(page).display("1"))),
      )
      line(length: 100%, stroke: 0.4pt + luma(210))
    }
  },
  footer: []
)

#set text(font: "New Computer Modern", size: 10.5pt, lang: "en")
#set heading(numbering: "1.1")
#set par(leading: 0.75em, justify: true)
#show heading.where(level: 1): it => {
  v(1.4em)
  text(size: 14pt, weight: "bold", it)
  v(0.4em)
}
#show heading.where(level: 2): it => {
  v(0.9em)
  text(size: 11.5pt, weight: "bold", fill: rgb("#1d4ed8"), it)
  v(0.25em)
}
#show heading.where(level: 3): it => {
  v(0.6em)
  text(size: 10.5pt, weight: "bold", it)
  v(0.15em)
}
#show raw: it => box(
  fill: luma(245),
  inset: (x: 5pt, y: 3pt),
  radius: 3pt,
  text(font: "Courier New", size: 9.5pt, it)
)
#show raw.where(block: true): it => block(
  fill: luma(245),
  inset: (x: 12pt, y: 10pt),
  radius: 4pt,
  width: 100%,
  text(font: "Courier New", size: 9pt, it)
)

// ── Cover ──────────────────────────────────────────────────────────
#align(center)[
  #v(3cm)
  #text(size: 28pt, weight: "bold")[Wallpaper Planner]
  #v(0.4cm)
  #text(size: 13pt, fill: luma(100))[User Manual]
  #v(0.6cm)
  #line(length: 8cm, stroke: 1pt + rgb("#1d4ed8"))
  #v(0.5cm)
  #text(size: 10pt, fill: luma(130))[A browser-rendered productivity dashboard \ displayed as your macOS desktop wallpaper]
  #v(3cm)
]

#pagebreak()

// ── TOC ────────────────────────────────────────────────────────────
#outline(indent: 1.5em, depth: 2)

#pagebreak()

// ══════════════════════════════════════════════════════════════════
= Overview

The Wallpaper Planner turns your Mac desktop into a live productivity dashboard. Three panels are always visible behind your windows, giving you a persistent view of:

- *Today's tasks* — a checklist for the current daily sprint
- *1-Week Horizon* — a Gantt chart of everything due this week
- *2-Week Radar* — a wider Gantt chart covering the next two weeks

The dashboard is a local HTML file rendered by *Plash*, a free macOS utility that displays web pages as desktop wallpaper. All content is controlled by a single plain-text file (`tasks.js`) — no coding required to keep it up to date.

== File Structure

#block(fill: luma(245), inset: (x: 12pt, y: 10pt), radius: 4pt, width: 100%)[
```
planning-wallpaper/
├── tasks.js          ← edit this daily
├── wallpaper.html    ← dashboard UI (edit for deeper customisation)
├── wallpaper.jpg     ← background image
└── manual.pdf        ← this document
```
]

#pagebreak()

// ══════════════════════════════════════════════════════════════════
= Setup

== Step 1 — Install Plash

1. Open the *Mac App Store* and search for _Plash_, or visit #link("https://sindresorhus.com/plash")[sindresorhus.com/plash].
2. Click *Get* and install. Plash is free.
3. Launch Plash. A small icon appears in the menu bar.

== Step 2 — Point Plash at the Dashboard

1. Click the Plash menu bar icon.
2. Choose *Open URL…*
3. Paste this path (adjust if you moved the folder):

```
file:///Users/YOUR_USERNAME/temp/planning-wallpaper/wallpaper.html
```

Replace `YOUR_USERNAME` with your macOS username.

4. Press *Enter*. The planning dashboard appears as your wallpaper immediately.

== Step 3 — Set Plash to Launch at Login _(optional)_

In the Plash menu: *Settings → Launch at Login*. The dashboard will be ready every time you start your Mac.

#pagebreak()

// ══════════════════════════════════════════════════════════════════
= Daily Workflow

The typical daily routine takes *under two minutes*:

+ Open `tasks.js` in any text editor (TextEdit, VS Code, Cursor, etc.)
+ Update `done:` flags for completed tasks
+ Add or remove tasks for the day
+ Adjust Gantt bar dates if project timelines shifted
+ Save the file
+ Click the Plash menu bar icon → *Reload*

The dashboard updates instantly.

== Morning Routine

At the start of each day:

1. Reset all task `done` values to `false`
2. Set `sprintName` to describe today's focus (e.g. `"Design Sprint Day 3"`)
3. Update `mustWinsRange` if your target count changed
4. Slide Gantt bars forward by one day (increment all `start`/`end` values by −1, or shift relative to today)

== Evening Shutdown Ritual

The yellow *Shutdown Ritual* note at the bottom of the Command Center panel reminds you what to do before closing up:

1. Mark completed tasks as `done: true`
2. Review Gantt bars and push any slipped deadlines
3. Prepare tomorrow's task list
4. Reload Plash to confirm the dashboard looks correct for tomorrow

#pagebreak()

// ══════════════════════════════════════════════════════════════════
= Editing `tasks.js`

Open `tasks.js` in any text editor. The structure is:

```js
window.tasksData = {
  appearance: { ... },   // visual settings
  sprintName: "...",
  mustWinsRange: "...",
  shutdownRitual: "...",
  dailyTasks: [ ... ],
  weekProjects: [ ... ],
  twoWeekProjects: [ ... ]
};
```

== Command Center (Left Panel)

=== Sprint Name

```js
sprintName: "Design Sprint Day 3",
```

Displayed under the *Command Center* heading as "TODAY: Design Sprint Day 3".

=== Must-Wins Badge

```js
mustWinsRange: "3-5",
```

Shown in the red badge at the bottom of the left panel. Use it to set your minimum/maximum non-negotiable completions for the day.

=== Shutdown Ritual Note

```js
shutdownRitual: "Check Deadlines, Set 3-5",
```

Displayed in the yellow note at the very bottom. Write whatever end-of-day reminder works for you.

=== Daily Task Checklist

```js
dailyTasks: [
  { text: "Review and prioritize inbox",   done: true  },
  { text: "Complete top 3 priority tasks", done: false },
  { text: "Team standup meeting",          done: false },
],
```

Each entry has:
- `text` — the label shown on screen
- `done` — `true` shows a filled ✓ circle with strikethrough; `false` shows an empty circle

You can have any number of tasks. Five to seven items is recommended for readability.

== Gantt Charts

Both Gantt panels use the same format. Days are *relative to today*:

#align(center)[
  #table(
    columns: (auto, auto),
    stroke: 0.4pt + luma(200),
    inset: 8pt,
    fill: (_, row) => if row == 0 { luma(240) } else { white },
    [*Value*], [*Meaning*],
    [`start: 0`], [Starts today],
    [`start: 2`], [Starts in 2 days],
    [`end: 5`],   [Ends in 5 days],
    [`end: 7`],   [Ends at the 7-day boundary (last column)],
  )
]

=== 1-Week Gantt (`weekProjects`)

Shows days 0 through 7. Use for tasks and deliverables due this week.

```js
weekProjects: [
  { name: "Website Redesign",  start: 0, end: 3, color: "#4a90d9" },
  { name: "Q1 Report Draft",   start: 1, end: 5, color: "#5ba85b" },
  { name: "Client Deadline",   start: 4, end: 7, color: "#e84a5f" },
],
```

=== 2-Week Gantt (`twoWeekProjects`)

Shows days 0 through 14. Use for longer-running milestones and upcoming deadlines.

```js
twoWeekProjects: [
  { name: "Prep Milestone",  start: 0,  end: 5,  color: "#4a90d9" },
  { name: "Launch Prep",     start: 6,  end: 12, color: "#e8734a" },
  { name: "Review & Ship",   start: 10, end: 14, color: "#e84a5f" },
],
```

=== Color Reference

#align(center)[
  #table(
    columns: (auto, auto, auto),
    stroke: 0.4pt + luma(200),
    inset: 8pt,
    fill: (_, row) => if row == 0 { luma(240) } else { white },
    [*Hex*],        [*Swatch*], [*Suggested Use*],
    [`#4a90d9`],    [#box(fill: rgb("#4a90d9"), width: 24pt, height: 10pt, radius: 2pt)], [Normal / in-progress],
    [`#5ba85b`],    [#box(fill: rgb("#5ba85b"), width: 24pt, height: 10pt, radius: 2pt)], [On-track / near done],
    [`#e8734a`],    [#box(fill: rgb("#e8734a"), width: 24pt, height: 10pt, radius: 2pt)], [At risk],
    [`#e84a5f`],    [#box(fill: rgb("#e84a5f"), width: 24pt, height: 10pt, radius: 2pt)], [Critical deadline],
    [`#9b6dff`],    [#box(fill: rgb("#9b6dff"), width: 24pt, height: 10pt, radius: 2pt)], [Blocked / waiting],
    [`#f0c040`],    [#box(fill: rgb("#f0c040"), width: 24pt, height: 10pt, radius: 2pt)], [On hold],
  )
]

#pagebreak()

// ══════════════════════════════════════════════════════════════════
= Appearance Settings

All visual controls live in the `appearance` block at the top of `tasks.js`. Save the file and reload Plash to see changes.

```js
appearance: {
  wallpaper:    "wallpaper.jpg",
  scrimOpacity: 0.38,
  panelOpacity: 0.62,
  panelBlur:    22,
},
```

== `wallpaper` — Background Image

The filename of the image to use as the background. The file must be in the same folder as `wallpaper.html`.

```js
wallpaper: "wallpaper.jpg"    // use this image
wallpaper: ""                 // transparent (shows macOS wallpaper behind Plash)
```

To use a different image, place it in the `planning-wallpaper/` folder and update the filename. Supported formats: JPG, PNG, WebP.

*Tip:* To convert a macOS `.heic` wallpaper to JPG, run in Terminal:
```
sips -s format jpeg /path/to/source.heic \
  --out ~/temp/planning-wallpaper/wallpaper.jpg
```

== `scrimOpacity` — Darkness Overlay

A semi-transparent black layer sits between the wallpaper and the panels. Increasing it darkens the wallpaper and improves text contrast.

#align(center)[
  #table(
    columns: (auto, auto),
    stroke: 0.4pt + luma(200),
    inset: 8pt,
    fill: (_, row) => if row == 0 { luma(240) } else { white },
    [*Value*], [*Effect*],
    [`0.0`],   [No overlay — raw wallpaper],
    [`0.20`],  [Light tint — wallpaper dominates],
    [`0.38`],  [*Default* — balanced contrast],
    [`0.55`],  [Heavy tint — strong readability],
    [`1.0`],   [Solid black — wallpaper invisible],
  )
]

== `panelOpacity` — Glass Panel Fill

Controls how opaque the frosted-glass panels are.

#align(center)[
  #table(
    columns: (auto, auto),
    stroke: 0.4pt + luma(200),
    inset: 8pt,
    fill: (_, row) => if row == 0 { luma(240) } else { white },
    [*Value*], [*Effect*],
    [`0.20`],  [Nearly transparent — maximum wallpaper bleed-through],
    [`0.40`],  [Light glass],
    [`0.62`],  [*Default* — balanced glass],
    [`0.80`],  [Solid-looking panels],
    [`1.0`],   [Fully opaque (no glass effect)],
  )
]

== `panelBlur` — Backdrop Blur

Controls the frosted-glass blur strength in pixels.

#align(center)[
  #table(
    columns: (auto, auto),
    stroke: 0.4pt + luma(200),
    inset: 8pt,
    fill: (_, row) => if row == 0 { luma(240) } else { white },
    [*Value*], [*Effect*],
    [`0`],     [No blur — sharp background visible through panels],
    [`10`],    [Light frost],
    [`22`],    [*Default* — standard frosted glass],
    [`40`],    [Heavy frost — background almost unrecognisable],
  )
]

== Showing the Live Aerial Wallpaper Behind Plash

To let macOS's live Aerial video show through the dashboard instead of a static image:

1. Set `wallpaper: ""` in `tasks.js`
2. In Plash: right-click the menu bar icon → *Settings* → enable *Transparent Background*
3. Set `scrimOpacity: 0.0` (or a small value like `0.2`) since the scrim won't have a static image to darken
4. Reload Plash

#pagebreak()

// ══════════════════════════════════════════════════════════════════
= Reloading After Changes

Every time you save `tasks.js`, reload Plash to apply the changes:

#align(center)[
  #table(
    columns: (auto, auto),
    stroke: 0.4pt + luma(200),
    inset: 8pt,
    fill: (_, row) => if row == 0 { luma(240) } else { white },
    [*Method*], [*How*],
    [Menu bar], [Click Plash icon → *Reload*],
    [Keyboard shortcut], [Assign one in Plash → Settings → Keyboard Shortcut],
  )
]

#pagebreak()

// ══════════════════════════════════════════════════════════════════
= Troubleshooting

== Dashboard is blank or shows an error

- Make sure the path in Plash starts with `file:///` (three slashes)
- Confirm `tasks.js` is in the same folder as `wallpaper.html`
- Open `wallpaper.html` in Safari or Chrome directly — any JavaScript errors will appear in Developer Tools (`Cmd+Option+I`)

== Wallpaper image not showing

- Confirm `wallpaper.jpg` (or your custom filename) exists in the `planning-wallpaper/` folder
- Check the `wallpaper:` value in `tasks.js` matches the exact filename including extension

== Panels are unreadable (too transparent or dark)

Adjust `scrimOpacity` and `panelOpacity` in `tasks.js`:
- If text is hard to read → increase `scrimOpacity` toward `0.55`
- If panels look too solid → decrease `panelOpacity` toward `0.40`

== Gantt bars are missing or wrong

- All `start` and `end` values must be within range: 0–7 for `weekProjects`, 0–14 for `twoWeekProjects`
- `start` must be less than `end`
- Check for missing commas or unclosed brackets in `tasks.js` — open the file in a code editor with syntax highlighting to spot errors quickly

== Dashboard does not update after saving `tasks.js`

Click the Plash icon → *Reload*. Plash does not watch for file changes automatically.

#pagebreak()

// ══════════════════════════════════════════════════════════════════
= Quick Reference

#align(center)[
  #table(
    columns: (auto, 1fr),
    stroke: 0.4pt + luma(200),
    inset: 8pt,
    fill: (_, row) => if row == 0 { luma(240) } else if calc.odd(row) { luma(250) } else { white },
    [*Task*], [*What to change*],
    [Update today's tasks],       [`dailyTasks` in `tasks.js`],
    [Mark a task done],           [`done: true` on that task entry],
    [Change sprint name],         [`sprintName` in `tasks.js`],
    [Change must-wins count],     [`mustWinsRange` in `tasks.js`],
    [Adjust project timelines],   [`start` / `end` in `weekProjects` or `twoWeekProjects`],
    [Change a project colour],    [`color` on that project entry],
    [Darken/lighten background],  [`scrimOpacity` in `appearance`],
    [Make panels more/less opaque], [`panelOpacity` in `appearance`],
    [Change blur strength],       [`panelBlur` in `appearance`],
    [Swap background image],      [`wallpaper` filename in `appearance`],
    [Reload the dashboard],       [Plash menu bar icon → *Reload*],
  )
]
