-- there's no drop column, but we can
-- juggle the data around to simulate one

create table tmp_tils (
  id integer primary key,
  contents text not null
);

insert into tmp_tils
  select id, contents from tils;

drop table tils;

alter table tmp_tils rename to tils;
