evelynDesktopApp.factory('sessionDataService', function sessionDataService() {
    localStorage.evelynSessionData = {};

    return {
        setToken: function(token) {
            localStorage.evelynSessionData.token = token;
            return this;
        },

        getToken: function() {
            return localStorage.evelynSessionData.token;
        },

        destroy: function() {
            localStorage.removeItem('evelynSessionData');
        },
    };
});
