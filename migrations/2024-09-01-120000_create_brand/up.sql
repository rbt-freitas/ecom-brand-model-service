-- /////////////////////////////////////////////////////
-- TABLE BRAND
CREATE TABLE brand
(
    id        uuid         NOT NULL PRIMARY KEY,
    name      varchar(100) NOT NULL,
    is_active boolean      NOT NULL DEFAULT false,
    CONSTRAINT brand_uk UNIQUE (name)
);

CREATE INDEX brand_ix01 ON brand (name);
