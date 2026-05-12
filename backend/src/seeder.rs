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
use std::collections::HashMap;
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
        inserts += &format!(
            "insert into oficina (nombre, municipio_id) values ('{}', {});\n",
            (CityName().fake::<String>() + " Ofic").replace("'", " "),
            id,
        );
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
        inserts += &format!(
            "insert into bodega (nombre, oficina_id) values ('{}', {});\n",
            (CityName().fake::<String>() + " Bodega").replace("'", " "),
            id,
        );
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
        let cant_personas = rand::random_range::<u16, _>(1..=3);

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

async fn persona_id_por_oficina(
    pool: &Pool<Postgres>,
    oficina_id: i32,
    offset: i64,
) -> Result<sqlx::types::Uuid, sqlx::Error> {
    sqlx::query_scalar(
        "select p.id from persona p join nucleo n on p.nucleo_id = n.id join bodega bo on bo.id = n.bodega_id where bo.oficina_id = $1 offset $2 limit 1;",
    )
    .bind(oficina_id)
    .bind(offset)
    .fetch_one(pool)
    .await
}

async fn crear_usuarios_base(pool: &Pool<Postgres>, config: &EnvConfig) -> Result<(), sqlx::Error> {
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
    let persona_id_consumidor: sqlx::types::Uuid = persona_id_por_oficina(pool, 1, 1).await?;

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
    let persona_id_registrador: sqlx::types::Uuid = persona_id_por_oficina(pool, 1, 2).await?;

    sqlx::query(
        "insert into usuario (email, pass_word, persona_id, rol_id, oficina_id) values ($1, $2, $3, 2, $4);",
    )
    .bind(&config.registrar_email)
    .bind(hashear(&config.registrar_password).await?)
    .bind(persona_id_registrador)
    .bind(1)
    .execute(pool)
    .await?;

    info!("Usuario registrador creado: {}", &config.registrar_email);

    Ok(())
}

async fn crear_registradores_por_oficina(
    pool: &Pool<Postgres>,
    config: &EnvConfig,
) -> Result<(), sqlx::Error> {
    let oficinas = sqlx::query("select id from oficina where id > 1;")
        .fetch_all(pool)
        .await?;

    let personas = sqlx::query(
        "select distinct on (bo.oficina_id) bo.oficina_id, p.id
         from persona p
         join nucleo n on p.nucleo_id = n.id
         join bodega bo on bo.id = n.bodega_id
         order by bo.oficina_id, p.id;",
    )
    .fetch_all(pool)
    .await?;

    let persona_map: HashMap<i32, sqlx::types::Uuid> = personas
        .iter()
        .map(|r| (r.get("oficina_id"), r.get("id")))
        .collect();

    let pass_hash = hashear(&config.registrar_password).await?;

    let mut inserts = String::new();
    for row in oficinas {
        let oficina_id: i32 = row.get("id");

        let persona_id = persona_map
            .get(&oficina_id)
            .ok_or(sqlx::Error::RowNotFound)?;
        let email = format!("registrador{}@seed.local", oficina_id);

        inserts += &format!(
            "insert into usuario (email, pass_word, persona_id, rol_id, oficina_id) values ('{}', '{}', '{}', 2, {});\n",
            email, pass_hash, persona_id, oficina_id
        );
    }

    if !inserts.is_empty() {
        sqlx::raw_sql(&inserts).execute(pool).await?;
    }

    Ok(())
}

async fn crear_usuarios_faltantes(
    pool: &Pool<Postgres>,
    config: &EnvConfig,
) -> Result<(), sqlx::Error> {
    let personas = sqlx::query(
        "select p.id, p.nombre, p.apellido
         from persona p
         left join usuario u on u.persona_id = p.id
         where u.id is null;",
    )
    .fetch_all(pool)
    .await?;

    if personas.is_empty() {
        info!("No hay personas sin usuario");
        return Ok(());
    }

    let pass_hash = hashear(&config.consumer_password).await?;

    let mut inserts = String::new();
    let mut count = 0u64;
    for row in &personas {
        let persona_id: sqlx::types::Uuid = row.get("id");
        let nombre: String = row.get("nombre");
        let apellido: String = row.get("apellido");
        let random_suffix: u32 = rand::random_range::<u32, _>(100..999);
        let email = format!(
            "{}.{}{}@seed.local",
            nombre.to_lowercase().replace(' ', "."),
            apellido.to_lowercase().replace(' ', "."),
            random_suffix,
        );

        inserts += &format!(
            "insert into usuario (email, pass_word, persona_id, rol_id) values ('{}', '{}', '{}', 1);\n",
            email, pass_hash, persona_id
        );
        count += 1;
    }

    sqlx::raw_sql(&inserts).execute(pool).await?;
    info!("Usuarios creados para personas sin usuario: {}", count);

    Ok(())
}

async fn generar_tramites(pool: &Pool<Postgres>, cantidad: usize) -> Result<(), sqlx::Error> {
    let personas = sqlx::query(
        "select p.id, p.nucleo_id, bo.oficina_id
         from persona p
         join nucleo n on p.nucleo_id = n.id
         join bodega bo on bo.id = n.bodega_id;",
    )
    .fetch_all(pool)
    .await?;

    let registradores = sqlx::query("select id, oficina_id from usuario where rol_id = 2;")
        .fetch_all(pool)
        .await?;

    let mut inserts = String::new();

    for _ in 0..cantidad {
        let persona_row = &personas[rand::random_range::<usize, _>(0..personas.len())];
        let persona_id: sqlx::types::Uuid = persona_row.get("id");
        let nucleo_id: i32 = persona_row.get("nucleo_id");
        let oficina_id: i32 = persona_row.get("oficina_id");

        let tipo_id = rand::random_range::<i32, _>(1..=3);
        let estado_id = rand::random_range::<i32, _>(1..=5);

        let dias_atras = rand::random_range::<i64, _>(0..365);
        let fecha_solicitud = format!("current_date - interval '{} days'", dias_atras);

        let registrador_id: Option<sqlx::types::Uuid> = if estado_id == 1 || estado_id == 5 {
            None
        } else {
            let reg_row = registradores
                .iter()
                .find(|r| r.get::<i32, _>("oficina_id") == oficina_id)
                .unwrap_or(&registradores[0]);
            Some(reg_row.get("id"))
        };

        let fecha_finalizado = if estado_id == 3 || estado_id == 4 || estado_id == 5 {
            "current_date"
        } else {
            "null"
        };

        if let Some(reg_id) = registrador_id {
            inserts += &format!(
                "insert into tramite (persona_id, registrador_id, nucleo_id, tipo_id, estado_id, fecha_solicitud, fecha_finalizado) values ('{}', '{}', {}, {}, {}, {}, {});\n",
                persona_id,
                reg_id,
                nucleo_id,
                tipo_id,
                estado_id,
                fecha_solicitud,
                fecha_finalizado
            );
        } else {
            inserts += &format!(
                "insert into tramite (persona_id, nucleo_id, tipo_id, estado_id, fecha_solicitud) values ('{}', {}, {}, {}, {});\n",
                persona_id, nucleo_id, tipo_id, estado_id, fecha_solicitud
            );
        }
    }

    let result = sqlx::raw_sql(&inserts).execute(pool).await?;
    info!("Tramites creados: {}", result.rows_affected());

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
    info!("Creando usuarios base");
    crear_usuarios_base(&pool, &config).await.map_err(|e| {
        tracing::error!("Error creando usuarios iniciales: {}", e);
        e
    })?;
    info!("Creando registradores por oficina");
    crear_registradores_por_oficina(&pool, &config)
        .await
        .map_err(|e| {
            tracing::error!("Error creando registradores por oficina: {}", e);
            e
        })?;
    info!("Creando usuarios faltantes");
    crear_usuarios_faltantes(&pool, &config)
        .await
        .map_err(|e| {
            tracing::error!("Error creando usuarios faltantes: {}", e);
            e
        })?;
    info!("Generando tramites");
    generar_tramites(&pool, 10000).await.map_err(|e| {
        tracing::error!("Error generando tramites: {}", e);
        e
    })?;
    info!("Seeding completado");
    Ok(())
}
