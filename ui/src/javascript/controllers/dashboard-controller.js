evelynDesktopApp.controller('DashboardController', ['$scope', 'WelcomeMessageService', function DashboardController($scope, welcomeMessageService) {
    this.welcomeMessage = welcomeMessageService.getWelcomeMessage();

    $scope.appDashboardToggleShrink = function(event) {
        // See foundation-building-blocks/app-dashboard-layout
        event.preventDefault();
        $(event.target).parents('.app-dashboard').toggleClass('shrink-medium').toggleClass('shrink-large');
    }
}]);
