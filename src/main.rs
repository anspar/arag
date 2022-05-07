#[macro_use]
extern crate rocket;
use console::style;
use handlebars::Handlebars;
use notify::{watcher, RecursiveMode, Watcher};
use opener;
use rocket::fairing::AdHoc;
use rocket::figment::Figment;
use rocket::http::Status;
use rocket::response::content::{Html, Json};
use rocket::response::status;
use rocket::{tokio, State};
use std::env;
use std::error::Error;
use std::fs::File;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use utils::cache::cache_from_web;
mod types;
mod utils;
use utils::{cli, helpers, yaml_parser};

use crate::types::{AragConf, AragState};
mod constants;
mod project_creator;

fn pkg(state: AragState, file: bool) -> String {
    let mut hb = state.hb.lock().unwrap();
    hb.clear_templates();
    match hb.register_templates_directory(".html", state.entry_dir) {
        Err(e) => {
            println!("Can't render the template: {}", style(e).red().bold());
            return "".to_owned();
        }
        _ => {}
    };
    if file {
        let path = format!("{}/build.html", &state.root_dir_path);
        match File::create(&path) {
            Ok(mut output_file) => {
                match hb.render_to_write("index", &state.conf.ipfs_gateway, &mut output_file) {
                    Err(e) => {
                        println!("Can't write to the build file: {}", style(e).red().bold());
                        return "".to_owned();
                    }
                    _ => {}
                };
            }
            Err(e) => {
                println!("Can't create the build file: {}", style(e).red().bold());
                return "".to_owned();
            }
        }
        println!("{}", style("Build finished").green());
        path
    } else {
        match hb.render("index", &state.conf.ipfs_gateway) {
            Err(e) => {
                println!("Can't render 'index.html': {}", style(e).red().bold());
                return "".to_owned();
            }
            Ok(v) => v,
        }
    }
}

fn start_dir_watcher(state: AragState) {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();
    watcher
        .watch(&state.entry_dir, RecursiveMode::Recursive)
        .unwrap();
    watcher
        .watch(
            format!("{}/{}", &state.root_dir_path, &constants::STATIC_DIR),
            RecursiveMode::Recursive,
        )
        .expect(&format!("No {} dir", style(&constants::STATIC_DIR).red()));
    loop {
        match rx.recv() {
            Ok(e) => match e {
                notify::DebouncedEvent::NoticeWrite(_) => {
                    // println!("{}", style("File changes detected").green().bold());
                    let mut fu = state.files_updated.lock().unwrap();
                    *fu = true;
                }
                _ => continue,
            },
            Err(e) => println!("{}", style(e).red().bold()),
        }
    }
}

async fn cache_dependencies(dep: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    for d in dep {
        let hash = cache_from_web(d).await?;
        println!("Cached as {}", style(hash).green().bold());
    }
    Ok(())
}

#[get("/")]
pub async fn index(state: &State<AragState<'_>>) -> status::Custom<Html<String>> {
    let state = &**state;
    let build = pkg(state.clone(), false);
    let mut su = state.files_updated.lock().unwrap();
    *su = false;
    status::Custom(Status::Ok, Html(build))
}

#[get("/update")]
pub async fn update(state: &State<AragState<'_>>) -> status::Custom<Json<String>> {
    let state = &**state;
    let fu = state.files_updated.lock().unwrap();
    status::Custom(Status::Ok, Json(format!("{{\"update\": {fu}}}")))
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = cli::get_args();
    let r_path = match env::current_dir() {
        Ok(v) => v,
        Err(e) => {
            println!(
                "Can't get current directory path: {}",
                style(e).red().bold()
            );
            return Ok(());
        }
    };

    let conf = match yaml_parser::get_conf(&format!("{}/arag.yml", r_path.display())) {
        Ok(v) => v,
        Err(_) => {
            println!("No arag.yml found, using defaults");
            AragConf::default()
        }
    };
    match cache_dependencies(&conf.dependencies).await {
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
        _ => {}
    }
    let entry_dir = match opt.entry {
        Some(v) => format!("{}/{}", &r_path.display().to_string(), v),
        None => format!(
            "{}/{}",
            &r_path.display().to_string(),
            constants::TEMPLATE_DIR
        ),
    };

    let mut hb = Handlebars::new();

    hb.register_helper("import_js", Box::new(helpers::import_js));
    hb.register_helper("import_js_web", Box::new(helpers::import_js_web));
    hb.register_helper("import_img", Box::new(helpers::import_img));
    hb.register_helper("import_video", Box::new(helpers::import_video));
    hb.register_helper("import_audio", Box::new(helpers::import_audio));
    hb.register_helper("import_css", Box::new(helpers::import_css));
    hb.register_helper("import_css_web", Box::new(helpers::import_css_web));
    hb.register_helper("import_json", Box::new(helpers::import_json));
    hb.register_helper("import_wasm", Box::new(helpers::import_wasm));
    hb.register_helper("import_bytes", Box::new(helpers::import_bytes));
    hb.register_helper("import_bytes_web", Box::new(helpers::import_bytes_web));
    hb.register_helper("import_content", Box::new(helpers::import_content));
    hb.register_helper("web_component", Box::new(helpers::web_component));
    hb.register_helper("inject_gateway", Box::new(helpers::inject_gateway));
    hb.register_helper("live_update", Box::new(helpers::live_update));

    // let hb = Arc::new(Mutex::new(hb));
    let arag_state = AragState {
        root_dir_path: r_path.display().to_string(),
        conf,
        entry_dir,
        hb: Arc::new(Mutex::new(hb)),
        files_updated: Arc::new(Mutex::new(true)),
    };
    if opt.pkg {
        let p = pkg(arag_state, true);
        if !p.eq("") {
            println!("Find the file at: {}", style(p).green().bold());
        }
        return Ok(());
    }

    if opt.show {
        let figment = Figment::from(rocket::Config::default())
            .merge(("log_level", "off"))
            .merge(("port", 16161u64))
            .merge(("address", "0.0.0.0"));
        let _ = rocket::custom(figment)
            .mount("/", routes![index, update])
            .manage(arag_state)
            .attach(AdHoc::on_liftoff("Directory Watcher", |r| {
                Box::pin(async move {
                    let state = r.state::<AragState>().unwrap().clone();
                    tokio::spawn(async move {
                        start_dir_watcher(state);
                    });
                })
            }))
            .attach(AdHoc::on_liftoff("Directory Watcher", |_| {
                Box::pin(async move {
                    println!(
                        "\n\tServing on {}",
                        style("http://0.0.0.0:16161").green().bold()
                    );
                    let _ = opener::open("http://0.0.0.0:16161").unwrap();
                })
            }))
            .launch()
            .await;

        return Ok(());
    }

    if let Some(v) = opt.cmd {
        match v {
            cli::Command::New { name } => {
                project_creator::create_new_project(&r_path.display().to_string(), &name)
                    .await
                    .unwrap_or_else(|e| {
                        println!("Failed to create new project: {}", style(e).red())
                    });
                return Ok(());
            }
        }
    }

    println!("Use -h to see available options");
    Ok(())
}
