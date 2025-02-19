// fn get_username(user_id: i32) -> Option<string> {
//     let query = format!("GET username FROM users Where id={user_id}");
//     let db_result = query_db(query);
//     db_result.ok()
// }
//
// fn query_db(query: String) -> Result<String, String> {
//     if query.is_empty() {
//         Err(String::from("Query string is empty"))
//     } else {
//         Ok(String::from("Fies"))
//     }
// }