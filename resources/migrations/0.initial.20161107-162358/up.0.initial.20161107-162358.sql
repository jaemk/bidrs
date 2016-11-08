create table biddy_user (
    id serial PRIMARY KEY,
    username varchar(255) UNIQUE NOT NULL,
    uuid_ uuid UNIQUE NOT NULL,
    date_created timestamp WITH TIME ZONE NOT NULL DEFAULT NOW(),
    date_modified timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);
create unique index on biddy_user (uuid_);

create table organization (
    id serial PRIMARY KEY,
    name varchar(255) UNIQUE NOT NULL,
    extra json
);

create table bidder (
    id serial PRIMARY KEY,
    organization_id integer NOT NULL UNIQUE REFERENCES "organization" ("id"),
    date_created timestamp WITH TIME ZONE NOT NULL DEFAULT NOW(),
    date_modified timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);

create table item (
    id serial PRIMARY KEY,
    organization_id integer NOT NULL REFERENCES "organization" ("id"),
    owning_bidder_id integer REFERENCES "bidder" ("id"),
    is_goal boolean DEFAULT FALSE,
    title VARCHAR(255),
    description text
);
create index on item ((lower(title)));

create table profile (
    id serial PRIMARY KEY,
    user_id integer NOT NULL UNIQUE REFERENCES "biddy_user" ("id") ON DELETE CASCADE,
    bidder_id integer REFERENCES "bidder" ("id"),
    level integer NOT NULL CHECK (level >= 0),
    is_primary boolean DEFAULT FALSE,
    name varchar(255),
    phone_cc varchar(3),
    phone_number varchar(10),
    phone_ext varchar(4),
    email varchar(254),
    cc_info json,
    extra json,
    date_created timestamp WITH TIME ZONE NOT NULL DEFAULT NOW(),
    date_modified timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);
create index on profile ((lower(email)));

create table bid (
    id serial PRIMARY KEY,
    bidder_id integer REFERENCES "bidder" ("id"),
    item_id integer REFERENCES "item" ("id"),
    amount numeric(20, 4) NOT NULL,
    date_created timestamp WITH TIME ZONE NOT NULL DEFAULT NOW(),
    date_modified timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);

