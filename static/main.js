let db = null;

function viewNotes() {
  const tx = db.transaction("personal_notes", "readonly");
  const pNotes = tx.objectStore("personal_notes");
  const request = pNotes.openCursor();
  request.onsuccess = (e) => {
    const cursor = e.target.result;

    if (cursor) {
      alert(`Title: ${cursor.key} Text: ${cursor.value.text} `);
      //do something with the cursor
      cursor.continue();
    }
  };
}

export function addPupil(id, name, photos_bytes_array) {
  const pupil = {
    id,
    name,
    photos_bytes_array,
  };

  const tx = window.indexedPupilDatabase.transaction("pupils", "readwrite");
  tx.onerror = (e) => alert(` Error! ${e.target.error}  `);
  const pNotes = tx.objectStore("pupils");
  pNotes.add(pupil);
}

export function createDB(name) {
  const request = indexedDB.open(name, 1);

  //on upgrade needed
  request.onupgradeneeded = (e) => {
    db = e.target.result;
    window.indexedPupilDatabase = db;
    /* note = {
                        title: "note1",
                        text: "this is a note"
                    }*/
    const pNotes = db.createObjectStore("pupils", {
      keyPath: "id",
    });

    console.log(
      `upgrade is called database name: ${db.name} version : ${db.version}`
    );
  };
  //on success
  request.onsuccess = (e) => {
    db = e.target.result;
    window.indexedPupilDatabase = db;
    console.log(
      `success is called database name: ${db.name} version : ${db.version}`
    );
  };
  //on error
  request.onerror = (e) => {
    alert(`error: ${e.target.error} was found `);
  };
}
createDB("pupilsDB");
// addPupil(1, "name", [13131312131, 312113123, , 31214141]);
