evelynDesktopApp.component('homeComponent', {
    template: '@@include(cleanHtml("src/components/home/home.partial.html"))',

    controller: function ($scope, alertify, welcomeMessageService) {
        this.welcomeMessage = welcomeMessageService.getWelcomeMessage();
    },
});
