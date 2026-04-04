const invoke = window.__TAURI__.core.invoke;

let currentData = null;

function clone(value) {
  return JSON.parse(JSON.stringify(value));
}

function mergeNestedSection(templateSection, savedSection) {
  const templateValue = templateSection && typeof templateSection === 'object' ? clone(templateSection) : {};
  const savedValue = savedSection && typeof savedSection === 'object' ? savedSection : {};
  return {
    ...templateValue,
    ...savedValue
  };
}

function mergeEditorData(template, saved) {
  if (!saved || saved === null) return clone(template);

  return {
    ...clone(template),
    ...saved,
    appearance: mergeNestedSection(template.appearance, saved.appearance),
    labels: mergeNestedSection(template.labels, saved.labels),
    privacy: mergeNestedSection(template.privacy, saved.privacy),
    dailyTasks: Array.isArray(saved.dailyTasks) ? saved.dailyTasks : [],
    weekProjects: Array.isArray(saved.weekProjects) ? saved.weekProjects : [],
    twoWeekProjects: Array.isArray(saved.twoWeekProjects) ? saved.twoWeekProjects : []
  };
}

async function init() {
  try {
    const saved = await invoke("get_tasks");
    
    // Use window.tasksData (from tasks.js) as the master template
    const template = clone(window.tasksData);

    if (saved && saved !== null) {
      // Merge saved content into the template so nested settings survive editor saves.
      currentData = mergeEditorData(template, saved);
    } else {
      // First time: use template with cleared tasks
      currentData = template;
      currentData.dailyTasks = [];
      currentData.weekProjects = [];
      currentData.twoWeekProjects = [];
    }
    render();
  } catch (e) {
    console.error("Failed to load tasks:", e);
  }
}

function render() {
  const viewModeEl = document.getElementById('viewMode');
  if (viewModeEl) viewModeEl.value = currentData.appearance.viewMode || 'rolling';

  const taskList = document.getElementById('taskList');
  if (!taskList) return;
  taskList.innerHTML = '';
  
  currentData.dailyTasks.forEach((t, i) => {
    const div = document.createElement('div');
    div.className = 'task-row';
    div.innerHTML = `
      <input type="checkbox" ${t.done ? 'checked' : ''} onchange="updateTask(${i}, 'done', this.checked)">
      <input type="text" value="${t.text}" oninput="updateTask(${i}, 'text', this.value)">
      <button class="danger" onclick="removeTask(${i})">×</button>
    `;
    taskList.appendChild(div);
  });

  renderProjects('weekProjects', currentData.weekProjects);
  renderProjects('twoWeekProjects', currentData.twoWeekProjects);
}

function renderProjects(id, projects) {
  const container = document.getElementById(id);
  if (!container) return;
  container.innerHTML = '';
  projects.forEach((p, i) => {
    const div = document.createElement('div');
    div.className = 'task-row';
    div.style.flexDirection = 'column';
    div.style.alignItems = 'stretch';
    div.style.background = 'rgba(0,0,0,0.1)';
    div.style.padding = '10px';
    div.style.borderRadius = '8px';
    div.style.gap = '5px';

    div.innerHTML = `
      <div style="display: flex; gap: 10px;">
        <input type="text" placeholder="Project Name" value="${p.name}" oninput="updateProject('${id}', ${i}, 'name', this.value)" style="flex: 1">
        <button class="danger" onclick="removeProject('${id}', ${i})">×</button>
      </div>
      <div style="display: flex; gap: 10px; font-size: 0.8rem; color: #94a3b8;">
        <div style="flex: 1">
          <label style="display:block;margin-bottom:2px">Start Date</label>
          <input type="date" value="${p.start}" oninput="updateProject('${id}', ${i}, 'start', this.value)" style="width: 100%">
        </div>
        <div style="flex: 1">
          <label style="display:block;margin-bottom:2px">Deadline</label>
          <input type="date" value="${p.deadline || ''}" oninput="updateProject('${id}', ${i}, 'deadline', this.value)" style="width: 100%">
        </div>
      </div>
    `;
    container.appendChild(div);
  });
}

window.updateViewMode = (val) => {
  currentData.appearance.viewMode = val;
};

window.addTask = () => {
  currentData.dailyTasks.push({ text: "", done: false });
  render();
};

window.removeTask = (i) => {
  currentData.dailyTasks.splice(i, 1);
  render();
};

window.updateTask = (i, key, val) => {
  currentData.dailyTasks[i][key] = val;
};

window.addProject = (type) => {
  const key = type === 'week' ? 'weekProjects' : 'twoWeekProjects';
  const today = new Date().toISOString().split('T')[0];
  currentData[key].push({ name: "", start: today, deadline: today, color: "#4a90d9" });
  render();
};

window.removeProject = (containerId, i) => {
  currentData[containerId].splice(i, 1);
  render();
};

window.updateProject = (containerId, i, key, val) => {
  currentData[containerId][i][key] = val;
};

window.saveData = async () => {
  try {
    await invoke("save_tasks", { data: currentData });
    const emit = window.__TAURI__.event.emit;
    await emit('tasks-updated', currentData);
    window.close();
  } catch (e) {
    console.error("Failed to save tasks:", e);
  }
};

init();
