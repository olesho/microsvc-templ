use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;
use serde_json;
use tokio_postgres::{NoTls, Error};
#[derive(Debug)]
#[derive(Serialize)]
struct Person {
	id: i32,
	name: String,
	//data: Option<&[u8]>,
}

const DB_STRING: &str = "host=172.22.0.3 user=rust_user password=qwerty88 dbname=rust_db";

async fn init() -> Result<(), Error> {
	let (client, connection) = tokio_postgres::connect(DB_STRING, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

	let result = client.batch_execute("
		CREATE TABLE person (
			id      SERIAL PRIMARY KEY,
			name    TEXT NOT NULL,
			data    BYTEA
		);
		INSERT INTO person (name, data) VALUES ('Test User', decode('013d7d16d7ad4fefb61bd95b765c8ceb', 'hex'));
	").await;

	return result;
}

async fn get_one(id: i32) -> Result<Person, Error> {
	let (client, connection) = tokio_postgres::connect(DB_STRING, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
	});
	
	let row = client.query_one("SELECT id, name, data FROM person WHERE id = $1", &[&id]).await?;
	let s: &str = row.get(1);
	Ok(Person{
		id: row.get(0),
		name: String::from(s),
		//data: None, //row.get(2),
	})
}

async fn get_all() -> Result<Vec<Person>, Error> {
	let (client, connection) = tokio_postgres::connect(DB_STRING, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
	});
	
	let query_res;
	let mut r: Vec<Person> = Vec::new();
	query_res = client.query("SELECT id, name, data FROM person", &[]).await?;
	for row in query_res {
		let s: &str = row.get(1);
		//String::from()
		r.push(Person{
			id: row.get(0),
			name: String::from(s),
			//data: None, //row.get(2),
		});
	};
	Ok(r)
}

#[get("/init")]
async fn index_handler() -> impl Responder {
	match init().await {
		Ok(v) => web::HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&v).unwrap()),
		Err(e) => web::HttpResponse::InternalServerError().content_type("application/json").body("Internal Server Error"),
	};
	return web::Json(())
}

#[get("/get/{id}")]
async fn person_handler(p: actix_web::web::Path<i32>) -> impl Responder {
	match get_one(p.into_inner()).await {
		Ok(v) => web::HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&v).unwrap()),
		Err(e) => {
			eprintln!("{:?}", e);
			web::HttpResponse::NotFound().content_type("application/json").body("Not found")
		}
	}
}


#[get("/get")]
async fn persons_handler() -> impl Responder {
	match get_all().await {
		Ok(v) => web::HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&v).unwrap()),
		Err(e) => web::HttpResponse::NotFound().content_type("application/json").body("Not found"),
	}
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(persons_handler).service(person_handler).service(index_handler))
        .bind("127.0.0.1:9091")?
        .run()
        .await
}

