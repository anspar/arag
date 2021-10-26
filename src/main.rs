use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::{env, fs};
use std::error::Error;
use std::fs::File;
use std::io::Write;

use handlebars::Handlebars;

use opener;
use console::style;
mod cli;
mod helpers;

static ENTRY_TEMPLATE: &str = "index.html";
static TEMPLATE_DIR: &str = "templates";
static STATIC_DIR: &str = "static";
static JAVASCRIPT_DIR: &str = "js";
static CSS_DIR: &str = "css";
static JAVASCRIPT_FILE: &str = "index.js";
static CSS_FILE: &str = "index.css";

fn pkg(hb: &mut Handlebars, r_path: &str) -> String{
    hb.unregister_template("index");
    match hb.register_template_file("index", format!("{}/{}/{}", r_path, TEMPLATE_DIR, ENTRY_TEMPLATE)){
        Err(e)=>{println!("Can't render the template: {}", style(e).red().bold()); return "".to_owned();}
        _=>{}
    };
    let path = format!("{}/out.html", r_path);
    match File::create(&path){
        Ok(mut output_file)=>{
            match hb.render_to_write("index", &false, &mut output_file){
                Err(e)=>{println!("Can't write to the build file: {}", style(e).red().bold()); return "".to_owned();}
                _=>{}
            };
        }
        Err(e)=>{println!("Can't create the build file: {}", style(e).red().bold()); return "".to_owned();}
    }
    println!("Build finished");
    path
}

fn read_char() -> Result<char, Box<dyn Error>> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = event::read()
        {
            return Ok(c);
        }
    }
}

fn create_new_project(r_path: &str, name: &str){
    match fs::create_dir(format!("{}/{}", r_path, &name)){
        Err(_)=>{println!("Project {} exists", style(name).red().bold()); return}
        _=>{}
    };
    fs::create_dir(format!("{}/{}/{}", r_path, &name, TEMPLATE_DIR)).unwrap_or_else(|_| {println!("Can't create {} dir",style(TEMPLATE_DIR).red().bold())});
    fs::create_dir(format!("{}/{}/{}", r_path, &name, STATIC_DIR)).unwrap_or_else(|_| {println!("Can't create {} dir",style(STATIC_DIR).red().bold())});
    fs::create_dir(format!("{}/{}/{}/{}", r_path, &name, STATIC_DIR, JAVASCRIPT_DIR)).unwrap_or_else(|_| {println!("Can't create {} dir",style(JAVASCRIPT_DIR).red().bold())});
    fs::create_dir(format!("{}/{}/{}/{}", r_path, &name, STATIC_DIR, CSS_DIR)).unwrap_or_else(|_| {println!("Can't create {} dir",style(CSS_DIR).red().bold())});
    match File::create(format!("{}/{}/{}/{}", r_path, &name, TEMPLATE_DIR, ENTRY_TEMPLATE)){
        Ok(mut ih_file)=>{
            ih_file.write_all(format!("
<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>{}</title>
    {{{{import_css \"{}/{}/{}\"}}}}
</head>
<body>
    <div class=\"main\">
        <h1>Anspar DApp builder</h1>
    </div>
    {{{{import_js \"{}/{}/{}\"}}}}
</body>
</html>
            ", name,
            STATIC_DIR,CSS_DIR,CSS_FILE,
            STATIC_DIR,JAVASCRIPT_DIR,JAVASCRIPT_FILE

        ).as_bytes()).unwrap_or_else(|_| {println!("Can't write to {} file",style(ENTRY_TEMPLATE).red().bold())});
        }
        Err(e)=>{println!("Can't create {} file: {}",style(ENTRY_TEMPLATE).red().bold(), e); return;}
    }
    
    match File::create(format!("{}/{}/{}/{}/{}", r_path, &name, STATIC_DIR, JAVASCRIPT_DIR, JAVASCRIPT_FILE)){
        Ok(mut js_file)=>{
            js_file.write_all(b"console.log('js is included!');").unwrap_or_else(|_| {println!("Can't write to {} file",style(JAVASCRIPT_FILE).red().bold())});
        }
        Err(e)=>{println!("Can't create {} file: {}",style(JAVASCRIPT_FILE).red().bold(), e); return}
    }

    match File::create(format!("{}/{}/{}/{}/{}", r_path, &name, STATIC_DIR, CSS_DIR, CSS_FILE)){
        Ok(mut css_file)=>{
            css_file.write_all(b"body{ background: #5dc0be;}").unwrap_or_else(|_| {println!("Can't write to {} file",style(CSS_FILE).red().bold())});
        }
        Err(e)=>{println!("Can't create {} file: {}",style(CSS_FILE).red().bold(), e); return;}
    }
    
    println!("Created project {}", style(name).green().bold());
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = cli::get_args();

    let r_path = match env::current_dir(){
        Ok(v)=>v,
        Err(e)=>{println!("Can't get current directory path: {}", style(e).red().bold()); return Ok(())}
    };

    let mut hb = Handlebars::new();

    hb.register_helper("import_js", Box::new(helpers::import_js));
    hb.register_helper("import_js_web", Box::new(helpers::import_js_web));
    hb.register_helper("import_html", Box::new(helpers::import_html));
    hb.register_helper("import_raw", Box::new(helpers::import_html));
    hb.register_helper("import_img", Box::new(helpers::import_img));
    hb.register_helper("import_css", Box::new(helpers::import_css));
    hb.register_helper("import_css_web", Box::new(helpers::import_css_web));
    hb.register_helper("import_json", Box::new(helpers::import_json));
    hb.register_helper("import_wasm", Box::new(helpers::import_wasm));


    if opt.pkg{
        let p = pkg(&mut hb, &r_path.display().to_string());
        if !p.eq(""){println!("Find the file at: {}", style(p).green().bold());}
        return Ok(())
    }

    if opt.show{
        let p = pkg(&mut hb, &r_path.display().to_string());
        if p.eq(""){return Ok(())}
        let _ = opener::open(p)?;
        println!(
            "Type {} or {}",
            style("'r' to rebuild").green().bold(),
            style("'q' to quit").red().bold()
        );
        loop {
            match read_char()? {
                'r' => pkg(&mut hb, &r_path.display().to_string()),
                'q' => break,
                _ => continue
            };
        }
        return Ok(())        
    }

    match opt.cmd {
        Some(v)=>{
            match v {
                cli::Command::New{name} => {
                    create_new_project(&r_path.display().to_string(), &name);
                    return Ok(());
                }
                
            }
        }
        None => {}
    }

    println!("Use -h to see available options");
    Ok(())
}