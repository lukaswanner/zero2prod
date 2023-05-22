-- We wrap the whole migration in a transatction to make sure
-- it succeeds or fails atomically.
-- Add migration script here
BEGIN;
  	-- Backfill 'status' for historical entries
  	UPDATE subscriptions
		SET status = 'confirmed'
		WHERE status IS NULL;
	-- Make 'status' mandatory
	ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;
