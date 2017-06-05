evelynDesktopApp.config(function ($stateProvider, $urlRouterProvider) {
    $urlRouterProvider.otherwise('/logon');

    $stateProvider
        .state(
            {
                name: 'logon',
                url: '/logon',
                template: '@@include(cleanHtml("src/components/logon/logon.partial.html"))',
                controller: 'LogonController',
                controllerAs: 'logonController',
            }
        )
        .state(
            {
                name: 'signup',
                url: '/signup',
                template: '@@include(cleanHtml("src/components/logon/signup.partial.html"))',
                controller: 'SignupController',
                controllerAs: 'signupController',
            }
        )
        .state(
            {
                name: 'dashboard',
                url: '/dashboard',
                template: '@@include(cleanHtml("src/components/dashboard/dashboard.partial.html"))',
                controller: 'DashboardController',
                controllerAs: 'dashboardController',
            }
        )
        .state(
            {
                name: 'dashboard.home',
                url: '/home',
                component: 'homeComponent',
            }
        )
        .state(
            {
                name: 'dashboard.simpletask',
                url: '/simpletask',
                component: 'simpleTaskComponent',
                resolve: {
                    simpleTasks: function (serverBridgeService) {
                        return new Promise(function (resolve, reject) {
                            // TODO fetch via cache.
                            serverBridgeService.send_to_server('/simpletask/lookup', {
                                Limit: 0,
                                ShowCompleted: false
                            }, function (response) {
                                // TODO handle response error.
                                resolve(response.SimpleTasks);
                            });
                        });
                    },
                },
            }
        )
        .state(
            {
                name: 'dashboard.createsimpletask',
                url: '/createsimpletask',
                component: 'createSimpleTaskComponent',
            }
        );
});

evelynDesktopApp.run(['$state', function ($state) {
    $state.go('initial');
}]);
