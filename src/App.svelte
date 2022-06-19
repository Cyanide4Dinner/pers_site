<script lang="ts">
	import Matrix from './Matrix.svelte';
	import Hoodie from './Hoodie.svelte';
	import Readme from './Readme.svelte';
	import Links	from './Links.svelte';

	function getLayout() {
		if(window.innerWidth < 1200) {
			return "layout-container-mobile";
		} else {
			return "layout-container-desktop";
		}
	}
</script>

<main id="main">
	<Matrix />
		<div class="{getLayout()}">
			{#if window.innerWidth <= 1200}
				<Links />
			{/if}
			<Hoodie />
			<Readme />
			{#if window.innerWidth > 1200}
				<Links />
			{/if}
		</div>
</main>


<style>
#main {
	height: 100%;
	overflow: hidden;
}

.layout-container-desktop {
	margin-top: 10vh;
	display: flex;
	justify-content: space-around;
}

.layout-container-mobile {
	display: flex;
	flex-direction: column;
	justify-content: space-around;
	background-color: blue;
	align-items: center;
	gap: 2vh;
}

</style>
