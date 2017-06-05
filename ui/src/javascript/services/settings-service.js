evelynDesktopApp.factory('settingsService', function SettingsService() {
    return {
        // TODO this needs to be a user setting, ideally collected at create user and
        // with some option to update later.
        date_format: "dd/mm/yyyy hh:ii",

        get_date_format: function () {
            return date_format;
        },

        // TODO derive this from the above.
        moment_date_format: "DD/MM/YYYY HH:mm",

        get_moment_date_format: function () {
            return this.moment_date_format;
        },
    }
});
