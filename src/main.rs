use std::{env, path::PathBuf, process::exit};

use monocode_scheme_rs::scheme::{get_lynx_palette, get_panther_palette};
use sniffer_rs::sniffer::Sniffer;
use tigris_rs::features::{
    actions::{CopyTextAction, ResultAction},
    api::{get_extension_request, send_search_results},
    search::get_search_query,
    search_results::SearchResult,
};

pub fn get_panther_icon(name: &str) -> PathBuf {
    env::current_dir()
        .unwrap()
        .join(format!("src/icons/panther/{name}.svg"))
}

pub fn get_lynx_icon(name: &str) -> PathBuf {
    env::current_dir()
        .unwrap()
        .join(format!("src/icons/lynx/{name}.svg"))
}

fn main() {
    let request = get_extension_request().get_results_request.unwrap();
    let search_query = get_search_query(&request.search_text);
    let mut results = Vec::<SearchResult>::new();
    let sniffer = Sniffer::new();
    let panther_palette = get_panther_palette();
    let lynx_palette = get_lynx_palette();

    if let Some(keyword) = search_query.keyword.clone() {
        if keyword == "rgb" {
            for color in panther_palette.clone() {
                if sniffer.matches(&color.name, &search_query.search_text) {
                    results.push(
                        SearchResult::new(&format!("Panther {}", &color.name))
                            .set_action(&ResultAction::new_copy_text_action(&CopyTextAction::new(
                                &color.rgb.display,
                            )))
                            .set_description(&color.rgb.display)
                            .set_icon_path(&get_panther_icon(&color.name)),
                    );
                }
            }

            for color in lynx_palette.clone() {
                if sniffer.matches(&color.name, &search_query.search_text) {
                    results.push(
                        SearchResult::new(&format!("Lynx {}", &color.name))
                            .set_action(&ResultAction::new_copy_text_action(&CopyTextAction::new(
                                &color.rgb.display,
                            )))
                            .set_description(&color.rgb.display)
                            .set_icon_path(&get_lynx_icon(&color.name)),
                    );
                }
            }

            send_search_results(&results);
            exit(0);
        }
    }

    for color in panther_palette.clone() {
        if sniffer.matches(&color.name, &search_query.search_text) {
            results.push(
                SearchResult::new(&format!("Panther {}", &color.name))
                    .set_action(&ResultAction::new_copy_text_action(&CopyTextAction::new(
                        &color.hex,
                    )))
                    .set_description(&color.hex)
                    .set_icon_path(&get_panther_icon(&color.name)),
            );
        }
    }

    for color in lynx_palette.clone() {
        if sniffer.matches(&color.name, &search_query.search_text) {
            results.push(
                SearchResult::new(&format!("Lynx {}", &color.name))
                    .set_action(&ResultAction::new_copy_text_action(&CopyTextAction::new(
                        &color.hex,
                    )))
                    .set_description(&color.hex)
                    .set_icon_path(&get_lynx_icon(&color.name)),
            );
        }
    }

    send_search_results(&results);
    exit(0);
}
