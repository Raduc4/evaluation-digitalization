use indexed_db_futures::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::DomException;

pub async fn example() -> Result<(), DomException> {
    // Open my_db v1
    let mut db_req: OpenDbRequest = IdbDatabase::open_u32("my_db", 1)?;
    db_req.set_on_upgrade_needed(Some(|evt: &IdbVersionChangeEvent| -> Result<(), JsValue> {
        // Check if the object store exists; create it if it doesn't
        if let None = evt.db().object_store_names().find(|n| n == "my_store") {
            evt.db().create_object_store("my_store")?;
        }
        Ok(())
    }));

    let db: IdbDatabase = db_req.into_future().await?;

    // Insert/overwrite a record
    let tx: IdbTransaction =
        db.transaction_on_one_with_mode("my_store", IdbTransactionMode::Readwrite)?;
    let store: IdbObjectStore = tx.object_store("my_store")?;

    // let value_to_put: JsValue = get_some_js_value();
    // store.put_key_val_owned("my_key", &value_to_put)?;

    // IDBTransactions can have an Error or an Abort event; into_result() turns both into a
    // DOMException
    tx.await.into_result()?;

    // Delete a record
    let tx = db.transaction_on_one_with_mode("my_store", IdbTransactionMode::Readwrite)?;
    let store = tx.object_store("my_store")?;
    store.delete_owned("my_key")?;
    tx.await.into_result()?;

    // Get a record
    let tx = db.transaction_on_one("my_store")?;
    let store = tx.object_store("my_store")?;

    let value: Option<JsValue> = store.get_owned("my_key")?.await?;
    // use_value(value);

    // All of the requests in the transaction have already finished so we can just drop it to
    // avoid the unused future warning, or assign it to _.
    let _ = tx;

    Ok(())
}
