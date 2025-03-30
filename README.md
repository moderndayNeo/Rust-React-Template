# This is a template for a full-stack Rust & React project.

## Overview

- BE: Rust HTTP Server
  - Actix-web, Serde, Tokio, Diesel, Sqlite
- FE: React + TypeScript + Vite
  - TailwindCSS, Shadcn/ui (components), Lucide-react (icons), React-hook-form, Zod (form validation)

## Important Notes

- Do not edit this template. Instead, whenever you need to create a project, clone this and edit the clone.
- The FE and BE each have their own `.env` files.

## Updating the DB schema ⬇️

- This project uses Diesel. Diesel is an ORM and query builder for Rust. It's designed to provide a safe and efficient way to interact with databases in Rust.
  https://diesel.rs/guides/getting-started

### Workflow for adding a new DB table:

1. Create a table for the new data type. Create a migration for that:

   `$ diesel migration generate create_ai_tools`

   Diesel creates the migration files to hold the up and down functions.

2. Write the SQL code to create the table in the `up` function (the `up.sql` file).

   ```
   CREATE TABLE ai_tools (
   id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
   name VARCHAR NOT NULL,
   company VARCHAR NOT NULL,
   description TEXT NOT NULL,
   image_url TEXT,
   );
   ```

3. Write the SQL code to drop the table in the `down` function (the `down.sql` file).

   `DROP TABLE posts;`

4. Apply the new migration:

   `$ diesel migration run`

5. Check the database to see if the new table was created:

   `$ sqlite3 ai_tools.db .tables`

6. Check the schema.rs file that Diesel created in the migrations folder. It should have a struct for the new table.

   ```
   diesel::table! {
               ai_tools (id) {
                   id -> Integer,
                   name -> Text,
                   company -> Text,
                   description -> Text,
                   image_url -> Nullable<Text>,
               }
           }
   ```

7. Create a model for the new table in the `src/models.rs` file.
   ```
   #[derive(Debug, Serialize, Deserialize, Queryable, Selectable, AsChangeset)]
   #[diesel(table_name = crate::schema::ai_tools)]
   #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
   pub struct AiTool {
       pub id: i32,
       pub name: String,
       pub company: String,
       pub description: String,
       pub image_url: Option<String>,
   }
   ```
8. Use your new model in your application code to interact with the new table:
   ```
   let tools = ai_tools
      .load::<AiTool>(&mut conn)
      .expect("Error loading AI tools");
   ```
