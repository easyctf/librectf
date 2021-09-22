var __extends = (this && this.__extends) || function (d, b) {
    for (var p in b) if (b.hasOwnProperty(p)) d[p] = b[p];
    function __() { this.constructor = d; }
    d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
};
function get_child_node_of_class(node, cls) {
    for (var i = 0; i < node.childNodes.length; i++) {
        if (node.childNodes[i].className == cls) {
            return node.childNodes[i];
        }
    }
}
var DialogueAction = (function () {
    function DialogueAction() {
    }
    return DialogueAction;
}());
var ShowCharacterAction = (function () {
    function ShowCharacterAction() {
    }
    return ShowCharacterAction;
}());
var HideCharacterAction = (function () {
    function HideCharacterAction() {
    }
    return HideCharacterAction;
}());
var ShowProblemAction = (function () {
    function ShowProblemAction() {
    }
    return ShowProblemAction;
}());
// TODO: actually deal with network errors
var ServerAPI = (function () {
    function ServerAPI(urls, csrf_token) {
        if (csrf_token === void 0) { csrf_token = null; }
        this.urls = urls;
        this.csrf_token = csrf_token;
    }
    ServerAPI.prototype.get = function (url, callback) {
        var r = new XMLHttpRequest();
        r.open("GET", url, true);
        r.onreadystatechange = function () {
            if (this.readyState == 4 && this.status == 200) {
                var obj = JSON.parse(this.responseText);
                callback(obj);
            }
        };
        r.send();
    };
    ServerAPI.prototype.post = function (url, data, callback) {
        data['csrf_token'] = this.csrf_token; // TODO: actually copy the dict
        var r = new XMLHttpRequest();
        r.open("POST", url, true);
        r.setRequestHeader("Content-type", "application/json");
        r.onreadystatechange = function () {
            if (this.readyState == 4 && this.status == 200) {
                var obj = JSON.parse(this.responseText);
                callback(obj);
            }
        };
        r.send(JSON.stringify(data));
    };
    ServerAPI.prototype.get_game = function (callback) {
        this.get(this.urls['game'], callback);
    };
    ServerAPI.prototype.get_problems = function (callback) {
        this.get(this.urls['problems'], callback);
    };
    ServerAPI.prototype.get_state = function (callback) {
        this.get(this.urls['state'], callback);
    };
    ServerAPI.prototype.submit_flag = function (pid, flag, callback) {
        this.post(this.urls['submit'], { 'pid': pid, 'flag': flag }, function (obj) {
            callback(obj['result'], obj['message']);
        });
    };
    ServerAPI.prototype.upload_game_state = function (state, callback) {
        if (callback === void 0) { callback = null; }
        this.post(this.urls['game_state_update'], { 'state': JSON.stringify(state) }, function (obj) {
            if (callback !== null) {
                callback(true);
            }
        });
    };
    return ServerAPI;
}());
var DebugServerAPI = (function (_super) {
    __extends(DebugServerAPI, _super);
    function DebugServerAPI(urls) {
        _super.call(this, urls);
        this.solved_problems = [];
        this.game_state = '{}';
    }
    DebugServerAPI.prototype.submit_flag = function (pid, flag, callback) {
        this.solved_problems.push(pid);
        callback('success', 'gj');
    };
    DebugServerAPI.prototype.upload_game_state = function (state, callback) {
        this.game_state = JSON.stringify(state);
        if (callback !== null) {
            callback(true);
        }
    };
    DebugServerAPI.prototype.get_problems = function (callback) {
        var _this = this;
        _super.prototype.get_problems.call(this, function (problems_obj) {
            for (var problem in problems_obj) {
                if (_this.solved_problems.indexOf(parseInt(problem)) !== -1) {
                    problems_obj[problem]['solved'] = 1;
                }
            }
            callback(problems_obj);
        });
    };
    DebugServerAPI.prototype.get_state = function (callback) {
        callback(JSON.parse(this.game_state));
    };
    return DebugServerAPI;
}(ServerAPI));
var Master = (function () {
    function Master(server_api, debug) {
        if (debug === void 0) { debug = false; }
        this.element = document.getElementById('vn');
        this.scenes = {};
        this.current_scene = null;
        this.current_scene_name = null;
        this.current_scene_type = null;
        this.route = null;
        this.problem_triggers = {};
        this.routes = {};
        this.layout = new Layout();
        this.game_interface = new GameInterface(this.layout);
        this.problems = {};
        this.solved_problems = [];
        this.viewed_scenes = [];
        this.completed_routes = [];
        this.server_api = server_api;
        this.debug = debug;
    }
    Master.prototype.from_json = function (obj) {
        this.images = obj['images'] || {};
        this.layout.problem_list_widget.category_icons = obj['category_icons'] || {};
        this.scenes_from_json(obj['scenes'] || {});
        this.problem_triggers = obj['problem_triggers'] || {};
        this.routes = obj['routes'] || {};
    };
    Master.prototype.scenes_from_json = function (obj) {
        for (var key in obj) {
            var scene_data = obj[key];
            var scene = new Scene(this.layout, this);
            scene.id = key;
            scene.images = this.images;
            scene.background_image = scene_data['background_image'] || null;
            scene.choreography = scene_data['choreography'];
            scene.trigger_spec = scene_data['on'];
            this.scenes[key] = scene;
        }
    };
    // NEEDS TRIGGERS FIRST
    Master.prototype.problems_from_json = function (obj) {
        this.problems = {};
        this.solved_problems = [];
        for (var key in obj) {
            var problem = obj[key];
            if (this.problem_triggers[key] !== undefined && 'route' in this.problem_triggers[key]) {
                problem.is_route = true;
            }
            this.problems[problem.pid] = problem;
            if (obj[key]['solved']) {
                this.solved_problems.push(problem.pid);
            }
        }
    };
    Master.prototype.load_resources = function (callback) {
        var _this = this;
        this.server_api.get_game(function (obj) {
            _this.from_json(obj);
            var images_to_load = [];
            for (var image in _this.images) {
                images_to_load.push(_this.images[image]);
            }
            var progress_counter = new ProgressCounter(images_to_load.length);
            progress_counter.progress_callback = function (current, total) {
                document.getElementById('loading_display').innerHTML = 'Loading... ' + current + '/' + total;
            };
            progress_counter.done_callback = callback;
            for (var _i = 0, images_to_load_1 = images_to_load; _i < images_to_load_1.length; _i++) {
                var image_to_load = images_to_load_1[_i];
                var temp_img = new Image();
                // TODO: handle error
                temp_img.onload = function (e) {
                    progress_counter.advance();
                };
                temp_img.src = image_to_load;
            }
        });
    };
    Master.prototype.fetch_problems = function (callback) {
        var _this = this;
        this.server_api.get_problems(function (obj) {
            _this.problems_from_json(obj);
            callback();
        });
    };
    Master.prototype.update_problems_list = function () {
        var unsolved_problems = [];
        var solved_problems = [];
        for (var problem in this.problems) {
            if (!this.is_triggered(this.problem_triggers[problem])) {
                continue;
            }
            if (this.solved_problems.indexOf(parseInt(problem)) != -1) {
                solved_problems.push(this.problems[problem]);
            }
            else {
                unsolved_problems.push(this.problems[problem]);
            }
        }
        this.game_interface.set_problems(solved_problems, unsolved_problems);
    };
    Master.prototype.refresh_problems = function (network, callback) {
        var _this = this;
        if (network === void 0) { network = true; }
        var after = function () {
            if (_this.current_scene_type !== "dialogue") {
                _this.update_problems_list();
            }
            callback();
        };
        if (network) {
            this.fetch_problems(after);
        }
        else {
            after();
        }
    };
    Master.prototype.fetch_game_state = function (callback) {
        var _this = this;
        this.server_api.get_state(function (obj) {
            _this.viewed_scenes = (obj['scenes'] || []);
            _this.route = obj['route'] || null;
            _this.completed_routes = (obj['completed_routes'] || []);
            callback();
        });
    };
    Master.prototype.generate_game_state = function () {
        return {
            'scenes': this.viewed_scenes,
            'route': this.route,
            'completed_routes': this.completed_routes
        };
    };
    Master.prototype.upload_game_state = function (callback) {
        if (callback === void 0) { callback = null; }
        this.server_api.upload_game_state(this.generate_game_state(), callback);
    };
    Master.prototype.refresh = function (callback, network) {
        var _this = this;
        if (network === void 0) { network = true; }
        var after = function () {
            _this.refresh_problems(network, function () {
                if (_this.check_for_route_trigger()) {
                    _this.refresh(callback);
                    return;
                }
                if (_this.check_for_route_completion()) {
                    _this.refresh(callback);
                    return;
                }
                callback();
            });
        };
        if (network) {
            this.fetch_game_state(after);
        }
        else {
            after();
        }
    };
    Master.prototype.set_scene = function (scene_name, scene_type) {
        this.current_scene_name = scene_name;
        this.current_scene = this.scenes[scene_name];
        this.current_scene_type = scene_type;
    };
    Master.prototype.start_problem_scene = function (scene_name) {
        var _this = this;
        if (this.is_dialogue_scene_active()) {
            return;
        }
        this.clear_scene();
        this.layout.character_elements[0].style.opacity = '0.7';
        this.set_scene(scene_name, "problem");
        this.current_scene.start(function () {
            // TODO: move this to be right after last dialogue of problem scene finishes
            _this.layout.dialogue_widget.dialogue_next_arrow_element.style.visibility = 'hidden';
        });
    };
    Master.prototype.start_dialogue_scene = function (scene_name) {
        var _this = this;
        if (this.is_dialogue_scene_active()) {
            return;
        }
        this.clear_scene();
        this.game_interface.disable();
        this.layout.problem_display_widget.hide();
        this.layout.background_element.style.opacity = '0.5';
        this.set_scene(scene_name, "dialogue");
        var current_scene = this.current_scene;
        this.current_scene.start(function () {
            _this.scene_finished(current_scene);
        });
    };
    Master.prototype.scene_finished = function (scene) {
        var _this = this;
        this.viewed_scenes.push(scene.id);
        this.clear_scene();
        this.upload_game_state(function () {
            _this.refresh(function () {
                if (!_this.check_for_scene_trigger()) {
                    _this.game_interface.reset_display();
                }
            });
        });
    };
    Master.prototype.clear_scene = function () {
        this.layout.reset_scene_elements();
        this.current_scene = null;
        this.current_scene_name = null;
        this.current_scene_type = null;
        this.game_interface.reset_display();
    };
    Master.prototype.is_scene_active = function () {
        return this.current_scene !== null;
    };
    Master.prototype.is_problem_scene_active = function () {
        return this.current_scene_type === "problem";
    };
    Master.prototype.is_dialogue_scene_active = function () {
        return this.current_scene_type === "dialogue";
    };
    Master.prototype.check_for_scene_trigger = function () {
        for (var scene in this.scenes) {
            if (this.viewed_scenes.indexOf(scene) != -1)
                continue;
            if (this.is_triggered(this.scenes[scene].trigger_spec)) {
                // handle if is dialogue or not
                this.start_dialogue_scene(scene);
                return true;
            }
        }
        return false;
    };
    Master.prototype.check_for_problem_scene = function (problem_id) {
        for (var scene in this.scenes) {
            if (this.is_triggered_for_problem(this.scenes[scene].trigger_spec, problem_id)) {
                this.start_problem_scene(scene);
                return true;
            }
        }
        return false;
    };
    Master.prototype.check_for_route_trigger = function () {
        if (this.route !== null) {
            return false;
        }
        for (var route in this.routes) {
            if (this.completed_routes.indexOf(parseInt(route)) !== -1) {
                continue;
            }
            if (this.is_triggered(this.routes[route]['trigger_spec'])) {
                this.route = parseInt(route);
                this.upload_game_state();
                return true;
            }
        }
        return false;
    };
    Master.prototype.check_for_route_completion = function () {
        if (this.route === null) {
            return false;
        }
        if (this.is_triggered(this.routes[this.route]['release_spec'])) {
            this.completed_routes.push(this.route);
            this.route = null;
            this.upload_game_state();
            return true;
        }
        return false;
    };
    // passive triggering
    // all trigger keys are ANDed
    Master.prototype.is_triggered = function (trigger) {
        var _this = this;
        if (trigger === undefined)
            return false;
        var trigger_handlers = {
            'after_scenes': function (scenes) {
                for (var _i = 0, scenes_1 = scenes; _i < scenes_1.length; _i++) {
                    var scene = scenes_1[_i];
                    if (_this.viewed_scenes.indexOf(scene) === -1) {
                        return false;
                    }
                }
                return true;
            },
            'problems_solved': function (spec) {
                var solved = 0;
                var problems = spec['problems'] || [];
                var categories = spec['categories'] || [];
                for (var _i = 0, _a = _this.solved_problems; _i < _a.length; _i++) {
                    var pid = _a[_i];
                    if (!_this.is_triggered(_this.problem_triggers[pid])) {
                        continue;
                    }
                    if (problems.indexOf(pid) !== -1 || categories.indexOf(_this.problems[pid].category) !== -1) {
                        solved++;
                    }
                }
                return solved >= spec['thresh'];
            },
            'return': function (result) {
                return result;
            },
            'route_active': function (route) {
                return _this.route === route;
            },
            'route': function (route) {
                return _this.route === route || _this.completed_routes.indexOf(route) !== -1;
            }
        };
        var result = true;
        var processed_triggers = 0;
        for (var trigger_key in trigger_handlers) {
            if (trigger[trigger_key] !== undefined) {
                if (!trigger_handlers[trigger_key](trigger[trigger_key])) {
                    result = false;
                }
                processed_triggers++;
            }
        }
        return processed_triggers > 0 && result;
    };
    Master.prototype.is_triggered_for_problem = function (trigger, problem_id) {
        return trigger !== undefined && trigger['problem'] == problem_id;
    };
    Master.prototype.init = function () {
        var _this = this;
        this.register_event_listener();
        this.game_interface.flag_submitted_callback = function (problem, flag) {
            _this.server_api.submit_flag(problem.pid, flag, function (result, message) {
                if (result === 'success') {
                    _this.layout.problem_display_widget.set_submit_response_color('#0f0');
                }
                else if (result === 'error') {
                    _this.layout.problem_display_widget.set_submit_response_color('#ff0');
                }
                else if (result === 'failure') {
                    _this.layout.problem_display_widget.set_submit_response_color('#f00');
                }
                setTimeout(function () {
                    _this.layout.problem_display_widget.clear_submit_response_color();
                    if (result === "success" && _this.layout.problem_display_widget.get_current_pid() === problem.pid) {
                        _this.layout.problem_display_widget.set_problem(null);
                    }
                }, 2000);
                _this.refresh(function () {
                    _this.check_for_scene_trigger();
                });
            });
        };
    };
    Master.prototype.start = function () {
        var _this = this;
        this.load_resources(function () {
            _this.refresh(function () {
                document.getElementById('loading_screen').style.display = "none";
                document.getElementById('vn_scene').style.display = "initial";
                _this.layout.reset_scene_elements();
                _this.game_interface.disable();
                _this.game_interface.images = _this.images;
                _this.game_interface.problem_selected_callback = function (problem) {
                    _this.clear_scene();
                    _this.layout.problem_display_widget.clear_submit_response_color();
                    _this.check_for_problem_scene(problem.pid);
                };
                if (!_this.check_for_scene_trigger()) {
                    _this.game_interface.reset_display();
                }
                /*this.current_scene.start(() => {
                 this.scene_finished(this.current_scene)
                 });*/
            });
        });
    };
    Master.prototype.keydown = function (event) {
        var handled = false;
        if (this.current_scene !== null) {
            if (this.current_scene.keydown(event)) {
                handled = true;
            }
        }
        if (handled) {
            event.preventDefault();
            event.stopPropagation();
        }
    };
    Master.prototype.register_event_listener = function () {
        var _this = this;
        document.addEventListener('keydown', function (event) {
            _this.keydown(event);
        }, false);
    };
    return Master;
}());
var ProgressCounter = (function () {
    function ProgressCounter(total) {
        this.done = false;
        this.current = 0;
        this.total = total;
    }
    ProgressCounter.prototype.log = function () {
        console.log(this.current + '/' + this.total);
    };
    ProgressCounter.prototype.advance = function () {
        this.current++;
        this.progress_callback(this.current, this.total);
        if (this.current >= this.total) {
            this.finish();
        }
    };
    ProgressCounter.prototype.finish = function () {
        this.done = true;
        if (this.done_callback !== undefined) {
            this.done_callback();
        }
    };
    return ProgressCounter;
}());
var Layout = (function () {
    function Layout() {
        this.element = document.getElementById('vn_scene');
        this.background_element = document.getElementById('scene_background');
        this.character_elements = [
            document.getElementById('character_1'),
            document.getElementById('character_2'),
            document.getElementById('character_3'),
        ];
        this.speaker_element = document.getElementById('speaker_display_inner');
        this.dialogue_widget = new TextWidget(function () {
        });
        this.problem_display_widget = new ProblemDisplayWidget();
        this.problem_list_widget = new ProblemListWidget(this.problem_display_widget);
    }
    Layout.prototype.reset_scene_elements = function () {
        for (var _i = 0, _a = this.character_elements; _i < _a.length; _i++) {
            var element = _a[_i];
            element.style.display = 'none';
            element.style.opacity = '1.0';
        }
        //this.background_element.style.backgroundImage = 'url("' + this.images[this.background_image] + '")';
        this.background_element.style.opacity = '1.0';
        this.dialogue_widget.setText('');
        this.dialogue_widget.finish_now();
        this.speaker_element.parentElement.style.display = 'none';
        this.dialogue_widget.dialogue_next_arrow_element.style.visibility = 'hidden';
    };
    Layout.prototype.set_background = function (image_url) {
        this.background_element.style.backgroundImage = 'url("' + image_url + '")';
    };
    return Layout;
}());
var GameInterface = (function () {
    function GameInterface(layout) {
        this.background_image = 'game_interface_bg';
        this.layout = layout;
        this.solved_problems = [];
        this.unsolved_problems = [];
    }
    GameInterface.prototype.repopulate = function () {
        this.layout.problem_list_widget.populate(this.unsolved_problems);
    };
    GameInterface.prototype.set_problems = function (solved, unsolved) {
        this.solved_problems = solved;
        this.unsolved_problems = unsolved;
        this.repopulate();
    };
    GameInterface.prototype.reset_background = function () {
        this.layout.set_background(this.images[this.background_image]);
    };
    GameInterface.prototype.reset_display = function () {
        this.enable();
        this.show();
        this.repopulate();
        this.reset_background();
        // reset problem view too
    };
    // TODO: actually show/hide things
    GameInterface.prototype.show = function () {
        this.layout.problem_list_widget.show();
        this.layout.problem_display_widget.show();
    };
    GameInterface.prototype.hide = function () {
        this.layout.problem_list_widget.hide();
        this.layout.problem_display_widget.hide();
    };
    GameInterface.prototype.enable = function () {
        this.layout.problem_list_widget.enable();
        this.layout.problem_display_widget.enable();
    };
    GameInterface.prototype.disable = function () {
        this.layout.problem_list_widget.disable();
        this.layout.problem_display_widget.disable();
    };
    Object.defineProperty(GameInterface.prototype, "problem_selected_callback", {
        set: function (callback) {
            this.layout.problem_list_widget.problem_selected_callback = callback;
        },
        enumerable: true,
        configurable: true
    });
    Object.defineProperty(GameInterface.prototype, "flag_submitted_callback", {
        set: function (callback) {
            this.layout.problem_display_widget.flag_submitted_callback = callback;
        },
        enumerable: true,
        configurable: true
    });
    return GameInterface;
}());
var Scene = (function () {
    function Scene(layout, master) {
        this.started = false;
        this.done = false;
        this.pos = 0;
        this.layout = layout;
        this.choreography = [];
        this.background_image = null;
        this.master = master;
    }
    Scene.prototype.keydown = function (event) {
        if (!this.started)
            return false;
        switch (event.keyCode) {
            case 32:
            case 39:
            case 40:
                if (!this.layout.dialogue_widget.done) {
                    this.layout.dialogue_widget.finish_now();
                }
                else {
                    this.nextAction();
                }
                return true;
            default:
                return false;
        }
    };
    Scene.prototype.nextAction = function () {
        if (this.pos == this.choreography.length) {
            this.finish();
            return;
        }
        var current_action = this.choreography[this.pos];
        this.pos++;
        if (current_action.action == 'dialogue') {
            var action = current_action;
            this.layout.speaker_element.innerHTML = action.speaker;
            if (action.speaker === null) {
                this.layout.speaker_element.parentElement.style.display = 'none';
            }
            else {
                this.layout.speaker_element.parentElement.style.display = 'initial';
            }
            this.layout.dialogue_widget.setText(action.text);
            this.layout.dialogue_widget.start(30);
        }
        else if (current_action.action == 'show_character') {
            var action = current_action;
            this.layout.character_elements[action.position - 1].style.backgroundImage = 'url("' + this.images[action.sprite] + '")';
            this.layout.character_elements[action.position - 1].style.display = 'initial';
            this.nextAction();
        }
        else if (current_action.action == 'hide_character') {
            var action = current_action;
            this.layout.character_elements[action.position - 1].style.display = 'none';
            this.nextAction();
        }
        else if (current_action.action == 'show_problem') {
            var action = current_action;
            this.layout.problem_list_widget.add_highlighted_problem(this.master.problems[action.problem_id]);
            this.layout.problem_display_widget.set_problem(this.master.problems[action.problem_id]);
            this.nextAction();
        }
        else {
            this.nextAction();
        }
    };
    Scene.prototype.finish = function () {
        this.done = true;
        if (this.done_callback !== undefined) {
            this.done_callback();
        }
    };
    Scene.prototype.reset_display = function () {
        for (var _i = 0, _a = this.layout.character_elements; _i < _a.length; _i++) {
            var element = _a[_i];
            element.style.display = 'none';
        }
        if (this.background_image !== null) {
            this.layout.background_element.style.backgroundImage = 'url("' + this.images[this.background_image] + '")';
        }
        this.layout.dialogue_widget.setText('');
        this.layout.dialogue_widget.finish_now();
        this.layout.speaker_element.parentElement.style.display = 'none';
        //this.speaker_element.innerHTML = '';
    };
    Scene.prototype.reset = function () {
        this.done = false;
        this.pos = 0;
    };
    Scene.prototype.start = function (done_callback) {
        if (done_callback === void 0) { done_callback = undefined; }
        this.done_callback = done_callback;
        this.reset();
        this.started = true;
        this.reset_display();
        this.nextAction();
    };
    return Scene;
}());
var TextWidget = (function () {
    function TextWidget(finished_callback) {
        if (finished_callback === void 0) { finished_callback = null; }
        this.done = false;
        this.started = false;
        this.pos = 0;
        this.lastTick = -1;
        this.element = document.getElementById('dialogue_box_inner');
        this.dialogue_next_arrow_element = document.getElementById('dialogue_next_arrow');
        this.finished_callback = finished_callback;
    }
    TextWidget.prototype.setText = function (text) {
        this.text = text;
        this.pos = 0;
    };
    TextWidget.prototype.start = function (tick_ms) {
        var _this = this;
        if (this.pos == this.text.length) {
            this.finish();
        }
        else {
            this.started = true;
            this.done = false;
            this.lastTick = performance.now();
            this.dialogue_next_arrow_element.style.visibility = 'hidden';
            window.requestAnimationFrame(function (now) { return _this.tick(now, tick_ms); });
        }
    };
    TextWidget.prototype.finish = function () {
        this.done = true;
        this.dialogue_next_arrow_element.style.visibility = 'inherit';
        if (this.finished_callback !== null) {
            this.finished_callback();
        }
    };
    TextWidget.prototype.finish_now = function () {
        this.finish();
        this.pos = this.text.length;
        this.element.innerHTML = this.text;
    };
    // TODO: longer wait times on punctuation
    TextWidget.prototype.tick = function (now, tick_ms) {
        var _this = this;
        if ((now - this.lastTick) / tick_ms >= 1) {
            this.pos = Math.min(this.pos + Math.floor((now - this.lastTick) / tick_ms), this.text.length);
            this.element.innerHTML = this.text.substring(0, this.pos);
            this.lastTick = now;
        }
        if (this.pos == this.text.length) {
            if (!this.done) {
                this.finish();
            }
        }
        else {
            window.requestAnimationFrame(function (now) { return _this.tick(now, tick_ms); });
        }
    };
    return TextWidget;
}());
var Problem = (function () {
    function Problem() {
        this.is_route = false;
    }
    return Problem;
}());
var ProblemListWidget = (function () {
    function ProblemListWidget(problem_display_widget) {
        this.element = document.getElementById('problem_list_box_inner');
        this.problem_list_element = document.getElementById('problem_list');
        this.problem_list_route_element = document.getElementById('problem_list_route');
        this.problem_list_normal_element = document.getElementById('problem_list_normal');
        this.template_entry_element = document.getElementById('problem_entry_template');
        this.problem_hover_label_element = document.getElementById('problem_hover_label');
        this.problem_display_widget = problem_display_widget;
        this.problems = [];
        this.highlighted = [];
        this.enabled = true;
    }
    Object.defineProperty(ProblemListWidget.prototype, "problem_selected_callback", {
        set: function (value) {
            this._problem_selected_callback = value;
        },
        enumerable: true,
        configurable: true
    });
    ProblemListWidget.prototype.show = function () {
        this.element.style.visibility = 'visible';
    };
    ProblemListWidget.prototype.hide = function () {
        this.element.style.visibility = 'hidden';
    };
    ProblemListWidget.prototype.enable = function () {
        this.enabled = true;
    };
    ProblemListWidget.prototype.disable = function () {
        this.enabled = false;
    };
    ProblemListWidget.prototype.populate = function (problems, highlghted) {
        if (highlghted === void 0) { highlghted = null; }
        if (highlghted === null) {
            highlghted = [];
        }
        this.problems = problems;
        this.highlighted = highlghted;
        this.repopulate();
    };
    ProblemListWidget.prototype.repopulate = function () {
        var _this = this;
        this.problems.sort(function (p1, p2) {
            return p1.value - p2.value;
        });
        this.problem_list_route_element.innerHTML = '';
        this.problem_list_normal_element.innerHTML = '';
        this.problem_list_route_element.style.display = 'none';
        var _loop_1 = function(problem) {
            var problem_element = this_1.template_entry_element.cloneNode(true);
            problem_element.style.visibility = 'inherit';
            problem_element.style.backgroundColor = 'blue';
            if (this_1.highlighted.indexOf(problem.pid) !== -1) {
                problem_element.style.border = '2px yellow solid';
            }
            //problem_element.style.backgroundImage = 'url(' + this.category_icons[problem.category] + ')';
            var point_value_label = get_child_node_of_class(problem_element, 'point_value_label');
            point_value_label.innerText = problem.value.toString();
            problem_element.addEventListener('mouseenter', function (e) {
                // TODO: make this disappear when mouse leaves
                _this.problem_hover_label_element.innerText = problem.title + ' - ' + problem.category + ' ' + problem.value;
            });
            problem_element.addEventListener('mouseleave', function (e) {
                // TODO: make this disappear when mouse leaves
                _this.problem_hover_label_element.innerText = '';
            });
            problem_element.addEventListener('click', function (e) {
                if (_this.enabled) {
                    _this.problem_display_widget.set_problem(problem);
                    _this._problem_selected_callback(problem);
                }
            });
            if (problem.is_route) {
                this_1.problem_list_route_element.appendChild(problem_element);
                this_1.problem_list_route_element.style.display = 'block';
            }
            else {
                this_1.problem_list_normal_element.appendChild(problem_element);
            }
        };
        var this_1 = this;
        for (var _i = 0, _a = this.problems; _i < _a.length; _i++) {
            var problem = _a[_i];
            _loop_1(problem);
        }
    };
    ProblemListWidget.prototype.add_highlighted_problem = function (problem) {
        this.highlighted.push(problem.pid);
        this.problems.push(problem);
        this.repopulate();
    };
    return ProblemListWidget;
}());
var ProblemDisplayWidget = (function () {
    function ProblemDisplayWidget() {
        var _this = this;
        this.problem = null;
        this._flag_submitted_callback = null;
        this.element = document.getElementById('problem_box_inner');
        this.title_element = document.getElementById('problem_title');
        this.body_element = document.getElementById('problem_body');
        this.flag_box_element = document.getElementById('flag_box');
        this.flag_input_box_element = document.getElementById('flag_input_box');
        this.input_element = document.getElementById('flag_input');
        this.submit_element = document.getElementById('flag_submit_button');
        this.programming_button_element = document.getElementById('flag_programming_button');
        this.programming_button_form_element = document.getElementById('flag_programming_form_button');
        this.input_element.addEventListener('keydown', function (e) {
            if (e.keyCode === 13) {
                _this.handle_submit();
            }
        });
        this.submit_element.addEventListener('click', function (e) {
            _this.handle_submit();
        });
        this.hide();
    }
    ProblemDisplayWidget.prototype.set_problem = function (problem) {
        this.problem = problem;
        this.input_element.value = '';
        if (problem === null) {
            this.hide();
            return;
        }
        // TODO: prettier display
        this.title_element.innerHTML = problem.title + ' - ' + problem.category + ' ' + problem.value;
        this.body_element.innerHTML = problem.description;
        if (this.problem.programming) {
            this.programming_button_form_element.action = '/chals/programming/' + problem.pid;
            this.programming_button_element.style.visibility = 'inherit';
            this.flag_input_box_element.style.visibility = 'hidden';
        }
        else {
            this.programming_button_element.style.visibility = 'hidden';
            this.flag_input_box_element.style.visibility = 'inherit';
        }
        this.show();
    };
    ProblemDisplayWidget.prototype.get_current_pid = function () {
        if (this.problem === null) {
            return null;
        }
        else {
            return this.problem.pid;
        }
    };
    ProblemDisplayWidget.prototype.show = function () {
        if (this.problem === null) {
            return;
        }
        this.element.style.visibility = 'visible';
    };
    ProblemDisplayWidget.prototype.hide = function () {
        this.element.style.visibility = 'hidden';
    };
    ProblemDisplayWidget.prototype.enable = function () {
        this.input_element.disabled = false;
        this.submit_element.disabled = false;
        this.programming_button_element.disabled = false;
    };
    ProblemDisplayWidget.prototype.disable = function () {
        this.input_element.disabled = true;
        this.submit_element.disabled = true;
        this.programming_button_element.disabled = true;
    };
    Object.defineProperty(ProblemDisplayWidget.prototype, "flag_submitted_callback", {
        set: function (callback) {
            this._flag_submitted_callback = callback;
        },
        enumerable: true,
        configurable: true
    });
    ProblemDisplayWidget.prototype.handle_submit = function () {
        if (this._flag_submitted_callback !== null) {
            this._flag_submitted_callback(this.problem, this.input_element.value);
        }
    };
    ProblemDisplayWidget.prototype.set_submit_response_color = function (color) {
        this.input_element.style.borderColor = color;
        this.submit_element.style.borderColor = color;
    };
    ProblemDisplayWidget.prototype.clear_submit_response_color = function () {
        this.input_element.style.borderColor = '#fff';
        this.submit_element.style.borderColor = '#fff';
    };
    return ProblemDisplayWidget;
}());
