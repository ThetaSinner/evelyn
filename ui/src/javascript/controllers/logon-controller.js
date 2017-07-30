evelynDesktopApp.controller('LogonController', 
    [
        '$scope',
        'alertify',
        'serverBridgeService',
        function LogonController($scope, alertify, serverBridgeService) {
            alertify.logPosition("bottom right");
            alertify.maxLogItems(10);

            $scope.emailAddress = '';
            $scope.password = '';

            $scope.submit = function (event) {
                serverBridgeService.send_to_server('/user/logon', {
                    EmailAddress: $scope.emailAddress,
                    Password: $scope.password
                }, function (response) {
                    if (response.Error === null) {
                        alertify.success("Logged in successfully");
                        window.location.href = "#!/dashboard/home";
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
