let ws = new WebSocket("ws://" + window.location.host +"/ev_gl")

let currentOrder = 1;

ws.addEventListener("open", (event) => {
    console.log("opened");
    ws.send(JSON.stringify({
        ty: "initialize"
    }));
});

ws.addEventListener("close", (event) => {
    console.log("closed");
})

ws.addEventListener("message", (event) => {
    let m = JSON.parse(event.data);
    let rp = document.getElementById("total_players");
    let bp = document.getElementById("buzzed_players");
    switch (m.ty) {
        case "buzz_recieve": {
            bp.innerHTML += "<p id=buzz_" + m.name + ">" + currentOrder + ": " + m.name + "</p>"
            currentOrder += 1
            break;
        }
        case "register_recieve": {
            let pr = rp.querySelector("#reg_" + m.name)
            let br = bp.querySelector("#buzz_" + m.name)
            if (pr != null) pr.remove()
            if (br != null) br.remove()
            rp.innerHTML += "<p id=reg_" + m.name + ">" + m.name + "</p>"
            break;
        }
        case "remove_recieve": {
            let pr = rp.querySelector("#reg_" + m.name)
            let br = bp.querySelector("#buzz_" + m.name)
            if (pr != null) pr.remove()
            if (br != null) br.remove()
            break;
        }
    }
    console.log(event);
});

function toggleBuzzers() {
    console.log("toggling buzzer...")
    let bp = document.getElementById("buzzed_players");
    currentOrder = 1;
    bp.innerHTML = "";
    ws.send(JSON.stringify({
        ty: "toggle_buzzer"
    }));
}