use crate::dl_lite::abox::AB;
use crate::dl_lite::abox_item::ABI;
use crate::dl_lite::native_filetype_utilities::find_bound_of_symbols;
use crate::dl_lite::node::Node;
use crate::dl_lite::sqlite_structs::{
    AboxItemCDb, AboxItemRDb, NodeDb, SymbolDb, TableDb, TboxItemDb,
};
use crate::dl_lite::string_formatter::{node_to_string, string_to_abi, string_to_tbi};
use crate::dl_lite::tbox::TB;
use crate::dl_lite::tbox_item::TBI;
use crate::dl_lite::types::DLType;
use crate::interface::cli::Task;
use rusqlite::{Connection, Result, NO_PARAMS};
use std::collections::HashMap;

pub fn connect_to_db(filename: &str, verbose: bool) -> Connection {
    if verbose {
        println!("attempting to connect to database: {}", filename);
    }

    let conn = Connection::open(filename);

    match conn {
        Err(e) => {
            println!("an error occurred: {}", &e);
            println!("the program will exit");

            std::process::exit(exitcode::IOERR);
        }
        Ok(c) => {
            if verbose {
                println!("connection succeed");
            }
            c
        }
    }
}

pub fn add_basic_tables_to_db(conn: &Connection, verbose: bool) {
    // create relation types
    let mut query = "
            CREATE TABLE IF NOT EXISTS relation(
                type TEXT NON NULL PRIMARY KEY
            )";

    let mut res = conn.execute(query, NO_PARAMS);

    if verbose {
        println!("query: {} \nreturned: {:?}", query, &res);
    }

    query = "INSERT OR IGNORE INTO relation(type) VALUES ('role')";

    res = conn.execute(query, NO_PARAMS);

    if verbose {
        println!("query: {} \nreturned: {:?}", query, &res);
    }

    query = "INSERT OR IGNORE INTO relation(type) VALUES ('concept')";

    res = conn.execute(query, NO_PARAMS);

    if verbose {
        println!("query: {} \nreturned: {:?}", query, &res);
    }

    // create dl type
    // create relation types
    query = "
            CREATE TABLE IF NOT EXISTS dltype(
                id INTEGER PRIMARY KEY,
                type TEXT NON NULL
            )";

    res = conn.execute(query, NO_PARAMS);

    if verbose {
        println!("query: {} \nreturned: {:?}", query, &res);
    }

    // create symbols table
    query = "
            CREATE TABLE IF NOT EXISTS symbols(
                id INTEGER PRIMARY KEY,
                name TEXT NON NULL,
                type INTEGER NON NULL,
                FOREIGN KEY(type) REFERENCES dltype(id)
        )
        ";

    res = conn.execute(query, NO_PARAMS);

    if verbose {
        println!("query: {} \nreturned: {:?}", query, &res);
    }

    // create node items table
    // auto increment table
    query = "
            CREATE TABLE IF NOT EXISTS nodes(
                id INTEGER PRIMARY KEY,
                name TEXT NON NULL,
                type INTEGER NON NULL,
                FOREIGN KEY(type) REFERENCES dltype(id),
                UNIQUE(name, type)
           )
        ";

    res = conn.execute(query, NO_PARAMS);

    if verbose {
        println!("query: {} \nreturned: {:?}", query, &res);
    }

    // create tbox items table
    // auto increment table
    query = "
            CREATE TABLE IF NOT EXISTS tbox_items(
                id INTEGER PRIMARY KEY,
                lside_name TEXT NON NULL,
                rside_name TEXT NON NULL,
                lside INTEGER NON NULL,
                rside INTEGER NON NULL,
                FOREIGN KEY(lside) REFERENCES nodes(id),
                FOREIGN KEY(rside) REFERENCES nodes(id),
                UNIQUE(lside_name, rside_name, lside, rside)
           )
        ";

    res = conn.execute(query, NO_PARAMS);

    if verbose {
        println!("query: {} \nreturned: {:?}", query, &res);
    }

    for t in DLType::types().iter() {
        let id = t.to_usize_for_db();
        let dltype = t.to_string_for_db();
        let command = format!(
            "INSERT OR IGNORE INTO dltype(id, type) VALUES ({}, '{}')",
            id, dltype
        );

        query = command.as_str();
        res = conn.execute(query, NO_PARAMS);

        if verbose {
            println!("query: {} \nreturned: {:?}", query, &res);
        }
    }
}

pub fn add_symbol_to_db(symbol: (&String, &(usize, DLType)), conn: &Connection, verbose: bool) {
    let (s, (id, t)) = symbol;

    // correct id for dltype
    let t_to_id_db = t.to_usize_for_db();

    let command = format!(
        "INSERT OR IGNORE INTO symbols VALUES({}, '{}', '{}')",
        id, s, t_to_id_db
    );

    let query = command.as_str();
    let res = conn.execute(query, NO_PARAMS);

    if verbose {
        println!("query: {}\nreturned: {:?}", query, &res);
    }
}

pub fn add_symbols_to_db(
    symbols: &HashMap<String, (usize, DLType)>,
    conn: &Connection,
    verbose: bool,
) {
    for symbol in symbols {
        add_symbol_to_db(symbol, conn, verbose);
    }
}

pub fn add_nodes_to_db(
    symbols: &HashMap<String, (usize, DLType)>,
    nodes: Vec<&Node>,
    conn: &Connection,
    verbose: bool,
) {
    for node in nodes {
        add_node_to_db(symbols, node, conn, verbose);
    }
}

pub fn add_node_to_db(
    symbols: &HashMap<String, (usize, DLType)>,
    node: &Node,
    conn: &Connection,
    verbose: bool,
) {
    let node_string_op = node_to_string(node, symbols, String::from(""));

    match node_string_op {
        Option::None => (),
        Some(node_string) => {
            let is_base = node.t().is_base_type();
            let is_nominal = node.t().is_nominal_type();

            let command = format!(
                "INSERT OR IGNORE INTO nodes(name, type) VALUES ('{}', {})",
                node_string,
                node.t().to_usize_for_db()
            );

            let query = command.as_str();
            let res = conn.execute(query, NO_PARAMS);

            if verbose {
                println!("query: {}\nreturned: {:?}", query, &res);
            }

            // add the childs too
            if !is_base {
                let child = Node::child(Some(&node)).unwrap();

                if verbose {
                    println!("calling method on child")
                }

                add_node_to_db(symbols, child, conn, verbose);
            }
        }
    }
}

pub fn get_node_from_db(
    symbols: &HashMap<String, (usize, DLType)>,
    node: &Node,
    conn: &Connection,
    verbose: bool,
) -> Option<Vec<NodeDb>> {
    let query = format!(
        "\
            SELECT id, name, type FROM nodes where name = '{}'
        ",
        node_to_string(node, symbols, String::from("")).unwrap()
    );

    let mut smt_res = conn.prepare(query.as_str());

    match smt_res {
        Err(e) => {
            println!("an error ocurred: {}", &e);
            Option::None
        }
        Ok(mut smt) => {
            let nodes = smt
                .query_map(NO_PARAMS, |row| {
                    Ok(NodeDb {
                        id_db: row.get(0)?,
                        name_db: row.get(1)?,
                        type_db: row.get(2)?,
                    })
                })
                .ok()?;

            let mut v: Vec<NodeDb> = Vec::new();
            for node in nodes {
                v.push(node.unwrap().clone());
            }

            if verbose {
                println!("returning nodes: {:?}", &v);
            }

            Some(v)
        }
    }
}
pub fn add_tbi_to_db(
    symbols: &HashMap<String, (usize, DLType)>,
    tbi: &TBI,
    conn: &Connection,
    verbose: bool,
) {
    add_node_to_db(symbols, tbi.lside(), conn, verbose);
    add_node_to_db(symbols, tbi.rside(), conn, verbose);

    // now need to find the ids
    let lside_id: usize =
        get_node_from_db(symbols, tbi.lside(), conn, verbose).unwrap()[0].id_db as usize;
    let rside_id: usize =
        get_node_from_db(symbols, tbi.rside(), conn, verbose).unwrap()[0].id_db as usize;

    let command = format!(
        "INSERT OR IGNORE INTO tbox_items(lside_name, rside_name, lside, rside) VALUES ('{}', '{}', {}, {})",
        node_to_string(tbi.lside(), symbols, String::from("")).unwrap(),
        node_to_string(tbi.rside(), symbols, String::from("")).unwrap(),
        lside_id,
        rside_id
    );

    let query = command.as_str();

    let res = conn.execute(query, NO_PARAMS);

    if verbose {
        println!("query: {}\nreturned: {:?}", query, &res);
    }
}

pub fn add_tbis_to_db(
    symbols: &HashMap<String, (usize, DLType)>,
    tbis: &Vec<TBI>,
    conn: &Connection,
    verbose: bool,
) {
    for tbi in tbis {
        add_tbi_to_db(symbols, tbi, conn, verbose);
    }
}

pub fn add_abi_to_db(
    symbols: &HashMap<String, (usize, DLType)>,
    ab_name: &str,
    abi: &ABI,
    conn: &Connection,
    verbose: bool,
) {
    // define the stuff
    let mut command: String;
    let mut query: &str;
    let mut res: Result<usize>;

    // match abi type
    match abi {
        ABI::CA(c, a) => {
            add_nodes_to_db(symbols, vec![c, a], conn, verbose);

            let c_id = get_node_from_db(symbols, c, conn, verbose).unwrap()[0].id_db as usize;
            let a_id = get_node_from_db(symbols, a, conn, verbose).unwrap()[0].id_db as usize;

            command = format!("INSERT OR IGNORE INTO {}_abox_concept(constant_name, concept_name, constant, concept) VALUES ('{}', '{}', {}, {})",
                              ab_name,
                              node_to_string(a, symbols, String::from("")).unwrap(),
                              node_to_string(c, symbols, String::from("")).unwrap(),
                              a_id,
                              c_id,
            );

            query = command.as_str();

            res = conn.execute(query, NO_PARAMS);

            if verbose {
                println!("query: {}\nreturned: {:?}", query, &res);
            }
        }
        ABI::RA(r, a, b) => {
            add_nodes_to_db(symbols, vec![r, a, b], conn, verbose);

            let r_id = get_node_from_db(symbols, r, conn, verbose).unwrap()[0].id_db as usize;
            let a_id = get_node_from_db(symbols, a, conn, verbose).unwrap()[0].id_db as usize;
            let b_id = get_node_from_db(symbols, b, conn, verbose).unwrap()[0].id_db as usize;

            command = format!("INSERT OR IGNORE INTO \
                    {}_abox_role(constant1_name, constant2_name, role_name, constant1, constant2, role) VALUES ('{}', '{}', '{}', {}, {}, {})",
                              ab_name,
                              node_to_string(a, symbols, String::from("")).unwrap(),
                              node_to_string(b, symbols, String::from("")).unwrap(),
                              node_to_string(r, symbols, String::from("")).unwrap(),
                              a_id,
                              b_id,
                              r_id,
            );

            query = command.as_str();

            res = conn.execute(query, NO_PARAMS);

            if verbose {
                println!("query: {}\nreturned: {:?}", query, &res);
            }
        }
    }
}

pub fn add_abis_to_db(
    symbols: &HashMap<String, (usize, DLType)>,
    abis: &Vec<ABI>,
    ab_name: &str,
    conn: &Connection,
    verbose: bool,
) {
    // first create table it they don't exist

    // first concept table
    let mut command = format!(
        "\
        CREATE TABLE IF NOT EXISTS {}_abox_concept(
            id INTEGER PRIMARY KEY,
            constant_name TEXT NON NULL,
            concept_name TEXT NON NULL,
            constant INTEGER NON NULL,
            concept INTEGER NON NULL,
            UNIQUE(constant, concept),
            FOREIGN KEY(constant) REFERENCES nodes(id),
            FOREIGN KEY(concept) REFERENCES nodes(id)
        )
    ",
        ab_name
    );

    let mut query = &command;

    let mut res = conn.execute(query, NO_PARAMS);

    if verbose {
        println!("query: {} \nreturned: {:?}", query, &res);
    }

    // second role table
    let mut command = format!(
        "\
        CREATE TABLE IF NOT EXISTS {}_abox_role(
            id INTEGER PRIMARY KEY,
            constant1_name TEXT NON NULL,
            constant2_name TEXT NON NULL,
            role_name TEXT NON NULL,
            constant1 INTEGER NON NULL,
            constant2 INTEGER NON NULL,
            role INTEGER NON NULL,
            UNIQUE(constant1, constant2, role),
            FOREIGN KEY(constant1) REFERENCES nodes(id),
            FOREIGN KEY(constant2) REFERENCES nodes(id),
            FOREIGN KEY(role) REFERENCES nodes(id)
        )
    ",
        ab_name
    );

    let mut query = &command;

    let mut res = conn.execute(query, NO_PARAMS);

    if verbose {
        println!("query: {} \nreturned: {:?}", query, &res);
    }

    for abi in abis {
        add_abi_to_db(symbols, ab_name, abi, conn, verbose);
    }
}

pub fn add_symbols_from_db(
    symbols: &mut HashMap<String, (usize, DLType)>,
    conn: &Connection,
    verbose: bool,
) -> Result<usize> {
    let query = "SELECT * FROM symbols";

    let mut smt_res = conn.prepare(query);

    match smt_res {
        Err(e) => {
            if verbose {
                println!("an error ocurred: {}", &e);
            }
            Err(e)
        }
        Ok(mut smt) => {
            if verbose {
                println!("query: {} succeed", query);
            }

            let symbols_row = smt.query_map(NO_PARAMS, |row| {
                Ok(SymbolDb {
                    id_db: row.get(0)?,
                    name_db: row.get(1)?,
                    type_db: row.get(2)?,
                })
            })?;

            for symbol in symbols_row {
                let symbol = symbol.unwrap();
                let t_id = symbol.type_db as usize;
                let t_op = DLType::to_type_from_usize_for_db(t_id);

                if verbose {
                    println!("attempting to insert symbol {:?}", &symbol);
                }

                match t_op {
                    Some(t) => {
                        let id = symbol.id_db as usize;
                        symbols.insert(symbol.name_db, (id, t));

                        if verbose {
                            println!("insertion succeed");
                        }
                    }
                    Option::None => {
                        if verbose {
                            println!("insertion failed");
                        }
                    }
                }
            }

            Ok(0)
        }
    }
}

pub fn add_tbis_from_db(
    symbols: &HashMap<String, (usize, DLType)>,
    tbox: &mut TB,
    conn: &Connection,
    verbose: bool,
) -> Result<usize> {
    let query = "SELECT * FROM tbox_items";
    let mut smt_res = conn.prepare(query);

    match smt_res {
        Err(e) => {
            if verbose {
                println!("an error ocurred: {}", &e);
            }
            Err(e)
        }
        Ok(mut smt) => {
            if verbose {
                println!("query: {} succeed", query);
            }

            let tbis_row = smt.query_map(NO_PARAMS, |row| {
                Ok(TboxItemDb {
                    id_db: row.get(0)?,
                    lside_name_db: row.get(1)?,
                    rside_name_db: row.get(2)?,
                    lside_db: row.get(3)?,
                    rside_db: row.get(4)?,
                })
            })?;

            for tbi in tbis_row {
                let tbi = tbi.unwrap();

                let mut tbi_string = String::new();
                let lside_str = tbi.lside_name_db;
                let rside_str = tbi.rside_name_db;

                tbi_string.push_str(&lside_str);
                tbi_string.push_str("<");
                tbi_string.push_str(&rside_str);

                let tbi_res = string_to_tbi(&tbi_string, symbols);

                if verbose {
                    println!("attempting to insert tbi {:?}", &tbi_res);
                }

                match tbi_res {
                    Ok(v) => {
                        for tbox_item in &v {
                            tbox.add(tbox_item.clone());
                        }

                        if verbose {
                            println!("insertion succeed");
                        }
                    }
                    Err(e) => {
                        if verbose {
                            println!("insertion failed: {}", &e);
                        }
                    }
                }
            }

            Ok(0)
        }
    }
}

pub fn add_abis_from_db(
    symbols: &mut HashMap<String, (usize, DLType)>,
    abox: &mut AB,
    conn: &Connection,
    ab_name: &str,
    verbose: bool,
) -> Result<usize> {
    let command_c = format!("SELECT * FROM {}_abox_concept", ab_name);
    let query_c = &command_c;

    let command_r = format!("SELECT * FROM {}_abox_role", ab_name);
    let query_r = &command_r;

    let smt_res_c = conn.prepare(query_c);

    let res_c = match smt_res_c {
        Err(e) => {
            if verbose {
                println!("an error ocurred: {}", &e);
            }
            Err(e)
        }
        Ok(mut smt) => {
            if verbose {
                println!("query: {} succeed", query_c);
            }

            let abis_row = smt.query_map(NO_PARAMS, |row| {
                Ok(AboxItemCDb {
                    id_db: row.get(0)?,
                    constant_name_db: row.get(1)?,
                    concept_name_db: row.get(2)?,
                    constant_db: row.get(3)?,
                    concept_db: row.get(4)?,
                })
            })?;

            // needed to update the symbols
            let (_, id_bound) = find_bound_of_symbols(symbols);
            let mut current_id = id_bound + 1;

            for abi in abis_row {
                //println!("---- tbi: {:?}", &tbi);

                let abi = abi.unwrap();

                let mut abi_string = String::new();
                let a_str = abi.constant_name_db;
                let c_str = abi.concept_name_db;

                abi_string.push_str(&a_str);
                abi_string.push_str(":");
                abi_string.push_str(&c_str);

                let (abi_res, current_id) = string_to_abi(&abi_string, symbols, current_id, true); // TODO: come back here for 'for_completion' argument

                if verbose {
                    println!("attempting to insert abi {:?}", &abi_res);
                }

                match abi_res {
                    Ok(v) => {
                        // destructure
                        let (abi, ss) = v;

                        // add abi
                        abox.add(abi);

                        // add symbols
                        for (s, it) in ss {
                            symbols.insert(s, it);
                        }

                        if verbose {
                            println!("insertion succeed");
                        }
                    }
                    Err(e) => {
                        if verbose {
                            println!("insertion failed: {}", &e);
                        }
                    }
                }
            }
            Ok(0)
        }
    };

    // now for roles
    let smt_res_r = conn.prepare(query_r);

    let res_r = match smt_res_r {
        Err(e) => {
            if verbose {
                println!("an error ocurred: {}", &e);
            }
            Err(e)
        }
        Ok(mut smt) => {
            if verbose {
                println!("query: {} succeed", query_r);
            }

            let abis_row = smt.query_map(NO_PARAMS, |row| {
                Ok(AboxItemRDb {
                    id_db: row.get(0)?,
                    constant1_name_db: row.get(1)?,
                    constant2_name_db: row.get(2)?,
                    role_name_db: row.get(3)?,
                    constant1_db: row.get(4)?,
                    constant2_db: row.get(5)?,
                    role_db: row.get(6)?,
                })
            })?;

            // needed to update the symbols
            let (_, id_bound) = find_bound_of_symbols(symbols);
            let mut current_id = id_bound + 1;

            for abi in abis_row {
                //println!("---- tbi: {:?}", &tbi);

                let abi = abi.unwrap();

                let mut abi_string = String::new();
                let a_str = abi.constant1_name_db;
                let b_str = abi.constant2_name_db;
                let r_str = abi.role_name_db;

                abi_string.push_str(&a_str);
                abi_string.push_str(", ");
                abi_string.push_str(&b_str);
                abi_string.push_str(":");
                abi_string.push_str(&r_str);

                let (abi_res, current_id) = string_to_abi(&abi_string, symbols, current_id, true); // TODO: same here, for_completion argument

                if verbose {
                    println!("attempting to insert abi {:?}", &abi_res);
                }

                match abi_res {
                    Ok(v) => {
                        // destructure
                        let (abi, ss) = v;

                        // add abi
                        abox.add(abi);

                        // add symbols
                        for (s, it) in ss {
                            symbols.insert(s, it);
                        }

                        if verbose {
                            println!("insertion succeed");
                        }
                    }
                    Err(e) => {
                        if verbose {
                            println!("insertion failed: {}", &e);
                        }
                    }
                }
            }
            Ok(0)
        }
    };

    match (res_c, res_r) {
        (Err(e), _) => {
            if verbose {
                println!("an error ocurred: {}", &e);
            }
            Err(e)
        }
        (_, Err(e)) => {
            if verbose {
                println!("an error ocurred: {}", &e);
            }
            Err(e)
        }
        (Ok(i1), _) => Ok(i1),
    }
}

// TODO: maybe more information in these functions that work with the database
pub fn update_symbols_to_db(
    symbols: &HashMap<String, (usize, DLType)>,
    conn: &Connection,
    verbose: bool,
) {
    for symbol in symbols {
        add_symbol_to_db(symbol, conn, verbose);
    }
}

pub fn get_table_names(conn: &Connection, verbose: bool) -> Result<Vec<String>> {
    let query = "\
    SELECT
        name
    FROM
        sqlite_master
    WHERE
        type='table' AND
        name NOT LIKE 'sqlite_%'
    ";

    let smt_res = conn.prepare(query);

    match smt_res {
        Err(e) => {
            if verbose {
                println!("an error ocurred: {}", &e);
            }
            Err(e)
        }
        Ok(mut smt) => {
            let tables = smt
                .query_map(NO_PARAMS, |row| {
                    Ok(TableDb {
                        name_db: row.get(0)?,
                    })
                })
                .ok()
                .unwrap();

            let mut v: Vec<String> = Vec::new();

            for table in tables {
                v.push(table.unwrap().name_db);
            }

            Ok(v)
        }
    }
}

pub fn drop_tables_from_database(conn: &Connection, tables_to_drop: Vec<&str>, verbose: bool) {
    let mut command: String;
    let mut query: &str;
    let mut res: Result<usize>;

    for table in tables_to_drop {
        // okay, otherwise, you can't be dropping table however you want (this also protect from some attacks...)
        if table.contains("_abox_concept") || table.contains("_abox_role") {
            command = format!("DROP TABLE {}", table);

            query = &command;

            res = conn.execute(query, NO_PARAMS);

            if verbose {
                println!("query: {}\nreturned: {:?}", query, &res);
            }
        }
    }
}
