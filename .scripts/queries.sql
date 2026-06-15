SELECT * FROM users;

UPDATE users
SET status = 'confirmed'
WHERE email = 'kostiantyn.salnykov@gmail.com'
RETURNING *;

SELECT * FROM wishlists;