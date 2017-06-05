evelynDesktopApp.component('createSimpleTaskComponent', {
    template: '@@include(cleanHtml("src/components/simpletask/create-simple-task.partial.html"))',

    controller: function($scope, $state, serverBridgeService) {
        $scope.title = "";
        $scope.description = "";
        $scope.dueDate = new Date();

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
