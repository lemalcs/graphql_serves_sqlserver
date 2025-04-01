use crate::types::Person;
use async_graphql::futures_util::TryStreamExt;
use tiberius::{AuthMethod, Client, Config, QueryItem};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

async fn connect_to_db() -> Result<Client<Compat<TcpStream>>, Box<dyn std::error::Error>> {
    let mut config = Config::new();
    config.authentication(AuthMethod::Integrated);
    config.host("127.0.0.1");
    config.port(22828);
    config.database("GraphDemo");
    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    let client = Client::connect(config, tcp.compat_write()).await?;

    Ok(client)
}

pub async fn get_persons() -> Result<Vec<Person>, Box<dyn std::error::Error>> {
    let mut client = connect_to_db().await?;
    let mut query = client.query("SELECT id, name FROM dbo.Person", &[]).await?;
    let mut persons = Vec::new();

    while let Some(row) = query.try_next().await.unwrap() {
        match row {
            QueryItem::Row(r) => {
                let id: i32 = r.get(0).unwrap();
                let name: String = r.get(1).unwrap_or_else(|| "").to_string();
                persons.push(Person { id, name });
            }
            QueryItem::Metadata(_) => {}
        }
    }

    Ok(persons)
}

pub async fn get_person(id: i32) -> Result<Person, Box<dyn std::error::Error>> {
    let mut client = connect_to_db().await?;
    let mut query = client
        .query("SELECT id, name FROM dbo.Person where id=@P1", &[&id])
        .await?;
    let mut person: Option<Person> = None;

    while let Some(row) = query.try_next().await.unwrap() {
        match row {
            QueryItem::Row(r) => {
                let id: i32 = r.get(0).unwrap();
                let name: String = r.get(1).unwrap_or_else(|| "").to_string();
                person = Some(Person { id, name });
                break;
            }
            QueryItem::Metadata(_) => {}
        }
    }

    person.ok_or_else(|| format!("Person {} not found", id).into())
}

pub async fn get_friends(id: i32) -> Result<Vec<Person>, Box<dyn std::error::Error>> {
    let mut client = connect_to_db().await?;
    let mut query = client
        .query(
            "
select Person2.id, Person2.name
from Person,
     friendOf,
     Person as Person2
where match(Person - (friendof) -> Person2)
  and Person.id = @P1",
            &[&id],
        )
        .await?;
    let mut persons = Vec::new();

    while let Some(row) = query.try_next().await.unwrap() {
        match row {
            QueryItem::Row(r) => {
                let id: i32 = r.get(0).unwrap();
                let name: String = r.get(1).unwrap_or_else(|| "").to_string();
                persons.push(Person { id, name });
            }
            QueryItem::Metadata(_) => {}
        }
    }

    Ok(persons)
}

pub async fn update_person_name(
    id: i32,
    name: String,
) -> Result<Person, Box<dyn std::error::Error>> {
    let mut client = connect_to_db().await?;
    let query = client
        .execute("update Person set name=@P1 where id=@P2", &[&name, &id])
        .await?;

    if query.rows_affected()[0] == 0 {
        return Err(format!("Person {} not found", id).into());
    }

    Ok(Person { id, name })
}
