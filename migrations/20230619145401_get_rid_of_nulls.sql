ALTER TABLE user_organization
ALTER COLUMN user_id
SET NOT NULL;

ALTER TABLE user_organization
ALTER COLUMN organization_id
SET NOT NULL;

ALTER TABLE organization
ALTER COLUMN name
SET NOT NULL;

ALTER TABLE organization
ALTER COLUMN short_name
SET NOT NULL;

ALTER TABLE organization
ALTER COLUMN time_created
SET NOT NULL;

ALTER TABLE organization
ALTER COLUMN time_updated
SET NOT NULL;