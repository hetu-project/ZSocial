// 1. formula存储3041事件状态
// k : 3041_event-id_state
// v:  event_state

// // 创建一个空的 HashMap
//let mut map = HashMap::new();
//
// // 插入键值对
// map.insert("key1", "value1");
// map.insert("key2", "value2");
//
// // 获取值
// match map.get("key1") {
// Some(value) => println!("The value of key1 is: {}", value),
// None => println!("Key1 not found"),
// }

use std::collections::HashMap;
use std::str::Utf8Error;
use std::string::ToString;
use nostr_kv::lmdb::Db;

use nostr_kv::{Error, lmdb::{Db as Lmdb, Iter as LmdbIter, *}, scanner::{Group, GroupItem, MatchResult, Scanner}};
use proto::zchronod::Event;

use serde::{Serialize, Deserialize};
use log::info;

pub struct ZchronodDb {
    inner: Db,
    state: Tree,
}

#[derive(Serialize, Deserialize)]
pub struct OptionState {
    map: HashMap<String, i32>,
    // option_name : vote_num
    event: Event,
}

type Result<T, E = Error> = core::result::Result<T, E>;

const TREE_NAME: &str = "3041";

impl ZchronodDb {
    // kind_301_poll is to init kv
    // kind_309_vote is to update value
    // k : 3041_event-id_state
    // v:  event_state
    // event_state is  map[option_name]vote_num, event
    pub fn init() -> Result<Self> {
        let lmdb = Db::open("./db")?;
        let state = lmdb.open_tree(Some(TREE_NAME), 0)?;
        Ok(ZchronodDb {
            inner: lmdb,
            state,
        })
    }

    pub fn writer(&self) -> Result<Writer> {
        Ok(self.inner.writer()?)
    }

    pub fn reader(&self) -> Result<Reader> {
        Ok(self.inner.reader()?)
    }


    // let my_path = "./my_file_sglk";
    // let db = Db::open(my_path)?;
    // // let _t = db.open_tree(None, 0)?;
    // let t1 = db.open_tree(Some("t2"), 0)?;
    //
    // // let mut writer = db.writer()?;
    // // writer.put(&t1,b"k1", b"v1")?;
    // let reader = db.reader()?;
    // let _v2 = reader.get(&t1,"k1")?.unwrap();
    // println!("{:?}",std::str::from_utf8(_v2));
    // init kv

    // all poll_id in db is poll_id=event id, vec<string>

    //   let event_id_str = String::from_utf8_lossy(&event_id);

    // fn get_vote_null_option(e: Event) -> HashMap<String, i32> {
    //
    // }
    pub fn poll_write(&self, key: String, e: Event) -> Result<(), Error> {
        let reader = self.inner.reader()?;
        if reader.get(&self.state, key.clone())?.is_none() {
            let mut writer = self.inner.writer()?;
            // convert option_state to json, and write as bytes
            let o_s = OptionState {               //todo
                map: Default::default(),    // to generate option
                event: e.clone(),
            };
            let option_state = serde_json::to_string(&o_s).unwrap();
            writer.put(&self.state, key.clone(), option_state);
            match reader.get(&self.state, "poll_id".to_string())? {
                Some(t) => {
                    let mut poll_id_list: Vec<Vec<u8>> = serde_json::from_str(std::str::from_utf8(t).unwrap()).unwrap();
                    poll_id_list.push(e.id.clone());
                    writer.put(&self.state, "poll_id".to_string(), serde_json::to_string(&poll_id_list).unwrap());
                }
                None => { writer.put(&self.state, "poll_id".to_string(), e.id.clone()); }
            }
            writer.commit()?;
        }
        Ok(())
    }

    pub fn vote_write(&self, key: &str, e: Event) -> Result<(), Error> { //todo
        let reader = self.inner.reader()?;
        if reader.get(&self.state, key.to_string())?.is_none() {
            let mut writer = self.inner.writer()?;
            // convert option_state to json, and write as bytes
            let o_s = OptionState {
                map: Default::default(),    // to generate option
                event: e.clone(),
            };
            let option_state = serde_json::to_string(&o_s).unwrap();
            writer.put(&self.state, key.to_string(), option_state);
            match reader.get(&self.state, "poll_id".to_string())? {
                Some(t) => {
                    let mut poll_id_list: Vec<Vec<u8>> = serde_json::from_str(std::str::from_utf8(t).unwrap()).unwrap();
                    poll_id_list.push(e.id.clone());
                    writer.put(&self.state, "poll_id".to_string(), serde_json::to_string(&poll_id_list).unwrap());
                }
                None => { writer.put(&self.state, "poll_id".to_string(), e.id.clone()); }
            }
            writer.commit()?;
        }
        Ok(())
    }
    pub fn query_all_event_id(&self) -> Result<(Vec<Vec<u8>>), Error> {
        let reader = self.inner.reader()?;
        match reader.get(&self.state, "poll_id".to_string())? {
            Some(t) => {
                let poll_id_list: Vec<Vec<u8>> = serde_json::from_str(std::str::from_utf8(t).unwrap()).unwrap();
                Ok::<Vec<Vec<u8>>, Error>(poll_id_list)
            }
            None => {
                println!("find none in query_all_event_id");
                info!("find none in query_all_event_id");
                Ok(vec![vec![]])
            }
        }.expect("err in query_all_event_id");

        Ok(vec![vec![]])
    }

    fn write_3041_db(&self, key: &str, option_state: HashMap<String, i32>) -> Result<(), Error> {
        let reader = self.inner.reader()?;
        let mut op_state = "".to_string();
        match reader.get(&self.state, key.to_string())? {
            None => {
                return Err(Error::Message("failed to get state in db".to_string()));
            }
            Some(t) => {
                let state_bytes = reader.get(&self.state, key.to_string());
                match reader.get(&self.state, key.to_string()) {
                    Ok(s) => {
                        match std::str::from_utf8(s.unwrap()) {
                            Ok(i) => { op_state = i.to_string() }
                            Err(_) => {
                                return Err(Error::Message("failed to transfer to string".to_string()));
                            }
                        }
                    }
                    Err(_) => { return Err(Error::Message("failed to get state in db".to_string())); }// 如果结果是 Err，则返回错误消息
                }
                //  op_state = std::str::from_utf8(state_bytes);
            }
        }


        Ok(())
    }

    fn query_by_event_id() {}
}