evelynDesktopApp.component('simpleTaskComponent', {
    template: '@@include(cleanHtml("src/components/simpletask/simple-task-dashboard-container.partial.html"))',

    bindings: {
        simpleTasks: '<',
    },

    controller: function($scope, $state, serverBridgeService) {
        var ctrl = this;

        $scope.edit = function(taskId) {
            $state.go(
                'dashboard.updatesimpletask',
                {simpleTask: _.find(ctrl.simpleTasks, function (obj) { return obj.TaskId === taskId; })}
            );
        };

        $scope.done = function(taskId) {
            serverBridgeService.send_to_server('/simpletask/update', {
                TaskId: taskId,
                NewCompleted: true,
            }, function (response) {
                if (response.Error === null) {
                    $state.reload();
                }
                else {
                    // TODO handle error.
                    console.log(response);
                }
            });
        };

        $scope.delete = function(taskId) {
            serverBridgeService.send_to_server('/simpletask/remove', {
                TaskId: taskId,
            }, function (response) {
                if (response.Error === null) {
                    $state.reload();
                }
                else {
                    // TODO handle error.
                    console.log(response);
                }
            });
        };

        $scope.create = function() {
            $state.go("dashboard.createsimpletask");
        };
    }
});
