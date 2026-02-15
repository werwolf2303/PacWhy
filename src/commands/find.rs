use crate::schemas::{Package, PackageRaw};
use comfy_table::{Row, Table};
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait};
use sea_orm::sqlx::types::chrono::{DateTime, Local, NaiveDateTime, Utc};
use comfy_table::presets::UTF8_FULL;

pub async fn find_handler(name: String, description: String, time: String, parseable: String, bypass_term_check: bool, db: &DatabaseConnection) {
    let mut query = Package::find();
    let mut did_provide_search_term = false;

    if !name.is_empty() {
        query = query.filter(PackageRaw::Column::Name.eq(name));
        did_provide_search_term = true;
    }

    if !description.is_empty() {
        query = query.filter(PackageRaw::Column::Description.eq(description));
        did_provide_search_term = true;
    }

    if !time.is_empty() {
        query = query.filter(PackageRaw::Column::DateAdded.eq(time));
        did_provide_search_term = true;
    }
    
    if !bypass_term_check && !did_provide_search_term {
        eprintln!("No search terms were provided");
        return;
    }

    let result = query.all(db).await;

    if result.is_err() {
        println!("{:#?}", result.err());
        return;
    }

    let result_unwrapped = result.unwrap();

    if parseable.to_lowercase().eq("false") {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

        table.set_header(Row::from(vec![
            "Name",
            "Description",
            "DateAdded",
            "Reason"
        ]));

        for model in result_unwrapped {
            table.add_row(Row::from(vec![
                model.name,
                model.description,
                format_date(model.date_added),
                model.reason
            ]));
        }

        println!("{}", table.to_string());
    } else {
        for model in result_unwrapped {
            print!("{}---", model.name);
            print!("{}---", model.description);
            print!("{}---", model.date_added);
            println!("{}", model.reason);
        }
    }
}

fn format_date(date: NaiveDateTime) -> String {
    let dt_utc: DateTime<Utc> = DateTime::<Utc>::from_naive_utc_and_offset(date, Utc);
    let dt_local: DateTime<Local> = dt_utc.with_timezone(&Local);
    dt_local.format("%Y-%m-%d %H:%M:%S").to_string()
}