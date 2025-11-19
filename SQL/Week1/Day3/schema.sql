DROP TABLE IF EXISTS players;
DROP TABLE IF EXISTS player_health_status_dialogs;

CREATE TABLE players (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    level INT NOT NULL,
    health INT NOT NULL,
    max_health INT NOT NULL,
    is_online BOOLEAN NOT NULL
);

CREATE TABLE player_health_status_dialogs (
    id SERIAL PRIMARY KEY,
    player_name TEXT NOT NULL,
    dead TEXT NOT NULL,
    near_dead TEXT NOT NULL,
    critical TEXT NOT NULL,
    wounded TEXT NOT NULL,
    scratch TEXT NOT NULL,
    healthy TEXT NOT NULL
);

INSERT INTO players (name, level, health, max_health, is_online) VALUES
('Astarion', 11, 32, 40, true),
('Karlach', 15, 97, 110, true),
('Shadowheart', 10, 20, 45, false),
('Wyll', 12, 0, 50, true),
('Lae''zel', 14, 85, 90, false);

INSERT INTO player_health_status_dialogs (player_name, dead, near_dead, critical, wounded, scratch, healthy) VALUES
('Astarion', 'Astarion Dead', 'Astarion Near Dead', 'Astarion Critical', 'Astarion Wounded', 'Astarion Scratch', 'Astarion Healthy'),
('Karlach', 'Karlach Dead', 'Karlach Near Dead', 'Karlach Critical', 'Karlach Wounded', 'Karlach Scratch', 'Karlach Healthy'),
('Shadowheart', 'Shadowheart Dead', 'Shadowheart Near Dead', 'Shadowheart Critical', 'Shadowheart Wounded', 'Shadowheart Scratch', 'Shadowheart Healthy'),
('Wyll', 'Wyll Dead', 'Wyll Near Dead', 'Wyll Critical', 'Wyll Wounded', 'Wyll Scratch', 'Wyll Healthy'),
('Lae''zel', 'Lae''zel Dead', 'Lae''zel Near Dead', 'Lae''zel Critical', 'Lae''zel Wounded', 'Lae''zel Scratch', 'Lae''zel Healthy');


