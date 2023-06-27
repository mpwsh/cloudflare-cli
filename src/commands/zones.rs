use std::error::Error;

use cloudflare::endpoints::zone::ListZonesParams;
use cloudflare::framework::response::{ApiFailure, ApiResponse};
use cloudflare::framework::{apiclient::ApiClient, HttpApiClient, OrderDirection};
use tabular::Row;

use crate::api::endpoints::zones::{ListZones, ZoneVec};
use crate::commands::table_from_cols;

pub fn list(api: &HttpApiClient, page: u32, limit: u32) {
    let response: ApiResponse<ZoneVec> = api.request(&ListZones {
        params: ListZonesParams {
            name: None,
            status: None,
            page: Some(page),
            per_page: Some(limit),
            order: None,
            direction: Some(OrderDirection::Ascending),
            search_match: None,
        },
    });
    match response {
        Ok(success) => {
            let list: ZoneVec = success.result;
            let columns = vec!["ID", "NAME", "STATUS", "PLAN", "DNS ONLY"];

            let mut table = table_from_cols(columns);

            let vec1 = list.zones;
            for record in vec1.iter() {
                let plan = match &record.plan {
                    Some(p) => p.name.to_owned(),
                    _ => "-".to_string(),
                };

                table.add_row(
                    Row::new()
                        .with_cell(&record.id)
                        .with_cell(&record.name)
                        .with_cell(format!("{:?}", record.status))
                        .with_cell(plan)
                        .with_cell(if record.paused { "Yes" } else { "No" }),
                );
            }
            print!("{}", table);
        }
        Err(e) => match e {
            ApiFailure::Error(code, err) => println!("Error {}: {:?}", code, err),
            ApiFailure::Invalid(err) => println!("{:?}", err.source()),
        },
    }
}
