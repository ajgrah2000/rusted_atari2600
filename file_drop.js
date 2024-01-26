"use strict"
let x = 3;

const drag_start = ['dragenter', 'dragover'];
const drag_end = ['dragleave', 'drop'];
const all_drag = drag_start.concat(drag_end);

let dropArea = document.getElementById('rom_drop');

// Entering/Exiting drop area
all_drag.forEach(eventName => {
  dropArea.addEventListener(eventName, preventDefaults, false)
  });

function preventDefaults(e) {
  e.preventDefault();
  e.stopPropagation();
}

drag_start.forEach(eventName => {
  dropArea.addEventListener(eventName, highlight, false)
  });

drag_end.forEach(eventName => {
  dropArea.addEventListener(eventName, unhighlight, false)
  });

function highlight(e) {
  dropArea.classList.add('highlight');
}

function unhighlight(e) {
  dropArea.classList.remove('highlight');
}

// Handling the drop

dropArea.addEventListener('drop', handleDrop, false);
function handleDrop(e) {
  let dt = e.dataTransfer;
  let files = dt.files;

  console.log("Drop event" + e);

  handleFiles(files);
}

function handleFiles(files) {
  console.log("Files" + files.length);
  ([...files]).forEach(processDroppedFile);
}

function processDroppedFile(file) {
  let reader = new FileReader();
  reader.onload = function (event) { 
    handleNewFileData(new Uint8Array(event.target.result));
  }
  reader.readAsArrayBuffer(file);
  dropArea.classList.add('hide_border');
}

{
  console.log("Script started");
}

export {x}
