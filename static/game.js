let ws = new WebSocket("ws://" + window.location.host + "/ev_g");

let currentOrder = 1;

ws.addEventListener("open", (event) => {
    console.log("opened");
    ws.send(JSON.stringify({
        ty: "register"
    }));
});

ws.addEventListener("close", (event) => {
    console.log("closed");
});

ws.addEventListener("message", (event) => {
    let m = JSON.parse(event.data);
    let rp = document.getElementById("total_players");
    switch (m.ty) {
        case "register_recieve": {
            let pr = rp.querySelector("#reg_" + m.name)
            if (pr != null) pr.remove()
            rp.innerHTML += "<p id=reg_" + m.name + ">" + m.name + "</p>"
            break;
        }
        case "remove_recieve": {
            let pr = rp.querySelector("#reg_" + m.name)
            if (pr != null) pr.remove()
            break;
        }
    }
    console.log(event);
});


function pressBuzzer() {
    console.log("pressing...");
    ws.send(JSON.stringify({
        ty: "buzz"
    }));
}