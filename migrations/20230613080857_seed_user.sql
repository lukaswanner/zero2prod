-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
  '8310b501-9d96-43d6-aadd-837076033c0c',
  'admin',
  '$argon2id$v=19$m=15000,t=2,p=1$JFUvlY1A7+FJXUPBmDDajw$qUB85PXKqv01JTDo1+jFUA/orKJkfE6e3t5Wb5NmdMs'
);