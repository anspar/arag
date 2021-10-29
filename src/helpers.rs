use std::{env, error::Error, fs, io::Read};

use console::style;
use handlebars::{Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError};

fn get_file_content_text(file_path: &str)-> Result<String, Box<dyn Error>>{
    let r_path = env::current_dir()?;
    let c = fs::read_to_string(format!("{}/{}", r_path.display(), file_path));
    Ok(c?)
}

fn get_file_content_bytes(file_path: &str)-> Result<Vec<u8>,  Box<dyn Error>>{
    let r_path = env::current_dir()?;
    let c = fs::read(format!("{}/{}", r_path.display(), file_path));
    Ok(c?)
}

fn get_web_content_text(url: &str)->Result<String, Box<dyn Error>>{
    let mut c: String = "".to_owned();
    let _ = reqwest::blocking::get(url)?.read_to_string(&mut c)?;
    Ok(c)
}

fn get_file_content_base64(file_path: &str) -> Result<String, Box<dyn Error>>{
    let r_path = env::current_dir()?;
    let img = fs::read(format!("{}/{}", r_path.display(), file_path));
    let b64 = base64::encode(img?);
    Ok(b64)
}

pub fn import_js(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for import_js helper."))?;
    let result = match get_file_content_text(&param.value().render()){
        Ok(v)=>v,
        Err(e)=>{println!("Error importing js: {}", style(e).red().bold()); "".to_owned()}
    };
    out.write(format!("<script>{}</script>", result).as_ref())?;
    Ok(())
}

pub fn import_js_web(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for import_js_web helper."))?;
    let result = match get_web_content_text(&param.value().render()){
        Ok(v)=>v,
        Err(e)=>{println!("Error importing js from web: {}", style(e).red().bold()); "".to_owned()}
    };
    out.write(format!("<script>{}</script>", result).as_ref())?;
    Ok(())
}

pub fn import_css(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for import_css helper."))?;
    let result = match get_file_content_text(&param.value().render()){
        Ok(v)=>v,
        Err(e)=>{println!("Error importing css: {}", style(e).red().bold()); "".to_owned()}
    };
    out.write(format!("<style>{}</style>", result).as_ref())?;
    Ok(())
}

pub fn import_css_web(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for import_js_web helper."))?;
    let result = match get_web_content_text(&param.value().render()){
        Ok(v)=>v,
        Err(e)=>{println!("Error importing css from web: {}", style(e).red().bold()); "".to_owned()}
    };
    out.write(format!("<style>{}</style>", result).as_ref())?;
    Ok(())
}

pub fn import_html(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for import_html helper."))?;
    let result = match get_file_content_text(&param.value().render()){
        Ok(v)=>v,
        Err(e)=>{println!("Error importing html: {}", style(e).red().bold()); "".to_owned()}
    };
    out.write(&result)?;
    Ok(())
}

pub fn import_img(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for import_img helper."))?;
    let file_path = param.value().render();
    let ext: Vec<&str> = file_path.split(".").collect();
    let ext = match ext.last(){
        Some(v)=>v.to_string(),
        None=>"".to_owned()
    };
    let result = match get_file_content_base64(&file_path){
        Ok(v)=>v,
        Err(e)=>{println!("Error importing img: {}", style(e).red().bold()); "".to_owned()}
    };
    out.write(format!("data:image/{};base64,{}", ext, result).as_ref())?;
    Ok(())
}

pub fn import_json(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for import_json helper."))?;
    let result = match get_file_content_text(&param.value().render()){
        Ok(v)=>v,
        Err(e)=>{println!("Error importing json: {}", style(e).red().bold()); "".to_owned()}
    };
    out.write(&result)?;
    Ok(())
}

pub fn import_wasm(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for import_json helper."))?;
    let result = match get_file_content_bytes(&param.value().render()){
        Ok(v)=>v,
        Err(e)=>{println!("Error importing wasm: {}", style(e).red().bold()); vec![]}
    };
    out.write(format!("new Uint8Array({:?}).buffer", result).as_ref())?;
    Ok(())
}