evelynDesktopApp.component('updateSimpleTaskComponent', {
    template: '@@include(cleanHtml("src/components/simpletask/update-simple-task.partial.html"))',

    controller: function($scope, $state, $stateParams, settingsService, serverBridgeService) {
        $(".date-input").fdatepicker({
            initialDate: moment().hour(12).minute(0).add(1, 'days').format(settingsService.get_moment_date_format()),
            format: settingsService.get_date_format(),
            disableDblClickSelection: true,
            leftArrow: '<<',
            rightArrow: '>>',
            closeIcon: 'X',
            closeButton: true,
            pickTime: true,
        });

        console.log("state params", $stateParams);

        $scope.title = "";
        $scope.description = "";
        $scope.dueDate;

        if (_.isObject($stateParams.simpleTask)) {
            $scope.title = $stateParams.simpleTask.title;
            $scope.description = $stateParams.simpleTask.description;
            $scope.dueDate = $stateParams.simpleTask.dueDate;
        }

        $scope.update = function() {
            serverBridgeService.send_to_server('/simpletask/update', {
                TaskId: $stateParams.simpleTask.taskId,
                Title: $scope.title,
                Description: $scope.description,
                DueDate: $scope.dueDate,
            }, function (response) {
                if (response.Error === null) {
                    $state.go("dashboard.simpletask");
                }
                else {
                    console.log(response);
                }
            });
        };
    }
});
