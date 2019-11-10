// remove padding from sides of site, to make room for canvas
document.body.style.margin = "0px";
document.body.style.padding = "0px";
document.body.style.overflow = "hidden";
document.body.innerHTML += '<canvas id="canv"></canvas>'; 

const xSize = 350;
const ySize = 350;

var c = document.getElementById("canv");
c.style.width = xSize;
c.style.height = ySize;
c.width = window.innerWidth;
c.height = window.innerHeight;

var ctx = c.getContext("2d");

ctx.font = "15px Arial";

ctx.scale(window.innerWidth/xSize, window.innerHeight/ySize);

let imgs = {
	"Heart": "Heart.png",
	"Eye": "Eye.jpg",
	"Flower3": "Flower3.png",
	"Background": "StageBackground.png",
	"Little Doll": "doll.png"
};

for (let img in imgs) {
	let new_img = new Image();
	new_img.src = "./img/" + imgs[img];
	new_img.onload = () => console.log('hi');
	imgs[img] = new_img;
}

function render({ents}) {
	ctx.fillStyle = "white";
	ctx.globalAlpha = 0.45;
	ctx.drawImage(
		imgs.Background,
		0,
		0,
		350,
		350,
	);
	ctx.fillRect(0, 0, xSize, ySize);

	ctx.save();
	ctx.translate(xSize/2, xSize/2);
	const screenRot = 0.010;
	ctx.rotate(Math.random() * screenRot - screenRot/2);
	ctx.translate(-xSize/2, -ySize/2);

	ctx.fillStyle = "black";
	ctx.globalAlpha = 1;

	ents.forEach((r) => {
		//ctx.fillText(r.appearance, r.iso.translation[0] * 10.0, r.iso.translation[1] * 10.0);
		ctx.drawImage(
			imgs[r.appearance],
			(r.iso.translation[0] - r.size[0]) * 10.0,
			(r.iso.translation[1] - r.size[1]) * 10.0,
			r.size[0] * 20.0,
			r.size[1] * 20.0,
		);

		ctx.fillStyle = "black";
		ctx.globalAlpha = 1;
	});

	ctx.restore();
}; 
