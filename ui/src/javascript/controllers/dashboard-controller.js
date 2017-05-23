evelynDesktopApp.controller('DesktopController', ['WelcomeMessageService', function DesktopController(welcomeMessageService) {
    this.welcomeMessage = welcomeMessageService.getWelcomeMessage();
}]);
