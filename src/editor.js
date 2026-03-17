const invoke = window.__TAURI__.core.invoke;

let currentData = null;

async function init() {
  try {
    const saved = await invoke("get_tasks");
    
    // Use window.tasksData (from tasks.js) as the master template
    const template = JSON.parse(JSON.stringify(window.tasksData));

    if (saved && saved !== null) {
      // Merge saved tasks into the template to preserve appearance/labels
      currentData = {
        ...template,
        dailyTasks: saved.dailyTasks || [],
        weekProjects: saved.weekProjects || [],
        twoWeekProjects: saved.twoWeekProjects || []
      };
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
    div.innerHTML = `
      <input type="text" placeholder="Name" value="${p.name}" oninput="updateProject('${id}', ${i}, 'name', this.value)" style="flex: 2">
      <input type="text" placeholder="Start" value="${p.start}" oninput="updateProject('${id}', ${i}, 'start', this.value)" style="flex: 0.5">
      <input type="text" placeholder="End" value="${p.end}" oninput="updateProject('${id}', ${i}, 'end', this.value)" style="flex: 0.5">
      <button class="danger" onclick="removeProject('${id}', ${i})">×</button>
    `;
    container.appendChild(div);
  });
}

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
  currentData[key].push({ name: "", start: 0, end: 1, color: "#4a90d9" });
  render();
};

window.removeProject = (containerId, i) => {
  currentData[containerId].splice(i, 1);
  render();
};

window.updateProject = (containerId, i, key, val) => {
  let finalVal = val;
  if (key === 'start' || key === 'end') finalVal = parseInt(val) || 0;
  currentData[containerId][i][key] = finalVal;
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
