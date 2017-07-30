evelynDesktopApp.component('userGroupsComponent', {
    template: '@@include(cleanHtml("src/components/user-group/user-groups-dashboard-container.partial.html"))',

    bindings: {
        userGroups: '<',
    },

    controller: function($scope, alertify, $state) {
        $scope.create = function() {
            $state.go('dashboard.createusergroup');
        };

        $scope.addMember = function(userGroupId) {
            $state.go('dashboard.addmembertousergroup', {
                userGroupId: userGroupId,
            });
        };

        $scope.viewGroup = function(userGroupId) {
            $state.go('dashboard.viewusergroup', {
                userGroupId: userGroupId,
            });
        };
    }
});
