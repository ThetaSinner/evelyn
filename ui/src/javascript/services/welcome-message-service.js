evelynDesktopApp.factory('WelcomeMessageService', function () {
    return {
        getWelcomeMessage: function () {
            switch (Math.floor(Math.random() * 3)) {
                case 0:
                    return "Evelyn welcomes you";
                case 1:
                    return "Evelyn is sorry for the lack of features";
                case 2:
                    return "Evelyn is happy to see you";
                default:
                    return "Evelyn feels indeterminate";
            }
        },
    };
});
