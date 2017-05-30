evelynDesktopApp.config(function ($stateProvider, $urlRouterProvider) {
    $urlRouterProvider.otherwise('/logon');

    $stateProvider
        .state(
            {
                name: 'logon',
                url: '/logon',
                template: '@@include(cleanHtml("src/components/logon/logon.partial.html"))',
                controller: 'LogonController',
                controllerAs: 'logonController'
            }
        )
        .state(
            {
                name: 'signup',
                url: '/signup',
                template: '@@include(cleanHtml("src/components/logon/signup.partial.html"))',
                controller: 'SignupController',
                controllerAs: 'signupController'
            }
        )
        .state(
            {
                name: 'dashboard',
                url: '/dashboard',
                template: '@@include(cleanHtml("src/components/dashboard/dashboard.partial.html"))',
                controller: 'DashboardController',
                controllerAs: 'dashboardController'
            }
        );
});

evelynDesktopApp.run(['$state', function ($state) {
    $state.go('initial');
}]);
