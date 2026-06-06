-- rest-starter の data.sql 相当
INSERT INTO company_position(name) VALUES ('CEO');
INSERT INTO company_position(name) VALUES ('Manager');
INSERT INTO company_position(name) VALUES ('Staff');

INSERT INTO member(name, company_position_id) VALUES ('John', 1);
INSERT INTO member(name, company_position_id) VALUES ('Daniel', 2);
INSERT INTO member(name, company_position_id) VALUES ('Lisa', 3);
