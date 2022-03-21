-- feedback -------
---------------
---------------
CREATE TABLE feedbacks (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    message TEXT
);
