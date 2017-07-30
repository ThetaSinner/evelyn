evelynDesktopApp.component('viewUserGroupComponent', {
    template: '@@include(cleanHtml("src/components/user-group/view-user-group.partial.html"))',

    bindings: {
        userGroup: '<',
    },

    controller: function($scope, alertify, $state) {

    }
});
