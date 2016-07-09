$(document).ready(function() {
	$(".panel-title > a[data-toggle=collapse]").click(function(e) {
		e.preventDefault();
	});
});

var create_problem = function() {
	var input = "#new_problem_form input";
	var data = $("#new_problem_form").serializeObject();

	var grader_contents = ace.edit("new_grader").getValue();
	data["grader_contents"] = grader_contents;

	var bonus = $("#bonus").val();
	data["bonus"] = bonus;

	var autogen = $("#autogen").is(":checked");
	data["autogen"] = autogen ? 1 : 0;

	$(input).attr("disabled", "disabled");
	api_call("POST", "/api/problem/add", data, function(result) {
		if (result["success"] == 1) {
			display_message("add-status", "success", result["message"], function() {
				$(input).removeAttr("disabled");
			});
		} else {
			display_message("add-status", "danger", result["message"], function() {
				$(input).removeAttr("disabled");
			});
		}
	}, function(jqXHR) {
		var result = jqXHR["responseText"];
		display_message("add-status", "danger", "Error " + jqXHR["status"] + ": " + result["message"], function() {
			$(input).removeAttr("disabled");
		});
	});
};

var update_problem = function(form_id) {
	var input = "#" + form_id + " input";
	var data = $("#" + form_id).serializeObject();
	var pid = data["pid"];

	var grader_contents = ace.edit(pid + "_grader").getValue();
	data["grader_contents"] = grader_contents;

	var bonus = $("#" + pid + "_bonus").val();
	data["bonus"] = bonus;

	var autogen = $("#" + pid + "_autogen").is(":checked");
	data["autogen"] = autogen ? 1 : 0;

	$(input).attr("disabled", "disabled");
	api_call("POST", "/api/problem/update", data, function(result) {
		if (result["success"] == 1) {
			display_message(pid + "_status", "success", result["message"], function() {
				$(input).removeAttr("disabled");
			});
		} else {
			display_message(pid + "_status", "danger", result["message"], function() {
				$(input).removeAttr("disabled");
			});
		}
	}, function(jqXHR) {
		var result = jqXHR["responseText"];
		display_message(pid + "_status", "danger", "Error " + jqXHR["status"] + ": " + result["message"], function() {
			$(input).removeAttr("disabled");
		});
	});
};

var delete_problem = function(form_id) {
	$('#confirm').modal("show", { backdrop: 'static', keyboard: false })
        .one('click', '#yes', function() {
		var input = "#" + form_id + " input";
		var pid = form_id.split("_")[1];
		$(input).attr("disabled", "disabled");
		api_call("POST", "/api/problem/delete", {"pid": pid}, function(result) {
			if (result["success"] == 1) {
				display_message(pid + "_status", "success", result["message"], function() {
					$(input).removeAttr("disabled");
				});
			} else {
				display_message(pid + "_status", "danger", result["message"], function() {
					$(input).removeAttr("disabled");
				});
			}
		}, function(jqXHR) {
			var result = jqXHR["responseText"];
			display_message(pid + "_status", "danger", "Error " + jqXHR["status"] + ": " + result["message"], function() {
				$(input).removeAttr("disabled");
			});
		});

        });
}

var update_settings = function() {
	var input = $("#update_settings_form input");
	var data = $(input).serializeObject();
	$(input).attr("disabled", "disabled");
	api_call("POST", "/api/admin/settings/update", data, function(result) {
		if (result["success"] == 1) {
			display_message("update_settings_msg", "success", result["message"], function() {
				$(input).removeAttr("disabled");
			});
		} else {
			display_message("update_settings_msg", "danger", result["message"], function() {
				$(input).removeAttr("disabled");
			});
		}
	}, function(jqXHR) {
		var result = jqXHR["responseText"];
		display_message("update_settings_msg", "danger", "Error " + jqXHR["status"] + ": " + result["message"], function() {
			$(input).removeAttr("disabled");
		});
	});
}

function clear_submissions(form_id) {
	$('#confirm').modal("show", { backdrop: 'static', keyboard: false })
        .one('click', '#yes', function() {
		var input = "#" + form_id + " input";
		var pid = form_id.split("_")[1];
		$(input).attr("disabled", "disabled");
		api_call("POST", "/api/problem/clear_submissions", {"pid": pid}, function(result) {
			if (result["success"] == 1) {
				display_message(pid + "_status", "success", result["message"], function() {
					$(input).removeAttr("disabled");
				});
			} else {
				display_message(pid + "_status", "danger", result["message"], function() {
					$(input).removeAttr("disabled");
				});
			}
		}, function(jqXHR) {
			var result = jqXHR["responseText"];
			display_message(pid + "_status", "danger", "Error " + jqXHR["status"] + ": " + result["message"], function() {
				$(input).removeAttr("disabled");
			});
		});

        });
}
