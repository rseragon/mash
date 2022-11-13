use std::process::exit;
use std::fmt::Write as _;
use typed_html::{html, dom::DOMTree, text, htmlescape};

use crate::{server::response::ResponseCode, cliparser::Config};

/// Returns and error page
pub fn error_page_builder(code: &ResponseCode, msg: &String) -> String {

    let err_page: DOMTree<String> = html!(

        <html>
            <head>
                <title>{text!("{}", code.to_string())}</title>
            </head>

            <body>
                <h1>"Error occured :("</h1>
                <hr/>
                <h4>{text!("Status code: {}", code.to_string())}</h4>
                <p>{text!("{}", msg)}</p>
            </body>
        </html>
    );

    err_page.to_string()

}

/// builds dir listing html page
// TODO: remoe uselss stuff in this algo
pub fn dir_list_html(path_str: &String, config: &Config) -> String {

    // To add to the html
    let mut dirs_str = String::new();

    let absolute_path = match std::fs::canonicalize(&config.path) {
        Ok(p) => p.display().to_string(),
        Err(_) => {
            paris::error!("An unforseen error occured");
            exit(-1);
        },
        // Err(_) => path_str.to_string(), // TODO: This is wrong
                                        // if it failes to get absolute path
                                        // it shouldn't work
    };

    // make a list of dirs which can be used by the 
    // builder to make the build
    // Will look like
    // DIR TO SHOW   -> Points to certain link
    // src/          -> /asdf/bda/src/
    for p in std::fs::read_dir(&path_str).unwrap() {
        // remove(0) is to remove (./ -> /) the `.` which represents current dir
        // which is the relative path to the server not the browser

        
        // let dir_show = p.unwrap().path().display().to_string();
        let entity_path = match p {
            Ok(d) => d,
            Err(_) => continue
        };

        // Build relative path for href
        let absolute_entity_pathbuf = match std::fs::canonicalize(entity_path.path()) {
            Ok(pbuf) => pbuf,
            Err(_) => continue
        };

        let relative_path = match pathdiff::diff_paths(absolute_entity_pathbuf.as_path(), std::path::Path::new(&absolute_path)) {
            Some(p) => p,
            None => continue
        };

        let dir_href = relative_path.display().to_string();

        let dir_show = entity_path.path().display().to_string();
        let dir_show = dir_show.split('/').last().unwrap(); // gets the last part of path 
                                                            // Eg: (/a/b/c/d -> d)

        // TODO: remove expect
        match entity_path.file_type() {
            Ok(file) => {
                if file.is_dir() {
                    // dirs_str.push_str(&format!("<li><a href='/{}'>{}/</a></li>\n", &dir_href, &dir_show)); // Add an extra / at the end to show it's a directory
                    writeln!(dirs_str, "<li><a href='/{}'>{}/</a></li>", &dir_href, &dir_show).expect("Wut?");
                }
                else {
                    // dirs_str.push_str(&format!("<li><a href='/{}'>{}</a></li>\n", &dir_href, &dir_show));
                    writeln!(dirs_str, "<li><a href='/{}'>{}</a></li>", &dir_href, &dir_show).expect("Wut?");
                }
            },
            Err(_) => {
                // dirs_str.push_str(&format!("<li><a href='/{}'>{}</a></li>\n", &dir_href, &dir_show));
                writeln!(dirs_str, "<li><a href='/{}'>{}</a></li>", &dir_href, &dir_show).expect("Wut?");
            }
        }

    }

    let root_dir_path;


    // Since path_str comes with an extra "/" at the end
    // TODO: This is a bad algo
    if path_str == &format!("{}/", config.path) {
        let a: Vec<&str> = path_str.split('/').collect();
        if a.is_empty() {
            root_dir_path = "".to_string();
        }
        else {
            root_dir_path = a[a.len()-2].to_string(); // ?!?
        }
    }
    else {
        root_dir_path = match pathdiff::diff_paths(std::path::Path::new(&path_str), std::path::Path::new(&absolute_path)) {
            Some(p) => p.display().to_string(),
            None => path_str.to_string(),
        };
    }

    format!("
<!DOCTYPE html>\n\
<html>\n\
<head>\n\
<meta http-equiv='Content-Type' content='text/html; charset=utf-8'>\n\
<title> Directory listing for {} </title>\n\
<h1> Directory listing for {} </h1>\n\
<hr>\n\
<ul>\n\
{}\n\
</ul>\n\
<hr>\n\
</body></html>\n", root_dir_path, root_dir_path, dirs_str)

}

/// Any text can be shown using this
// TODO: mmd not working
pub fn text_page(path: &String) -> String {

    let mmd = r#";function mmd(s){var h='';function E(s){return new Option(s).innerHTML}function I(s){return E(s).replace(/!\[([^\]]*)]\(([^(]+)\)/g,'<img alt="$1" src="$2">').replace(/\[([^\]]+)]\(([^(]+)\)/g,'$1'.link('$2')).replace(/`([^`]+)`/g,'<code>$1</code>').replace(/\*\*([^*]+)\*\*/g,'<strong>$1</strong>').replace(/\*([^*]+)\*/g,'<em>$1</em>')}s.replace(/^\s+|\r|\s+$/g,'').replace(/\t/g,'    ').split(/\n\n+/).forEach(function(b,f,R){R={'*':[/\n\* /,'<ul><li>','</li></ul>'],1:[/\n[1-9]\d*\.? /,'<ol><li>','</li></ol>'],' ':[/\n    /,'<pre><code>','</code></pre>','\n'],'>':[/\n> /,'<blockquote>','</blockquote>','\n']}[f=b[0]];h+=R?R[1]+('\n'+b).split(R[0]).slice(1).map(R[3]?E:I).join(R[3]||'</li>\n<li>')+R[2]:f=='#'?'<h'+(f=b.indexOf(' '))+'>'+I(b.slice(f+1))+'</h'+f+'>':f=='<'?b:'<p>'+I(b)+'</p>'});return h};"#;
    // let mmd = "console.log('awoo')";


    // TODO: Need to do async?
    let file_str = match std::fs::read_to_string(path) {
        Ok(fstr) => fstr,
        Err(_) => "".to_string(), // TODO: Handle this
    };

    // DOES NOT work
    // as it has to wrapped around mmd()
    // let a: DOMTree<String> = html!(
    //     <html>
    //         <head> 
    //             <title>{text!("{}", path)}</title>
    //         </head>
    //
    //         <body>
    //             {text!{"{}", file_str}}
    //             <script>
    //                 {text!("{}", mmd)}
    //             </script>
    //         </body>
    //
    //     </html>
    // );

    format!("
<!DOCTYPE html>\n\
<html>\n\
<head>\n\
<title> {} </title>\n\
<script>{}</script>\n\
</head>\n\
<body>\n\
<p id='stuff'> </p>
<script>\n\
var body = document.getElementById('stuff');\n\
body.innerHTML = mmd(`{}`);\n\
</script>\n\
</body>\n\
</html>", path, mmd, file_str)
}
