INSERT INTO user_profile (id, catchphrase)
VALUES (?1, ?2)
ON CONFLICT(id) DO UPDATE SET catchphrase=?2