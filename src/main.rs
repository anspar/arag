use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::{env, fs};
use std::error::Error;
use std::fs::File;
use std::io::{Read};
use base64;

use handlebars::{ Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError};

use opener;
use console::style;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Serve the packaged html
    #[structopt(short, long)]
    show: bool,

    /// package everything into a single html
    #[structopt(short, long)]
    pkg: bool,
}

fn get_file_content_text(file_path: &str)-> String{
    let r_path = env::current_dir().unwrap();
    let js = fs::read_to_string(format!("{}/{}", r_path.display(), file_path));
    js.unwrap()
}

fn get_web_content_text(url: &str)->String{
    let mut js: String = "".to_owned();
    let _ = reqwest::blocking::get(url).unwrap().read_to_string(&mut js).unwrap();
    js
}

// define a custom helper
fn import_js(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for import_js helper."))?;
    out.write(format!("<script>{}</script>", get_file_content_text(&param.value().render())).as_ref())?;
    Ok(())
}

fn import_js_web(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for import_js_web helper."))?;
    out.write(format!("<script>{}</script>", get_web_content_text(&param.value().render())).as_ref())?;
    Ok(())
}

fn import_css(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for import_css helper."))?;
    out.write(format!("<style>{}</style>", get_file_content_text(&param.value().render())).as_ref())?;
    Ok(())
}

fn import_css_web(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for import_js_web helper."))?;
    out.write(format!("<style>{}</style>", get_web_content_text(&param.value().render())).as_ref())?;
    Ok(())
}

fn import_html(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for import_html helper."))?;
    // let rendered = format!("{} pts", param.value().render());
    let r_path = env::current_dir().unwrap();
    let js = fs::read_to_string(format!("{}/{}", r_path.display(), param.value().render()));
    out.write(js.unwrap().as_ref())?;
    Ok(())
}

fn import_img(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for import_img helper."))?;
    // let rendered = format!("{} pts", param.value().render());
    let r_path = env::current_dir().unwrap();
    let rel_path = param.value().render();
    let img = fs::read(format!("{}/{}", r_path.display(), rel_path));
    let b64 = base64::encode(img.unwrap());
    let ext: Vec<&str> = rel_path.split(".").collect();
    out.write(format!("data:image/{};base64,{}", ext.last().unwrap().to_string(), b64).as_ref())?;
    Ok(())
}

fn pkg(hb: &Handlebars, r_path: &str) -> Result<String, Box<dyn Error>>{
    let path = format!("{}/out.html", r_path);
    let mut output_file = File::create(&path)?;
    hb.render_to_write("index", &false, &mut output_file)?;
    println!("generated out.html");
    Ok(path)
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
    // env_logger::init();
    let opt = Opt::from_args();

    let r_path = env::current_dir().unwrap();

    let mut hb = Handlebars::new();

    hb.register_helper("import_js", Box::new(import_js));
    hb.register_helper("import_js_web", Box::new(import_js_web));
    hb.register_helper("import_html", Box::new(import_html));
    hb.register_helper("import_img", Box::new(import_img));
    hb.register_helper("import_css", Box::new(import_css));
    hb.register_helper("import_css_web", Box::new(import_css_web));
    // handlebars.register_helper("format", Box::new(FORMAT_HELPER));


    hb
        .register_template_file("index", format!("{}/templates/index.html.hbs", r_path.display()))
        .unwrap();

    if opt.pkg{
        pkg(&hb, &r_path.display().to_string());
        return Ok(())
    }

    if opt.show{
        let f = pkg(&hb, &r_path.display().to_string())?;
        let _ = opener::open(f)?;
        println!(
            "Type {} or {}",
            style("'r' to rebuild").green().bold(),
            style("'q' to quit").red().bold()
        );
        loop {
            match read_char()? {
                'r' => pkg(&hb, &r_path.display().to_string())?,
                'q' => break,
                _ => continue
            };
        }
        return Ok(())        
    }

    println!("Use -h to see available options");
    Ok(())
}