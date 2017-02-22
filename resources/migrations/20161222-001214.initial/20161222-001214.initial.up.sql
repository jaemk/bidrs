create table auth (
    id serial PRIMARY KEY,
    salt          bytea NOT NULL,
    password      bytea NOT NULL,
    date_created  timestamp WITH TIME ZONE NOT NULL DEFAULT NOW(),
    date_modified timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);


create table users (
    id            serial PRIMARY KEY,
    auth_id       integer NOT NULL UNIQUE REFERENCES "auth" ("id") ON DELETE CASCADE,
    email         text UNIQUE NOT NULL,
    uuid_         uuid UNIQUE NOT NULL,
    level_        integer NOT NULL CHECK (level_ >= 0),
    date_created  timestamp WITH TIME ZONE NOT NULL DEFAULT NOW(),
    date_modified timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);
create unique index on users (uuid_);
create unique index on users (email);


create table organizations (
    id            serial PRIMARY KEY,
    name          varchar(255) UNIQUE NOT NULL,
    extra         jsonb,
    date_created  timestamp WITH TIME ZONE NOT NULL DEFAULT NOW(),
    date_modified timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);


create table bidders (
    id              serial PRIMARY KEY,
    organization_id integer NOT NULL REFERENCES "organizations" ("id") ON DELETE SET NULL,
    id_name         text NOT NULL,
    date_created    timestamp WITH TIME ZONE NOT NULL DEFAULT NOW(),
    date_modified   timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);


create table payment_information (
    id             serial PRIMARY KEY,
    cc_number      text NOT NULL,
    cc_pin         text NOT NULL,
    cc_exp         date NOT NULL,
    date_created   timestamp WITH TIME ZONE NOT NULL DEFAULT NOW(),
    date_modified  timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);


create table profiles (
    id              serial PRIMARY KEY,
    user_id         integer NOT NULL UNIQUE REFERENCES "users" ("id") ON DELETE CASCADE,
    bidder_id       integer REFERENCES "bidders" ("id") ON DELETE SET NULL,
    payment_info_id integer REFERENCES "payment_information" ("id") ON DELETE SET NULL,
    is_primary      boolean DEFAULT FALSE,
    name            text,
    phone           text,
    extra           jsonb,
    date_created    timestamp WITH TIME ZONE NOT NULL DEFAULT NOW(),
    date_modified   timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);
create index on profiles ((lower(name)));
create unique index on profiles ((user_id));


create table items (
    id               serial PRIMARY KEY,
    organization_id  integer NOT NULL REFERENCES "organizations" ("id") ON DELETE SET NULL,
    owning_bidder_id integer REFERENCES "bidders" ("id") ON DELETE SET NULL,
    is_goal          boolean DEFAULT FALSE,
    title            text,
    description      text,
    value            bigint,
    starting         bigint,
    min_bid          bigint,
    date_created     timestamp WITH TIME ZONE NOT NULL DEFAULT NOW(),
    date_modified    timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);
create index on items ((lower(title)));


create table bids (
    id            serial PRIMARY KEY,
    bidder_id     integer NOT NULL REFERENCES "bidders" ("id") ON DELETE SET NULL,
    item_id       integer NOT NULL REFERENCES "items" ("id") ON DELETE SET NULL,
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
after insert or update on bids
for each row execute procedure notify_trigger();


-- auto-update timestamp function
create function update_date_modified() returns trigger as $$
declare
begin
    IF row(NEW.*) is distinct from row(OLD.*) THEN
        NEW.date_modified = NOW();
        return NEW;
    ELSE
        return OLD;
    END IF;
END;
$$ LANGUAGE plpgsql;

create trigger date_modified_trigger
before update on auth
for each row execute procedure update_date_modified();

create trigger date_modified_trigger
before update on users
for each row execute procedure update_date_modified();

create trigger date_modified_trigger
before update on organizations
for each row execute procedure update_date_modified();

create trigger date_modified_trigger
before update on bidders
for each row execute procedure update_date_modified();

create trigger date_modified_trigger
before update on items
for each row execute procedure update_date_modified();

create trigger date_modified_trigger
before update on payment_information
for each row execute procedure update_date_modified();

create trigger date_modified_trigger
before update on profiles
for each row execute procedure update_date_modified();

create trigger date_modified_trigger
before update on bids
for each row execute procedure update_date_modified();

