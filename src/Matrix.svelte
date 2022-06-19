<div id="matrix-bg-div">
	<canvas id="matrix-bg-1" />
</div>

<style>
	#matrix-bg-1 {
		position: absolute;
		top: 0;
		left: 0;
	}

	#matrix-bg-1 {
		z-index: 2;
	}

	#matrix-bg-div {
		position: absolute;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
		background-color: black;
	}
</style>

<script>
	import init, { start } from 'matrix_rain_wasm';
	function initWasm() {
		var matrix_bg_1 = document.getElementById("matrix-bg-1");
		matrix_bg_1.width = window.innerWidth;
		matrix_bg_1.height = window.innerHeight;

		var hoodie = document.getElementById("hoodie-canvas");
		hoodie.width = 50 * 15;
		hoodie.height = 52 * 15;

		init().then(() => {
			console.log('init wasm-pack');
			start();
		 });
	}
	document.addEventListener('DOMContentLoaded', initWasm, false);
	window.onresize = initWasm;
</script>
