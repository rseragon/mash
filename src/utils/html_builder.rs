use std::collections::HashMap;

use crate::server::response::ResponseCode;

/// Returns and error page
pub fn error_page_builder(code: &ResponseCode, msg: &String) -> String {

    format!("
<!DOCTYPE html>
<html>
<head>
<title>{}</title>
</head>

<body>
Error occured :( 
<br/>
Status Code: {}
<hr/>

Reason: {}
</body>

</html>", code.to_string(), code.to_string(), msg)

}

/// builds dir listing html page
pub fn dir_list_html(path_str: &String) -> String {

    // To add to the html
    let mut dirs_str = String::new();

    // make a list of dirs which can be used by the 
    // builder to make the build
    // Will look like
    // DIR TO SHOW   -> Points to certain link
    // src/          -> /asdf/bda/src/
    for p in std::fs::read_dir(&path_str).unwrap() {
        // remove(0) is to remove (./ -> /) the `.` which represents current dir
        // which is the relatvie path to the server not the browser

        
        // let dir_show = p.unwrap().path().display().to_string();
        let entity_path = match p {
            Ok(d) => d,
            Err(_) => continue
        };

        let dir_show = entity_path.path().display().to_string();

        let mut dir_href = dir_show.clone();
        dir_href.remove(0);  
        let dir_show = dir_show.split("/").last().unwrap(); // gets the last part of path 
                                                            // Eg: (/a/b/c/d -> d)

        match entity_path.file_type() {
            Ok(file) => {
                if file.is_dir() {
                    dirs_str.push_str(&format!("<li><a href='{}'>{}/</a></li>\n", &dir_href, &dir_show));
                }
                else {
                    dirs_str.push_str(&format!("<li><a href='{}'>{}</a></li>\n", &dir_href, &dir_show));
                }
            },
            Err(_) => {
                dirs_str.push_str(&format!("<li><a href='{}'>{}</a></li>\n", &dir_href, &dir_show));
            }
        }

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
</body></html>\n", path_str, path_str, dirs_str)

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
