create table biddy_user (
    id            serial PRIMARY KEY,
    username      varchar(255) UNIQUE NOT NULL,
    uuid_         uuid UNIQUE NOT NULL,
    date_created  timestamp WITH TIME ZONE NOT NULL DEFAULT NOW(),
    date_modified timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);
create unique index on biddy_user (uuid_);

create table organization (
    id            serial PRIMARY KEY,
    name          varchar(255) UNIQUE NOT NULL,
    extra         json,
    date_created  timestamp WITH TIME ZONE NOT NULL DEFAULT NOW(),
    date_modified timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);

create table bidder (
    id              serial PRIMARY KEY,
    organization_id integer NOT NULL UNIQUE REFERENCES "organization" ("id"),
    date_created    timestamp WITH TIME ZONE NOT NULL DEFAULT NOW(),
    date_modified   timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);

create table item (
    id               serial PRIMARY KEY,
    organization_id  integer NOT NULL REFERENCES "organization" ("id"),
    owning_bidder_id integer REFERENCES "bidder" ("id"),
    is_goal          boolean DEFAULT FALSE,
    title            varchar(255),
    description      text,
    value            bigint,
    min_bid          bigint,
    date_created     timestamp WITH TIME ZONE NOT NULL DEFAULT NOW(),
    date_modified    timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);
create index on item ((lower(title)));

create table profile (
    id            serial PRIMARY KEY,
    user_id       integer NOT NULL UNIQUE REFERENCES "biddy_user" ("id") ON DELETE CASCADE,
    bidder_id     integer REFERENCES "bidder" ("id"),
    level_        integer NOT NULL CHECK (level_ >= 0),
    is_primary    boolean DEFAULT FALSE,
    name          varchar(255),
    phone_cc      varchar(3),
    phone_number  varchar(10),
    phone_ext     varchar(4),
    email         varchar(254),
    cc_info       json,
    extra         json,
    date_created  timestamp WITH TIME ZONE NOT NULL DEFAULT NOW(),
    date_modified timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);
create index on profile ((lower(email)));

create table bid (
    id            serial PRIMARY KEY,
    bidder_id     integer NOT NULL REFERENCES "bidder" ("id"),
    item_id       integer NOT NULL REFERENCES "item" ("id"),
    amount        bigint NOT NULL,
    date_created  timestamp WITH TIME ZONE NOT NULL DEFAULT NOW(),
    date_modified timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);


-- notification function
create function notify_trigger() returns trigger as $$
declare
begin
    -- TG_TABLE_NAME : table triggered
    -- TG            : trigger op
    -- NEW           : new val
    execute 'NOTIFY '
    || TG_TABLE_NAME
    || ', '''
    || TG_OP
    || ' '
    || NEW
    || '''';
    return NEW;
end;
$$ LANGUAGE plpgsql;

create trigger bid_trigger
after insert or update on bid
for each row execute procedure notify_trigger();
