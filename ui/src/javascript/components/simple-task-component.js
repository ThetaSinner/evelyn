evelynDesktopApp.component('simpleTaskComponent', {
    template: '@@include(cleanHtml("src/components/simpletask/simple-task-dashboard-container.partial.html"))',

    bindings: {
        simpleTasks: '<',
    },

    controller: function($scope, $state) {
        var ctrl = this;

        $scope.edit = function(taskId) {
            $state.go(
                'dashboard.updatesimpletask',
                {simpleTask: _.find(ctrl.simpleTasks, function (obj) { return obj.taskId === taskId; })},
            );
        };

        $scope.done = function(taskId) {
            alert(taskId);
        };

        $scope.delete = function(taskId) {
            alert(taskId);
        };

        $scope.create = function() {
            $state.go("dashboard.createsimpletask");
        };
    }
});
