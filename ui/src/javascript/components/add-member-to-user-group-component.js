evelynDesktopApp.component('addMemberToUserGroupComponent', {
    template: '@@include(cleanHtml("src/components/user-group/add-member-to-user-group.partial.html"))',

    controller: function($scope, $state, alertify, $stateParams, serverBridgeService) {
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
                    alertify.success("Sucessfully added member to user group");
                    $state.go('dashboard.usergroups');
                }
                else {
                    alertify.error("" + response.Error.ErrorCode + " : " + response.Error.ErrorMessage);
                }
            });
        }
    }
});
