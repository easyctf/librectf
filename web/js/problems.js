function render_problems() {
    $.post("/api/problem/data", {
    }, function(data) {
        data = data["data"];
        for (var i = 0; i < data.length; i++) {
            files = data[i]["files"];
            problem =
`<div class="panel panel-info">
    <div class="panel-heading">
        <h3 class="panel-title">` + data[i]["name"] + ` | ` + data[i]["category"] + `<span style="float: right">` + data[i]["value"] + ` points</span></h3>
    </div>
    <div class="panel-body">
       <p>` + data[i]["description"]  + `</p>
        <div class="input-group">
            <input type="text" class="form-control" placeholder="Flag">
            <span class="input-group-btn">
                <button class="btn btn-success" id="hint" type="button" onclick="show_hint(\'` + data[i]["pid"] + `\');">Hint</button>
                <button class="btn btn-success" type="button">Submit!</button>
            </span>
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
</div></div>`
            $("#problems").append(problem);
        }
    });
}

function show_hint(pid) {
    $("#hint_" + pid).slideToggle(120, "swing");
}

$(document).ready( render_problems() );
