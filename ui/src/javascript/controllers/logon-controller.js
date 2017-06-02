evelynDesktopApp.controller('LogonController', ['$scope', 'serverBridgeService', function LogonController($scope, serverBridgeService) {
    $scope.emailAddress = '';
    $scope.password = '';

    $scope.submit = function (event) {
        serverBridgeService.send_to_server('/user/logon', {
            EmailAddress: $scope.emailAddress,
            Password: $scope.password
        }, function (response) {
            if (response.Error === null) {
                window.location.href = "#!/dashboard/home";
            }
            else {
                console.log(response);
            }
        });
    };
}]);
