use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Tab {
    co1: String,
    co2: i32
}

pub fn crud() {

    let pool = mysql::Pool::new("mysql://root:root@localhost:3306/tpl")
        .unwrap();
    let mut conn = pool.get_conn()
        .unwrap();

    let rows = vec![
        Tab { co1: "hi".to_string(),    co2: 2, },
        Tab { co1: "hello".to_string(), co2: 4, },
    ];

    // Insert rows to tab table using raw string literals, 
    // do not process any escapes
    conn.exec_batch(
        r"INSERT INTO tpl.tab (co1, co2)
        VALUES (:co1, :co2)",
        rows.iter().map(|p| params! {
            "co1" => String::from(&p.co1),
            "co2" => p.co2,
        })
    ).unwrap();

    // Select
    let selected_tab = conn.query_map(
        "SELECT co1, co2 FROM tpl.tab",
        | (co1, co2) | { 
            Tab { co1,co2 }
        },
    ).unwrap();

    for r in selected_tab.iter() {
        println!("{}: {}", r.co1, r.co2);
    }
}