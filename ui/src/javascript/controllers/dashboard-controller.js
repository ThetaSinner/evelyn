evelynDesktopApp.controller('DashboardController', ['WelcomeMessageService', function DashboardController(welcomeMessageService) {
    this.welcomeMessage = welcomeMessageService.getWelcomeMessage();
}]);
