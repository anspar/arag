use std::{env, fs, io::Read};

use handlebars::{Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError};

fn get_file_content_text(file_path: &str)-> String{
    let r_path = env::current_dir().unwrap();
    let js = fs::read_to_string(format!("{}/{}", r_path.display(), file_path));
    js.unwrap()
}

fn get_file_content_bytes(file_path: &str)-> Vec<u8>{
    let r_path = env::current_dir().unwrap();
    let js = fs::read(format!("{}/{}", r_path.display(), file_path));
    js.unwrap()
}

fn get_web_content_text(url: &str)->String{
    let mut js: String = "".to_owned();
    let _ = reqwest::blocking::get(url).unwrap().read_to_string(&mut js).unwrap();
    js
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
    out.write(format!("<script>{}</script>", get_file_content_text(&param.value().render())).as_ref())?;
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
    out.write(format!("<script>{}</script>", get_web_content_text(&param.value().render())).as_ref())?;
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
    out.write(format!("<style>{}</style>", get_file_content_text(&param.value().render())).as_ref())?;
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
    out.write(format!("<style>{}</style>", get_web_content_text(&param.value().render())).as_ref())?;
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
    // let rendered = format!("{} pts", param.value().render());
    let r_path = env::current_dir().unwrap();
    let js = fs::read_to_string(format!("{}/{}", r_path.display(), param.value().render()));
    out.write(js.unwrap().as_ref())?;
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
    // let rendered = format!("{} pts", param.value().render());
    let r_path = env::current_dir().unwrap();
    let rel_path = param.value().render();
    let img = fs::read(format!("{}/{}", r_path.display(), rel_path));
    let b64 = base64::encode(img.unwrap());
    let ext: Vec<&str> = rel_path.split(".").collect();
    out.write(format!("data:image/{};base64,{}", ext.last().unwrap().to_string(), b64).as_ref())?;
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
    out.write(get_file_content_text(&param.value().render()).as_ref())?;
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
    out.write(format!("new Uint8Array({:?}).buffer", get_file_content_bytes(&param.value().render())).as_ref())?;
    Ok(())
}