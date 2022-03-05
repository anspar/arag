use console::style;
use handlebars::{Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError};

macro_rules! basic_import_helper {
    ($name:ident => $fn:expr) => {
        pub fn $name(
            h: &Helper,
            _: &Handlebars,
            _: &Context,
            _: &mut RenderContext,
            out: &mut dyn Output,
        ) -> Result<(), RenderError> {
            let param = h.param(0)
                        .ok_or(RenderError::new(format!("Param 0 is required for {}", stringify!($name))))?;
            match out.write(
                &$fn(param.value().render())
            ){
                Ok(_)=>Ok(()),
                Err(e)=>(Err(RenderError::new(format!("Error rendering: {}", e))))
            }
        }
    };
}

macro_rules! context_import_helper {
    ($name:ident => $fn:expr) => {
        pub fn $name(
            h: &Helper,
            _: &Handlebars,
            c: &Context,
            _: &mut RenderContext,
            out: &mut dyn Output,
        ) -> Result<(), RenderError> {
            let param = h.param(0)
                        .ok_or(RenderError::new(format!("Param 0 is required for {}", stringify!($name))))?;
            match out.write(
                &$fn(param.value().render(), c)
            ){
                Ok(_)=>Ok(()),
                Err(e)=>(Err(RenderError::new(format!("Error rendering: {}", e))))
            }
        }
    };

    ($name:ident = $fn:expr) => {
        pub fn $name(
            _: &Helper,
            _: &Handlebars,
            c: &Context,
            _: &mut RenderContext,
            out: &mut dyn Output,
        ) -> Result<(), RenderError> {
            match out.write(
                &$fn(&c)
            ){
                Ok(_)=>Ok(()),
                Err(e)=>(Err(RenderError::new(format!("Error rendering: {}", e))))
            }
        }
    };
}


basic_import_helper!(import_bytes => |param: String|{
    match super::get_file_content_bytes(&param){
        Ok(v)=>format!("new Uint8Array({:?})", v),
        Err(e)=>{println!("Error importing bytes: {}", style(e).red().bold()); "[]".to_owned()}
    }
});

basic_import_helper!(import_wasm => |param: String|{
    match super::get_file_content_bytes(&param){
        Ok(v)=>format!("new Uint8Array({:?}).buffer", v),
        Err(e)=>{println!("Error importing wasm: {}", style(e).red().bold()); "[]".to_owned()}
    }
});

basic_import_helper!(import_json => |param: String|{
    match super::get_file_content_text(&param){
        Ok(v)=>v,
        Err(e)=>{println!("Error importing json: {}", style(e).red().bold()); "".to_owned()}
    }
});

basic_import_helper!(import_html => |param: String|{
    match super::get_file_content_text(&param){
        Ok(v)=>v,
        Err(e)=>{println!("Error importing html: {}", style(e).red().bold()); "".to_owned()}
    }
});

basic_import_helper!(import_audio => |param: String|{
    let ext: Vec<&str> = param.split(".").collect();
    let ext = match ext.last(){
        Some(v)=>v.to_string(),
        None=>"".to_owned()
    };
    match super::get_file_content_base64(&param){
        Ok(v)=>format!("data:audio/{};base64,{}", ext.to_lowercase(), v),
        Err(e)=>{println!("Error importing audio: {}", style(e).red().bold()); "".to_owned()}
    }
});

basic_import_helper!(import_video => |param: String|{
    let ext: Vec<&str> = param.split(".").collect();
    let ext = match ext.last(){
        Some(v)=>v.to_string(),
        None=>"".to_owned()
    };
    match super::get_file_content_base64(&param){
        Ok(v)=>format!("data:video/{};base64,{}", ext.to_lowercase(), v),
        Err(e)=>{println!("Error importing video: {}", style(e).red().bold()); "".to_owned()}
    }
});

basic_import_helper!(import_img => |param: String|{
    let ext: Vec<&str> = param.split(".").collect();
    let ext = match ext.last(){
        Some(v)=>v.to_string(),
        None=>"".to_owned()
    };
    match super::get_file_content_base64(&param){
        Ok(v)=>format!("data:image/{};base64,{}", ext.to_lowercase(), v),
        Err(e)=>{println!("Error importing img: {}", style(e).red().bold()); "".to_owned()}
    }
});

basic_import_helper!(import_js => |param: String|{
    match super::get_file_content_text(&param){
        Ok(v)=>format!("<script>{}</script>", v),
        Err(e)=>{println!("Error importing js: {}", style(e).red().bold()); "".to_owned()}
    }
});

basic_import_helper!(import_css => |param: String|{
    match super::get_file_content_text(&param){
        Ok(v)=>format!("<style>{}</style>", v),
        Err(e)=>{println!("Error importing css: {}", style(e).red().bold()); "".to_owned()}
    }
});

basic_import_helper!(import_js_web => |param: String|{
    match super::get_web_content_text(&param){
        Ok(v)=>format!("<script>{}</script>", v),
        Err(e)=>{println!("Error importing js from web: {}", style(e).red().bold()); "".to_owned()}
    }
});

basic_import_helper!(import_css_web => |param: String|{
    match super::get_web_content_text(&param){
        Ok(v)=>format!("<style>{}</style>", v),
        Err(e)=>{println!("Error importing css from web: {}", style(e).red().bold()); "".to_owned()}
    }
});


context_import_helper!(import_js_ipfs => |param: String, c: &Context|{
    match super::get_web_content_text(&format!("{}/{}", c.data().to_string(), &param)){
        Ok(v)=>format!("<script>{}</script>", v),
        Err(e)=>{println!("Error importing js from web: {}", style(e).red().bold()); "".to_owned()}
    }
});

context_import_helper!(import_css_ipfs => |param: String, c: &Context|{
    match super::get_web_content_text(&format!("{}/{}", c.data().to_string(), &param)){
        Ok(v)=>format!("<style>{}</style>", v),
        Err(e)=>{println!("Error importing css from web: {}", style(e).red().bold()); "".to_owned()}
    }
});

context_import_helper!(import_raw_ipfs => |param: String, c: &Context|{
    match super::get_web_content_text(&format!("{}/{}", c.data().to_string(), &param)){
        Ok(v)=>v,
        Err(e)=>{println!("Error importing css from web: {}", style(e).red().bold()); "".to_owned()}
    }
});

context_import_helper!(import_bytes_ipfs => |param: String, c: &Context|{
    match super::get_web_content_bytes(&format!("{}/{}", c.data().to_string(), &param)){
        Ok(v)=>format!("new Uint8Array({:?})", v),
        Err(e)=>{println!("Error importing bytes: {}", style(e).red().bold()); "[]".to_owned()}
    }
});

context_import_helper!(inject_gateway = |c: &Context|{
    format!("let IPFS_GATEWAY = {}
    document.addEventListener('DOMContentLoaded', function() {{
        setInterval(()=>{{
            try{{
                if(IPFS_GATEWAY_INJECTED){{ IPFS_GATEWAY=IPFS_GATEWAY_INJECTED }}
            }}catch(e){{}}
            document.querySelectorAll('[ipfs]').forEach((e, i)=>{{
                e.setAttribute('src', `${{IPFS_GATEWAY}}/${{e.getAttribute('ipfs')}}`);
                e.removeAttribute('ipfs');
            }})
        }}, 1000)
    }});", c.data().to_string())
});