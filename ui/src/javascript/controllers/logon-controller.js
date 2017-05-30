evelynDesktopApp.controller('LogonController', ['$scope', 'ServerBridgeService', function LogonController($scope, ServerBridgeService) {
    $scope.emailAddress = '';
    $scope.password = '';

    $scope.submit = function (event) {
        ServerBridgeService.send_to_server('/user/logon', {
            EmailAddress: $scope.emailAddress,
            Password: $scope.password
        }, function (response) {
            if (response.Error === null) {
                window.location.href = "#!/dashboard";
            }
            else {
                console.log(response);
            }
        });
    };
}]);
