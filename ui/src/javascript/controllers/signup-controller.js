evelynDesktopApp.controller('SignupController', ['$scope', 'ServerBridgeService', function SignupController($scope, ServerBridgeService) {
    $scope.userName = '';
    $scope.emailAddress = '';
    $scope.password = '';

    $scope.submit = function () {
        ServerBridgeService.send_to_server('/user/create', {
            UserName: $scope.userName,
            EmailAddress: $scope.emailAddress,
            Password: $scope.password
        }, function (response) {
            if (response.Error === null) {
                window.location.href = "#!/logon";
            }
            else {
                console.log(response);
            }
        });
    };
}]);
