evelynDesktopApp.component('createUserGroupComponent', {
    template: '@@include(cleanHtml("src/components/user-group/create-user-group.partial.html"))',

    controller: function($scope, $state, serverBridgeService) {
        $scope.name = "";
        $scope.description = "";

        $scope.create = function() {
            serverBridgeService.send_to_server('/usergroup/create', {
                Name: $scope.name,
                Description: $scope.description,
            }, function (response) {
                if (response.Error === null) {
                    $state.go('dashboard.usergroups');
                }
                else {
                    console.log(response);
                }
            });
        }
    }
});
