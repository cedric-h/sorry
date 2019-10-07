// remove padding from sides of site, to make room for canvas
document.body.style.margin = "0px";
document.body.style.padding = "0px";
document.body.style.overflow = "hidden";

let renderer = new THREE.WebGLRenderer( { antialias: true } );
renderer.setSize(window.innerWidth, window.innerHeight);

// camera setup
let camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.01, 100);
// this is used by the Rust part of things to read the cam movement.
// the movement is accumulated by JS event listeners, and when Rust reads it,
// it's cleared out. Rust calls the function that this closure returns.
let camMovement = (() => {
	let movementX = 0;
	let movementY = 0;

	// Hook pointer lock state change events for different browsers
	document.addEventListener('pointerlockchange', lockChangeAlert, false);
	document.addEventListener('mozpointerlockchange', lockChangeAlert, false);

	function lockChangeAlert() {
		if (document.pointerLockElement === renderer.domElement ||
			document.mozPointerLockElement === renderer.domElement) {
			console.log('The pointer lock status is now locked');
			document.addEventListener("mousemove", updatePosition, false);
		} else {
			console.log('The pointer lock status is now unlocked');  
			document.removeEventListener("mousemove", updatePosition, false);
		}
	}

	function updatePosition(e) {
		movementX += e.movementX;
		movementY += e.movementY;
	}

	// this is what's called when this var is called from Rust:
	return () => {
		let movement = [movementX, movementY];
		movementX = 0;
		movementY = 0;
		return movement;
	}
})();
// this grabs the mouse when the canvas is clicked.
renderer.domElement.onclick = function() {
  renderer.domElement.requestPointerLock();
};

// resize of camera
window.addEventListener('resize', () => {
	camera.aspect = window.innerWidth / window.innerHeight;
	camera.updateProjectionMatrix();
	renderer.setSize(window.innerWidth, window.innerHeight);
}, false);

// where all of the things to render are stored
let scene = new THREE.Scene();

// placeholder appearance values
let geometry = new THREE.BoxGeometry(0.2, 0.2, 0.2);
let material = new THREE.MeshNormalMaterial();

// map of appearance name -> mesh
let meshes = {};

// add renderer to doc
document.body.appendChild(renderer.domElement);

let cam_dir_vec = new THREE.Vector3(0.0, 0.0, 0.0);
function render({ents, camera_direction}) {
	ents.forEach(({ent, appearance, iso, camera_guide}) => {
		if (meshes[ent] == undefined) {
			let mesh = new THREE.Mesh(geometry, material);
			scene.add(mesh);
			meshes[ent] = mesh;
		}
		let mesh = camera_guide ? camera : meshes[ent];
		mesh.quaternion.fromArray(iso.rotation);
		mesh.position.fromArray(iso.translation);
	});

	camera.lookAt(cam_dir_vec.fromArray(camera_direction).add(camera.position));

	renderer.render(scene, camera); 
}
