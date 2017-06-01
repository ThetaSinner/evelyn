evelynDesktopApp.controller('DashboardController', ['$scope', '$state', 'sessionDataService', 'WelcomeMessageService', function DashboardController($scope, $state, sessionDataService, welcomeMessageService) {
    this.welcomeMessage = welcomeMessageService.getWelcomeMessage();

    $scope.appDashboardToggleShrink = function(event) {
        // See foundation-building-blocks/app-dashboard-layout
        event.preventDefault();
        $(event.target).parents('.app-dashboard').toggleClass('shrink-medium').toggleClass('shrink-large');
    };

    $scope.logout = function() {
        sessionDataService.destroy();
        $state.go('logon');
    };
}]);
