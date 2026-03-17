// ============================================================
//  tasks.js — full configuration for the planning wallpaper
//  Edit this file, then reload Plash to apply changes.
// ============================================================
window.tasksData = {

  // ── APPEARANCE ───────────────────────────────────────────────────

  appearance: {

    // Background
    wallpaper: "wallpaper.jpg", // image filename (must be in this folder), or "" for none
    scrimOpacity: 0.0,            // dark overlay on wallpaper  — 0.0 (none) → 1.0 (black)

    // Panels (glass cards)
    panelOpacity: 0.0,            // glass fill opacity         — 0.0 (clear) → 1.0 (solid)
    panelBlur: 0,              // backdrop blur in px        — 0 (sharp)   → 40 (frosted)
    panelRadius: 14,              // corner radius in px
    panelGap: 14,              // gap between panels in px

    // Layout — use "1fr", "2fr", or "300px" for each column
    viewMode: "rolling", // "rolling" (7/14 days) or "fixed" (Calendar Week)
    columns: {
      left: "1fr",    // 2-week radar
      center: "600px",  // command center (today's tasks)
      right: "1fr",    // 1-week horizon
    },
    ganttLabelWidth: 118,            // px — label column inside each Gantt chart
    showTopbar: false,           // show/hide the date & time bar at the top

    // Colors
    textColor: "#f0f6ff",       // primary text
    mutedColor: "#94a3b8",       // secondary text, subtitles, labels
    todayLineColor: "#ef4444",       // "Today" marker line in Gantt charts
    mustWinsColor: "#b91c1c",       // must-wins badge background

    // Typography
    baseFontSize: 20,              // base font size in px — scales all text proportionally
    ganttLabelSize: 15,            // Gantt project name label size in px
    ganttLabelColor: "#dde6f5",       // Gantt project name label color
    ganttDaySize: 12,              // Gantt day-number label size in px
    ganttDayColor: "#94a3b8",       // Gantt day-number label color

    // Gantt chart
    ganttMaxRowHeight: 76,           // max height per row in px — prevents bars from bloating
    ganttBarOpacity: 0.92,         // bar fill opacity
  },

  // ── LABELS ──────────────────────────────────────────────────────
  // Rename any text visible on the wallpaper. Omit a key to keep the default.

  labels: {
    // Top panel headers
    commandTitle: "Today's Tasks",
    twoWeekTitle: "The Radar: 2 Weeks",
    twoWeekSubtitle: "Timeline",
    twoWeekDaysLeft: "14 Days Left",
    weekTitle: "The Horizon: 1 Week",
    weekSubtitle: "Timeline",
    weekDaysLeft: "7 Days Left",

    // Command Center text
    todayPrefix: "",       // shown as "TODAY: Daily Sprint"
    mustWinsLabel: "Must-Wins:",  // label above the red badge
    shutdownPrefix: "SHUTDOWN RITUAL",  // prefix on the yellow note
  },

  // ── Today's tasks (middle panel) ───────────────────────────────

  sprintName: "Personal focus",           // shown as "TODAY: ..."
  mustWinsRange: "3-5",                    // number shown in red badge
  shutdownRitual: "Check Deadlines, Set 3-5", // yellow note at the bottom

  // ── Privacy (today panel only) ──────────────────────────────────
  // This masks sensitive details on the desktop so other people only
  // see generic placeholders. It does not prevent someone from opening
  // tasks.js directly on your machine.
  privacy: {
    maskTodayTasks: false,
    privateTaskLabel: "task",
    hideSprintName: true,
    hiddenSprintLabel: "Personal Focus",
    hideShutdownRitual: true,
    hiddenShutdownLabel: "Review and reset",
  },

  dailyTasks: [
  ],

  // ── 1-WEEK GANTT (right panel) ──────────────────────────────────
  // days are relative to today: 0 = today, 7 = one week out
  // start must be < end; both must be within 0–7

  weekProjects: [
    //{ name: "", start: 0, end: 5, color: "#4a90d9" },

  ],

  // ── 2-WEEK GANTT (left panel) ───────────────────────────────────
  // days are relative to today: 0 = today, 14 = two weeks out
  // start must be < end; both must be within 0–14

  twoWeekProjects: [
    //{ name: "Preparation Milestone", start: 0, end: 4, color: "#4a90d9" },

  ],

};

// ── COLOR REFERENCE ─────────────────────────────────────────────
//   "#4a90d9"  blue    — normal / in-progress
//   "#5ba85b"  green   — on track / nearly done
//   "#e8734a"  orange  — at risk
//   "#e84a5f"  red     — critical / overdue
//   "#9b6dff"  purple  — blocked / waiting
//   "#f0c040"  yellow  — on hold
