-- rest-starter の schema.sql 相当 (SQLite 版)
-- SQLite では:
--   - AUTO_INCREMENT → INTEGER PRIMARY KEY (ROWID alias, 自動採番される)
--   - VARCHAR(n)     → TEXT (SQLite は型サイズに緩い)
--   - 外部キーは PRAGMA foreign_keys=ON で有効化が必要だが、ここでは省略
DROP TABLE IF EXISTS member;
DROP TABLE IF EXISTS company_position;

CREATE TABLE company_position
(
    company_position_id INTEGER PRIMARY KEY,
    name                TEXT NOT NULL
);

CREATE TABLE member
(
    member_id           INTEGER PRIMARY KEY,
    name                TEXT NOT NULL,
    company_position_id INTEGER,
    FOREIGN KEY (company_position_id) REFERENCES company_position (company_position_id)
);
