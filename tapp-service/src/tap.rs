use chrono::{Utc, offset::Local, TimeZone};
use serde_json::{Value, from_str, *};
use std::{fs::OpenOptions, io::Write};

 struct TapStorage {}
 
 impl TapStorage {
     const TAP_HISTORY_FILE_PATH: &'static str = "/tmp/tap_history.json"; 
 
     // Try to read file path and if there is an problem reading it (no file), it will return None
     // if file exists it will try to convert read string to a JSON object
     pub fn read(file_path: &str) -> Option<Value> {
         let read_from_storage = std::fs::read_to_string(file_path)
                                                                 .unwrap_or("{}".to_string());
         let tap_data: Value = from_str(&read_from_storage).unwrap_or(json!({}));
         if tap_data == json!({}) {
             None
         } else { 
             Some(tap_data)
         }
     }

     pub fn write(buf: String, file_path: &str) {
        let mut f = OpenOptions::new()
                                .create(true)
                                .write(true)
                                .truncate(true)
                                .open(file_path).expect("err opening file");
        let _ = f.write(buf.as_bytes());
     }
 }
 
 pub struct Tap {}
 
 impl Tap {
     fn init_data() -> Value {
         json!({"num_taps": 0, "taps": []})
     }
     
     fn read_storage() -> Value {
         match TapStorage::read(TapStorage::TAP_HISTORY_FILE_PATH) {
             Some(history) => history,
             None => Tap::init_data(),
         }
     }

     fn write_to_tap_storage(tap_data: String) {
        TapStorage::write(tap_data, TapStorage::TAP_HISTORY_FILE_PATH)
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
         Tap::write_to_tap_storage(upd_data_pretty.clone());
         upd_data_pretty
     }
 
     pub fn history() -> String {
         let tap_data: Value = TapStorage::read(TapStorage::TAP_HISTORY_FILE_PATH).unwrap();
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

     #[test]
     fn test_tapstroage_read_bad_path() {
        let tp_read = TapStorage::read("/tmp/none");
        assert_eq!(tp_read, None)
     }
 
 }
