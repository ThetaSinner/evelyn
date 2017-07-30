evelynDesktopApp.component('todoListComponent', {
    template: '@@include(cleanHtml("src/components/todo-list/todo-list-dashboard-container.partial.html"))',

    bindings: {
        todoLists: '<',
    },

    controller: function($scope, $state, alertify, serverBridgeService) {
        var ctrl = this;

        $scope.viewList = function(todoListId) {
            $state.go('dashboard.viewtodolist', {
                todoListId: todoListId,
            });
        };

        $scope.create = function() {
            $state.go("dashboard.createtodolist");
        };
    }
});
