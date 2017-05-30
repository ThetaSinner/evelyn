evelynDesktopApp.config(function ($routeProvider) {
    $routeProvider
        .when('/', {
            template: '@@include(cleanHtml("src/components/logon/logon.partial.html"))',
            controller: 'LogonController',
            controllerAs: 'logonController',
        })
        .when('/logon', {
            template: '@@include(cleanHtml("src/components/logon/logon.partial.html"))',
            controller: 'LogonController',
            controllerAs: 'logonController',
        })
        .when('/signup', {
            template: '@@include(cleanHtml("src/components/logon/signup.partial.html"))',
            controller: 'SignupController',
            controllerAs: 'signupController',
        })
        .when('/dashboard', {
            template: '@@include(cleanHtml("src/components/dashboard/dashboard.partial.html"))',
            controller: 'DashboardController',
            controllerAs: 'dashboardController',
        });
});

evelynDesktopApp.run(function ($rootScope) {
    // Startup code which doesn't belong in a controller goes here.
});
