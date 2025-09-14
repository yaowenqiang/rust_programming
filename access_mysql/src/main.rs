#![allow(
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports,
    unused_variables
)]
use mysql::prelude::*;
use mysql::*;
mod types;
use types::Employee;

fn main() {
    let mut conn = get_conn();
    create_temp_table(&mut conn);
    insert_employee(&mut conn);
    let employees = select_employees(&mut conn);
    println!("\nEmployees");
    for e in employees {
        println!("{}", e);
    }
}

fn get_conn() -> mysql::PooledConn {
    let builder = OptsBuilder::new()
        .ip_or_hostname(Some("localhost"))
        .tcp_port(3306)
        .db_name(Some("MYSCHEMA"))
        .user(Some("root"))
        .pass(Some("123456"));
    let pool = Pool::new(builder).unwrap();
    pool.get_conn().unwrap()
}

fn create_temp_table(conn: &mut PooledConn) {
    conn.query_drop(
        r"
    CREATE temporary table employees_temp (
     employee_id INT not null,
     name VARCHAR(50) not null,
     salary DOUBLE not null,
     region VARCHAR(50) not null,
     primary key (employee_id)
    )
    ",
    )
    .unwrap();
}

fn insert_employee(conn: &mut PooledConn) {
    let employees = vec![
        Employee::new(1, String::from("Andy"), 25_000.0, String::from("Vales")),
        Employee::new(2, String::from("Jayne"), 35_000.0, String::from("Vales")),
        Employee::new(3, String::from("Emily"), 45_000.0, String::from("Scotland")),
        Employee::new(4, String::from("Tom"), 55_000.0, String::from("London")),
    ];

    conn.exec_batch(
        "INSERT INTO employees_temp (employee_id, name, salary, region) VALUES (:i, :n, :s, :r)",
        employees.iter().map(
            |e| params! {"i" => e.employee_id,"n" =>&e.name,  "s" => e.salary, "r" => &e.region},
        ),
    )
    .unwrap()
}

fn select_employees(conn: &mut PooledConn) -> Vec<Employee> {
    conn.query_map(
        "select employee_id, name, salary, region from employees_temp",
        |(i, n, s, r)| Employee::new(i, n, s, r),
    )
    .unwrap()
}
