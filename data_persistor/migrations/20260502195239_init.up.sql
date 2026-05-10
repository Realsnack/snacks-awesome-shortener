CREATE TABLE shorts (
    short_url VARCHAR(8) PRIMARY KEY NOT NULL,
    long_url VARCHAR(1024) NOT NULL,
    expiration integer NOT NULL,
    created timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used timestamp,
    use_counter integer NOT NULL default 0,
    is_expired bool default false
);

CREATE UNIQUE INDEX short_urlx ON shorts (short_url);

CREATE VIEW non_expired_shorts AS
  SELECT * FROM shorts WHERE is_expired = false;

CREATE FUNCTION mark_expired_rows()
    RETURNS void
    LANGUAGE 'sql' 
AS $BODY$
UPDATE shorts
SET is_expired = true
WHERE (created < NOW() - (expiration * INTERVAL '1 second')
		AND last_used < NOW() - (expiration * INTERVAL '1 second'))
	AND is_expired = false;
$BODY$;

ALTER FUNCTION mark_expired_rows()
    OWNER TO sas_app;

CREATE FUNCTION retrieve_short(
	short character varying)
    RETURNS shorts
    LANGUAGE 'sql'
    COST 100
    VOLATILE PARALLEL UNSAFE
AS $BODY$
UPDATE shorts
SET last_used = NOW(), use_counter = use_counter + 1
WHERE short_url = short
	AND is_expired=false;
SELECT *
FROM shorts
WHERE short_url = short
	AND is_expired=false;
$BODY$;

ALTER FUNCTION retrieve_short(character varying)
    OWNER TO sas_app;

