-----------------------------------------------------------
    -- Create the database
-----------------------------------------------------------
if not exists(select 1 from sys.databases where name='graphdemo')
    create database GraphDemo

use GraphDemo

--------------------------------------------------------------
    -- Create table nodes
--------------------------------------------------------------
create table Person(
    id integer primary key,
    name varchar(100)
) as node;

create table Restaurant(
    id integer not null,
    name varchar(100),
    city varchar(100)
) as node

create table City (
    id integer primary key,
    name varchar(100),
    stateName varchar(100)
) as node


--------------------------------------------------------------
    -- Create edge tables
--------------------------------------------------------------

create table likes(rating integer) as edge
create table friendOf as edge
create table livesIn as edge
create table locatedIn as Edge


--------------------------------------------------------------
    -- Inserta data into dones tables
--------------------------------------------------------------
truncate table Person
insert Person(id,name)
values
    (1,'John'),
    (2,'Mary'),
    (3,'Alice'),
    (4,'Jacob'),
    (5,'Julie')

truncate table Restaurant
insert Restaurant(id,name,city)
values
    (1,'Taco Dell','Bellevue'),
    (2,'Ginger and Spice','Seattle'),
    (3,'Noodle Land','Redmond')


insert City (id,name,stateName)
values
    (1,'Bellevue','WA'),
    (2,'Seattle','WA'),
    (3,'Redmond','WA')


--------------------------------------------------------------
    -- Insert into edge tables
--------------------------------------------------------------

insert likes
values
     ((select $node_id from Person where id=1), (select $node_id from Restaurant where id=1),9),
     ((select $node_id from Person where id=2), (select $node_id from Restaurant where id=2),9),
     ((select $node_id from Person where id=3), (select $node_id from Restaurant where id=3),9),
     ((select $node_id from Person where id=4), (select $node_id from Restaurant where id=3),9),
     ((select $node_id from Person where id=5), (select $node_id from Restaurant where id=3),9)

INSERT INTO locatedIn
    VALUES ((SELECT $node_id FROM Restaurant WHERE ID = 1), (SELECT $node_id FROM City WHERE ID =1))
         , ((SELECT $node_id FROM Restaurant WHERE ID = 2), (SELECT $node_id FROM City WHERE ID =2))
         , ((SELECT $node_id FROM Restaurant WHERE ID = 3), (SELECT $node_id FROM City WHERE ID =3));

INSERT INTO friendOf
    VALUES ((SELECT $NODE_ID FROM Person WHERE ID = 1), (SELECT $NODE_ID FROM Person WHERE ID = 2))
         , ((SELECT $NODE_ID FROM Person WHERE ID = 2), (SELECT $NODE_ID FROM Person WHERE ID = 3))
         , ((SELECT $NODE_ID FROM Person WHERE ID = 3), (SELECT $NODE_ID FROM Person WHERE ID = 1))
         , ((SELECT $NODE_ID FROM Person WHERE ID = 4), (SELECT $NODE_ID FROM Person WHERE ID = 2))
         , ((SELECT $NODE_ID FROM Person WHERE ID = 5), (SELECT $NODE_ID FROM Person WHERE ID = 4));

/* Associate in which city live each person*/
INSERT INTO livesIn
    VALUES ((SELECT $node_id FROM Person WHERE ID = 1), (SELECT $node_id FROM City WHERE ID = 1))
         , ((SELECT $node_id FROM Person WHERE ID = 2), (SELECT $node_id FROM City WHERE ID = 2))
         , ((SELECT $node_id FROM Person WHERE ID = 3), (SELECT $node_id FROM City WHERE ID = 3))
         , ((SELECT $node_id FROM Person WHERE ID = 4), (SELECT $node_id FROM City WHERE ID = 3))
         , ((SELECT $node_id FROM Person WHERE ID = 5), (SELECT $node_id FROM City WHERE ID = 1));
