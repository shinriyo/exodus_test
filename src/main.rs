use std::fs;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::collections::HashMap;

fn main() {
    // exodus g item name:string price:integer description:text
    let suffix;
    let x = 3;
    match (x % 10, x % 100) {
        (1, 1) | (1, 21...91) => {
            suffix = "st";
        }
        (2, 2) | (2, 22...92) => {
            suffix = "nd";
        }
        (3, 3) | (3, 23...93) => {
            suffix = "rd";
        }
        _                     => {
            suffix = "th";
        }
    }
    println!("{}{}", x, suffix);

    // 後で変える名前
    let name = "hoge";
    let capitalized_name = format!("{}{}", &name[0..1].to_uppercase(), &name[1..name.len()]);
    let args: Vec<_> = env::args().collect();
    if args.len() < 0 {
        println!("Error.");
        return;
    }
    for argument in env::args() {
        println!("{}", argument);
    }

    let x = "release_year:integer".to_string();
    let d: Vec<_> = x.split(':').collect();
    if d.len() != 2 {
        println!("format");
    }
    println!("{}", d[0]);
    println!("{}", d[1]);

    // ハッシュ
    let mut map = HashMap::new();
    map.insert("title", "string");
    map.insert("release_year", "integer");
    map.insert("genre", "string");
    map.insert("director", "string");

    let mut as_str: Vec<String> = Vec::new();

    // CREATE TABLE
    let mut create_table_str: Vec<String> = Vec::new();
    // $1, $2, $3, $4
    let mut create_table_val_str: Vec<String> = Vec::new();

    // SELECT
    let mut select_table_str: Vec<String> = Vec::new();

    let mut idx = 0;

    // key: column name
    for (key, val) in &map {
        let capitalized_val = format!("{}{}", &name[0..1].to_uppercase(), &name[1..name.len()]);
        let raw = format!(r#"<div class="form-group">
<div class="form-group">
    <label for="{1}" class="col-sm-2 control-label">{2}</label>
    <div class="col-sm-10">
        <input type="text" ng-model="{0}.{1}" class="form-control" id="{1}" placeholder="{0}'s {2}"/>
    </div>
</div>"#, name, key, capitalized_val);
        as_str.push(raw);

        let mut comma = ", ";
        if (map.len() - 1) == idx {
            comma = "";
        }

        // CREATE TABLE
        // TODO: あとは SMALLINT, VARCHAR変換
        let raw = format!("{0} {1} (50) NOT NULL{2}",
            key, "integer", comma);
        create_table_str.push(raw);

        let raw = format!("${0}{1}", idx+1, comma);
        create_table_val_str.push(raw);

        // SELECT
        let raw = format!("{0}{1}", key, comma);
        select_table_str.push(raw);

        idx += 1;
    }

    println!("{}", as_str.iter().cloned().collect::<String>());

    // CREATE TABLE
    println!("CREATE TABLE {0} (id SERIAL PRIMARY KEY, {1})",
        name, create_table_str.iter().cloned().collect::<String>());

    println!("{}", create_table_val_str.iter().cloned().collect::<String>());

    // SELECT
    println!("SELECT {0} FROM {1}", select_table_str.iter().cloned().collect::<String>(), name);

    // INSERT
    println!("INSERT INTO {1} ({0}) VALUES ({2})", select_table_str.iter().cloned().collect::<String>(),
        name,
        create_table_val_str.iter().cloned().collect::<String>());

    // 開始
    // フォルダ生成
    let partials_path = "assets/partials";
    match fs::create_dir_all(partials_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }

    /*
        HTML系のファイルたち
    */
    // ファイル
    // partials/_form.html
    let mut form_f = File::create(format!("{}/{}_form.html", partials_path, name)).unwrap();
    let form_raw = format!(r#"<div class="form-group">
    <label for="title" class="col-sm-2 control-label">Title</label>
    <div class="col-sm-10">
        <input type="text" ng-model="{0}.title" class="form-control" id="title" placeholder="{1} Title Here"/>
    </div>
</div>
<div class="form-group">
    <label for="year" class="col-sm-2 control-label">Release Year</label>
    <div class="col-sm-10">
        <input type="text" ng-model="{0}.releaseYear" class="form-control" id="year" placeholder="When was the {0} released?"/>
    </div>
</div>

<div class="form-group">
    <label for="director" class="col-sm-2 control-label">Director</label>
    <div class="col-sm-10">
        <input type="text" ng-model="{0}.director" class="form-control" id="director" placeholder="Who directed the {0}?"/>
    </div>
</div>

<div class="form-group">
    <label for="plot" class="col-sm-2 control-label">{1} Genre</label>
    <div class="col-sm-10">
        <input type="text" ng-model="{0}.genre" class="form-control" id="plot" placeholder="{1} genre here"/>
    </div>
</div>

<div class="form-group">
    <div class="col-sm-offset-2 col-sm-10">
        <input type="submit" class="btn btn-primary" value="Save"/>
    </div>
</div>"#, name, capitalized_name);
    form_f.write_all(form_raw.as_bytes());

    // partials/hoge-add.html
    let mut add_f = File::create(format!("{}/{}-add.html", partials_path, name)).unwrap();
    let add_raw = format!(r#"<form class="form-horizontal" role="form" ng-submit="add{1}()">
    <div ng-include="'{}/partials/_form.html'"></div>
</form>"#, name, capitalized_name);
    add_f.write_all(add_raw.as_bytes());

    // partials/hoge-edit.html
    let mut edit_f = File::create(format!("{}/{}-edit.html", partials_path, name)).unwrap();
    let add_raw = format!(r#"<form class="form-horizontal" role="form" ng-submit="update{1}()">
    <div ng-include="'{0}/partials/_form.html'"></div>
</form>"#, name, capitalized_name);
    edit_f.write_all(add_raw.as_bytes());

    // 複数形
    // まだ仮実装
    let mut index_f = File::create(format!("{}/{}s.html", partials_path, name)).unwrap();
    let index_raw = format!(r#"<a ui-sref="new{1}" class="btn-primary btn-lg nodecoration">Add New {1}</a>
<table class="table {0}table">
    <tr>
        <td><h3>All {1}s</h3></td>
        <td></td>
    </tr>
    <tr ng-repeat="{0} in {0}s">
        <td>{{{0}.title}}</td>
        <td>
            <a class="btn btn-primary" ui-sref="view{1}({{id:{0}._id}})">View</a>
            <a class="btn btn-danger"  ng-click="delete{1}({0})">Delete</a>
        </td>
    </tr>
</table>
"#, name, capitalized_name);
    index_f.write_all(index_raw.as_bytes());

    /*
        js系
    */
    let js_path = "assets/js";
    match fs::create_dir_all(js_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }

    // ファイル
    // app.js
    let mut js_app_f = File::create(format!("{}/app.js", js_path)).unwrap();
    let js_app_raw = format!(r#"angular.module('{0}App',['ui.router','ngResource','{0}App.controllers','{0}App.services']);
angular.module('{0}App').config(function($stateProvider,$httpProvider){{
    $stateProvider.state('{0}s',{{
        url:'/{0}s',
        templateUrl:'{0}/partials/{0}s.html',
        controller:'{1}ListController'
    }}).state('view{1}',{{
       url:'/{0}s/:id/view',
       templateUrl:'{0}/partials/{0}-view.html',
       controller:'{1}ViewController'
    }}).state('new{1}',{{
        url:'/{0}s/new',
        templateUrl:'{0}/partials/{0}-add.html',
        controller:'{1}CreateController'
    }}).state('edit{1}',{{
        url:'/{0}s/:id/edit',
        templateUrl:'{0}/partials/{0}-edit.html',
        controller:'{1}EditController'
    }});
}}).run(function($state){{
   $state.go('{0}s');
}});
"#, name, capitalized_name);
    js_app_f.write_all(form_raw.as_bytes());

    let mut js_controllers_f = File::create(format!("{}/controller.js", js_path)).unwrap();
    let js_controllers_raw = format!(r#"angular.module('{0}App.controllers',[]).controller('{1}ListController',function($scope,$state,popupService,$window,{1}){{

    $scope.{0}s={1}.query();

    $scope.delete{1}=function({0}){{
        if(popupService.showPopup('Really delete this?')){{
            {0}.$delete(function(){{
                $window.location.href='';
            }});
        }}
    }}

}}).controller('{1}ViewController',function($scope,$stateParams,{1}){{

    $scope.{0}={1}.get({{id:$stateParams.id}});

}}).controller('{1}CreateController',function($scope,$state,$stateParams,{1}){{

    $scope.{0}=new {1}();

    $scope.add{1}=function(){{
        $scope.{0}.$save(function(){{
            $state.go('{0}s');
        }});
    }}

}}).controller('{1}EditController',function($scope,$state,$stateParams,{1}){{

    $scope.update{1}=function(){{
        $scope.{0}.$update(function(){{
            $state.go('{0}s');
        }});
    }};

    $scope.load{1}=function(){{
        $scope.{0}={1}.get({{id:$stateParams.id}});
    }};

    $scope.load{1}();
}});
"#, name, capitalized_name);
    js_controllers_f.write_all(js_controllers_raw.as_bytes());

//    directives.js
//    fixlters.js

    let mut js_services_f = File::create(format!("{}/services.js", js_path)).unwrap();
    let js_services_raw = format!(r#"angular.module('{0}App.services',[]).factory('{1}',function($resource){{
    return $resource('http://localhost:6767/api/{0}s/:id',{{id:'@_id'}},{{
        update: {{
            method: 'PUT'
        }}
    }});
}}).service('popupService',function($window){{
    this.showPopup=function(message){{
        return $window.confirm(message);
    }}
}});"#, name, capitalized_name);
    js_services_f.write_all(js_services_raw.as_bytes());

    // movie/views/index.tpl
    // フォルダ生成
    let index_tpl_path = format!("{}/views", name);
    match fs::create_dir_all(&index_tpl_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }

    let mut index_t = File::create(format!("{}/index.tpl", &index_tpl_path)).unwrap();
    let index_raw = format!(r#"<!DOCTYPE html>
<html data-ng-app="{0}App">
<head lang="en">
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <base href="/"/>
    <title>The {1} App</title>
    <link rel="stylesheet" type="text/css" href="css/bootstrap.min.css"/>
    <link rel="stylesheet" type="text/css" href="css/app.css"/>
</head>
<body>
    <nav class="navbar navbar-default" role="navigation">
        <div class="container-fluid">
            <div class="navbar-header">
                <a class="navbar-brand" ui-sref="{0}s">The {1} App</a>
            </div>
            <div class="collapse navbar-collapse">
                <ul class="nav navbar-nav">
                    <li class="active"><a ui-sref="{0}s">Home</a></li>
                </ul>
            </div>
        </div>
    </nav>
    <div class="container">
        <div class="row top-buffer">
            <div class="col-xs-8 col-xs-offset-2">
                <div ui-view></div>
            </div>
        </div>
    </div>
    <script type="text/javascript" src="lib/angular.min.js"></script>
    <script type="text/javascript" src="{0}/js/app.js"></script>
    <script type="text/javascript" src="{0}/js/controllers.js"></script>
    <script type="text/javascript" src="{0}/js/services.js"></script>
    <script type="text/javascript" src="{0}/js/directives.js"></script>
    <script type="text/javascript" src="{0}/js/filters.js"></script>
    <script type="text/javascript" src="lib/angular-ui-router.min.js"></script>
    <script type="text/javascript" src="lib/angular-resource.min.js"></script>
</body>
</html>
"#, name, capitalized_name);
    index_t.write_all(index_raw.as_bytes());

    /*
    Rustコード
    */
    let index_tpl_path = "src";
    match fs::create_dir_all(&index_tpl_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }

    let rust_raw = format!(r#"extern crate postgres;
extern crate openssl;
extern crate hyper;
use nickel::{{Router, HttpRouter, MediaType, JsonBody}};
use nickel::status::StatusCode;
use postgres::{{Connection}};
use std::sync::{{Arc, Mutex}};

use std::vec::Vec;

extern crate rustc_serialize;
use rustc_serialize::{{json}};

#[derive(RustcDecodable, RustcEncodable)]
struct {1} {{
    _id: Option<i32>,
    title: String,
    director: String,
    releaseYear: i16,
    genre: String,
}}

pub fn url(shared_connection: Arc<Mutex<Connection>>, router: &mut Router) {{
    let conn = shared_connection.clone();
    router.get("/setup/{0}", middleware! {{ |_, response|

    return match conn.lock().unwrap().execute("CREATE TABLE {1} (
            id          SERIAL PRIMARY KEY,
            title       VARCHAR (50) NOT NULL,
            releaseYear SMALLINT NOT NULL,
            director    VARCHAR (18) NOT NULL,
            genre       VARCHAR (50) NOT NULL
        )",
    &[]) {{
            Ok(_) => return response.send("{1} table was created."),
            Err(err) => return response.send(format!("Error running query: {{:?}}", err))
        }};
    }});

    router.get("/", middleware! {{ |_, mut response|
        response.set(MediaType::Html);
        return response.send_file("app/{0}/views/index.tpl")
    }});

    // select all
    let conn = shared_connection.clone();
    router.get("/api/{0}s", middleware! {{ |_, mut response|
        let conn = conn.lock().unwrap();
        let {0}s = conn.query("SELECT id, title, releaseYear, director, genre from {0}", &[]).unwrap();
        let mut v: Vec<{1}> = vec![];

        for row in &{0}s {{
            let {0} = {1} {{
                _id: row.get(0),
                title: row.get(1),
                releaseYear: row.get(2),
                director: row.get(3),
                genre: row.get(4),
            }};

            v.push({0});
        }}

        let json_obj = json::encode(&v).unwrap();
        response.set(MediaType::Json);
        response.set(StatusCode::Ok);
        return response.send(json_obj);
    }});

    // insert
    let conn = shared_connection.clone();
    router.post("/api/{0}s", middleware! {{ |request, mut response|
        let conn = conn.lock().unwrap();
        let stmt = match conn.prepare("INSERT INTO {0} (title, releaseYear, director, genre)
            VALUES ($1, $2, $3, $4)") {{
            Ok(stmt) => stmt,
            Err(e) => {{
                return response.send(format!("Preparing query failed: {{}}", e));
            }}
        }};

        let {0} = request.json_as::<{1}>().unwrap();
        match stmt.execute(&[
            &{0}.title.to_string(),
            &{0}.releaseYear,
            &{0}.director.to_string(),
            &{0}.genre.to_string()
        ]) {{
            Ok(_) => {{
                println!("Inserting {0} was Success.");
                response.set(StatusCode::Ok);
            }},
            Err(e) => println!("Inserting {0} failed. => {{:?}}", e),
        }};

        return response.send("");
    }});

    // select one
    let conn = shared_connection.clone();
    router.get("/api/{0}s/:id", middleware! {{ |request, mut response|
        let conn = conn.lock().unwrap();
        let {0} = conn.query(
            "select id, title, releaseYear, director, genre from {0} where id = $1",
            &[&request.param("id").unwrap().parse::<i32>().unwrap()]
        ).unwrap();

        for row in &{0} {{
            let {0} = {1} {{
                _id: row.get(0),
                title: row.get(1),
                releaseYear: row.get(2),
                director: row.get(3),
                genre: row.get(4),
            }};

            let json_obj = json::encode(&{0}).unwrap();
            // MediaType can be any valid type for reference see
            response.set(MediaType::Json);
            response.set(StatusCode::Ok);
            return response.send(json_obj);
        }}
    }});

    // update
    let conn = shared_connection.clone();
    router.put("/api/{0}s/:id", middleware! {{ |request, mut response|
        let conn = conn.lock().unwrap();
        let stmt = match conn.prepare("UPDATE {0} SET title=$1, releaseYear=$2,
            director=$3, genre=$4
            WHERE id = $5") {{
            Ok(stmt) => stmt,
            Err(e) => {{
                return response.send(format!("Preparing query failed: {{}}", e));
            }}
        }};

        // JSON to object
        let {0} = request.json_as::<{1}>().unwrap();
        match stmt.execute(&[
            &{0}.title.to_string(),
            &{0}.releaseYear,
            &{0}.director.to_string(),
            &{0}.genre.to_string(),
            &{0}._id
        ]) {{
            Ok(_) => {{
                println!("Updating {0} was Success.");
                response.set(StatusCode::Ok);
            }},
            Err(e) => println!("Updating {0} failed. => {{:?}}", e),
        }};

        return response.send("");
    }});

    // delete
    let conn = shared_connection.clone();
    router.delete("/api/{0}s/:id", middleware! {{ |request, mut response|
        let conn = conn.lock().unwrap();
        let stmt = match conn.prepare("delete from {0} where id = $1") {{
            Ok(stmt) => stmt,
            Err(e) => {{
                return response.send(format!("Preparing query failed: {{}}", e));
            }}
        }};

        match stmt.execute(&[
            &request.param("id").unwrap().parse::<i32>().unwrap()
        ]) {{
            Ok(_) => {{
                println!("Deleting {0} was Success.");
                response.set(StatusCode::Ok);
            }},
            Err(e) => println!("Deleting {0} failed. => {{:?}}", e),
        }};

        return response.send("");
    }});
}}
"#, name, capitalized_name);
    let mut rust_f = File::create(format!("{}/mod.rs", &index_tpl_path)).unwrap();
    rust_f.write_all(rust_raw .as_bytes());
}
