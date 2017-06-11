evelynDesktopApp.controller('LogonController', ['$scope', 'serverBridgeService', function LogonController($scope, serverBridgeService) {
    $scope.error = null;
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
                $scope.error = "" + response.Error.ErrorCode + " : " + response.Error.ErrorMessage;
                $scope.$apply();
                console.log(response);
            }
        });
    };
}]);
