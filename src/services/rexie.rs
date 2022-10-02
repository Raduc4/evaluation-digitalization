use rexie::*;
use yew::Context;

use crate::pages::home::Home;

pub async fn build_database(component: &mut Home) -> () {
    // Create a new database
    let rexie = Rexie::builder("test")
        // Set the version of the database to 1.0
        .version(1)
        // Add an object store named `employees`
        .add_object_store(
            ObjectStore::new("employees")
                // Set the key path to `id`
                .key_path("id")
                // Enable auto increment
                .auto_increment(true)
                // Add an index named `email` with the key path `email` with unique enabled
                .add_index(Index::new("email", "email").unique(true)),
        )
        // Build the database
        .build()
        .await
        .unwrap();

    // Check basic details of the database
    // assert_eq!(rexie.name(), "test");
    // assert_eq!(rexie.version(), 1.0);
    // assert_eq!(rexie.store_names(), vec!["employees"]);

    ()
}
