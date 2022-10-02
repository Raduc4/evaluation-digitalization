export function addPupil(id, name, photos_bytes_array) {
  const pupil = { id, name, photos_bytes_array };
  const tx = window.indexedPupilDatabase.transaction("pupils", "readwrite");
  tx.onerror = (e) => alert(` Error! ${e.target.error} `);
  const pNotes = tx.objectStore("pupils");
  pNotes.add(pupil);
  console.log("cf");
}
