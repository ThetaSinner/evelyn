var simpleTaskApp = angular.module('simpleTaskApp', []);

simpleTaskApp.controller('SimpleTaskController', function SimpleTaskController($scope) {
  $scope.name = 'Greg';
});
