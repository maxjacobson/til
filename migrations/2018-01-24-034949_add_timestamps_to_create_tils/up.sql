-- need to juggle data a little to add CURRENT_TIMESTAMP type for existing table

create table tmp_tils (
  id integer primary key,
  contents text not null,
  created_at timestamp not null default CURRENT_TIMESTAMP
);

insert into tmp_tils
  select id, contents, CURRENT_TIMESTAMP from tils;

drop table tils;

alter table tmp_tils rename to tils;
