evelynDesktopApp.controller('SignupController', ['$scope', 'serverBridgeService', function SignupController($scope, serverBridgeService) {
    $scope.error = null;
    $scope.userName = '';
    $scope.emailAddress = '';
    $scope.password = '';

    $scope.submit = function () {
        serverBridgeService.send_to_server('/user/create', {
            UserName: $scope.userName,
            EmailAddress: $scope.emailAddress,
            Password: $scope.password
        }, function (response) {
            if (response.Error === null) {
                window.location.href = "#!/logon";
            }
            else {
                $scope.error = "" + response.Error.ErrorCode + " : " + response.Error.ErrorMessage;
                $scope.$apply();
                console.log(response);
            }
        });
    };
}]);
