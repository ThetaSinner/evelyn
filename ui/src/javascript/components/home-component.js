evelynDesktopApp.component('homeComponent', {
    template: '@@include(cleanHtml("src/components/home/home.partial.html"))',

    controller: function ($scope, welcomeMessageService) {
        this.welcomeMessage = welcomeMessageService.getWelcomeMessage();
    },
});
