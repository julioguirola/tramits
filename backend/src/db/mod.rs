use crate::config::EnvConfig;
use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use fake::Fake;
use fake::faker::address::en::{CityName, StreetName, StreetSuffix};
use fake::faker::name::en::{FirstName, LastName};
use sqlx::{Pool, Postgres, Row, postgres::PgPoolOptions};
use std::fs;
use tracing::info;

async fn generar_oficinas(pool: &Pool<Postgres>) -> Result<i32, sqlx::Error> {
    let municipio_id: i32 =
        sqlx::query_scalar("select id from municipio where nombre = 'CIEGO DE AVILA' limit 1;")
            .fetch_one(pool)
            .await?;

    let oficina_id: i32 = sqlx::query_scalar(
        "insert into oficina (nombre, municipio_id) values ($1, $2) returning id;",
    )
    .bind((CityName().fake::<String>() + " Ofic").replace("'", " "))
    .bind(municipio_id)
    .fetch_one(pool)
    .await?;

    info!("Oficina creada: id={}", oficina_id);

    Ok(oficina_id)
}

async fn generar_bodegas(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let result_ofics = sqlx::query("select id from oficina;")
        .fetch_all(pool)
        .await?;

    let ids_ofics: Vec<i32> = result_ofics.iter().map(|r| r.get::<i32, _>("id")).collect();

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

    info!("Bodegas creadas: {}", result.rows_affected());

    Ok(())
}

async fn generar_nucleos(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let result_bodeg = sqlx::query("select id from bodega;")
        .fetch_all(pool)
        .await?;

    let ids_bodeg: Vec<i32> = result_bodeg.iter().map(|r| r.get::<i32, _>("id")).collect();

    let mut inserts = String::from("");

    for id in ids_bodeg {
        let cant_nucleos = rand::random_range::<u16, _>(20..=30);

        for _ in 0..cant_nucleos {
            inserts += &format!(
                "insert into nucleo (direccion, bodega_id) values ('{}', {});\n",
                (StreetName().fake::<String>() + " " + &StreetSuffix().fake::<String>() + " st.")
                    .replace("'", " "),
                id,
            );
        }
    }

    let result = sqlx::raw_sql(&inserts).execute(pool).await?;

    info!("Nucleos creados: {}", result.rows_affected());

    Ok(())
}

async fn generar_personas(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let result_nuc = sqlx::query("select id from nucleo;")
        .fetch_all(pool)
        .await?;

    let ids_nuc: Vec<i32> = result_nuc.iter().map(|r| r.get::<i32, _>("id")).collect();

    let mut inserts = String::from("");

    for id in ids_nuc {
        let cant_personas = rand::random_range::<u16, _>(1..=10);

        for _ in 0..cant_personas {
            inserts += &format!(
                "insert into persona (nombre, apellido, nucleo_id, carnet) values ('{}', '{}', {}, '{:02}{:02}{:02}{}');\n",
                FirstName().fake::<String>().replace("'", " "),
                LastName().fake::<String>().replace("'", " "),
                id,
                rand::random_range::<u16, _>(0..=99),
                rand::random_range::<u16, _>(1..=12),
                rand::random_range::<u16, _>(1..=31),
                rand::random_range::<u32, _>(10000..=99999),
            )
        }
    }

    let result = sqlx::raw_sql(&inserts).execute(pool).await?;

    info!("Personas creadas: {}", result.rows_affected());

    Ok(())
}

async fn hashear(password: &str) -> Result<String, sqlx::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| sqlx::Error::Protocol(e.to_string()))
        .map(|h| h.to_string())
}

async fn crear_usuarios(
    pool: &Pool<Postgres>,
    oficina_id: i32,
    config: &EnvConfig,
) -> Result<(), sqlx::Error> {
    let persona_id: sqlx::types::Uuid = sqlx::query_scalar("select id from persona limit 1;")
        .fetch_one(pool)
        .await?;

    sqlx::query(
        "insert into usuario (email, pass_word, persona_id, rol_id) values ($1, $2, $3, 3);",
    )
    .bind(&config.admin_email)
    .bind(hashear(&config.admin_password).await?)
    .bind(persona_id)
    .execute(pool)
    .await?;

    info!("Usuario administrador creado: {}", &config.admin_email);

    // Consumidor
    let persona_id_consumidor: sqlx::types::Uuid =
        sqlx::query_scalar("select id from persona offset 1 limit 1;")
            .fetch_one(pool)
            .await?;

    sqlx::query(
        "insert into usuario (email, pass_word, persona_id, rol_id) values ($1, $2, $3, 1);",
    )
    .bind(&config.consumer_email)
    .bind(hashear(&config.consumer_password).await?)
    .bind(persona_id_consumidor)
    .execute(pool)
    .await?;

    info!("Usuario consumidor creado: {}", &config.consumer_email);

    // Registrador
    let persona_id_registrador: sqlx::types::Uuid =
        sqlx::query_scalar("select id from persona offset 2 limit 1;")
            .fetch_one(pool)
            .await?;

    sqlx::query(
        "insert into usuario (email, pass_word, persona_id, rol_id, oficina_id) values ($1, $2, $3, 2, $4);",
    )
    .bind(&config.registrar_email)
    .bind(hashear(&config.registrar_email).await?)
    .bind(persona_id_registrador)
    .bind(oficina_id)
    .execute(pool)
    .await?;

    info!("Usuario registrador creado: {}", &config.registrar_email);

    Ok(())
}

pub async fn init_db(config: &EnvConfig, migrate: bool) -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_user, config.db_password, config.db_host, config.db_port, config.db_name,
    );

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    if migrate {
        let migration_query = fs::read_to_string("src/db/migration.sql")?;

        sqlx::raw_sql(&migration_query).execute(&pool).await?;

        let oficina_id = generar_oficinas(&pool).await?;
        generar_bodegas(&pool).await?;
        generar_nucleos(&pool).await?;
        generar_personas(&pool).await?;
        crear_usuarios(&pool, oficina_id, config).await?;
    }

    Ok(pool)
}
