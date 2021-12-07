use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::{env};
use std::error::Error;
use std::fs::File;
use handlebars::Handlebars;
use opener;
use console::style;
mod cli;
mod helpers;
mod project_creator;

fn pkg(hb: &mut Handlebars, r_path: &str, entry_file: &str) -> String{
    hb.unregister_template("index");
    match hb.register_template_file("index", format!("{}/{}", r_path, entry_file)){
        Err(e)=>{println!("Can't render the template: {}", style(e).red().bold()); return "".to_owned();}
        _=>{}
    };
    let path = format!("{}/build.html", r_path);
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

fn main() -> Result<(), Box<dyn Error>> {
    let opt = cli::get_args();

    let r_path = match env::current_dir(){
        Ok(v)=>v,
        Err(e)=>{println!("Can't get current directory path: {}", style(e).red().bold()); return Ok(())}
    };

    let entry_html = match opt.entry {
        Some(v) if v.ne("")=>v,
        _=>format!("{}/{}", project_creator::TEMPLATE_DIR, project_creator::ENTRY_TEMPLATE)
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


    if opt.pkg{
        let p = pkg(&mut hb, &r_path.display().to_string(), &entry_html);
        if !p.eq(""){println!("Find the file at: {}", style(p).green().bold());}
        return Ok(())
    }

    if opt.show{
        let p = pkg(&mut hb, &r_path.display().to_string(), &entry_html);
        if p.eq(""){return Ok(())}
        let _ = opener::open(p)?;
        println!(
            "Type {} or {}",
            style("'r' to rebuild").green().bold(),
            style("'q' to quit").red().bold()
        );
        loop {
            match read_char()? {
                'r' => pkg(&mut hb, &r_path.display().to_string(), &entry_html),
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
                    project_creator::create_new_project(&r_path.display().to_string(), &name);
                    return Ok(());
                }
                
            }
        }
        None => {}
    }

    println!("Use -h to see available options");
    Ok(())
}