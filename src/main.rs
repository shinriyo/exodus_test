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
    let add_raw = r#"<form class="form-horizontal" role="form" ng-submit="addMovie()">
    <div ng-include="'partials/_form.html'"></div>
</form>"#;
    add_f.write_all(add_raw.as_bytes());

    // partials/hoge-edit.html
    let mut edit_f = File::create(format!("{}/{}-edit.html", partials_path, name)).unwrap();
    let add_raw = r#"<form class="form-horizontal" role="form" ng-submit="updateMovie()">
    <div ng-include="'partials/_form.html'"></div>
</form>"#;
    edit_f.write_all(add_raw.as_bytes());

    // 複数形
    // まだ仮実装
    let mut index_f = File::create(format!("{}/{}s.html", partials_path, name)).unwrap();
    let index_raw = r#"<a ui-sref="newMovie" class="btn-primary btn-lg nodecoration">Add New Movie</a>
<table class="table movietable">
    <tr>
        <td><h3>All Movies</h3></td>
        <td></td>
    </tr>
    <tr ng-repeat="movie in movies">
        <td>{{movie.title}}</td>
        <td>
            <a class="btn btn-primary" ui-sref="viewMovie({id:movie._id})">View</a>
            <a class="btn btn-danger"  ng-click="deleteMovie(movie)">Delete</a>
        </td>
    </tr>
</table>"#;
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
    let js_app_raw = r#"angular.module('movieApp',['ui.router','ngResource','movieApp.controllers','movieApp.services']);

angular.module('movieApp').config(function($stateProvider,$httpProvider){
    $stateProvider.state('movies',{
        url:'/movies',
        templateUrl:'partials/movies.html',
        controller:'MovieListController'
    }).state('viewMovie',{
       url:'/movies/:id/view',
       templateUrl:'partials/movie-view.html',
       controller:'MovieViewController'
    }).state('newMovie',{
        url:'/movies/new',
        templateUrl:'partials/movie-add.html',
        controller:'MovieCreateController'
    }).state('editMovie',{
        url:'/movies/:id/edit',
        templateUrl:'partials/movie-edit.html',
        controller:'MovieEditController'
    });
}).run(function($state){
   $state.go('movies');
});"#;
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
    let js_services_raw = r#"angular.module('movieApp.services',[]).factory('Movie',function($resource){
    return $resource('http://localhost:6767/api/movies/:id',{id:'@_id'},{
        update: {
            method: 'PUT'
        }
    });
}).service('popupService',function($window){
    this.showPopup=function(message){
        return $window.confirm(message);
    }
});"#;
    js_services_f.write_all(js_services_raw.as_bytes());

    // movie/views/index.tpl
    // フォルダ生成
    let index_tpl_path = format!("{}/views", name);
    match fs::create_dir_all(&index_tpl_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }

    let mut index_t = File::create(format!("{}/index.tpl", &index_tpl_path)).unwrap();
    let index_raw = r#"<!DOCTYPE html>
<html data-ng-app="movieApp">
<head lang="en">
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <base href="/"/>
    <title>The Movie App</title>
    <link rel="stylesheet" type="text/css" href="css/bootstrap.min.css"/>
    <link rel="stylesheet" type="text/css" href="css/app.css"/>
</head>
<body>
    <nav class="navbar navbar-default" role="navigation">
        <div class="container-fluid">
            <div class="navbar-header">
                <a class="navbar-brand" ui-sref="movies">The Movie App</a>
            </div>
            <div class="collapse navbar-collapse">
                <ul class="nav navbar-nav">
                    <li class="active"><a ui-sref="movies">Home</a></li>
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
    <script type="text/javascript" src="movie/js/app.js"></script>
    <script type="text/javascript" src="movie/js/controllers.js"></script>
    <script type="text/javascript" src="movie/js/services.js"></script>
    <script type="text/javascript" src="movie/js/directives.js"></script>
    <script type="text/javascript" src="movie/js/filters.js"></script>
    <script type="text/javascript" src="lib/angular-ui-router.min.js"></script>
    <script type="text/javascript" src="lib/angular-resource.min.js"></script>
</body>
</html>"#;
    index_t.write_all(index_raw.as_bytes());

    /*
    Rustコード
    */
    let index_tpl_path = format!("{}/views", name);
    match fs::create_dir_all(&index_tpl_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }

    let rust_raw = r#"TODO:
    "#;
    let mut rust_f = File::create(format!("{}/mod.rc", &index_tpl_path)).unwrap();
    rust_f.write_all(rust_raw .as_bytes());
}
