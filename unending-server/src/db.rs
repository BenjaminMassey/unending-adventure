use crate::data;

const DB_FILE: &str = "data.db";

pub fn initialize() {
    let conn = rusqlite::Connection::open(DB_FILE).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS areas (
             uuid TEXT PRIMARY KEY,
             name TEXT,
             description TEXT,
             quest_uuids TEXT)",
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
    let string_area = data::StringArea::from_area(area);
    let mut conn = rusqlite::Connection::open(DB_FILE).unwrap();
    let tx = conn.transaction().unwrap();
    tx.execute(
        "INSERT INTO areas (uuid, name, description, quest_uuids) VALUES (?1, ?2, ?3, ?4)",
        [
            &string_area.uuid,
            &string_area.name,
            &string_area.description,
            &string_area.to_comma_separated_quest_ids(),
        ],
    ).unwrap();
    tx.commit().unwrap();
}

pub fn get_area_by_uuid(uuid: uuid::Uuid) -> data::Area {
    let conn = rusqlite::Connection::open(DB_FILE).unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM areas WHERE uuid = ?1")
        .unwrap();
    let mut rows = stmt.query([&uuid.hyphenated().to_string()]).unwrap();
    let row = rows.next().unwrap().unwrap(); // TODO: better dynamicness
    let quest_ids_string: String = row.get(3).unwrap();
    let string_area = data::StringArea {
        uuid: row.get(0).unwrap(),
        name: row.get(1).unwrap(),
        description: row.get(2).unwrap(),
        quest_ids: data::StringArea::from_comma_separated_quest_ids(&quest_ids_string),
    };
    string_area.to_area()
}

pub fn get_random_area() -> data::Area {
    let conn = rusqlite::Connection::open(DB_FILE).unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM areas WHERE rowid IN (SELECT rowid FROM areas ORDER BY RANDOM() LIMIT 1)")
        .unwrap();
    // https://stackoverflow.com/a/24591688
    let mut rows = stmt.query([]).unwrap();
    let row = rows.next().unwrap().unwrap(); // TODO: better dynamicness
    let quest_ids_string: String = row.get(3).unwrap();
    let string_area = data::StringArea {
        uuid: row.get(0).unwrap(),
        name: row.get(1).unwrap(),
        description: row.get(2).unwrap(),
        quest_ids: data::StringArea::from_comma_separated_quest_ids(&quest_ids_string),
    };
    string_area.to_area()
}

pub fn add_quest(quest: &data::Quest) {
    let string_quest = data::StringQuest::from_quest(quest);
    let mut conn = rusqlite::Connection::open(DB_FILE).unwrap();
    let tx = conn.transaction().unwrap();
    tx.execute(
        "INSERT INTO quests (uuid, area_uuid, type, giver, description, number, enemy, item, boss, npc) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        [
            &string_quest.uuid,
            &string_quest.area_id,
            &string_quest.the_type,
            &string_quest.giver,
            &string_quest.description,
            &string_quest.number,
            &string_quest.enemy,
            &string_quest.item,
            &string_quest.boss,
            &string_quest.npc,
        ],
    ).unwrap();
    tx.commit().unwrap();
}

pub fn get_quest_by_uuid(uuid: uuid::Uuid) -> data::Quest {
    let conn = rusqlite::Connection::open(DB_FILE).unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM quests WHERE uuid = ?1")
        .unwrap();
    let mut rows = stmt.query([&uuid.hyphenated().to_string()]).unwrap();
    let row = rows.next().unwrap().unwrap(); // TODO: better dynamicness
    let string_quest = data::StringQuest {
        uuid: row.get(0).unwrap(),
        area_id: row.get(1).unwrap(),
        the_type: row.get(2).unwrap(),
        giver: row.get(3).unwrap(),
        description: row.get(4).unwrap(),
        number: row.get(5).unwrap(),
        enemy: row.get(6).unwrap(),
        item: row.get(7).unwrap(),
        boss: row.get(8).unwrap(),
        npc: row.get(9).unwrap(),
    };
    string_quest.to_quest()
}