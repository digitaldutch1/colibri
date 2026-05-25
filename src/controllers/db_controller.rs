


use std::env;
use tokio_postgres::NoTls;
use crate::db::Accommodation;

pub async fn get_all_accommodations() -> Vec<Accommodation> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect manually like in your backup
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("Failed to connect");

    // Spawn the connection in the background
    actix_web::rt::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client
        .query("SELECT id, name, total_units, created_at FROM accommodation", &[])
        .await
        .expect("Query failed");

    let mut accommodations = Vec::new();
    for row in rows {
        accommodations.push(Accommodation {
            id: row.get(0),
            name: row.get(1),
            total_units: row.get(2),
            created_at: row.get(3),
        });
    }
    accommodations
}


pub async fn get_unavailable_dates(
    accommodation_id: i32,
    exclude_booking_id: i32,
) -> Result<Vec<String>, String> {

    let database_url =
        std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

    let (client, connection) =
        tokio_postgres::connect(
            &database_url,
            tokio_postgres::NoTls
        )
        .await
        .map_err(|error| error.to_string())?;

    actix_web::rt::spawn(async move {

        if let Err(error) = connection.await {
            eprintln!(
                "Database connection error: {}",
                error
            );
        }
    });

    let rows = client
        .query(
            "
            SELECT booked_days.day::date::text
            FROM (
                SELECT generate_series(
                    b.check_in_date,
                    b.check_out_date - INTERVAL '1 day',
                    INTERVAL '1 day'
                ) AS day

                FROM booking b
                WHERE b.accommodation_id = $1
                AND b.id != $2
                AND b.status != 'cancelled'
                AND (
                    b.locked_until IS NULL
                    OR b.locked_until > NOW()
                )

            ) booked_days

            GROUP BY booked_days.day

            HAVING COUNT(*) >= (
                SELECT total_units
                FROM accommodation
                WHERE id = $1
            )

            ORDER BY booked_days.day ASC
            ",
            &[
                &accommodation_id,
                &exclude_booking_id,
            ],
        )
        .await
        .map_err(|error| error.to_string())?;

    let mut dates = Vec::new();

    for row in rows {

        let date: String =
            row.get(0);

        dates.push(date);
    }

    Ok(dates)
}

pub async fn cleanup_expired_booking_locks() -> Result<(), String> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (client, connection) = tokio_postgres::connect(&database_url, tokio_postgres::NoTls)
        .await
        .map_err(|error| error.to_string())?;

    actix_web::rt::spawn(async move {
        if let Err(error) = connection.await {
            eprintln!("Database connection error: {}", error);
        }
    });

    // Remove expired booking locks
    client
        .execute(
            "
            DELETE FROM booking
            WHERE status = 'pending'
            AND locked_until IS NOT NULL
            AND locked_until < NOW()
            ",
            &[],
        )
        .await
        .map_err(|error| error.to_string())?;

    // Remove unused temporary customers
    client
        .execute(
            "
            DELETE FROM customer
            WHERE email LIKE 'lock-%@temporary.local'
            AND id NOT IN (
                SELECT customer_id
                FROM booking
            )
            ",
            &[],
        )
        .await
        .map_err(|error| error.to_string())?;

    Ok(())
}

pub async fn get_all_customers() -> Vec<crate::db::CustomerRow> {

    let database_url =
        env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

    let (client, connection) =
        tokio_postgres::connect(&database_url, NoTls)
            .await
            .expect("Failed to connect");

    actix_web::rt::spawn(async move {

        if let Err(error) = connection.await {
            eprintln!("connection error: {}", error);
        }
    });

    let rows = client
        .query(
            "
            SELECT
                id,
                first_name,
                last_name,
                email,
                phone,
                address,
                postal_code,
                city

            FROM customer

            ORDER BY id ASC
            ",
            &[],
        )
        .await
        .expect("Query failed");

    let mut customers = Vec::new();

    for row in rows {
        customers.push(crate::db::CustomerRow {
            id:
                row.get(0),
            first_name:
                row.get(1),
            last_name:
                row.get(2),
            email:
                row.get(3),
            phone:
                row.get(4),
            address:
                row.get(5),
            postal_code:
                row.get(6),
            city:
                row.get(7),
        });
    }

    customers
}

pub async fn get_all_staff() -> Vec<crate::db::StaffRow> {
    let database_url =
        env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

    let (client, connection) =
        tokio_postgres::connect(&database_url, NoTls)
            .await
            .expect("Failed to connect");

    actix_web::rt::spawn(async move {

        if let Err(error) = connection.await {
            eprintln!("connection error: {}", error);
        }
    });

    let rows = client
        .query(
            "
            SELECT
                id,
                first_name,
                last_name,
                email,
                role
            FROM  \"user\"
            ORDER BY id ASC
            ",
            &[],
        )
        .await
        .expect("Query failed");

    let mut staff = Vec::new();

    for row in rows {
        staff.push(crate::db::StaffRow {
            id:
                row.get(0),
            first_name:
                row.get(1),
            last_name:
                row.get(2),
            email:
                row.get(3),
            role:
                row.get(4),
        });
    }

    staff
}