CREATE TABLE IF NOT EXISTS answers (
    id uuid PRIMARY KEY,
    content TEXT NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    question_id uuid REFERENCES questions
);

