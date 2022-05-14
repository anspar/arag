use console::style;
use handlebars::{Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError};

macro_rules! basic_import_helper {
    ($(#[$meta:meta])* $name:ident => $fn:expr) => {
        $(#[$meta])*
        pub fn $name(
            h: &Helper,
            _: &Handlebars,
            _: &Context,
            _: &mut RenderContext,
            out: &mut dyn Output,
        ) -> Result<(), RenderError> {
            let param = h.param(0).ok_or(RenderError::new(format!(
                "Param 0 is required for {}",
                stringify!($name)
            )))?;
            match out.write(&$fn(param.value().render())) {
                Ok(_) => Ok(()),
                Err(e) => (Err(RenderError::new(format!("Error rendering: {}", e)))),
            }
        }
    };

    ($(#[$meta:meta])* $name:ident = $fn:expr) => {
        $(#[$meta])*
        pub fn $name(
            _: &Helper,
            _: &Handlebars,
            _: &Context,
            _: &mut RenderContext,
            out: &mut dyn Output,
        ) -> Result<(), RenderError> {
            match out.write(&$fn()) {
                Ok(_) => Ok(()),
                Err(e) => (Err(RenderError::new(format!("Error rendering: {}", e)))),
            }
        }
    };
}

macro_rules! context_import_helper {
    ($(#[$meta:meta])* $name:ident => $fn:expr) => {
        $(#[$meta])*
        pub fn $name(
            h: &Helper,
            _: &Handlebars,
            c: &Context,
            _: &mut RenderContext,
            out: &mut dyn Output,
        ) -> Result<(), RenderError> {
            let param = h.param(0).ok_or(RenderError::new(format!(
                "Param 0 is required for {}",
                stringify!($name)
            )))?;
            match out.write(&$fn(param.value().render(), c)) {
                Ok(_) => Ok(()),
                Err(e) => (Err(RenderError::new(format!("Error rendering: {}", e)))),
            }
        }
    };

    ($(#[$meta:meta])* $name:ident = $fn:expr) => {
        $(#[$meta])*
        pub fn $name(
            _: &Helper,
            _: &Handlebars,
            c: &Context,
            _: &mut RenderContext,
            out: &mut dyn Output,
        ) -> Result<(), RenderError> {
            match out.write(&$fn(&c)) {
                Ok(_) => Ok(()),
                Err(e) => (Err(RenderError::new(format!("Error rendering: {}", e)))),
            }
        }
    };
}

basic_import_helper!(
    /// Creates Uint8Array containing bytes of a file
    ///
    /// # Example
    /// ```
    /// <script>
    ///     var bytes = {{import_bytes "static/file"}}
    /// </script>
    /// ```
    import_bytes => |param: String|{
    match super::get_file_content_bytes(&param){
        Ok(v)=>format!("new Uint8Array({:?})", v),
        Err(e)=>{println!("Error importing bytes: {}", style(e).red().bold()); "[]".to_owned()}
    }
});

basic_import_helper!(
    /// Creates Uint8Array buffer containing bytes of a file
    ///
    /// # Example
    /// ```
    /// <script>
    ///     var bytes = {{import_wasm "static/file.wasm"}}
    /// </script>
    /// ```
    import_wasm => |param: String|{
    match super::get_file_content_bytes(&param){
        Ok(v)=>format!("new Uint8Array({:?}).buffer", v),
        Err(e)=>{println!("Error importing wasm: {}", style(e).red().bold()); "[]".to_owned()}
    }
});

basic_import_helper!(
    /// Loads the .json file
    ///
    /// # Example
    /// ```
    /// <script>
    ///     var data = {{import_json "static/file.json"}}
    /// </script>
    /// ```
    import_json => |param: String|{
    match super::get_file_content_text(&param){
        Ok(v)=>v,
        Err(e)=>{println!("Error importing json: {}", style(e).red().bold()); "".to_owned()}
    }
});

basic_import_helper!(
    /// Loads the content of a file
    ///
    /// # Example
    /// ```
    /// <script>
    ///    {{import_content "static/file.js"}}
    /// </script>
    /// ```
    import_content => |param: String|{
    match super::get_file_content_text(&param){
        Ok(v)=>v,
        Err(e)=>{println!("Error importing file content: {}", style(e).red().bold()); "".to_owned()}
    }
});

basic_import_helper!(
    /// Creates base64 url of a audio file
    ///
    /// # Example
    /// ```
    /// <audio src="{{import_audio "static/file.mp3"}}"></audio>
    /// ```
    import_audio => |param: String|{
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

basic_import_helper!(
    /// Creates base64 url of a video file
    ///
    /// # Example
    /// ```
    /// <video src="{{import_video "static/file.mp4"}}"></video>
    /// ```
    import_video => |param: String|{
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

basic_import_helper!(
    /// Creates base64 url of a audio file
    ///
    /// # Example
    /// ```
    /// <img src="{{import_img "static/file.png"}}"></img>
    /// ```
    import_img => |param: String|{
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

basic_import_helper!(
    /// Creates 'script' tag and puts minified javascript in it
    ///
    /// # Example
    /// ```
    /// ...
    ///     <div>...</div>
    ///     {{import_js "static/js/index.js"}}
    /// </body>
    /// ```
    import_js => |param: String|{
    match super::get_file_content_text(&param){
        Ok(v)=>format!("<script>{}</script>", minifier::js::minify(&v)),
        Err(e)=>{println!("Error importing js: {}", style(e).red().bold()); "".to_owned()}
    }
});

basic_import_helper!(
    /// Creates 'style' tag and puts minified css in it
    ///
    /// # Example
    /// ```
    /// ...
    ///     {{import_css "static/css/index.css"}}
    /// </head>
    /// ```
    import_css => |param: String|{
    match super::get_file_content_text(&param){
        Ok(v)=>format!("<style>{}</style>", minifier::css::minify(&v).unwrap()),
        Err(e)=>{println!("Error importing css: {}", style(e).red().bold()); "".to_owned()}
    }
});

basic_import_helper!(
    /// Loads an arag component from web
    ///
    /// # Example
    /// ```
    /// ...
    ///     {{web_component "https://example.com/component"}}
    ///     <div>...</div>
    /// </body>
    /// ```
    /// * the url should also be declared in arag.yml file under dependencies
    /// ```
    /// dependencies:
    ///     - https://example.com/component
    /// ```
    web_component => |param: String|{
    match super::get_file_content_text(&param){
        Ok(v)=>v,
        Err(e)=>{println!("Error importing web component: {}", style(e).red().bold()); "".to_owned()}
    }
});

basic_import_helper!(
    /// Creates 'script' tag and puts minified javascript downloaded from web
    ///
    /// # Example
    /// ```
    /// ...
    ///     <div>...</div>
    ///     {{import_js_web "https://example.com/index.js"}}
    /// </body>
    /// ```
    /// * the url should also be declared in arag.yml file under dependencies
    /// ```
    /// dependencies:
    ///     - https://example.com/index.js
    /// ```
    import_js_web => |param: String|{
    match super::get_cached_content_text(&param){
        Ok(v)=>format!("<script>{}</script>", minifier::js::minify(&v)),
        Err(e)=>{println!("Error importing js from web: {}", style(e).red().bold()); "".to_owned()}
    }
});

basic_import_helper!(
    /// Creates 'style' tag and puts minified css downloaded from web
    ///
    /// # Example
    /// ```
    /// ...
    ///     <div>...</div>
    ///     {{import_css_web "https://example.com/index.css"}}
    /// </head>
    /// ```
    /// * the url should also be declared in arag.yml file under dependencies
    /// ```
    /// dependencies:
    ///     - https://example.com/index.css
    /// ```
    import_css_web => |param: String|{
    match super::get_cached_content_text(&param){
        Ok(v)=>format!("<style>{}</style>", v),
        Err(e)=>{println!("Error importing css from web: {}", style(e).red().bold()); "".to_owned()}
    }
});

basic_import_helper!(
    /// Creates Uint8Array containing bytes of a downloaded file
    ///
    /// # Example
    /// ```
    /// <script>
    ///     var bytes = {{import_bytes_web "https://example.com/file"}}
    /// </script>
    /// ```
    /// * the url should also be declared in arag.yml file under dependencies
    /// ```
    /// dependencies:
    ///     - https://example.com/file
    /// ```
    import_bytes_web => |param: String|{
    match super::get_cached_content_bytes(&param){
        Ok(v)=>format!("new Uint8Array({:?})", v),
        Err(e)=>{println!("Error importing bytes: {}", style(e).red().bold()); "[]".to_owned()}
    }
});

context_import_helper!(
    /// Will prepend an available ipfs gateway to the CID and add it to the tag as 'src'
    ///
    /// # Example
    /// ```
    ///     ...
    ///
    ///     <img ipfs="bafkreihu2gmfalwwrgv6mgizjbncsy2b5surjjefbpwtkravfyvq5niqcm">
    ///
    ///     ...
    ///
    ///     {{inject_gateway}}
    /// </body>
    /// ```
    inject_gateway = |c: &Context| {
        let ipfs = c.data().get("ipfs_gateway").unwrap();
        minifier::js::minify(&format!(
            "<script>
            let IPFS_GATEWAY = {};
            document.addEventListener('DOMContentLoaded', function() {{
                setInterval(()=>{{
                    try{{
                        if(IPFS_GATEWAY_INJECTED){{ IPFS_GATEWAY=IPFS_GATEWAY_INJECTED }}
                    }}catch(e){{}}
                    document.querySelectorAll('[ipfs]').forEach((e, i)=>{{
                        e.setAttribute('src', `${{IPFS_GATEWAY}}${{IPFS_GATEWAY.endsWith('/')?'':'/'}}${{e.getAttribute('ipfs')}}`);
                        e.removeAttribute('ipfs');
                    }})
                }}, 1000)
            }});
            </script>",
            ipfs
        ))
    }
);

basic_import_helper!(
    /// Reloads the page if files are updated
    /// * Should only be used during development
    ///
    /// # Example
    /// ```
    /// ...
    ///     {{live_update}}
    /// </body>
    /// ```
    live_update = || {
        "<script>
    setInterval(()=>{
        fetch('/update').then(res=>{
            if(res.status!==200) console.error('failed fetching state');
            res.json().then(data=>{
                if(data.update===true) window.location.reload();
            })
        })
        }, 1000
    );
    </script>
    "
    }
);
