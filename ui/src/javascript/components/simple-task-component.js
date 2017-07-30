evelynDesktopApp.component('simpleTaskComponent', {
    template: '@@include(cleanHtml("src/components/simpletask/simple-task-dashboard-container.partial.html"))',

    bindings: {
        simpleTasks: '<',
    },

    controller: function($scope, $state, alertify, serverBridgeService) {
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
                    alertify.success("Sucessfully updated simple task");
                    $state.reload();
                }
                else {
                    alertify.error("" + response.Error.ErrorCode + " : " + response.Error.ErrorMessage);
                    console.log(response);
                }
            });
        };

        $scope.delete = function(taskId) {
            serverBridgeService.send_to_server('/simpletask/remove', {
                TaskId: taskId,
            }, function (response) {
                if (response.Error === null) {
                    alertify.success("Sucessfully deleted simple task");
                    $state.reload();
                }
                else {
                    alertify.error("" + response.Error.ErrorCode + " : " + response.Error.ErrorMessage);
                    console.log(response);
                }
            });
        };

        $scope.create = function() {
            $state.go("dashboard.createsimpletask");
        };
    }
});
