/**
 * Storage本地存储
 * 保存一个json对象
 * 1.基本的存取操作
 * 2.过期清理
 */

use std::io::prelude::*;
use std::fs::File;
use serde_json;
use serde_json::json;
use serde::{Deserialize, Serialize};

pub struct Storage;

const FILE_PATH: &str = "storage.txt";

#[derive(Serialize, Deserialize)]
struct Data {
  credentials_key: String,
}

impl Storage {
  fn get_file() -> Option<File> {
    let mut file = File::open(FILE_PATH);
    match file {
      Ok(f) => Some(f),
      Err(e) => {
        println!("open file error: {:#?}", e);
        let mut new_file = File::create(FILE_PATH);
        match new_file {
          Ok(v) => Some(v),
          Err(e) => println!("create file error: {:#?}", e),
        }
      }
    }
  }

  pub fn set_item(key: &str, val: &str) {
    let mut file = Storage::get_file();
    match file {
      Some(f) => {
        let mut buf = String::new();
        f.read_to_string(&mut buf);
        let data: Data = serde_json::from_str(&buf).unwrap();
        
      },
      None => {},
    }
  }

  pub fn get_item(key: &str) -> String {
    return "".to_string();
  }

  pub fn remove_item(key: &str) {

  }
}
