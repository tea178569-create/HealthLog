CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    birth_date TEXT,
    gender TEXT,
    height_cm REAL
);

CREATE TABLE measurements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    metric_type TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    value REAL NOT NULL,
    source TEXT NOT NULL,
    is_outlier BOOLEAN NOT NULL DEFAULT 0
);

CREATE TABLE locations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    prefecture TEXT NOT NULL,
    venue_name TEXT NOT NULL,
    donation_type TEXT NOT NULL
);
