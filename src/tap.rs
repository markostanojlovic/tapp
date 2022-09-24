use chrono::{Utc, offset::Local, TimeZone};
use serde_json::{Value, from_str, *};
use std::{fs::OpenOptions, io::Write};

 struct TapStorage {}
 
 impl TapStorage {
     const TAP_HISTORY_FILE_PATH: &'static str = "/tmp/tap_history_file.json"; 
 
     // TODO can I utilise serialise/deserialise here? 
     pub fn read_tap_storage() -> Option<Value> {
         let read_from_storage = std::fs::read_to_string(TapStorage::TAP_HISTORY_FILE_PATH)
                                                                 .unwrap_or("{}".to_string());
         let tap_data: Value = from_str(&read_from_storage).unwrap();
         if tap_data == json!({}) {
             None
         } else { 
             Some(tap_data)
         }
     }
     pub fn write_to_tap_storage(buf: String) {
        let mut f = OpenOptions::new()
                                .create(true)
                                .write(true)
                                .truncate(true)
                                .open(TapStorage::TAP_HISTORY_FILE_PATH).expect("err opening file");
        let _ = f.write(buf.as_bytes());
     }
 }
 
 pub struct Tap {}
 
 impl Tap {
     fn init_data() -> Value {
         json!({"num_taps": 0, "taps": []})
     }
     
     fn read_storage() -> Value {
         match TapStorage::read_tap_storage() {
             Some(history) => history,
             None => Tap::init_data(),
         }
     }
 
     pub fn now(comment: &str) -> String {
         let now: i64 = Local::now().timestamp(); // unix format i64 
         let old_tap_data: Value = Tap::read_storage();
         let new_tap: Value = json!({"timestamp": now, "comment": comment});
         let mut tap_vec = old_tap_data["taps"].as_array().unwrap().clone();
         tap_vec.push(new_tap);
         // TODO what if there is no field num_taps? how to handle it?
         let num_taps: u8 = from_str::<u8>(&old_tap_data["num_taps"].to_string()).unwrap() + 1;
         let updated_data = json!({"num_taps": num_taps, "taps": tap_vec});
         let upd_data_pretty = to_string_pretty(&updated_data).unwrap();
         TapStorage::write_to_tap_storage(upd_data_pretty.clone());
         upd_data_pretty
     }
 
     pub fn history() -> String {
         let tap_data: Value = TapStorage::read_tap_storage().unwrap();
         let tap_vec = tap_data["taps"].as_array().unwrap().clone();
         let mut hr_history: String = String::new();
         for tap_item in tap_vec.iter() {
             let mut td = Utc.timestamp(tap_item["timestamp"].as_i64().unwrap(), 0).to_string();
             td.push_str(" with commment: ");
             let comment: String = tap_item["comment"].to_string();
             td.push_str(&comment);
             td.push_str("\n");
             hr_history.push_str(&td);
         }
         hr_history
     }
 }
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_init_data() {
         let initialised_data = Tap::init_data();
         let expected_data: Value = from_str("{\"num_taps\": 0, \"taps\": []}").unwrap();
         assert_eq!(initialised_data, expected_data);
     }
 
 }
