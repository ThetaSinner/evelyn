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
                    simpleTasks: function (serverBridgeService, settingsService) {
                        return new Promise(function (resolve, reject) {
                            // TODO fetch via cache.
                            serverBridgeService.send_to_server('/simpletask/lookup', {
                                Limit: 0,
                                ShowCompleted: false
                            }, function (response) {
                                // TODO handle response error.
                                for (var i = 0; i < response.SimpleTasks.length; i++) {
                                    response.SimpleTasks[i].dueDate =
                                        moment(response.SimpleTasks[i].dueDate)
                                        .format(settingsService.get_moment_date_format());
                                }

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
                url: '/simpletask/create',
                component: 'createSimpleTaskComponent',
            }
        )
        .state(
            {
                name: 'dashboard.updatesimpletask',
                url: '/simpletask/update',
                component: 'updateSimpleTaskComponent',
                params: {
                    simpleTask: null,
                },
            }
        )
        .state(
            {
                name: 'dashboard.todolist',
                url: '/todolist',
                component: 'todoListComponent',
                resolve: {
                    todoLists: function (serverBridgeService, settingsService) {
                        return new Promise(function (resolve, reject) {
                            // TODO fetch via cache.
                            serverBridgeService.send_to_server('/todolist/lookuplists', {}, function (response) {
                                // TODO handle response error.
                                resolve(response.TodoLists);
                            });
                        });
                    },
                },
            }
        )
        .state(
            {
                name: 'dashboard.viewtodolist',
                url: '/todolist/view',
                component: 'viewTodoListComponent',
                params: {
                    todoListId: null,
                },
                resolve: {
                    todoList: function ($stateParams, serverBridgeService, settingsService) {
                        return new Promise(function (resolve, reject) {
                            // TODO fetch via cache.
                            // TODO handle todoListId null. Display a 'no todo list selected'.
                            serverBridgeService.send_to_server('/todolist/lookuplist', {
                                TodoListId: $stateParams.todoListId,
                            }, function (response) {
                                // TODO handle response error.
                                var todoList = response.TodoList;
                                todoList.TodoListId = $stateParams.todoListId;
                                resolve(todoList);
                            });
                        });
                    },
                },
            }
        )
        .state(
            {
                name: 'dashboard.createtodolist',
                url: '/todolist/create',
                component: 'createTodoListComponent',
            }
        );
});

evelynDesktopApp.run(['$state', function ($state) {
    $state.go('initial');
}]);
