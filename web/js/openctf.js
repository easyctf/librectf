var app = angular.module("openctf", [ "ngRoute" ]);

app.config(["$compileProvider", function($compileProvider) {
	$compileProvider.aHrefSanitizationWhitelist(/^\s*(https?|ftp|mailto|file|javascript):/);
}]);
app.config(function($routeProvider, $locationProvider) {
	$routeProvider.when("/", {
		templateUrl: "pages/home.html",
		controller: "mainController"
	})
	.when("/about", {
		templateUrl: "pages/about.html",
		controller: "mainController"
	})
	.when("/chat", {
		templateUrl: "pages/chat.html",
		controller: "mainController"
	})
	.when("/learn", {
		templateUrl: "pages/learn.html",
		controller: "mainController"
	})
	.when("/login", {
		templateUrl: "pages/login.html",
		controller: "mainController"
	})
	.when("/logout", {
		templateUrl: "pages/blank.html",
		controller: "logoutController"
	})
	.when("/profile", {
		templateUrl: "pages/profile.html",
		controller: "profileController"
	})
	.when("/profile/:username", {
		templateUrl: "pages/profile.html",
		controller: "profileController"
	})
	.when("/register", {
		templateUrl: "pages/register.html",
		controller: "mainController"
	})
	.when("/scoreboard", {
		templateUrl: "pages/scoreboard.html",
		controller: "scoreboardController"
	})
	.when("/settings", {
		templateUrl: "pages/settings.html",
		controller: "settingsController"
	})
	.when("/settings/twofactor", {
		templateUrl: "pages/twofactor.html",
		controller: "twofactorController"
	})
	.when("/forgot", {
		templateUrl: "pages/forgot.html",
		controller: "resetController"
	})
	.when("/forgot/:token", {
		templateUrl: "pages/forgot.html",
		controller: "resetController"
	})
	.when("/team", {
		templateUrl: "pages/team.html",
		controller: "teamController"
	})
	.when("/team/:teamname", {
		templateUrl: "pages/team.html",
		controller: "teamController"
	})
	.when("/admin/stats", {
		templateUrl: "pages/admin/statistics.html",
		controller: "adminStatisticsController"
	})
	.when("/admin/settings", {
		templateUrl: "pages/admin/settings.html",
		controller: "adminSettingsController"
	})
	.otherwise({
		templateUrl: "pages/404.html",
		controller: "mainController"
	});
	$locationProvider.html5Mode(true);
});

function api_call(method, url, data, callback_success, callback_fail) {
	if (method.toLowerCase() == "post") {
		data["csrf_token"] = $.cookie("csrf_token");
	}
	$.ajax({
		"type": method,
		"datatype": "json",
		"data": data,
		"url": url,
		"cache": false
	}).done(callback_success).fail(function(xhr) {
		callback_fail();
	});
}

function permanent_message(containerId, alertType, message, callback) {
	$("#" + containerId).html("<div class=\"alert alert-" + alertType + "\" style=\"margin:0;\">" + message + "</div>");
	$("#" + containerId).hide().slideDown("fast", "swing");
};

function display_message(containerId, alertType, message, callback) {
	$("#" + containerId).html("<div class=\"alert alert-" + alertType + "\">" + message + "</div>");
	$("#" + containerId).hide().slideDown("fast", "swing", function() {
		window.setTimeout(function () {
			$("#" + containerId).slideUp("fast", "swing", callback);
		}, message.length * 55);
	});
};

app.controller("mainController", ["$scope", "$http", function($scope, $http) {
	$scope.config = { navbar: { } };
	$scope.timestamp = Date.now();
	api_call("GET", "/api/user/status", {}, function(result) {
		if (result["success"] == 1) {
			delete result["success"];
			$scope.config.navbar = result;
			$scope.$emit("loginStatus");
		} else {
			$scope.config.navbar.logged_in = false;
		}
		$scope.$apply();
	}, function() {
		$scope.config.navbar.logged_in = false;
		$scope.$apply();
		permanent_message("site-message", "danger", "<div class='container'>The OpenCTF API is down. Please wait while we try to resolve this issue.</div>");
	});
}]);

app.controller("logoutController", function() {
	api_call("GET", "/api/user/logout", {}, function(result) {
		location.href = "/";
	});
});

app.controller("profileController", ["$controller", "$scope", "$http", "$routeParams", "$sce", function($controller, $scope, $http, $routeParams, $sce) {
	var data = { };
	if ("username" in $routeParams) data["username"] = $routeParams["username"];
	$controller("mainController", { $scope: $scope });
	api_call("GET", "/api/user/info", data, function(result) {
		if (result["success"] == 1) {
			$scope.user = result["user"];
			for(var i=0; i<$scope.user.activity.length; i++) {
				$scope.user.activity[i].message_clean = $sce.trustAsHtml($scope.user.activity[i].message);
			}
		}
		$scope.$apply();
		$(".timeago").timeago();
	});
}]);

app.controller("loginController", ["$controller", "$scope", "$http", function($controller, $scope, $http) {
	$controller("mainController", { $scope: $scope });
	$scope.$on("loginStatus", function() {
		if ($scope.config["navbar"].logged_in != true) {
			location.href = "/login";
			return;
		}
	});
}]);

app.controller("teamController", ["$controller", "$scope", "$http", "$routeParams", function($controller, $scope, $http, $routeParams) {
	var data = { };
	if ("teamname" in $routeParams) {
		data["teamname"] = $routeParams["teamname"];
	} else {
		$controller("loginController", { $scope: $scope });
	}
	api_call("GET", "/api/team/info", data, function(result) {
		if (result["success"] == 1) {
			$scope.team = result["team"];
		}
		$scope.$apply();
		$(".timeago").timeago();
	});
}]);

app.controller("scoreboardController", ["$controller", "$scope", "$http", function($controller, $scope, $http) {
	$controller("mainController", { $scope: $scope });
	api_call("GET", "/api/stats/scoreboard", { }, function(result) {
		if (result["success"] == 1) {
			$scope.scoreboard = result["scoreboard"];
			$scope.$apply();
		}
	});
}]);

app.controller("resetController", ["$controller", "$scope", "$http", "$routeParams", function($controller, $scope, $http, $routeParams) {
	var data = { };
	$scope.token = false;
	data["csrf_token"] = $.cookie("csrf_token");
	if ("token" in $routeParams) {
		$scope.token = true;
		token = $routeParams["token"];
		api_call("GET", "/api/user/forgot/" + token, data, function(data) {
			$scope.body = data["message"];
			$scope.success = data["success"]
			$scope.$apply();
		});
	} else {
		$controller("mainController", { $scope: $scope });
	}
}]);

app.controller("adminController", ["$controller", "$scope", "$http", function($controller, $scope, $http) {
	$controller("mainController", { $scope: $scope });
	$scope.$on("loginStatus", function() {
		if ($scope.config["navbar"].logged_in != true) {
			location.href = "/login";
			return;
		}
		if ($scope.config["navbar"].admin != true) {
			location.href = "/profile";
			return;
		}
	});
}]);

app.controller("adminStatisticsController", ["$controller", "$scope", "$http", function($controller, $scope, $http) {
	$controller("adminController", { $scope: $scope });
	api_call("GET", "/api/admin/stats/overview", {}, function(result) {
		if (result["success"] == 1) {
			$scope.overview = result["overview"];
		} else {
			$scope.overview = [];
		}
		$scope.$apply();
	});
}]);

app.controller("adminSettingsController", ["$controller", "$scope", "$http", function($controller, $scope, $http) {
	$controller("adminController", { $scope: $scope });
	api_call("GET", "/api/admin/stats/overview", {}, function(result) {
		if (result["success"] == 1) {
			$scope.overview = result["overview"];
		} else {
			$scope.overview = [];
		}
		$scope.$apply();
	});
}]);

app.controller("settingsController", ["$controller", "$scope", "$http", function($controller, $scope, $http) {
	$controller("loginController", { $scope: $scope });
	api_call("GET", "/api/user/info", {}, function(result) {
		if (result["success"] == 1) {
			$scope.user = result["user"];
		}
		$scope.$apply();
	});
}]);

app.controller("twofactorController", ["$controller", "$scope", "$http", function($controller, $scope, $http) {
	$controller("loginController", { $scope: $scope });
	api_call("GET", "/api/user/info", {}, function(result) {
		if (result["success"] == 1) {
			$scope.user = result["user"];
		}
		$scope.$apply();
	});
}]);

$.fn.serializeObject = function() {
	var a, o;
	o = {};
	a = this.serializeArray();
	$.each(a, function() {
		if (o[this.name]) {
			if (!o[this.name].push) {
				o[this.name] = [o[this.name]];
			}
			return o[this.name].push(this.value || "");
		} else {
			return o[this.name] = this.value || "";
		}
	});
	return o;
};

// register page

var register_form = function() {
	var input = "#register_form input";
	var data = $("#register_form").serializeObject();
	$(input).attr("disabled", "disabled");
	api_call("POST", "/api/user/register", data, function(result) {
		if (result["success"] == 1) {
			location.href = "/profile";
		} else {
			display_message("register_msg", "danger", result["message"], function() {
				$(input).removeAttr("disabled");
			});
		}
	}, function(jqXHR, status, error) {
		var result = jqXHR["responseText"];
		display_message("register_msg", "danger", "Error " + jqXHR["status"] + ": " + result["message"], function() {
			$(input).removeAttr("disabled");
		});
	});
};

// password reset
var request_reset_form = function() {
	var data = $("#request_reset_form").serializeObject();
	$(input).attr("disabled", "disabled");
	api_call("POST", "/api/user/forgot", data, function(result) {
		if (result["success"] == 1) {
			display_message("reset_msg", "success", result["message"]);
		} else {
			display_message("reset_msg", "danger", result["message"], function() {
				$(input).removeAttr("disabled");
			});
		}
	}, function(jqXHR, status, error) {
		var result = jqXHR["responseText"];
		display_message("reset_msg", "danger", "Error " + jqXHR["status"] + ": " + result["message"], function() {
			$(input).removeAttr("disabled");
		});
	});
}

var reset_form = function() {
	var data = $("#reset_form").serializeObject();
	data["csrf_token"] = $.cookie("csrf_token");
	var url = window.location.href;
	var token = url.substr(url.lastIndexOf("/")+1);
	$(input).attr("disabled", "disabled");
	api_call("POST", "/api/user/forgot/" + token, data, function(result) {
		if (result["success"] == 1) {
			display_message("reset_msg", "success", result["message"], function() {
				location.href = "/login";
			});
		} else {
			display_message("reset_msg", "danger", result["message"], function() {
				$(input).removeAttr("disabled");
			});
		}
	}, function(jqXHR, status, error) {
		var result = jqXHR["responseText"];
		display_message("reset_msg", "danger", "Error " + jqXHR["status"] + ": " + result["message"], function() {
			$(input).removeAttr("disabled");
		});
	});
}

// login page

var login_form = function() {
	var input = "#login_form input";
	var data = $("#login_form").serializeObject();
	$(input).attr("disabled", "disabled");
	api_call("POST", "/api/user/login", data, function(result) {
		if (result["success"] == 1) {
			location.href = "/profile";
		} else {
			display_message("login_msg", "danger", result["message"], function() {
				$(input).removeAttr("disabled");
			});
		}
	}, function(jqXHR, status, error) {
		var result = jqXHR["responseText"];
		display_message("login_msg", "danger", "Error " + jqXHR["status"] + ": " + result["message"], function() {
			$(input).removeAttr("disabled");
		});
	});
};

// team page

var create_team = function() {
	var input = "#create_team input";
	var data = $("#create_team").serializeObject();
	$(input).attr("disabled", "disabled");
	api_call("POST", "/api/team/create", data, function(result) {
		if (result["success"] == 1) {
			location.reload(true);
		} else {
			display_message("create_team_msg", "danger", result["message"], function() {
				$(input).removeAttr("disabled");
			});
		}
	}, function(jqXHR, status, error) {
		var result = JSON.parse(jqXHR["responseText"]);
		display_message("create_team_msg", "danger", "Error " + jqXHR["status"] + ": " + result["message"], function() {
			$(input).removeAttr("disabled");
		});
	});
};

var add_member = function() {
	var input = "#add_member input";
	var data = $("#add_member").serializeObject();
	$(input).attr("disabled", "disabled");
	api_call("POST", "/api/team/invite", data, function(result) {
		if (result["success"] == 1) {
			location.reload(true);
		} else {
			$(input).removeAttr("disabled");
		}
	}, function(jqXHR, status, error) {
		var result = JSON.parse(jqXHR["responseText"]);
		display_message("create_team_msg", "danger", "Error " + jqXHR["status"] + ": " + result["message"], function() {
			$(input).removeAttr("disabled");
		});
	});
};

var rescind_invitation = function(uid) {
	var input = "#add_member input";
	var data = { "uid": uid };
	api_call("POST", "/api/team/invite/rescind", data, function(result) {
		if (result["success"] == 1) {
			location.reload(true);
		}
	});
};

var request_invitation = function(tid) {
	var input = "#add_member input";
	var data = { "tid": tid };
	api_call("POST", "/api/team/invite/request", data, function(result) {
		if (result["success"] == 1) {
			location.reload(true);
		}
	});
};

var accept_invitation = function(tid) {
	var data = { "tid": tid };
	api_call("POST", "/api/team/invite/accept", data, function(result) {
		if (result["success"] == 1) {
			location.reload(true);
		}
	});
};

var accept_invitation_request = function(uid) {
	var data = { "uid": uid };
	api_call("POST", "/api/team/invite/request/accept", data, function(result) {
		if (result["success"] == 1) {
			location.reload(true);
		}
	});
};

var twofactor_form = function() {
	var data = $("#twofactor_form").serializeObject();
	data["csrf_token"] = $.cookie("csrf_token");
	api_call("POST", "/api/user/twofactor/verify", data, function(result) {
		if (result["success"] == 1) {
			location.href = "/settings";
		}
	});
};

// settings page

var remove_profile_picture = function() {
	api_call("POST", "/api/user/avatar/remove", { }, function(result) {
		if (result["success"] == 1) {
			location.reload(true);
		}
	});
};