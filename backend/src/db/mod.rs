use std::{ fs };
use sqlx::{ Pool, Postgres, Row, postgres::PgPoolOptions};
use rand;
use fake::faker::address::en::{ CityName, StreetName, StreetSuffix };
use fake::faker::name::en::{ FirstName, LastName };
use fake::Fake;
use crate::config::EnvConfig;

struct DataBase {
    db_host: String,
    db_user: String,
    db_password: String,
    db_name: String,
    db_port: String,
}

async fn generar_oficinas (pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let result = sqlx::query("select id from municipio;")
        .fetch_all(pool).await?;
    
    let ids: Vec<i32> = result.iter()
        .map(|r| r.get::<i32,_>("id")).collect();

    let mut inserts = String::from("");

    for id in ids {
        let cant_oficinas = rand::random_range::<u8, _>(1..=3);

        for _ in 0..cant_oficinas {
            inserts += &format!(
                "insert into oficina (nombre, municipio_id) values ('{}', {});\n",
                (CityName().fake::<String>() + " Ofic").replace("'", " "),
                id,
            );
        }
    }

    let result = sqlx::raw_sql(&inserts).execute(pool).await?;

    println!("Oficinas creadas: {}", result.rows_affected());

    Ok(())
}

async fn generar_bodegas (pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let result_ofics = sqlx::query("select id from oficina;")
        .fetch_all(pool).await?;

    let ids_ofics: Vec<i32> = result_ofics.iter()
        .map(|r| r.get::<i32,_>("id")).collect();

    let mut inserts = String::from("");

    for id in ids_ofics {
        let cant_bodegas = rand::random_range::<u8, _>(1..=10);

        for _ in 0..cant_bodegas {
            inserts += &format!(
                "insert into bodega (nombre, oficina_id) values ('{}', {});\n",
                (CityName().fake::<String>() + " Bodega").replace("'", " "),
                id,
            );
        }
    }

    let result = sqlx::raw_sql(&inserts).execute(pool).await?;

    println!("Bodegas creadas: {}", result.rows_affected());

    Ok(())
}

async fn generar_nucleos (pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let result_bodeg = sqlx::query("select id from bodega;")
        .fetch_all(pool).await?;

    let ids_bodeg: Vec<i32> = result_bodeg.iter()
        .map(|r| r.get::<i32,_>("id")).collect();

    let mut inserts = String::from("");

    for id in ids_bodeg {
        let cant_nucleos = rand::random_range::<u16, _>(20..=30);

        for _ in 0..cant_nucleos {
            inserts += &format!(
                "insert into nucleo (direccion, bodega_id) values ('{}', {});\n",
                (StreetName().fake::<String>() + " " + &StreetSuffix().fake::<String>() + " st.").replace("'", " "),
                id,
            );
        }
    }

    let result = sqlx::raw_sql(&inserts).execute(pool).await?;

    println!("Nucleos creados: {}", result.rows_affected());

    Ok(())
}

async fn generar_personas (pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let result_nuc = sqlx::query("select id from nucleo;")
        .fetch_all(pool).await?;

    let ids_nuc: Vec<i32> = result_nuc.iter()
        .map(|r| r.get::<i32,_>("id")).collect();

    let mut inserts = String::from("");

    for id in ids_nuc {
        let cant_personas = rand::random_range::<u16, _>(1..=10);

        for _ in 0..cant_personas {
            inserts += &format!(
                "insert into persona (nombre, apellido, nucleo_id, carnet) values ('{}', '{}', {}, '{}');\n",
                FirstName().fake::<String>().replace("'", " "),
                LastName().fake::<String>().replace("'", " "),
                id,
                format!(
                    "{:02}{:02}{:02}{}",
                    rand::random_range::<u16, _>(0..=99),
                    rand::random_range::<u16, _>(1..=12),
                    rand::random_range::<u16, _>(1..=31),
                    rand::random_range::<u32, _>(10000..=99999),
                ),
            )
        }
    }

    let result = sqlx::raw_sql(&inserts).execute(pool).await?;

    println!("Personas creadas: {}", result.rows_affected());

    Ok(())
}

impl DataBase {
    pub async fn new(config: &EnvConfig, migrate: bool) -> Result<Pool<Postgres>, sqlx::Error> {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.db_user,
            config.db_password,
            config.db_host,
            config.db_port,
            config.db_name,
        );

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url).await?;

        if migrate {
            let migration_query = fs::read_to_string("src/db/migration.sql")?;
        
            sqlx::raw_sql(&migration_query).execute(&pool).await?;
        
            generar_oficinas(&pool).await?;
            generar_bodegas(&pool).await?;
            generar_nucleos(&pool).await?;
            generar_personas(&pool).await?;
        }

        Ok(pool)
    }
}