-- Exercise 1: Basic SELECT
SELECT
    name,
    level
FROM players;

-- Exercise 2: Basic Filtering with WHERE
SELECT
    name,
    health,
    max_health
FROM players WHERE health < (max_health/2);

-- Exercise 3: CASE WHEN Status Field
SELECT
    name,
    health,
    CASE
        WHEN health = 0 THEN 'Dead'
        WHEN health < max_health * 0.25 THEN 'Critical'
        ELSE 'Healthy'
    END AS status
FROM players;

--Exercise 4 Filter WHERE and CASE WHEN
SELECT
    name,
    health,
    max_health,
    health*1.0/max_health AS health_ratio,
    CASE
        WHEN health = 0 THEN 'Dead'
        WHEN health < max_health * 0.25 THEN 'Critical'
        ELSE 'Healthy'
    END AS status
FROM players WHERE health < max_health * 0.5;

--Exercicio 5 ORDER BY, ROUND, STRING conversion, nested commands, multiple tables
SELECT
    name,
    health AS current_health,
    max_health,
    ROUND(100.0*health/max_health,2) || '%' AS health_percentage,
    (SELECT 
        (CASE
            WHEN health <= 0 THEN dead
            WHEN health = 1 THEN near_dead
            WHEN health/max_health < 0.25 THEN critical
            WHEN health/max_health < 0.90 THEN wounded
            WHEN health/max_health < 100 THEN scratch
            ELSE healthy
        END) FROM player_health_status_dialogs WHERE player_name = name) AS dialog
FROM players ORDER BY health_percentage;