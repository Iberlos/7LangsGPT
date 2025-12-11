# SQL (PostgreSQL) Overview
Version: PostgreSQL 16.x (current stable as of 2025)

---

## What PostgreSQL Is
PostgreSQL is a powerful open-source relational database system known for reliability, strong ACID compliance, extensibility, and advanced SQL features. It is widely used in production environments for analytics, transactional systems, geospatial data, and large-scale web applications.

---

## Core Strengths
- Fully ACID-compliant with strong consistency guarantees
- Rich SQL feature set (CTEs, window functions, JSONB, triggers)
- Extremely reliable and battle-tested
- Strong extensibility (custom types, functions, extensions)
- Built-in full-text search
- Excellent for OLTP and OLAP workloads
- First-class support for JSON and semi-structured data

## Weaknesses
- Requires careful indexing to achieve high performance
- Not ideal for real-time massive streaming ingestion (use specialized tools)
- Vertical scaling limits unless partitioning/sharding is used
- More operational overhead than embedded databases

---

## Common Use Cases
- Backend databases for web applications (Django, Rails, Node)
- Financial systems that require strong consistency
- Analytics pipelines using window functions and CTEs
- GIS / geolocation (PostGIS extension)
- Storing structured + semi-structured data (JSONB)
- Home automation configuration/state storage

---

## Syntax Highlights

### Selecting Data
    SELECT id, name, created_at
    FROM users
    WHERE active = TRUE
    ORDER BY created_at DESC;

### Inserting Data
    INSERT INTO users (name, email)
    VALUES ('Alex', 'alex@example.com');

### Updating Data
    UPDATE users
    SET last_login = NOW()
    WHERE id = 42;

### Deleting Data
    DELETE FROM users
    WHERE id = 42;

### SQL Conditional Logic
    SELECT
        name,
        CASE
            WHEN score >= 90 THEN 'S'
            WHEN score >= 75 THEN 'A'
            WHEN score >= 60 THEN 'B'
            ELSE 'C'
        END AS grade
    FROM students;

---

## Naming Conventions
| Object Type | Style            | Example            |
|-------------|------------------|--------------------|
| tables      | snake_case       | user_profiles      |
| columns     | snake_case       | created_at         |
| constraints | lowercase        | fk_user_profile    |
| functions   | snake_case       | calculate_score    |

---

## Best Practices
- Always define primary keys (usually SERIAL or IDENTITY)
- Use UUIDs for distributed systems
- Prefer `BIGINT` over `INTEGER` for long-term scalability
- Use JSONB for flexible data, but keep core data relational
- Use `EXPLAIN ANALYZE` to investigate performance
- Add indexes for frequently-used WHERE and JOIN columns
- Avoid SELECT * in production queries
- Use transactions for multi-step writes

---

## Tooling Ecosystem
- Administration: psql, pgAdmin, TablePlus, DBeaver
- Backup/restore: pg_dump, pg_restore
- Extensions: PostGIS, pgvector, hstore, uuid-ossp
- Migration Tools: Flyway, Liquibase, Prisma, Django Migrations
- Cloud Providers: AWS RDS, GCP Cloud SQL, Azure Database for PostgreSQL

---

## Mini Reference Tables

### Common Data Types
| Type       | Example / Notes                  |
|------------|----------------------------------|
| INTEGER    | 42                               |
| BIGINT     | Large counters, IDs              |
| VARCHAR    | VARCHAR(255)                     |
| TEXT       | Unlimited text                   |
| BOOLEAN    | TRUE / FALSE                     |
| DATE       | YYYY-MM-DD                       |
| TIMESTAMP  | Date + time                      |
| JSONB      | Binary JSON                      |
| UUID       | Universally unique identifier    |

### Useful SQL Keywords
| Keyword     | Meaning                   |
|-------------|---------------------------|
| SELECT      | query data                |
| JOIN        | combine tables            |
| GROUP BY    | aggregate                 |
| HAVING      | filter aggregates         |
| ORDER BY    | sort results              |
| LIMIT/OFFSET| pagination                |
| RETURNING   | return inserted/updated rows |

---

## Real-World Examples

### Game Development Example: High Score Leaderboard
    SELECT
        player_name,
        score,
        RANK() OVER (ORDER BY score DESC) AS ranking
    FROM leaderboard
    LIMIT 10;

### Game State Tracking
    INSERT INTO player_state (player_id, position, hp)
    VALUES (7, '(10,12)', 85)
    ON CONFLICT (player_id)
    DO UPDATE SET
        position = EXCLUDED.position,
        hp = EXCLUDED.hp;

---

### Home Automation Example: Log Motion Events
    INSERT INTO motion_logs (room, detected_at)
    VALUES ('kitchen', NOW());

    SELECT room, COUNT(*)
    FROM motion_logs
    WHERE detected_at > NOW() - INTERVAL '24 hours'
    GROUP BY room;

---

## When to Choose PostgreSQL
Choose PostgreSQL when:
- You need a reliable, enterprise-grade relational database
- You want strong consistency and complex transactions
- You need advanced querying (window functions, CTEs)
- You require structured + semi-structured data (JSONB)
- You want long-term scalability and SQL power

Avoid PostgreSQL if:
- You need extremely high ingestion rates (use ClickHouse, Kafka, or NoSQL)
- You require horizontal scaling without careful planning
- You need an embedded or zero-ops database

---
