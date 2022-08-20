let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");
canvas.height = document.documentElement.clientHeight;
canvas.width = document.documentElement.clientWidth ;
function getHexColor(r, g, b, a) {
    let toHex = (n) => Number(n).toString(16).padStart(2, "0");
    return "#" + [r, g, b, a].map(v => toHex(v)).join("");
}

function fillCircle(x, y, radius, r, g, b, a) {
    ctx.fillStyle = getHexColor(r, g, b, a);
    ctx.beginPath();
    ctx.arc(x, y, radius, 0, 2 * Math.PI);
    ctx.fill();
}

function fillPixel(x, y, r, g, b, a) {

    ctx.fillStyle = getHexColor(r, g, b, a);
    ctx.fillRect(x, y, 1, 1);
}

(async () => {
    let response = fetch("voronoi_rust_wasm.wasm");
    let { instance } = await WebAssembly.instantiateStreaming(response, {
        "env": {
            "fillCircle": fillCircle,
            "fillPixel": fillPixel,
            "rand": Math.random,
            "alert": alert,
        }
    });
    let no_of_seeds = 15;

    let game = instance.exports.init(document.documentElement.clientWidth, document.documentElement.clientHeight, no_of_seeds);

    let i = 1;
    instance.exports.draw(game,i);
    window.setInterval(
        () => window.requestAnimationFrame(
            () => {
                i++;
                if(i > no_of_seeds){
                    i = 1;
                    instance.exports.reset(game);
                }
                instance.exports.draw(game, i);
            }
        ), 100);


})();

