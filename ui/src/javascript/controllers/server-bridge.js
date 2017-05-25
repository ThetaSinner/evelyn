// TODO this needs to be a user setting, ideally collected at create user and
// with some option to update later.
var date_format = "dd/mm/yyyy hh:ii";
// TODO derive this from the above.
var moment_date_format = "DD/MM/YYYY HH:mm";

function EvelynServerBridge() {
    this.baseUrl = "http://localhost:8080";
}

EvelynServerBridge.prototype.make_url_from_action = function(action) {
    return this.baseUrl + action;
};

EvelynServerBridge.prototype.process_response = function(response) {
    if (response.hasOwnProperty("Token") && response.Token !== null) {
        localStorage.token = response.Token;
    }

    if (response.hasOwnProperty("SimpleTasks") && response.SimpleTasks !== null && response.SimpleTasks.length !== null) {
        simpleTaskCollection.reset();
        var tasks = response.SimpleTasks;
        for (var i = 0; i < tasks.length; i++) {
            tasks[i]['dueDate'] = moment(tasks[i]['dueDate']).format(moment_date_format);

            simpleTaskCollection.push(render_simple_task(tasks[i]));
        }
    }

};

EvelynServerBridge.prototype.process_request = function(request) {
    if (localStorage.token) {
        request.Token = localStorage.token;
    }

    for (var attr in request) {
        if (attr.indexOf('date') !== -1 || attr.indexOf('Date') !== -1) {
            request[attr] = moment(request[attr], moment_date_format).toISOString();
        }
    }

    return request;
};


EvelynServerBridge.prototype.send_to_server = function(action, payload, callback) {
    var url = this.make_url_from_action(action);
    var processed_payload = this.process_request(payload);
    var _this = this;
    $.ajax({
        method: "POST",
        url: url,
        data: JSON.stringify(processed_payload),
        dataType: "json",
        // The first request may be slow because a data connection must be opened on the server
        // but after that, such a long timeout is a problem...
        timeout: 3000,
        success: function(response) {
            console.log("Response from evelyn", response);

            _this.process_response(response);

            callback(response);
        },
        error: function(jqxhr, text_status, error) {
            if (text_status === "timeout") {
                render_response_payload({
                    Error: {
                        ErrorCode: 0,
                        ErrorMessage: "Evelyn service not available.",
                    }
                });
            } else {
                console.log(jqxhr);
                render_response_payload({
                    Error: {
                        ErrorCode: 0,
                        ErrorMessage: "Unhandled error occured in Evelyn bridge [" + text_status + "]",
                    }
                });
            }
        },
    });
}

function serialize_form(form_selector) {
    return _.object(_.map(form_selector.serializeArray(), function(item) {
        // {name: "name", value: "data"}

        if (item.value === "on") {
            return [item.name, true];
        } else if (item.value == "false") {
            return [item.name, false];
        } else if (!isNaN(item.value)) { //check if value is convertible to number
            return [item.name, Number(item.value)];
        } else {
            return [item.name, item.value];
        }
    }));
};

function add_submit_hook(form_id) {
    $(form_id).on("submit", function(event) {
        event.preventDefault();

        var form_selector = $(this);
        var form_submit_data = serialize_form(form_selector);

        evelynServerBridge.send_to_server(form_selector.attr("action"), form_submit_data, function() {});
    });
}

$(document).ready(function() {
    if (typeof(Storage) === "undefined") {
        $("body").text("Local storage not available in this environment, cannot function correctly.");
        return;
    }

    $(".date-input").fdatepicker({
        initialDate: moment().hour(12).minute(0).add(1, 'days').format(moment_date_format),
        format: date_format,
        disableDblClickSelection: true,
        leftArrow: '<<',
        rightArrow: '>>',
        closeIcon: 'X',
        closeButton: true,
        pickTime: true,
    });

    add_submit_hook("#form_create_user");
    add_submit_hook("#form_logon_user");
    add_submit_hook("#form_create_simple_task");
    add_submit_hook("#form_lookup_simple_tasks");
    add_submit_hook("#form_create_todo_list");
    add_submit_hook("#form_lookup_todo_lists");

    todoListViewHandle = new TodoListView({
        model: new TodoListModel()
    });
});

var evelynServerBridge = new EvelynServerBridge();
