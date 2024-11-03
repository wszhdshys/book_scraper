use std::collections::HashMap;
use crate::functions;
use fnck_sql::db::DataBaseBuilder;
use functions::gui::Sort;
use functions::update::House;
use itertools::Itertools;
use lazy_static::lazy_static;
use crate::functions::update::name;

lazy_static! {
    pub static ref data_conclusion:[String; 5] = [
        data_select(Sort::Ppm),
        data_select(Sort::Tp),
        data_select(Sort::Size),
        data_select(Sort::Rb),
        data_select(Sort::Default)
        ];
}


pub fn sort(position: String, sort: Sort, u_d: u8) -> Vec<House> {
    let datapath = format!("{}{}", r#"./data/"#, &position);
    if let Ok(database) = DataBaseBuilder::path(datapath).build() {
        let create_table_sql = r#"
        create table if not exists mytable (
            id int primary key,
            title varchar(200),
            community varchar(50),
            area varchar(20),
            price_per_meter int,
            total_price int,
            layout varchar(40),
            floor varchar(60),
            size int,
            decoration varchar(20),
            orientation varchar(30),
            time varchar(10),
            property_ownership_type varchar(30),
            property_type varchar(20),
            ownership_duration varchar(40),
            reference_budget int,
            publishing_company varchar(30),
            business_license varchar(20),
            update_date varchar(20),
            link varchar(260)
        )
        "#;
        let mut updown: &str;
        if u_d == 0 {
            updown = "ASC";
        } else {
            updown = "DESC";
        }
        let sort = match sort {
            Sort::Default => {
                "select * from mytable".to_string()
            }
            Sort::Ppm => {
                format!("select * from mytable order by price_per_meter {}", updown)
            }
            Sort::Tp => {
                format!("select * from mytable order by total_price {}", updown)
            }
            Sort::Size => {
                format!("select * from mytable order by size {}", updown)
            }
            Sort::Rb => {
                format!("select * from mytable order by reference_budget {}", updown)
            }
        };
        if let Ok(_) = database.run(create_table_sql) {
            if let Ok((schema, tuple)) = database.run(sort) {
                let tuples = tuple
                    .into_iter()
                    .map(|tuple| House::from((&schema, tuple)))
                    .collect_vec();
                //println!("{:#?}", tuples);
                //let _ = database.run("drop table mytable").unwrap();
                tuples
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

fn data_select(sorts: Sort) -> String {
    let mut sum: HashMap<i64, String> = HashMap::new();
    for city in name() {
        match sorts {
            Sort::Default => {
                let mut sum = 0;
                for city in name() {
                    sum += sort(city, Sort::Default, 1).len();
                }
                return sum.to_string();
            }
            Sort::Ppm => {
                let onr = sort(city, Sort::Ppm, 1);
                for iter in onr.iter() {
                    sum.insert(
                        iter.price_per_meter,
                        format!(
                            "来自{} {}的{}元/㎡",
                            iter.area, iter.community, iter.price_per_meter
                        ),
                    );
                    break;
                }
            }
            Sort::Tp => {
                let onr = sort(city, Sort::Tp, 1);
                for iter in onr.iter() {
                    sum.insert(
                        iter.total_price,
                        format!(
                            "来自{} {}的{}万元",
                            iter.area, iter.community, iter.total_price
                        ),
                    );
                    break;
                }
            }
            Sort::Size => {
                let onr = sort(city, Sort::Size, 1);
                for iter in onr.iter() {
                    sum.insert(
                        iter.size,
                        format!(
                            "来自{} {}的{}㎡",
                            iter.area, iter.community, iter.size
                        ),
                    );
                    break;
                }
            }
            Sort::Rb => {
                let onr = sort(city, Sort::Size, 1);
                for iter in onr.iter() {
                    sum.insert(
                        iter.reference_budget,
                        format!(
                            "来自{} {}的{}万元",
                            iter.area, iter.community, iter.reference_budget
                        ),
                    );
                    break;
                }
            }
        }

    }
    if let Some(&max_key) = sum.keys().max() {
        if let Some(&ref max_value) = sum.get(&max_key) {
            max_value.to_string()
        }
        else {
            "无数据".to_string()
        }
    } else {
        "无数据".to_string()
    }
}
