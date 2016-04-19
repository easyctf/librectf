function render_problems() {
    $.post("/api/admin/problem/data", {
    }, function(data) {
        data = data["data"];
        for (var i = 0; i < data.length; i++) {
            files = data[i]["files"];
            var checked = "";
            if (data[i]["disabled"]) {
                checked = "checked";
            }
            problem =
`<div class="panel panel-info">
    <form method="POST" onsubmit="return false;">
        <input type="hidden" name="pid" value="` + data[i]["pid"] + `">
        <div class="panel-heading">
            <div class="row">
            <div class="col-md-6">
                <input type="text" name="name" placeholder="Name" autocomplete="on" class="form-control" value="` + data[i]["name"] + `">
            </div>
            <div class="col-md-6">
                <input type="text" name="category" placeholder="Category" autocomplete="on" class="form-control" value="` + data[i]["category"] + `">
            </div>
            </div>
        </div>
        <div class="panel-body">
            <textarea type="text" name="description" placeholder="Description" autocomplete="on" class="form-control">` + data[i]["description"] + `</textarea>
            <br><br>
            <div class="row">
                <div class="col-md-6">
                <input type="text" name="flag" placeholder="Flag" autocomplete="off" class="form-control" value="` + data[i]["flag"] + `">
                </div>
                <div class="col-md-6">
                <input type="text" name="hint" placeholder="Hint" autocomplete="off" class="form-control" value="` + data[i]["hint"] + `">
                </div>
            </div>
            <br>
            <div class="row">
                <input type="number" name="value" placeholder="Value" autocomplete="off" class="form-control-number" value="` + data[i]["value"] + `">
        <label><input type="checkbox" name="disabled" value="1"` + checked + `>Disabled</label>
            </div>
        </div>
        <div class="panel-footer">`

            for (var j = 0; j < files.length; j++) {
                file_name = files[j].split("/").pop();
                problem +=
`<a href="` + files[j] + `" class="filelink" target="_blank">
    <h4 class="probfile">` + file_name + `</h4>
</a>`
            }

            problem += `<br>
        <div id="hint_` + data[i]["pid"] + `" style="display:none">` + data[i]["hint"] + `</div>
<div class="row" id="status_` + data[i]["pid"] + `"></div><br>
<input class="btn btn-success" type="submit" name="update" value="Update">
<input class="btn btn-danger" name="delete-modal" type="button" data-toggle="modal" data-target="#delete-modal" value="Delete">
</div></form></div>`
            $("#problems").append(problem);
        }
        $("[name=update]").click(function(e) {
            var problem = $(this).parents("form:first");
            var pid = $("input[name=pid]", problem).val();
            var name = $("input[name=name]", problem).val();
            var description = $("textarea[name=description]", problem).val();
            var hint = $("input[name=hint]", problem).val();
            var category = $("input[name=category]", problem).val();
            var value = $("input[name=value]", problem).val();
            var flag = $("input[name=flag]", problem).val();
            var disabled = $("input[name=disabled]", problem).prop("checked") ? 1 : 0;
            update_problem(pid, name, category, description, hint, flag, disabled, value);
        });
        $("[name=delete-modal]").click(function(e) {
            var problem = $(this).parents("form:first");
            var pid = $("input[name=pid]", problem).val();
            var div = $(this).closest("div.panel");
            $("#delete").off().click(function(e) {
                delete_problem(pid, div);
            });
        });
    });
}

function update_problem(pid, name, category, description, hint, flag, disabled, value) {
    $.post("/api/problem/update", {
        pid: pid,
        name: name,
        category: category,
        description: description,
        hint: hint,
        flag: flag,
        disabled: disabled,
        value: value
    }, function(data) {
        if (data.success == 1) {
            display_message("status_" + pid, "success", data.message, function() {});
        } else {
            display_message("status_" + pid, "danger", data.message, function() {});
        }
    });
}

function delete_problem(pid, div) {
    $.post("/api/problem/delete", {
        pid: pid
    }, function(data) {
        if (data.success == 1) {
            display_message("delete_status", "success", data.message, function() {
                div.slideUp("normal", function() {
                    $(this).remove();
                    $("#delete-modal").modal("hide");
                } );
            });
        } else {
            display_message("delete_status", "warning", data.message, function() {});
        }
    });
}

$(function() {
    render_problems();
});
