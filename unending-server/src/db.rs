use std::str::FromStr;

use crate::data;

const DB_FILE: &str = "data.db";

pub fn initialize() {
    let conn = rusqlite::Connection::open(DB_FILE).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS areas (
             uuid TEXT PRIMARY KEY,
             name TEXT,
             description TEXT)",
        [],
    ).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS quests (
             uuid TEXT PRIMARY KEY,
             area_uuid TEXT,
             type TEXT,
             giver TEXT,
             description TEXT,
             number TEXT,
             enemy TEXT,
             item TEXT,
             boss TEXT,
             npc TEXT)",
        [],
    ).unwrap();
}

pub fn add_area(area: &data::Area) {
    let mut conn = rusqlite::Connection::open(DB_FILE).unwrap();
    let tx = conn.transaction().unwrap();
    tx.execute(
        "INSERT INTO areas (uuid, name, description) VALUES (?1, ?2, ?3)",
        [&area.id.hyphenated().to_string(), &area.name, &area.description],
    ).unwrap();
    tx.commit().unwrap();
}

pub fn get_area(uuid: uuid::Uuid) -> data::Area {
    let conn = rusqlite::Connection::open(DB_FILE).unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM areas WHERE uuid = ?1")
        .unwrap();
    let mut rows = stmt.query([&uuid.hyphenated().to_string()]).unwrap();
    let row = rows.next().unwrap().unwrap(); // TODO: better dynamicness
    let name: String = row.get(1).unwrap();
    let description: String = row.get(2).unwrap();
    data::Area {
        id: uuid,
        name,
        description,
        quests: vec![], // TODO: shouldn't exist, or call some get_quest fn
    }
}

pub fn add_quest(quest: &data::Quest) {
    let area_id_string = 
        if let Some(id) = quest.area_id { 
            id.hyphenated().to_string()
        } else { 
            String::new()
        };
    let mut conn = rusqlite::Connection::open(DB_FILE).unwrap();
    let tx = conn.transaction().unwrap();
    tx.execute(
        "INSERT INTO quests (uuid, area_uuid, type, giver, description, number, enemy, item, boss, npc) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        [
            &quest.id.hyphenated().to_string(),
            &area_id_string,
            &format!("{:?}", quest.the_type),
            &quest.giver,
            &quest.description, // TODO: some helper functions for below or something better in general
            &{if let Some(number) = quest.number { number.to_string() } else { String::new() }},
            &{if let Some(enemy) = &quest.enemy { enemy.to_owned() } else { String::new() }},
            &{if let Some(item) = &quest.item { item.to_owned() } else { String::new() }},
            &{if let Some(boss) = &quest.boss { boss.to_owned() } else { String::new() }},
            &{if let Some(npc) = &quest.npc { npc.to_owned() } else { String::new() }},
        ],
    ).unwrap();
    tx.commit().unwrap();
}

pub fn get_quest(uuid: uuid::Uuid) -> data::Quest {
    let conn = rusqlite::Connection::open(DB_FILE).unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM quests WHERE uuid = ?1")
        .unwrap();
    let mut rows = stmt.query([&uuid.hyphenated().to_string()]).unwrap();
    let row = rows.next().unwrap().unwrap(); // TODO: better dynamicness
    let area_id_string: String = row.get(1).unwrap();
    let area_id = if area_id_string.is_empty() { None } else { Some(uuid::Uuid::from_str(&area_id_string).unwrap()) };
    let the_type_string: String = row.get(2).unwrap();
    let the_type = data::QuestType::from_str(&the_type_string).unwrap();
    let giver: String = row.get(3).unwrap();
    let description: String = row.get(4).unwrap();
    let number_string: String = row.get(5).unwrap();
    let number = if number_string.is_empty() { None } else { Some(number_string.parse::<u8>().unwrap()) };
    let enemy_string: String = row.get(6).unwrap();
    let enemy = if enemy_string.is_empty() { None } else { Some(enemy_string) };
    let item_string: String = row.get(7).unwrap();
    let item = if item_string.is_empty() { None } else { Some(item_string) };
    let boss_string: String = row.get(8).unwrap();
    let boss = if boss_string.is_empty() { None } else { Some(boss_string) };
    let npc_string: String = row.get(9).unwrap();
    let npc = if npc_string.is_empty() { None } else { Some(npc_string) };
    data::Quest {
        id: uuid,
        area_id,
        the_type,
        giver,
        description,
        number,
        enemy,
        item,
        boss,
        npc,
    }
}