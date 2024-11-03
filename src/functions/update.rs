use chrono::{DateTime, NaiveDateTime, Utc};
use fnck_sql::db::DataBaseBuilder;
use fnck_sql::implement_from_tuple;
use fnck_sql::types::tuple::SchemaRef;
use fnck_sql::types::tuple::Tuple;
use fnck_sql::types::value::DataValue;
use fnck_sql::types::LogicalType;
use itertools::Itertools;
use regex::Regex;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs;
use std::thread;
use std::time::Duration;
use std::time::UNIX_EPOCH;

#[derive(Default, Debug, PartialEq)]
pub struct House {
    pub title: String,
    pub community: String,
    pub area: String,
    pub price_per_meter: i64,
    pub total_price: i64,
    pub layout: String,
    pub floor: String,
    pub size: i64,
    pub decoration: String,
    pub orientation: String,
    pub time: String,
    pub property_ownership_type: String,
    pub property_type: String,
    pub ownership_duration: String,
    pub reference_budget: i64,
    pub publishing_company: String,
    pub business_license: String,
    pub update_date: String,
    pub link: String,
}

implement_from_tuple!(
    House, (
        title: String => |inner: &mut House, value| {
            if let DataValue::Utf8 { value: Some(val), .. } = value {
                inner.title = val;
            }
        },
        community: String => |inner: &mut House, value| {
            if let DataValue::Utf8 { value: Some(val), .. } = value {
                inner.community = val;
            }
        },
        area: String => |inner: &mut House, value| {
            if let DataValue::Utf8 { value: Some(val), .. } = value {
                inner.area = val;
            }
        },
        price_per_meter: i64 => |inner: &mut House, value| {
            if let DataValue::Int64(Some(val)) = value {
                inner.price_per_meter = val;
            }
        },
        total_price: i64 => |inner: &mut House, value| {
            if let DataValue::Int64(Some(val)) = value {
                inner.total_price = val;
            }
        },
        layout: String => |inner: &mut House, value| {
            if let DataValue::Utf8 { value: Some(val), .. } = value {
                inner.layout = val;
            }
        },
        floor: String => |inner: &mut House, value| {
            if let DataValue::Utf8 { value: Some(val), .. } = value {
                inner.floor = val;
            }
        },
        size: i64 => |inner: &mut House, value| {
            if let DataValue::Int64(Some(val)) = value {
                inner.size = val;
            }
        },
        decoration: String => |inner: &mut House, value| {
            if let DataValue::Utf8 { value: Some(val), .. } = value {
                inner.decoration = val;
            }
        },
        orientation: String => |inner: &mut House, value| {
            if let DataValue::Utf8 { value: Some(val), .. } = value {
                inner.orientation = val;
            }
        },
        time: String => |inner: &mut House, value| {
            if let DataValue::Utf8 { value: Some(val), .. } = value {
                inner.time = val;
            }
        },
        property_ownership_type: String => |inner: &mut House, value| {
            if let DataValue::Utf8 { value: Some(val), .. } = value {
                inner.property_ownership_type = val;
            }
        },
        property_type: String => |inner: &mut House, value| {
            if let DataValue::Utf8 { value: Some(val), .. } = value {
                inner.property_type = val;
            }
        },
        ownership_duration: String => |inner: &mut House, value| {
            if let DataValue::Utf8 { value: Some(val), .. } = value {
                inner.ownership_duration = val;
            }
        },
        reference_budget: i64 => |inner: &mut House, value| {
            if let DataValue::Int64(Some(val)) = value {
                inner.reference_budget = val;
            }
        },
        publishing_company: String => |inner: &mut House, value| {
            if let DataValue::Utf8 { value: Some(val), .. } = value {
                inner.publishing_company = val;
            }
        },
        business_license: String => |inner: &mut House, value| {
            if let DataValue::Utf8 { value: Some(val), .. } = value {
                inner.business_license = val;
            }
        },
        update_date: String => |inner: &mut House, value| {
            if let DataValue::Utf8 { value: Some(val), .. } = value {
                inner.update_date = val;
            }
        },
        link: String => |inner: &mut House, value| {
            if let DataValue::Utf8 { value: Some(val), .. } = value {
                inner.link = val;
            }
        }
    )
);

pub fn update(position: String) -> Result<(), Box<dyn Error>> {
    let datapath = format!("{}{}", r#"./data/"#, &position);
    let database = DataBaseBuilder::path(datapath).build()?;
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
    for i in 0..2 {
        let _ = database.run(create_table_sql)?;
        if i == 0 {
            let _ = database.run("drop table mytable")?;
        }
    }

    let mut j = 0;

    for i in 0..12 {
        let url_front = r#"https://"#;
        let url_middle = r#".anjuke.com/sale"#;
        let url_page = || {
            if i  == 0 {
                r#""#.to_string()
            } else {
                format!("{}{}", r#"/p"#, i + 1)
            }
        };
        let url_back = r#"/?from=HomePage_TopBar"#;
        let url = format!(
            "{}{}{}{}{}",
            url_front,
            position,
            url_middle,
            url_page(),
            url_back
        );

        //println!("{:?}", url);

        let response = reqwest::blocking::get(&url).expect("Could not load url.");
        let body = response.text().unwrap();
        let document = Html::parse_document(&body);
        //房子链接的选择器
        let page_selector = Selector::parse(r#"div.property a"#).unwrap();

        //遍历房子链接
        for element in document.select(&page_selector) {
            let mut temp = House {
                title: String::new(),
                community: String::new(),
                area: String::new(),
                price_per_meter: 0,
                total_price: 0,
                layout: String::new(),
                floor: String::new(),
                size: 0,
                decoration: String::new(),
                orientation: String::new(),
                time: String::new(),
                property_ownership_type: String::new(),
                property_type: String::new(),
                ownership_duration: String::new(),
                reference_budget: 0,
                publishing_company: String::new(),
                business_license: String::new(),
                update_date: String::new(),
                link: String::new(),
            };

            let Some(page) = element.value().attr("href") else {
                continue;
            };
            //println!("{}", page);
            //进入房子链接，获得其html信息
            let page_url = page;
            let re = Regex::new(r"^(.*?)\?auction").unwrap();
            if let Some(captures) = re.captures(page_url) {
                if let Some(matched) = captures.get(1) {
                    println!("{}", matched.as_str());
                    temp.link = matched.as_str().to_string();
                }
            }

            let page_document: Html;
            let page_element = reqwest::blocking::get(page_url).expect("Could not load url.");
            if let Ok(page_body) = page_element.text() {
                page_document = Html::parse_document(&page_body);
            } else {
                continue;
            }

            //标题选择器
            let title_selector = Selector::parse(r#"div.banner-title h1.title"#).unwrap();
            //名字选择器
            let name_selector = Selector::parse(r#"div.community-title h3"#).unwrap();
            //地址选择器
            let address_selector =
                Selector::parse(r#"span.maininfo-community-item-name a"#).unwrap();

            //
            struct InfoSelectors {
                price_per_meter: Selector,
                price_per_meter2: Selector,
                total_price: Selector,
                total_price2: Selector,
                info: Selector,
                trade: Selector,
            }
            //
            let info_selector = InfoSelectors {
                price_per_meter: Selector::parse(".maininfo-avgprice-price").unwrap(),
                price_per_meter2: Selector::parse(".maininfo-avgprice-3l-price-num").unwrap(),
                total_price: Selector::parse("div.maininfo-price-wrap").unwrap(),
                total_price2: Selector::parse(".maininfo-price-3l-num").unwrap(),
                info: Selector::parse(".maininfo-model").unwrap(),
                trade: Selector::parse(".houseInfo-main-item-name").unwrap(),
            };

            //寻找标题
            if let Some(title_element) = page_document.select(&title_selector).next() {
                temp.title = title_element.text().collect::<Vec<_>>().concat();
                //打印标题
                //println!("{:?} - ", temp.title);
            } else {
                // 找不到时继续
                continue;
            }

            //寻找名字
            let name_element = page_document
                .select(&name_selector)
                .next()
                .expect("name_error");
            temp.community = name_element.text().collect::<String>();
            //寻找地址
            let address_element = page_document
                .select(&address_selector)
                .next()
                .expect("address_error");
            temp.area = address_element.text().collect::<String>();

            if let Some(ppm_element) = page_document.select(&info_selector.price_per_meter).next() {
                let house_ppm = ppm_element.text().collect::<String>();
                //println!("{:#?} {:#?} {:#?}", house_ppm,&clean_string(&house_ppm),extract_number(&clean_string(&house_ppm)));
                temp.price_per_meter = extract_number(&clean_string(&house_ppm)) as i64;
            } else {
                let ppm_element = page_document
                    .select(&info_selector.price_per_meter2)
                    .next()
                    .expect("ppm_error");
                let house_ppm = ppm_element.text().collect::<String>();
                println!(
                    "{:#?} {:#?} {:#?}",
                    house_ppm,
                    &clean_string(&house_ppm),
                    extract_number(&clean_string(&house_ppm))
                );
                temp.price_per_meter = extract_number(&clean_string(&house_ppm)) as i64;
            }

            if let Some(tp_element) = page_document.select(&info_selector.total_price).next() {
                let house_tp = tp_element.text().collect::<String>();
                temp.total_price = extract_number(&house_tp) as i64;
            } else {
                let tp_element = page_document
                    .select(&info_selector.total_price2)
                    .next()
                    .expect("tp_error");
                let house_tp = tp_element.text().collect::<String>();
                temp.total_price = extract_number(&house_tp) as i64;
            }

            let info_element = page_document
                .select(&info_selector.info)
                .next()
                .expect("info_error");
            let house_info = info_element.text().collect::<String>();

            let re = Regex::new(r"([\d室\d厅\d卫]+)|([高中低层]+(\(.+?\))?|暂无楼层)|(\d+㎡)|([\u4e00-\u9fa5]+装修|毛坯)|([东南西北]+)|(\d+年竣工/[\u4e00-\u9fa5]+)").unwrap();

            // 创建一个存放匹配结果的 Vector
            let mut info_result: Vec<String> = Vec::new();

            // 查找所有匹配项并将其放入 Vector
            for cap in re.captures_iter(&house_info) {
                // 遍历每个捕获组
                for i in 1..cap.len() {
                    if let Some(matched) = cap.get(i) {
                        info_result.push(matched.as_str().trim().to_string());
                    }
                }
            }
            //println!("{:#?}", info_result);
            if info_result.len() == 6 {
                (
                    temp.layout,
                    temp.floor,
                    temp.size,
                    temp.decoration,
                    temp.orientation,
                    temp.time,
                ) = (
                    info_result[0].clone(),
                    info_result[1].clone(),
                    extract_number(&info_result[2]) as i64,
                    info_result[3].clone(),
                    info_result[4].clone(),
                    info_result[5].clone(),
                );
            } else if info_result.len() == 8 {
                (
                    temp.layout,
                    temp.floor,
                    temp.size,
                    temp.decoration,
                    temp.orientation,
                    temp.time,
                ) = (
                    info_result[0].clone(),
                    info_result[1].clone(),
                    extract_number(&info_result[3]) as i64,
                    info_result[5].clone(),
                    info_result[6].clone(),
                    info_result[7].clone(),
                );
            } else if info_result.len() == 7 {
                if info_result[2].find("层").is_some() {
                    (
                        temp.layout,
                        temp.floor,
                        temp.size,
                        temp.decoration,
                        temp.orientation,
                        temp.time,
                    ) = (
                        info_result[0].clone(),
                        info_result[1].clone(),
                        extract_number(&info_result[3]) as i64,
                        info_result[4].clone(),
                        info_result[5].clone(),
                        info_result[6].clone(),
                    );
                }
                (
                    temp.layout,
                    temp.floor,
                    temp.size,
                    temp.decoration,
                    temp.orientation,
                    temp.time,
                ) = (
                    info_result[0].clone(),
                    info_result[1].clone(),
                    extract_number(&info_result[2]) as i64,
                    info_result[4].clone(),
                    info_result[5].clone(),
                    info_result[6].clone(),
                );
            }

            // //temp.layout = info_result[0].clone();
            // print!("{}",temp.layout);
            // //temp.floor = info_result[1].clone();
            // print!("{}",temp.floor);
            // //temp.size = extract_number(&info_result[2]);
            // print!("{}",temp.size);
            // //temp.decoration = info_result[3].clone();
            // print!("{}",temp.decoration);
            // //temp.orientation = info_result[4].clone();
            // print!("{}",temp.orientation);
            // //temp.time = info_result[5].clone();
            // println!("{}",temp.time);

            let mut trade_result: Vec<String> = Vec::new();
            for trade_element in page_document.select(&info_selector.trade) {
                let house_trade = trade_element.text().collect::<String>();
                trade_result.push(clean_string(&house_trade));
            }
            //println!("{:#?}", trade_result);
            assign_values(&mut temp, trade_result);
            //println!("{:#?}",temp);
            let insert_value = format!("insert into mytable values({}, '{}', '{}', '{}', {}, {}, '{}', '{}', {}, '{}', '{}', '{}', '{}', '{}', '{}', {}, '{}', '{}', '{}', '{}')", j, temp.title, temp.community, temp.area, temp.price_per_meter, temp.total_price, temp.layout, temp.floor, temp.size, temp.decoration, temp.orientation, temp.time, temp.property_ownership_type, temp.property_type, temp.ownership_duration, temp.reference_budget, temp.publishing_company, temp.business_license, temp.update_date, temp.link);
            let _ = database.run(insert_value)?;
            //println!("OK");
            // let (schema, tuples) = database.run("select * from mytable")?;
            // let _tuples = tuples
            //     .into_iter()
            //     .map(|tuple| House::from((&schema, tuple)))
            //     .collect_vec();

            //println!("{:#?}", tuples);
            //打印
            println!("{:?} - {:?} - {:?} - {:?} - {:?} - {:?} - {:?} - {:?} - {:?} - {:?} - {:?} - {:?} - {:?} - {:?} - {:?} - {:?} - {:?}", temp.community, temp.area, temp.price_per_meter, temp.total_price, temp.layout, temp.floor, temp.size, temp.decoration, temp.orientation, temp.time, temp.property_ownership_type, temp.property_type, temp.ownership_duration, temp.reference_budget, temp.publishing_company, temp.business_license, temp.update_date);
            //println!("OK");
            j = j + 1;
            println!("{}", j);
            thread::sleep(Duration::from_secs(15));
        }
    }
    Ok(())
}

fn clean_string(input: &str) -> String {
    input.replace("\n", "").replace("\r", "").trim().to_string()
}

fn extract_number(input: &str) -> f64 {
    let re = Regex::new(r"[\d\.]+").unwrap(); // 匹配数字（包括小数点）
    if let Some(caps) = re.captures(input) {
        // 将第一个匹配的数字转换为 f64
        caps.get(0)
            .map(|m| m.as_str().parse::<f64>().ok())
            .flatten()
            .unwrap_or(0.0)
    } else {
        0.0
    }
}

fn extract_number2(input: &str) -> f64 {
    let re = Regex::new(r"(\d+\.?\d*)").unwrap(); // 匹配数字
    let mut numbers: Vec<f64> = Vec::new();
    for cap in re.captures_iter(input) {
        if let Some(num_str) = cap.get(0) {
            // 将提取的数字字符串转换为 f64
            if let Ok(num) = num_str.as_str().parse::<f64>() {
                numbers.push(num);
            }
        }
    }
    numbers[0]
}

fn assign_values(temp: &mut House, data: Vec<String>) {
    if data.len() == 7 {
        temp.property_ownership_type = data[0].clone();
        temp.property_type = data[1].clone();
        temp.ownership_duration = data[2].clone();
        temp.reference_budget = extract_number2(&data[3]) as i64;
        temp.publishing_company = data[4].clone();
        temp.business_license = data[5].clone();
        temp.update_date = data[6].clone();
    } else if data.len() == 8 {
        let mut set = 4;
        if is_valid_date(&data[6]) {
            set = 3;
        }
        temp.property_ownership_type = data[0].clone();
        temp.property_type = data[1].clone();
        temp.ownership_duration = data[2].clone(); // 填充默认值
        temp.reference_budget = extract_number2(&data[set]) as i64;
        temp.publishing_company = data[set + 1].clone();
        temp.business_license = data[set + 2].clone();
        temp.update_date = data[set + 3].clone();
    } else if data.len() == 9 {
        let mut set = 5;
        if is_valid_date(&data[7]) {
            set = 4;
        } else if is_valid_date(&data[6]) {
            set = 3;
        }
        temp.property_ownership_type = data[0].clone();
        temp.property_type = data[1].clone();
        temp.ownership_duration = data[2].clone(); // 填充默认值
        temp.reference_budget = extract_number2(&data[set]) as i64;
        temp.publishing_company = data[set + 1].clone();
        temp.business_license = data[set + 2].clone();
        temp.update_date = data[set + 3].clone();
    } else if data.len() == 10 {
        let mut set = 5;
        if is_valid_date(&data[7]) {
            set = 4;
        }
        temp.property_ownership_type = data[0].clone();
        temp.property_type = data[1].clone();
        temp.ownership_duration = data[2].clone(); // 填充默认值
        temp.reference_budget = extract_number2(&data[set]) as i64;
        temp.publishing_company = data[set + 1].clone();
        temp.business_license = data[set + 2].clone();
        temp.update_date = data[set + 3].clone();
    } else if data.len() == 11 {
        temp.property_ownership_type = data[0].clone();
        temp.property_type = data[1].clone();
        temp.ownership_duration = data[2].clone(); // 填充默认值
        temp.reference_budget = extract_number2(&data[5]) as i64;
        temp.publishing_company = data[6].clone();
        temp.business_license = data[7].clone();
        temp.update_date = data[8].clone();
    }
    // 处理其他情况...
}

fn is_valid_date(date: &str) -> bool {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    re.is_match(date)
}

pub fn date() -> Vec<String> {
    let path = "./data";
    let mut temp: Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        let name = entry.file_name();
                        let name_str = name.to_string_lossy();
                        if let Ok(modified_time) = metadata.modified() {
                            let duration = modified_time.duration_since(UNIX_EPOCH).unwrap();
                            let modified_timestamp = duration.as_secs();
                            let naive_datetime =
                                NaiveDateTime::from_timestamp(modified_timestamp as i64, 0);
                            let datetime: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
                            let formatted_date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                            temp.push(format!(r#"City: {}, Time: {}"#, name_str, formatted_date,));
                        }
                    }
                }
            }
        }
    }
    temp
}

pub fn name() -> Vec<String> {
    let path = "./data";
    let mut temp: Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        let name = entry.file_name();
                        let name_str = name.to_string_lossy();
                        temp.push(name_str.to_string());
                    }
                }
            }
        }
    }
    temp
}
