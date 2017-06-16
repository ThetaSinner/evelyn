evelynDesktopApp.component('addMemberToUserGroupComponent', {
    template: '@@include(cleanHtml("src/components/user-group/add-member-to-user-group.partial.html"))',

    controller: function($scope, $state, $stateParams, serverBridgeService) {
        $scope.query = "";

        $scope.search = function() {
            if ($scope.query === "") {
                return;
            }

            serverBridgeService.send_to_server('/user/search', {
                Query: $scope.query,
            }, function (response) {
                $scope.searchResults = response.SearchResults;
            });
        };

        $scope.addMember = function(userId) {
            serverBridgeService.send_to_server('/usergroup/member/add', {
                UserGroupId: $stateParams.userGroupId,
                Member: {
                    UserId: userId,
                },
            }, function (response) {
                if (response.Error === null) {
                    $state.go('dashboard.usergroups');
                }
                else {
                    console.log(response);
                }
            });
        }
    }
});
