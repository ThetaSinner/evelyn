evelynDesktopApp.component('simpleTaskComponent', {
    template: '@@include(cleanHtml("src/components/simpletask/simple-task-dashboard-container.partial.html"))',

    bindings: {
        simpleTasks: '<',
    },

    controller: function($scope) {
        $scope.edit = function(taskId) {
            alert(taskId);
        };

        $scope.done = function(taskId) {
            alert(taskId);
        };

        $scope.delete = function(taskId) {
            alert(taskId);
        };
    }
});
