evelynDesktopApp.controller('DashboardController', 
    [
        '$scope',
        '$state',
        'alertify',
        'sessionDataService',
        function DashboardController($scope, $state, alertify, sessionDataService) {
            alertify.logPosition("bottom right");
            alertify.maxLogItems(10);

            $scope.appDashboardToggleShrink = function(event) {
                // See foundation-building-blocks/app-dashboard-layout
                event.preventDefault();
                $(event.target).parents('.app-dashboard').toggleClass('shrink-medium').toggleClass('shrink-large');
            };

            $scope.logout = function() {
                alertify.success("You have logged out successfully. See you soon");
                sessionDataService.destroy();
                $state.go('logon');
            };
        }
    ]
);
