evelynDesktopApp.component('simpleTaskComponent', {
    template: '@@include(cleanHtml("src/components/simpletask/simple-task-dashboard-container.partial.html"))',

    bindings: {
        simpleTasks: '<',
    },

    controller: function ($scope) {
      $scope.edit = function(taskId) {
        alert(taskId);
      };
    }
});
