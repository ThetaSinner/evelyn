evelynDesktopApp.component('updateSimpleTaskComponent', {
    template: '@@include(cleanHtml("src/components/simpletask/update-simple-task.partial.html"))',

    controller: function($scope, $state, $stateParams, settingsService, serverBridgeService) {
        $(".date-input").fdatepicker({
            format: settingsService.get_date_format(),
            disableDblClickSelection: true,
            leftArrow: '<<',
            rightArrow: '>>',
            closeIcon: 'X',
            closeButton: true,
            pickTime: true,
        });

        $scope.title = "";
        $scope.description = "";
        $scope.dueDate;

        if (_.isObject($stateParams.simpleTask)) {
            $scope.title = $stateParams.simpleTask.Title;
            $scope.description = $stateParams.simpleTask.Description;
            $scope.dueDate = $stateParams.simpleTask.DueDate;
        }

        $scope.update = function() {
            serverBridgeService.send_to_server('/simpletask/update', {
                TaskId: $stateParams.simpleTask.TaskId,
                NewTitle: $scope.title,
                NewDescription: $scope.description,
                NewDueDate: $scope.dueDate,
            }, function (response) {
                if (response.Error === null) {
                    $state.go("dashboard.simpletask");
                }
                else {
                    // TODO handle error.
                    console.log(response);
                }
            });
        };
    }
});
