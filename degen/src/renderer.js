let camera = new THREE.PerspectiveCamera(70, window.innerWidth / window.innerHeight, 0.01, 10);
camera.position.z = 1;

let scene = new THREE.Scene();

let geometry = new THREE.BoxGeometry(0.2, 0.2, 0.2);
let material = new THREE.MeshNormalMaterial();

let meshes = {};

let renderer = new THREE.WebGLRenderer( { antialias: true } );
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

function render({ents}) {
	ents.forEach(({ent, appearance, iso}) => {
		if (meshes[ent] == undefined) {
			let mesh = new THREE.Mesh(geometry, material);
			scene.add(mesh);
			meshes[ent] = mesh;
		}
		let mesh = meshes[ent];
		mesh.quaternion.fromArray(iso.rotation);
		mesh.position.fromArray(iso.translation);
	});

	renderer.render(scene, camera); 
}
