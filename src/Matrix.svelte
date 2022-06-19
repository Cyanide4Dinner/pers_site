<div id="matrix-bg-div">
	<canvas id="matrix-bg-1" />
</div>

<style>
	#matrix-bg-1 {
		position: fixed;
		top: 0;
		left: 0;
	}

	#matrix-bg-1 {
		z-index: 2;
	}

	#matrix-bg-div {
		position: fixed;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
		background-color: black;
	}
</style>

<script>
	import init, { start, store_value_in_wasm_mem } from 'matrix_rain_wasm';
	function initWasm() {
		var matrix_bg_1 = document.getElementById("matrix-bg-1");
		matrix_bg_1.width = window.innerWidth;
		matrix_bg_1.height = window.innerHeight;

		var hoodie = document.getElementById("hoodie-canvas");
		var portrait_block_size = 15;
		if(window.innerWidth >= 1200 && window.innerWidth < 1900) {
			portrait_block_size = 10;
		} else if(window.innerWidth >= 1900 && window.innerWidth < 2500) {
			portrait_block_size = 15;
		} else if (window.innerWidth >= 2500 && window.innerWidth < 3800) {
			portrait_block_size = 20;
		} else if (window.innerWidth >= 3800) {
			portrait_block_size = 30;
		}

		hoodie.width = 50 * portrait_block_size;
		hoodie.height = 52 * portrait_block_size;

		init().then(() => {
			console.log('init wasm-pack');
			start();
		 });
	}
	document.addEventListener('DOMContentLoaded', initWasm, false);
	function changeValue() {
		store_value_in_wasm_mem(1);
		initWasm();
	}
	window.onresize = changeValue;
</script>
