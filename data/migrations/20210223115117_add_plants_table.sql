CREATE TABLE type_plants (
    id CHAR NOT NULL DEFAULT (
        lower(hex(randomblob(4))) ||
        '-' ||
        lower(hex(randomblob(2))) ||
        '-4' ||
        substr(lower(hex(randomblob(2))), 2) ||
        '-' ||
        substr('89ab', abs(random()) % 4 + 1, 1) ||
        substr(lower(hex(randomblob(2))), 2) ||
        '-' ||
        lower(hex(randomblob(6)))
    ),
    name VARCHAR NOT NULL,
    max_height_cm INTEGER NOT NULL,
    habit VARCHAR NOT NULL,

    PRIMARY KEY(id),
    UNIQUE(name),
    CHECK(
        -- id
        typeof(id) = 'text' AND
        length(id) = 36 AND
        substr(id, 15, 1) == '4' AND
        substr(id, 20, 1) IN ('8', '9', 'a', 'b') AND

        -- name
        length(name) > 0 AND

        -- max_size_cm
        max_height_cm > 0 AND

        -- habit
        habit in ('tree', 'shrub', 'grass', 'vine', 'herb')
    )
);
