use std::fs;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    // フォルダ生成
    let partials_path = "assets/partials";
    match fs::create_dir_all(partials_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }

    // 後で変える名前
    let name = "hoge";

    /*
        HTML系のファイルたち
    */
    // ファイル
    // partials/_form.html
    let mut form_f = File::create(format!("{}/{}_form.html", partials_path, name)).unwrap();
    let form_raw = r#"<div class="form-group">
    <label for="title" class="col-sm-2 control-label">Title</label>
    <div class="col-sm-10">
        <input type="text" ng-model="movie.title" class="form-control" id="title" placeholder="Movie Title Here"/>
    </div>
</div>
<div class="form-group">
    <label for="year" class="col-sm-2 control-label">Release Year</label>
    <div class="col-sm-10">
        <input type="text" ng-model="movie.releaseYear" class="form-control" id="year" placeholder="When was the movie released?"/>
    </div>
</div>
<div class="form-group">
    <label for="director" class="col-sm-2 control-label">Director</label>
    <div class="col-sm-10">
        <input type="text" ng-model="movie.director" class="form-control" id="director" placeholder="Who directed the movie?"/>
    </div>
</div>

<div class="form-group">
    <label for="plot" class="col-sm-2 control-label">Movie Genre</label>
    <div class="col-sm-10">
        <input type="text" ng-model="movie.genre" class="form-control" id="plot" placeholder="Movie genre here"/>
    </div>
</div>

<div class="form-group">
    <div class="col-sm-offset-2 col-sm-10">
        <input type="submit" class="btn btn-primary" value="Save"/>
    </div>
</div>"#;
    form_f.write_all(form_raw.as_bytes());

    // partials/hoge-add.html
    let mut add_f = File::create(format!("{}/{}-add.html", partials_path, name)).unwrap();
    let add_raw = format!(r#"<form class="form-horizontal" role="form" ng-submit="addMovie()">
    <div ng-include="'{}/partials/_form.html'"></div>
</form>"#, name);
    add_f.write_all(add_raw.as_bytes());

    // partials/hoge-edit.html
    let mut edit_f = File::create(format!("{}/{}-edit.html", partials_path, name)).unwrap();
    let add_raw = format!(r#"<form class="form-horizontal" role="form" ng-submit="updateMovie()">
    <div ng-include="'{}/partials/_form.html'"></div>
</form>"#, name);
    edit_f.write_all(add_raw.as_bytes());

    // 複数形
    // まだ仮実装
    let mut index_f = File::create(format!("{}/{}s.html", partials_path, name)).unwrap();
    let index_raw = format!(r#"<a ui-sref="newMovie" class="btn-primary btn-lg nodecoration">Add New {1}</a>
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
"#, name, "Movie");
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
"#, name, "Movie");
    js_app_f.write_all(form_raw.as_bytes());

    let mut js_controllers_f = File::create(format!("{}/controller.js", js_path)).unwrap();
    let js_controllers_raw = r#"angular.module('movieApp.controllers',[]).controller('MovieListController',function($scope,$state,popupService,$window,Movie){

    $scope.movies=Movie.query();

    $scope.deleteMovie=function(movie){
        if(popupService.showPopup('Really delete this?')){
            movie.$delete(function(){
                $window.location.href='';
            });
        }
    }

}).controller('MovieViewController',function($scope,$stateParams,Movie){

    $scope.movie=Movie.get({id:$stateParams.id});

}).controller('MovieCreateController',function($scope,$state,$stateParams,Movie){

    $scope.movie=new Movie();

    $scope.addMovie=function(){
        $scope.movie.$save(function(){
            $state.go('movies');
        });
    }

}).controller('MovieEditController',function($scope,$state,$stateParams,Movie){

    $scope.updateMovie=function(){
        $scope.movie.$update(function(){
            $state.go('movies');
        });
    };

    $scope.loadMovie=function(){
        $scope.movie=Movie.get({id:$stateParams.id});
    };

    $scope.loadMovie();
});
"#;
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
}});"#, name, "Movie");
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
<html data-ng-app="movieApp">
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
                <a class="navbar-brand" ui-sref="movies">The {1} App</a>
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
</html>"#, name, "Movie");
    index_t.write_all(index_raw.as_bytes());

    /*
    Rustコード
    */
    let index_tpl_path = "src";
    match fs::create_dir_all(&index_tpl_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }

    let rust_raw = r#"extern crate postgres;
extern crate openssl;
extern crate hyper;
use nickel::{Router, HttpRouter, MediaType, JsonBody};
use nickel::status::StatusCode;
use postgres::{Connection};
use std::sync::{Arc, Mutex};

use std::vec::Vec;

// json化
extern crate rustc_serialize;
use rustc_serialize::{json};

// モデル
#[derive(RustcDecodable, RustcEncodable)]
struct Movie {
    _id: Option<i32>,
    title: String,
    director: String,
    releaseYear: i16,
    genre: String,
}

pub fn url(shared_connection: Arc<Mutex<Connection>>, router: &mut Router) {
    let conn = shared_connection.clone();
    router.get("/setup/movie", middleware! { |_, response|

    // also print to stdout
    return match conn.lock().unwrap().execute("CREATE TABLE Movie (
            id          SERIAL PRIMARY KEY,
            title       VARCHAR (50) NOT NULL,
            releaseYear SMALLINT NOT NULL,
            director    VARCHAR (18) NOT NULL,
            genre       VARCHAR (50) NOT NULL
        )",
    &[]) {
            // http://www.rust-ci.org/Indiv0/paste/doc/nickel/struct.Response.html
            Ok(_) => return response.send("Movie table was created."),
            Err(err) => return response.send(format!("Error running query: {:?}", err))
        };
    });

    // APIs
    router.get("/", middleware! { |_, mut response|
        response.set(MediaType::Html);
        return response.send_file("app/movie/views/index.tpl")
    });

    // select all
    let conn = shared_connection.clone();
    router.get("/api/movies", middleware! { |_, mut response|
        let conn = conn.lock().unwrap();
        let movies = conn.query("select id, title, releaseYear, director, genre from movie", &[]).unwrap();
        let mut v: Vec<Movie> = vec![];

        for row in &movies {
            let movie = Movie {
                _id: row.get(0),
                title: row.get(1),
                releaseYear: row.get(2),
                director: row.get(3),
                genre: row.get(4),
            };

            v.push(movie);
        }

        let json_obj = json::encode(&v).unwrap();
        // MediaType can be any valid type for reference see
        response.set(MediaType::Json);
        response.set(StatusCode::Ok);
        return response.send(json_obj);
    });

    // insert
    let conn = shared_connection.clone();
    router.post("/api/movies", middleware! { |request, mut response|
        let conn = conn.lock().unwrap();
        let stmt = match conn.prepare("insert into movie (title, releaseYear, director, genre)
            values ($1, $2, $3, $4)") {
            Ok(stmt) => stmt,
            Err(e) => {
                return response.send(format!("Preparing query failed: {}", e));
            }
        };

        let movie = request.json_as::<Movie>().unwrap();
        match stmt.execute(&[
            &movie.title.to_string(),
            &movie.releaseYear,
            &movie.director.to_string(),
            &movie.genre.to_string()
        ]) {
            Ok(_) => {
                println!("Inserting movie was Success.");
                response.set(StatusCode::Ok);
            },
            Err(e) => println!("Inserting movie failed. => {:?}", e),
        };

        return response.send("");
    });

    // select one
    let conn = shared_connection.clone();
    router.get("/api/movies/:id", middleware! { |request, mut response|
        let conn = conn.lock().unwrap();
        let movie = conn.query(
            "select id, title, releaseYear, director, genre from movie where id = $1",
            // param string to int
            &[&request.param("id").unwrap().parse::<i32>().unwrap()]
        ).unwrap();

        // movie
        for row in &movie {
            let movie = Movie {
                _id: row.get(0),
                title: row.get(1),
                releaseYear: row.get(2),
                director: row.get(3),
                genre: row.get(4),
            };

            let json_obj = json::encode(&movie).unwrap();
            // MediaType can be any valid type for reference see
            response.set(MediaType::Json);
            response.set(StatusCode::Ok);
            return response.send(json_obj);
        }
    });

    // update
    let conn = shared_connection.clone();
    router.put("/api/movies/:id", middleware! { |request, mut response|
        let conn = conn.lock().unwrap();
        let stmt = match conn.prepare("update movie set title=$1, releaseYear=$2,
            director=$3, genre=$4
            where id = $5") {
            Ok(stmt) => stmt,
            Err(e) => {
                return response.send(format!("Preparing query failed: {}", e));
            }
        };

        // JSON to object
        let movie = request.json_as::<Movie>().unwrap();
        match stmt.execute(&[
            &movie.title.to_string(),
            &movie.releaseYear,
            &movie.director.to_string(),
            &movie.genre.to_string(),
            &movie._id
        ]) {
            Ok(_) => {
                println!("Updating movie was Success.");
                response.set(StatusCode::Ok);
            },
            Err(e) => println!("Updating movie failed. => {:?}", e),
        };

        return response.send("");
    });

    // delete
    // curl http://localhost:6767/api/movies/1 -X DELETE
    let conn = shared_connection.clone();
    router.delete("/api/movies/:id", middleware! { |request, mut response|
        let conn = conn.lock().unwrap();
        let stmt = match conn.prepare("delete from movie where id = $1") {
            Ok(stmt) => stmt,
            Err(e) => {
                return response.send(format!("Preparing query failed: {}", e));
            }
        };

        match stmt.execute(&[
            // param string to int
            &request.param("id").unwrap().parse::<i32>().unwrap()
        ]) {
            Ok(_) => {
                println!("Deleting movie was Success.");
                response.set(StatusCode::Ok);
            },
            Err(e) => println!("Deleting movie failed. => {:?}", e),
        };

        return response.send("");
    });
}"#;
    let mut rust_f = File::create(format!("{}/mod.rs", &index_tpl_path)).unwrap();
    rust_f.write_all(rust_raw .as_bytes());
}
