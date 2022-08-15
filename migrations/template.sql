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
migration_name := '<migration_name>';

-- your code here

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
