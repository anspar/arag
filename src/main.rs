use console::style;
use handlebars::Handlebars;
use notify::{watcher, RecursiveMode, Watcher};
use opener;
use std::env;
use std::error::Error;
use std::fs::File;
use std::sync::mpsc::channel;
use std::time::Duration;
mod types;
mod utils;
use utils::{cli, helpers, yaml_parser};

use crate::types::AragConf;
mod constants;
mod project_creator;

fn pkg(hb: &mut Handlebars, r_path: &str, entry: &str, gateway: &str) -> String {
    hb.clear_templates();
    match hb.register_templates_directory(".html", entry) {
        Err(e) => {
            println!("Can't render the template: {}", style(e).red().bold());
            return "".to_owned();
        }
        _ => {}
    };
    let path = format!("{}/build.html", r_path);
    match File::create(&path) {
        Ok(mut output_file) => {
            match hb.render_to_write("index", &gateway, &mut output_file) {
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
}

fn start_dir_watcher(hb: &mut Handlebars, r_path: &str, entry: &str, gateway: &str) {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();
    watcher.watch(&entry, RecursiveMode::Recursive).unwrap();
    watcher
        .watch(
            format!("{}/{}", &r_path, &constants::STATIC_DIR),
            RecursiveMode::Recursive,
        )
        .expect(&format!("No {} dir", style(&constants::STATIC_DIR).red()));
    loop {
        match rx.recv() {
            Ok(e) => match e {
                notify::DebouncedEvent::NoticeWrite(_) => {
                    pkg(hb, r_path, entry, gateway);
                }
                _ => continue,
            },
            Err(e) => println!("{}", style(e).red().bold()),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
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
            AragConf::defaults()
        }
    };

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
    hb.register_helper("import_html", Box::new(helpers::import_html));
    hb.register_helper("import_raw", Box::new(helpers::import_html));
    hb.register_helper("import_img", Box::new(helpers::import_img));
    hb.register_helper("import_video", Box::new(helpers::import_video));
    hb.register_helper("import_audio", Box::new(helpers::import_audio));
    hb.register_helper("import_css", Box::new(helpers::import_css));
    hb.register_helper("import_css_web", Box::new(helpers::import_css_web));
    hb.register_helper("import_json", Box::new(helpers::import_json));
    hb.register_helper("import_wasm", Box::new(helpers::import_wasm));
    hb.register_helper("import_bytes", Box::new(helpers::import_bytes));
    hb.register_helper("import_js_ipfs", Box::new(helpers::import_js_ipfs));
    hb.register_helper("import_css_ipfs", Box::new(helpers::import_css_ipfs));
    hb.register_helper("import_bytes_ipfs", Box::new(helpers::import_bytes_ipfs));
    hb.register_helper("import_raw_ipfs", Box::new(helpers::import_raw_ipfs));
    hb.register_helper("inject_gateway", Box::new(helpers::inject_gateway));

    if opt.pkg {
        let p = pkg(
            &mut hb,
            &r_path.display().to_string(),
            &entry_dir,
            &conf.ipfs_gateway,
        );
        if !p.eq("") {
            println!("Find the file at: {}", style(p).green().bold());
        }
        return Ok(());
    }

    if opt.show {
        let p = pkg(
            &mut hb,
            &r_path.display().to_string(),
            &entry_dir,
            &conf.ipfs_gateway,
        );
        if p.eq("") {
            return Ok(());
        }
        let _ = opener::open(p)?;

        start_dir_watcher(
            &mut hb,
            &r_path.display().to_string(),
            &entry_dir,
            &conf.ipfs_gateway,
        );
        return Ok(());
    }

    if let Some(v) = opt.cmd {
        match v {
            cli::Command::New { name } => {
                project_creator::create_new_project(&r_path.display().to_string(), &name)
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
