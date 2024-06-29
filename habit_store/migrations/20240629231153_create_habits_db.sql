CREATE TABLE IF NOT EXISTS tasks (
    id INT NOT NULL PRIMARY KEY ,
    title VARCHAR(255) NOT NULL,
    task_type INT NOT NULL,
    status INT NOT NULL,
    description TEXT,
    due_date TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC') NOT NULL
);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = (CURRENT_TIMESTAMP AT TIME ZONE 'UTC');
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


DROP TRIGGER IF EXISTS update_task_updated_at ON tasks;
CREATE TRIGGER update_task_updated_at
BEFORE UPDATE
ON tasks
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();