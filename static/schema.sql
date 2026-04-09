
-- Table to hold the name of the possible assignees
CREATE TABLE IF NOT EXISTS assignees (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    assignee_name TEXT NOT NULL,
);

-- Table to hold the name of the chores
CREATE TABLE IF NOT EXISTS chores (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    chore_name TEXT NOT NULL,
    description TEXT,
);

CREATE TABLE chore_assignments (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    assignee_id INTEGER NOT NULL,
    chore_id INTEGER NOT NULL,
    assigned_date DATE DEFAULT (date('now')),
    FOREIGN KEY (chore_id) REFERENCES chores(id) ON DELETE CASCADE,
    FOREIGN KEY (assignee_id) REFERENCES assignees(id) ON DELETE CASCADE,
    UNIQUE (chore_id, assignee_id)
);
