evelynDesktopApp.controller('SignupController', 
    [
        '$scope', 
        'alertify', 
        'serverBridgeService', 
        function SignupController($scope, alertify, serverBridgeService) {
            alertify.logPosition("bottom right");
            alertify.maxLogItems(10);

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
                        alertify.success("Account creation successful, Welcome to Evelyn!. <br><br>Please use your new account to log in");
                        window.location.href = "#!/logon";
                    }
                    else {
                        alertify.error("" + response.Error.ErrorCode + " : " + response.Error.ErrorMessage);
                        console.log(response);
                    }
                });
            };
        }
    ]
);
