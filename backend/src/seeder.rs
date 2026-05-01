mod config;
mod db;
use crate::config::EnvConfig;
use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use fake::Fake;
use fake::faker::address::en::{CityName, StreetName, StreetSuffix};
use fake::faker::name::en::{FirstName, LastName};
use sqlx::{Pool, Postgres, Row};
use tracing::info;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

async fn generar_oficinas(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let result_muncipios = sqlx::query("select id from municipio;")
        .fetch_all(pool)
        .await?;

    let ids_municipios: Vec<i32> = result_muncipios
        .iter()
        .map(|r| r.get::<i32, _>("id"))
        .collect();

    let mut inserts = String::from("");

    for id in ids_municipios {
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

    info!("Oficinas creadas: {}", result.rows_affected());

    Ok(())
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
    let result_nuc = sqlx::query(
        "select n.id, o.municipio_id
         from nucleo n
         join bodega b on b.id = n.bodega_id
         join oficina o on o.id = b.oficina_id;",
    )
    .fetch_all(pool)
    .await?;

    let mut inserts = String::from("");

    for row in &result_nuc {
        let nuc_id: i32 = row.get("id");
        let mun_id: i32 = row.get("municipio_id");
        let cant_personas = rand::random_range::<u16, _>(1..=10);

        for _ in 0..cant_personas {
            inserts += &format!(
                "insert into persona (nombre, apellido, nucleo_id, municipio_id, carnet) values ('{}', '{}', {}, {}, '{:02}{:02}{:02}{}');\n",
                FirstName().fake::<String>().replace("'", " "),
                LastName().fake::<String>().replace("'", " "),
                nuc_id,
                mun_id,
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
    // Administrador
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
        sqlx::query_scalar(
                "select p.id from persona p join nucleo n on p.nucleo_id = n.id join bodega bo on bo.id = n.bodega_id where bo.oficina_id = $1 offset 1 limit 1;"
            )
            .bind(oficina_id)
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
    .bind(hashear(&config.registrar_password).await?)
    .bind(persona_id_registrador)
    .bind(oficina_id)
    .execute(pool)
    .await?;

    info!("Usuario registrador creado: {}", &config.registrar_email);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    info!("Iniciando proceso de seeding");
    let config = config::EnvConfig::new();
    info!("Conectando a la base de datos");
    let pool = db::init_db(&config).await.map_err(|e| {
        tracing::error!("Error conectando a la base de datos en seeder: {}", e);
        e
    })?;

    info!("Generando oficina inicial");
    generar_oficinas(&pool).await.map_err(|e| {
        tracing::error!("Error generando oficinas: {}", e);
        e
    })?;
    info!("Generando bodegas");
    generar_bodegas(&pool).await.map_err(|e| {
        tracing::error!("Error generando bodegas: {}", e);
        e
    })?;
    info!("Generando nucleos");
    generar_nucleos(&pool).await.map_err(|e| {
        tracing::error!("Error generando nucleos: {}", e);
        e
    })?;
    info!("Generando personas");
    generar_personas(&pool).await.map_err(|e| {
        tracing::error!("Error generando personas: {}", e);
        e
    })?;
    info!("Creando usuarios iniciales");
    crear_usuarios(&pool, 1, &config).await.map_err(|e| {
        tracing::error!("Error creando usuarios iniciales: {}", e);
        e
    })?;
    info!("Seeding completado");
    Ok(())
}
