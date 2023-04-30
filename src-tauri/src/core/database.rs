use crate::utils::dirs::app_data_dir;
use crate::utils::string_util;
use anyhow::Result;
use rusqlite::{ffi::SQLITE_OPEN_CREATE, Connection, OpenFlags};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    path::Path,
};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ImageDataDB {
    pub width: usize,
    pub height: usize,
    pub base64: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, PartialEq)]
pub struct Record {
    pub id: u64,
    pub content: String,
    pub content_preview: Option<String>,
    // data_type(文本=text、图片=image)
    pub data_type: String,
    pub md5: String,
    pub active_time: u64,
    pub pined: bool,
    pub tags: String,
    // 仅在搜索返回时使用
    pub content_highlight: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct QueryReq {
    pub key: Option<String>,
    pub limit: Option<usize>,
    pub pined: Option<bool>,
    pub tags: Option<Vec<String>>,
}

pub struct SqliteDB {
    conn: Connection,
}

const SQLITE_FILE: &str = "data.sqlite";

#[allow(unused)]
impl SqliteDB {
    pub fn new() -> Self {
        // println!("new fn");
        let dir_path = app_data_dir().unwrap();
        let data_dir = dir_path.join(SQLITE_FILE);

        if !Path::new(&dir_path).exists() {
            fs::create_dir_all(&dir_path).unwrap();
        }

        let c = Connection::open_with_flags(
            &data_dir,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )
        .unwrap();
        let s = SqliteDB { conn: c };
        s
    }

    pub fn init() {
        let dir_path = app_data_dir().unwrap();
        let data_dir = dir_path.join(SQLITE_FILE);
        if !Path::new(&dir_path).exists() {
            fs::create_dir_all(&dir_path).unwrap();
        }
        let c = Connection::open_with_flags(
            data_dir,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )
        .unwrap();
        let sql = r#"
        create table if not exists record
        (
            id          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            content     TEXT,
            content_preview     TEXT,
            data_type   VARCHAR(20) DEFAULT '',
            md5         VARCHAR(200) DEFAULT '',
            active_time INTEGER,
            pined INTEGER DEFAULT 0,
            tags        VARCHAR(256) DEFAULT ''
        );
        "#;
        c.execute(sql, ()).unwrap();
    }

    pub fn insert_record(&self, r: Record) -> Result<i64> {
        let sql = "insert into record (content,md5,active_time,pined,data_type,content_preview) values (?1,?2,?3,?4,?5,?6)";
        let md5 = string_util::md5(r.content.as_str());
        let now = chrono::Local::now().timestamp_millis() as u64;
        let content_preview = r.content_preview.unwrap_or("".to_string());
        let res = self.conn.execute(
            sql,
            (
                &r.content,
                md5,
                now,
                &r.pined,
                &r.data_type,
                content_preview,
            ),
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    fn find_record_by_md5(&self, md5: String) -> Result<Record> {
        let sql = "SELECT id, content, md5, active_time, pined FROM record WHERE md5 = ?1";
        let r = self.conn.query_row(sql, [md5], |row| {
            Ok(Record {
                id: row.get(0)?,
                ..Default::default()
            })
        })?;
        Ok(r)
    }

    // 更新时间
    fn update_record_active_time(&self, r: Record) -> Result<()> {
        let sql = "update record set active_time = ?2 where id = ?1";
        // 获取当前毫秒级时间戳
        let now = chrono::Local::now().timestamp_millis() as u64;
        self.conn.execute(sql, [&r.id, &now])?;
        Ok(())
    }

    pub fn insert_if_not_exist(&self, r: Record) -> Result<()> {
        let md5 = string_util::md5(r.content.as_str());
        match self.find_record_by_md5(md5) {
            Ok(res) => {
                self.update_record_active_time(res)?;
            }
            Err(_e) => {
                self.insert_record(r)?;
            }
        }
        Ok(())
    }

    pub fn md5_is_exist(&self, md5: String) -> Result<bool> {
        let sql = "SELECT count(*) FROM record WHERE md5 = ?1";
        let count: u32 = self.conn.query_row(sql, [md5], |row| row.get(0))?;
        Ok(count > 0)
    }

    // 清除数据
    pub fn clear_data(&self) -> Result<()> {
        let sql = "delete from record where pined = 0";
        self.conn.execute(sql, ())?;
        Ok(())
    }

    pub fn delete_by_id(&self, id: u64) -> Result<()> {
        let sql = "delete from record where id = ?1";
        self.conn.execute(sql, [&id])?;
        Ok(())
    }

    // 标记为收藏,如有已经收藏了的则取消收藏
    pub fn mark_pined(&self, id: u64) -> Result<()> {
        let record = self.find_by_id(id)?;
        let sql = "update record set pined = ?2 where id = ?1";
        let pined = if record.pined { 0 } else { 1 };
        self.conn.execute(sql, [&id, &pined])?;
        Ok(())
    }

    pub fn save_tags(&self, id: u64, tags: String) -> Result<()> {
        let sql = "update record set tags = ?2 where id = ?1";
        self.conn.execute(sql, (&id, &tags))?;
        Ok(())
    }

    pub fn find_all(&self) -> Result<Vec<Record>> {
        let sql = "SELECT id, content_preview, data_type, md5, active_time, pined, tags FROM record order by active_time desc";
        let mut stmt = self.conn.prepare(sql)?;
        let mut rows = stmt.query([])?;
        let mut res = vec![];
        while let Some(row) = rows.next()? {
            let data_type: String = row.get(2)?;
            let content: String = row.get(1)?;
            let tags: String = row.get(6)?;
            let r = Record {
                id: row.get(0)?,
                content,
                content_preview: None,
                data_type,
                md5: row.get(3)?,
                active_time: row.get(4)?,
                pined: row.get(5)?,
                content_highlight: None,
                tags,
            };
            res.push(r);
        }
        Ok(res)
    }

    pub fn find_by_key(&self, req: QueryReq) -> Result<Vec<Record>> {
        let mut sql: String = String::new();
        sql.push_str(
            "SELECT id, content_preview, md5, active_time, pined, data_type, tags FROM record where 1=1",
        );
        let mut limit: usize = 300;
        let mut params: Vec<String> = vec![];
        if let Some(l) = req.limit {
            limit = l;
        }
        params.push(limit.to_string());
        if let Some(k) = &req.key {
            params.push(format!("%{}%", k));
            sql.push_str(
                format!(" and data_type='text' and content like ?{}", params.len()).as_str(),
            );
        }
        if let Some(is_fav) = req.pined {
            let is_fav_int = if is_fav { 1 } else { 0 };
            params.push(is_fav_int.to_string());
            sql.push_str(format!(" and pined = ?{}", params.len()).as_str());
        }
        if let Some(tags) = req.tags {
            for tag in tags.iter() {
                params.push(format!("%{}%", tag));
                sql.push_str(format!(" and tags like ?{}", params.len()).as_str());
            }
        }
        let sql = format!("{} order by active_time desc limit ?1", sql);
        let mut stmt = self.conn.prepare(&sql)?;
        let mut rows = stmt.query(rusqlite::params_from_iter(params))?;
        let mut res = vec![];
        while let Some(row) = rows.next()? {
            let data_type: String = row.get(5)?;
            let content: String = row.get(1)?;
            let tags: String = row.get(6)?;
            let content_highlight = req
                .key
                .as_ref()
                .map(|key| string_util::highlight(key.as_str(), content.as_str()));
            let r = Record {
                id: row.get(0)?,
                content,
                content_preview: None,
                data_type,
                md5: row.get(2)?,
                active_time: row.get(3)?,
                pined: row.get(4)?,
                content_highlight,
                tags,
            };
            res.push(r);
        }
        Ok(res)
    }

    //删除超过limit的记录
    pub fn delete_over_limit(&self, limit: usize) -> Result<bool> {
        // 先查询count，如果count - limit > 50 就删除 超出limit部分记录 主要是防止频繁重建数据库
        let mut stmt = self
            .conn
            .prepare("SELECT count(*) FROM record where pined = 0")?;
        let mut rows = stmt.query([])?;
        let count: usize = rows.next()?.unwrap().get(0).unwrap();
        if count < 10 + limit {
            return Ok(false);
        }
        let remove_num = count - limit;
        let sql = "DELETE FROM record WHERE pined = 0 and id in (SELECT id FROM record where pined = 0 order by active_time asc limit ?1)";
        self.conn.execute(sql, [remove_num])?;
        Ok(true)
    }

    pub fn delete_older_than_days(&self, days: i64) -> Result<bool> {
        let sql = "DELETE FROM record WHERE active_time < ?";

        let days_ago =
            (chrono::Local::now() - chrono::Duration::days(days)).timestamp_millis() as u64;
        match self.conn.execute(sql, [days_ago]) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn find_by_id(&self, id: u64) -> Result<Record> {
        let sql = "SELECT id, content, data_type, md5, active_time, pined, tags FROM record where id = ?1";
        let r = self.conn.query_row(sql, [&id], |row| {
            Ok(Record {
                id: row.get(0)?,
                content: row.get(1)?,
                content_preview: None,
                data_type: row.get(2)?,
                md5: row.get(3)?,
                active_time: row.get(4)?,
                pined: row.get(5)?,
                content_highlight: None,
                tags: row.get(6)?,
            })
        })?;
        Ok(r)
    }
}

#[test]
fn test_sqlite_insert() {
    SqliteDB::init();
    let r = Record {
        content: "123456".to_string(),
        md5: "e10adc3949ba59abbe56e057f20f883e".to_string(),
        active_time: 1234568,
        ..Default::default()
    };
    assert_eq!(SqliteDB::new().insert_record(r).unwrap(), 1_i64)
}
