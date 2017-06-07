evelynDesktopApp.component('createSimpleTaskComponent', {
    template: '@@include(cleanHtml("src/components/simpletask/create-simple-task.partial.html"))',

    controller: function($scope, $state, settingsService, serverBridgeService) {
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

        $scope.title = "";
        $scope.description = "";
        $scope.dueDate;

        $scope.create = function() {
            serverBridgeService.send_to_server('/simpletask/create', {
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
