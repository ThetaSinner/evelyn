evelynDesktopApp.factory('sessionDataService', function sessionDataService() {
    function getSessionData() {
        if (!localStorage.getItem('evelynSessionData')) {
            saveSessionData({});
        }

        return JSON.parse(localStorage.evelynSessionData);
    }

    function saveSessionData(sessionData) {
        localStorage.evelynSessionData = JSON.stringify(sessionData);
    }

    return {
        setToken: function(token) {
            var sessionData = getSessionData();
            sessionData.token = token;
            saveSessionData(sessionData);
            return this;
        },

        getToken: function() {
            return getSessionData().token;
        },

        destroy: function() {
            localStorage.removeItem('evelynSessionData');
            saveSessionData({});
        },
    };
});
