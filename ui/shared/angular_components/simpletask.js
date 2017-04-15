var simpleTaskApp = angular.module('simpleTaskApp', ['ngRoute']);

simpleTaskApp.config(function($routeProvider, $locationProvider) {
  $routeProvider
    .when('/', {
      templateUrl: 'partials/simpletask/lookup.html',
      controller: 'SimpleTaskLookupController',
    })
    .when('/create', {
      templateUrl: 'partials/simpletask/create.html',
      controller: 'SimpleTaskCreateController',
    })
    .when('/lookup', {
      templateUrl: 'partials/simpletask/lookup.html',
      controller: 'SimpleTaskLookupController',
    });

  $locationProvider
    .html5Mode(true)
    .hashPrefix('');
});

simpleTaskApp.controller('SimpleTaskLookupController', function SimpleTaskLookupController($scope) {
  $scope.title = 'Lookup';
});

simpleTaskApp.controller('SimpleTaskCreateController', function SimpleTaskCreateController($scope) {
  $scope.title = 'Create';
});
