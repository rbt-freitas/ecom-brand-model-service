-- /////////////////////////////////////////////////////
-- TABLE BRAND_MODEL
CREATE TABLE brand_model
(
    id         uuid         NOT NULL PRIMARY KEY,
    name       varchar(100) NOT NULL,
    is_active  boolean      NOT NULL DEFAULT false,
    brand_id   uuid         NOT NULL,
    CONSTRAINT brand_model_uk UNIQUE (name)
);

CREATE INDEX brand_model_ix01 on brand_model(name);
CREATE INDEX brand_model_ix02 on brand_model(brand_id);

ALTER TABLE brand_model
ADD CONSTRAINT brand_model_fk01
FOREIGN KEY (brand_id) REFERENCES brand(id);
