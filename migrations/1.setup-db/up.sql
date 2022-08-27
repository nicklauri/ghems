-- copyright (c) 2022 by Nick Lauri (Nguyen Tuong Anh Khoa)
-- start a transaction.
do
$$
declare
    migration_name  TEXT;
    v_state         TEXT;
    v_msg           TEXT;
    v_detail        TEXT;
    v_hint          TEXT;
    v_context       TEXT;
begin
------------------------------------------------------------------------------------------------
-- begin script --------------------------------------------------------------------------------
------------------------------------------------------------------------------------------------
migration_name := 'setup_db::up';

create or replace function set_updated_at()
    returns trigger as
$func$
begin
    NEW.updated_at = now();
    return NEW;
end;
$func$ language plpgsql;

create or replace function trigger_updated_at(tablename regclass)
    returns void as
$func$
begin
    execute format('CREATE TRIGGER set_updated_at
        BEFORE UPDATE
        ON %s
        FOR EACH ROW
        WHEN (OLD is distinct from NEW)
    EXECUTE FUNCTION set_updated_at();', tablename);
end;
$func$ language plpgsql;

create collation if not exists case_insensitive (provider = icu, locale = 'und-u-ks-level2', deterministic = false);
create collation if not exists ignore_accents (provider = icu, locale = 'und-u-ks-level1-kc-true', deterministic = false);

-- password_hash is hashed with sha512 is likely to be 88 chars, but we use 100 anyway.
create table if not exists user_account (
    id              uuid primary key default gen_random_uuid(),
    username        varchar(100) collate case_insensitive unique not null,
    email           text         collate case_insensitive unique not null,
    first_name      text                                         not null,
    last_name       text                                         not null,
    bio             text         default ''                      not null,
    picture         text,
    password_hash   text                                         not null,
    salt            text                                         not null,
    created_at      timestamptz  default now()                   not null,
    updated_at      timestamptz  default now()                   not null
);

create index if not exists idx__user_account__username on user_account (username);

perform trigger_updated_at('user_account');

insert into user_account
(
    id,
    username,
    email,
    first_name,
    last_name,
    bio,
    salt,
    password_hash
)
values
    (
        '00000000-0000-0000-0000-ff0000000001',
        'admin',
        'ghems_admin@mailinator.com',
        'Admin',
        '(Global)',
        'admin bio',
        'eAssyClQMNIQkd90vMf5vg',
        '$argon2id$v=19$m=4096,t=3,p=1$eAssyClQMNIQkd90vMf5vg$OdbkpzESdv6ySF1UWEfC1lMyjmf7uSRdPPcTF891LSc' -- admin
    ), 
    (
        '00000000-0000-0000-0000-ff0000000002',
        'nicklauri',
        'knta@mailinator.com',
        'Nick',
        'Lauri',
        'my bio',
        '0C/qZgTymIkWMuO6OwiCOg',
        '$argon2id$v=19$m=4096,t=3,p=1$0C/qZgTymIkWMuO6OwiCOg$fmmBssrq8SpeOoBzE6HEoQ3RcFnoX+Vr2XQ9+Qz14aU' -- nicklauri
    ) 
;

create table if not exists user_role (
    id          uuid primary key default gen_random_uuid(),
    role_name   text not null,
    role_ident  text not null
);

insert into user_role (id, role_name, role_ident) values
    ('00000000-0000-0000-0000-000000000001', 'Admin', 'admin'),
    ('00000000-0000-0000-0000-000000000002', 'Member', 'member');

create table if not exists users_roles (
    id uuid primary key default gen_random_uuid(),
    role_id uuid not null,
    user_id uuid not null,
    foreign key (role_id) references user_role(id),
    foreign key (user_id) references user_account(id)
);

insert into users_roles (role_id, user_id)
values
    ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-ff0000000001'),
    ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-ff0000000001'),
    ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-ff0000000002'),
    ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-ff0000000002');

------------------------------------------------------------------------------------------------
-- end script ----------------------------------------------------------------------------------
------------------------------------------------------------------------------------------------
raise notice 'migration "%" has been ran successfully', migration_name;
exception when others then
    rollback;
    raise notice 'the migration is aborted';

    get stacked diagnostics
        v_state   = returned_sqlstate,
        v_msg     = message_text,
        v_detail  = pg_exception_detail,
        v_hint    = pg_exception_hint,
        v_context = pg_exception_context;

    raise notice E'Got exception:
        state  : %
        message: %
        detail : %
        hint   : %
        context: %', v_state, v_msg, v_detail, v_hint, v_context;

    raise notice E'Got exception:
        SQLSTATE: % 
        SQLERRM: %', SQLSTATE, SQLERRM;     

    raise notice '%', message_text; -- invalid. message_text is contextual to GET STACKED DIAGNOSTICS only
end;
$$
language plpgsql;
